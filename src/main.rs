#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]
#![feature(decl_macro)]

extern crate rocket;
extern crate multipart;
extern crate rand;

mod paste_dog;
mod paste_id;
mod mpu;

use std::io;
use std::path::{Path, PathBuf};
use std::fs::{self, File};

use rocket::data::Data;
use rocket::response::NamedFile;

use mpu::MultipartUpload;

fn main() {
    if !Path::new("upload").exists()    {
        fs::create_dir("upload").unwrap();
    }

    let (_handle, _tx) = paste_dog::launch();

    let r = routes![index, static_file, retrieve, upload, upload_form];

    rocket::ignite().mount("/", r).launch();
}

#[get("/")]
fn index() -> Option<NamedFile>  {
    NamedFile::open("static/index.html").ok()
}

#[get("/static/<path..>")]
fn static_file(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}

#[get("/<id>")]
fn retrieve(id: String) -> Option<File> {
	let me: String = id.chars().take(3).collect();
    let filename = format!("upload/{}", me);
    File::open(&filename).ok()
}

#[post("/", data = "<paste>")]
fn upload(paste: Data) -> io::Result<String> {
    let (filename, url) = paste_id::generate();
    paste.stream_to_file(Path::new(&filename))?;
    Ok(format!("{}\n", url))
}

#[post("/upload", data = "<paste>")]
fn upload_form(paste: MultipartUpload) -> Option<String> {
    let (filename, url) = paste_id::generate();
    paste.write_to_file(filename);
    Some(format!("{}", url))
}
