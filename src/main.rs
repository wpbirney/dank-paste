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

use std::io::{self, Write, Read, Cursor};
use std::path::{Path, PathBuf};
use std::fs::{self, File};

use rocket::data::Data;
use rocket::response::{NamedFile, Stream};
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

use mpu::MultipartUpload;

#[derive(Serialize, Deserialize)]
pub struct PasteInfo    {
    expire: u64
}

impl PasteInfo  {
    pub fn new(hours: u64) -> PasteInfo   {
        if hours <= 48  {
            PasteInfo{expire: hours}
        } else {
            PasteInfo{expire: 48}
        }
    }

    pub fn load(path: &str) -> PasteInfo {
        let f = File::open(path).unwrap();
        serde_json::from_reader(f).unwrap()
    }

    pub fn write_to_file(&self, path: &str)   {
        let f = File::create(path).unwrap();
        serde_json::to_writer(f, &self).unwrap();
    }
}

impl <'a,'r>FromRequest<'a,'r> for PasteInfo  {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<PasteInfo, ()>  {
        if let Some(ex) = request.headers().get_one("expire")   {
            Outcome::Success(PasteInfo::new(ex.parse().unwrap()))
        } else {
            Outcome::Success(PasteInfo::new(48))
        }
    }
}

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
fn retrieve(id: String) -> Stream<Cursor<Vec<u8>>> {
	let me: String = id.chars().take(3).collect();
    let filename = format!("upload/{}", me);
    let jsonpath = format!("upload/{}.json", me);

    let file_size = Path::new(&filename).metadata().unwrap().len();
    let mut buf: Vec<u8> = vec![0; file_size as usize];
    File::open(filename).unwrap().read_to_end(&mut buf).unwrap();

    if let Ok(file) = File::open(jsonpath)  {
        let info: PasteInfo = serde_json::from_reader(file).unwrap();
        if info.expire == 0  {
            File::create(format!("upload/{}.delete", me)).unwrap();
        }
    }
    Stream::from(Cursor::new(buf))
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
