#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]
#![feature(decl_macro)]

extern crate multipart;
extern crate rand;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

//#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate dank_codegen;

mod paste_dog;
mod id;
mod info;
mod mpu;
mod limiting;

use id::{DankId, PasteId, UrlId};
use info::{PasteInfo, UrlInfo, RequestInfo, DankInfo};
use mpu::MultipartUpload;
use limiting::*;
use paste_dog::DEFAULT_AGE;

use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::prelude::*;
use std::env::args;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use rocket::data::Data;
use rocket::response::{NamedFile, Redirect};
use rocket::Request;
use rocket::State;

use rocket_contrib::Template;
use rocket_contrib::Json;

const VERSION: &'static str = "dank-paste v0.2.4";

pub fn proto() -> String {
    match args().nth(1) {
        Some(s) => {
            if s == "http" {
                return "http".to_string();
            }
            "https".to_string()
        }
        None => "https".to_string(),
    }
}

fn init_dir(path: &str) {
    if !Path::new(path).exists() {
        fs::create_dir(path).unwrap();
    }
}

//creates if needed, the required directories used by dank-paste
fn initialize() {
    init_dir("upload");
    init_dir("shorts");
}

//returns the total number of paste+short urls stored by dank-paste
fn count_paste() -> usize {
    let mut paste = 0;
    for path in fs::read_dir("upload").unwrap() {
        match path.unwrap().path().extension() {
            Some(_) => continue,
            None => paste += 1,
        }
    }
    let url = fs::read_dir("shorts").unwrap().count();
    paste + url
}

pub struct PasteCounter {
    pub count: AtomicUsize,
}

fn main() {
    //create ./upload and ./shorts if needed
    initialize();

    //initialize a PasteCounter based on the count_paste() result
    let counter = PasteCounter {
        count: AtomicUsize::new(count_paste()),
    };
    let a_counter = Arc::new(counter);

    //launch our paste watchdog thread
    let _handle = paste_dog::launch(a_counter.clone());

    let r = routes![
        index,
        static_file,
        retrieve,
        retrieve_pretty,
        upload,
        upload_form,
        create_url,
        redirect_short,
        get_count,
    ];

    rocket::ignite()
        .attach(Template::fairing())
        .manage(Limiter::create_state())
        .manage(a_counter)
        .catch(catchers![not_found])
        .mount("/", r)
        .launch();
}

#[derive(Serialize)]
struct IndexCtx {
    version: String,
    paste_count: usize,
}

#[get("/")]
fn index(paste_count: State<Arc<PasteCounter>>) -> Template {
    Template::render(
        "pastebin",
        IndexCtx {
            version: VERSION.to_string(),
            paste_count: paste_count.count.load(Ordering::Relaxed),
        },
    )
}

#[get("/static/<path..>")]
fn static_file(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}

fn get_paste(id: String) -> Option<File> {
    let pid = match id.rfind('.') {
        Some(idx) => id[..idx].to_string(),
        None => id,
    };

    let p = PasteId::from_id(&pid)?;

    if Path::new(&p.del()).exists() {
        return None;
    }

    if Path::new(&p.json()).exists() {
        let info = PasteInfo::load(&p.json());
        if info.expire == 0 {
            File::create(&p.del()).ok()?;
        }
    }

    File::open(p.filename()).ok()
}

#[get("/<id>")]
fn retrieve(id: String) -> Option<File> {
    get_paste(id.clone())
}

#[derive(Serialize)]
struct PrettyCtx {
    content: String,
    version: String,
    id: String,
    url: String,
    pretty: String,
}

impl PrettyCtx {
    pub fn new(id: PasteId, content: String, host: RequestInfo) -> PrettyCtx {
        PrettyCtx {
            content: content,
            version: VERSION.to_string(),
            id: id.id(),
            url: id.url(&host.host),
            pretty: id.source_url(&host.host),
        }
    }
}

#[get("/h/<id>")]
fn retrieve_pretty(id: String, host: RequestInfo) -> Result<Template, Option<Redirect>> {
    if let Some(mut f) = get_paste(id.clone()) {
        let mut buf = String::new();
        let i = PasteId::from_id(&id).unwrap();
        return match f.read_to_string(&mut buf) {
            Ok(_) => Ok(Template::render("pretty", PrettyCtx::new(i, buf, host))),
            Err(_) => Err(Some(Redirect::to(i.url(&host.host)))),
        };
    }
    Err(None)
}

/*
    UploadResponse is the response we send back after a succesful pastebin
    derives Serialize to be returned as json
*/
#[derive(Serialize)]
struct UploadResponse {
    id: String,
    expire: u64,
    raw_url: String,
    source_url: String,
}

impl UploadResponse {
    fn new(id: PasteId, info: PasteInfo, host: &str) -> UploadResponse {
        UploadResponse {
            id: id.id(),
            expire: info.expire,
            raw_url: id.url(host),
            source_url: id.source_url(host),
        }
    }
}

#[post("/", data = "<paste>")]
fn upload(
    paste: Data,
    info: RequestInfo,
    paste_count: State<Arc<PasteCounter>>,
    _limit: LimitGuard,
) -> Option<Json<UploadResponse>> {
    let id = PasteId::generate();
    let pinfo = PasteInfo::new(info.expire?);
    paste.stream_to_file(Path::new(&id.filename())).unwrap();
    pinfo.write_to_file(&id.json());
    paste_count.count.fetch_add(1, Ordering::Relaxed);
    Some(Json(UploadResponse::new(id, pinfo, &info.host)))
}

#[post("/upload", data = "<paste>")]
fn upload_form(
    paste: MultipartUpload,
    info: RequestInfo,
    paste_count: State<Arc<PasteCounter>>,
    _limit: LimitGuard,
) -> Option<Json<UploadResponse>> {
    let id = PasteId::generate();
    let pinfo = PasteInfo::new(info.expire?);
    paste.write_to_file(&id.filename());
    pinfo.write_to_file(&id.json());
    paste_count.count.fetch_add(1, Ordering::Relaxed);
    Some(Json(UploadResponse::new(id, pinfo, &info.host)))
}

/*
    the create_url route handles new short url creation
    PasteInfo request guard is used here soley to get the expire header
*/
#[post("/shorty", data = "<url>")]
fn create_url(
    url: String,
    info: RequestInfo,
    paste_count: State<Arc<PasteCounter>>,
    _limit: LimitGuard,
) -> String {
    let id = UrlId::generate();
    UrlInfo::new(info.expire.unwrap_or(DEFAULT_AGE), url).write_to_file(&id.filename());
    paste_count.count.fetch_add(1, Ordering::Relaxed);
    id.url(&info.host)
}

//simple enough to read... just redirects to the requested id's expanded url
#[get("/s/<id>")]
fn redirect_short(id: String) -> Option<Redirect> {
    let i = UrlId::from_id(&id)?;
    let info = UrlInfo::load(&i.filename());
    Some(Redirect::to(info.target))
}

//get_count provides a simple way for ajax request to get the paste count
#[get("/get/count")]
fn get_count(paste_count: State<Arc<PasteCounter>>) -> String {
    paste_count.count.load(Ordering::Relaxed).to_string()
}

/*
    Error Handlers
*/

#[derive(Serialize)]
struct NotFoundCtx {
    request: String,
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    Template::render(
        "404",
        NotFoundCtx {
            request: req.uri().to_string(),
        },
    )
}
