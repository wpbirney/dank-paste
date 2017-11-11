#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]
#![feature(decl_macro)]

extern crate rocket;
extern crate rocket_contrib;
extern crate multipart;
extern crate rand;

#[macro_use]
extern crate serde_derive;

//#[macro_use]
extern crate serde_json;

mod paste_dog;
mod paste_id;
mod paste_info;
mod mpu;

use paste_id::PasteId;
use paste_info::PasteInfo;
use mpu::MultipartUpload;

use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::prelude::*;

use rocket::data::Data;
use rocket::response::NamedFile;

use rocket_contrib::Template;
use rocket_contrib::Json;

const VERSION: &'static str = "dank-paste v0.1.1";
const URL: &'static str = "https://ganja.ml";

fn main() {
    if !Path::new("upload").exists()    {
        fs::create_dir("upload").unwrap();
    }

    let (_handle, _tx) = paste_dog::launch();

    let r = routes![index, static_file, retrieve, retrieve_pretty, upload, upload_form];

    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", r).launch();
}

#[get("/")]
fn index() -> Option<NamedFile>  {
    NamedFile::open("static/index.html").ok()
}

#[get("/static/<path..>")]
fn static_file(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}

fn get_paste(id: String) -> Option<File> {
	let pid = match id.rfind('.')	{
		Some(idx) => id[..idx].to_string(),
		None => id
	};

	let p = PasteId::from_id(&pid)?;

	if Path::new(&p.del()).exists()  {
        return None
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
	id:	String
}

#[get("/h/<id>")]
fn retrieve_pretty(id: String) -> Option<Template> {
    let mut f = get_paste(id.clone())?;
    let mut buf = String::new();
    f.read_to_string(&mut buf).ok()?;
    Some(Template::render("pretty", PrettyCtx{ content: buf, version: VERSION.to_string(), id: id }))
}

#[derive(Serialize)]
struct UploadResponse {
	id: String,
	expire: u64,
	raw_url: String,
	source_url: String
}

#[post("/", data = "<paste>")]
fn upload(paste: Data, info: PasteInfo) -> Option<Json<UploadResponse>> {
	let id = PasteId::generate();
    paste.stream_to_file(Path::new(&id.filename())).unwrap();
    info.write_to_file(&format!("{}.{}", id.filename(), "json"));
    Some(Json(UploadResponse{
		id: id.id(),
		expire: info.expire,
		raw_url: id.url(),
		source_url: id.source_url()
	}))
}

#[post("/upload", data = "<paste>")]
fn upload_form(paste: MultipartUpload, info: PasteInfo) -> Option<Json<UploadResponse>> {
	let id = PasteId::generate();
    paste.write_to_file(&id.filename());
    info.write_to_file(&format!("{}.{}", id.filename(), "json"));
	Some(Json(UploadResponse{
		id: id.id(),
		expire: info.expire,
		raw_url: id.url(),
		source_url: id.source_url()
	}))
}
