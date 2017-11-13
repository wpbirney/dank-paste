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
mod id;
mod paste_info;
mod mpu;
mod limiting;

use id::{DankId,PasteId,UrlId};
use paste_info::{PasteInfo, UrlInfo, HostInfo};
use mpu::MultipartUpload;
use limiting::*;

use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::prelude::*;

use rocket::data::Data;
use rocket::response::{NamedFile, Redirect};

use rocket_contrib::Template;
use rocket_contrib::Json;

const VERSION: &'static str = "dank-paste v0.1.2";

pub fn proto() -> String {
	if cfg!(feature = "debug") {
		return "http".to_string()
	} else	{ "https".to_string() }
}

fn init_dir(path: &str)	{
	if !Path::new(path).exists()    {
		fs::create_dir(path).unwrap();
	}
}

fn initialize() {
	init_dir("upload");
	init_dir("shorts");
}

fn main() {
	initialize();

	let _handle = paste_dog::launch();

	let r = routes![index, static_file, retrieve, retrieve_pretty,
					upload, upload_form, create_url, redirect_short];

	rocket::ignite()
		.attach(Template::fairing())
		.manage(Limiter::create_state())
		.mount("/", r).launch();
}

#[derive(Serialize)]
struct IndexCtx {
	version: String
}

#[get("/")]
fn index() -> Template  {
	Template::render("index", IndexCtx{ version: VERSION.to_string() })
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
fn retrieve_pretty(id: String, host: HostInfo) -> Result<Template, Option<Redirect>> {
	if let Some(mut f) = get_paste(id.clone()) {
		let mut buf = String::new();
		return match f.read_to_string(&mut buf) {
			Ok(_) => Ok(Template::render("pretty", PrettyCtx{ content: buf, version: VERSION.to_string(), id: id })),
			Err(_) => Err(Some(Redirect::to(&format!("{}/{}", host.host, id))))
		}
	}
	Err(None)
}

#[derive(Serialize)]
struct UploadResponse {
	id: String,
	expire: u64,
	raw_url: String,
	source_url: String
}

#[post("/", data = "<paste>")]
fn upload(paste: Data, info: PasteInfo, host: HostInfo, _limit: LimitGuard) -> Option<Json<UploadResponse>> {
	let id = PasteId::generate();
	paste.stream_to_file(Path::new(&id.filename())).unwrap();
	info.write_to_file(&format!("{}.{}", id.filename(), "json"));
	Some(Json(UploadResponse{
		id: id.id(),
		expire: info.expire,
		raw_url: id.url(&host.host),
		source_url: id.source_url(&host.host)
	}))
}

#[post("/upload", data = "<paste>")]
fn upload_form(paste: MultipartUpload, info: PasteInfo, host: HostInfo, _limit: LimitGuard) -> Option<Json<UploadResponse>> {
	let id = PasteId::generate();
	paste.write_to_file(&id.filename());
	info.write_to_file(&format!("{}.{}", id.filename(), "json"));
	Some(Json(UploadResponse{
		id: id.id(),
		expire: info.expire,
		raw_url: id.url(&host.host),
		source_url: id.source_url(&host.host)
	}))
}

#[post("/shorty", data = "<url>")]
fn create_url(url: Json<UrlInfo>, host: HostInfo) -> String {
	let id = UrlId::generate();
	let info = url.into_inner();
	info.write_to_file(&id.filename());
	id.url(&host.host)
}

#[get("/s/<id>")]
fn redirect_short(id: String) -> Redirect {
	let i = UrlId::from_id(&id).unwrap();
	let info = UrlInfo::load(&i.filename());
	Redirect::to(&info.target)
}
