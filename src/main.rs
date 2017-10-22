#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]
#![feature(decl_macro)]

extern crate rocket;
extern crate multipart;
extern crate rand;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

mod paste_dog;
mod paste_id;
mod mpu;

use std::io::{self, Write, Read};
use std::path::{Path, PathBuf};
use std::fs::{self, File};

use rocket::data::Data;
use rocket::response::NamedFile;

use mpu::MultipartUpload;

#[derive(Serialize, Deserialize)]
pub struct PasteInfo    {
    expire: u64
}

impl PasteInfo  {
    pub fn new(hours: u64) -> Option<PasteInfo>   {
        if hours <= 48  {
            Some(PasteInfo{expire: hours})
        } else { None }
    }

    pub fn write_to_file(&self, path: &String)   {
        let mut f = File::create(path).unwrap();
        f.write_all(serde_json::to_string_pretty(&self).unwrap().as_bytes()).unwrap();
    }
}

#[derive(Serialize, Deserialize)]
pub struct AjaxPaste {
    data: Vec<u8>,
    expire: u64
}

fn main() {
    if !Path::new("upload").exists()    {
        fs::create_dir("upload").unwrap();
    }

    let mut buf: Vec<u8> = vec![0; 1024*1024*128];
    let i = File::open("file").unwrap().read_to_end(&mut buf).unwrap();
    println!("file loaded");
    let p = AjaxPaste{
        data: buf[..i].to_vec(),
        expire: 24
    };

    let f = File::create("fuck.json").unwrap();
    serde_json::to_writer(f, &p).unwrap();

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
    paste.write_to_file(&filename);
    let info = PasteInfo::new(paste.expire).unwrap();
    info.write_to_file(&format!("{}.{}", &filename, "json"));
    Some(format!("{}", url))
}
