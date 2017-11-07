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

use paste_info::{PasteInfo, PastePath};
use mpu::MultipartUpload;

use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::prelude::*;

use rocket::data::Data;
use rocket::response::NamedFile;

use rocket_contrib::Template;

static VERSION: &str = "dank-paste v0.1.0";

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
	let p = PastePath::new(id.chars().take(3).collect());

	if Path::new(&p.del()).exists()  {
        return None
	}

    if Path::new(&p.json()).exists() {
        let info = PasteInfo::load(&p.json());
        if info.expire == 0 {
            File::create(&p.del()).unwrap();
        }
    }

    File::open(p.data()).ok()
}

#[get("/<id>")]
fn retrieve(id: String) -> Option<File> {
	get_paste(id)
}

#[derive(Serialize)]
struct PrettyCtx {
    content: String,
	version: String
}

#[get("/h/<id>")]
fn retrieve_pretty(id: String) -> Option<Template> {
    let mut f = get_paste(id)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf).ok()?;
    Some(Template::render("pretty", PrettyCtx{ content: buf, version: VERSION.to_string() }))
}


#[post("/", data = "<paste>")]
fn upload(paste: Data, info: PasteInfo) -> Option<String> {
    let (filename, url) = paste_id::generate();
    paste.stream_to_file(Path::new(&filename)).unwrap();
    info.write_to_file(&format!("{}.{}", &filename, "json"));
    Some(format!("{}\n", url))
}

#[post("/upload", data = "<paste>")]
fn upload_form(paste: MultipartUpload, info: PasteInfo) -> Option<String> {
    let (filename, url) = paste_id::generate();
    paste.write_to_file(&filename);
    info.write_to_file(&format!("{}.{}", &filename, "json"));
    Some(format!("{}", url))
}
