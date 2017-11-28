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
mod info;
mod mpu;
mod limiting;

use id::{DankId,PasteId,UrlId};
use info::{PasteInfo, UrlInfo, HostInfo};
use mpu::MultipartUpload;
use limiting::*;

use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::prelude::*;
use std::env::args;

use rocket::data::Data;
use rocket::response::{NamedFile, Redirect};
use rocket::Request;

use rocket_contrib::Template;
use rocket_contrib::Json;

const VERSION: &'static str = "dank-paste v0.2.2";
const LOG_PRE: &'static str = "[dank]: ";

pub fn proto() -> String {
	match args().nth(1) {
		Some(s) => {
			if s == "http" {
				return "http".to_string()
			}
			"https".to_string()
		},
		None => "https".to_string()
	}
}

pub fn dank_log(msg: &str)	{
	println!("{} {}", LOG_PRE, msg);
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
		.catch(errors![not_found])
		.mount("/", r).launch();
}

#[derive(Serialize)]
struct IndexCtx {
	version: String
}

#[get("/")]
fn index() -> Template  {
	Template::render("pastebin", IndexCtx{ version: VERSION.to_string() })
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
	id:	String,
	url: String,
	pretty: String
}

impl PrettyCtx {
	pub fn new(id: PasteId, content: String, host: HostInfo) -> PrettyCtx {
		PrettyCtx{
			content: content,
			version: VERSION.to_string(),
			id: id.id(),
			url: id.url(&host.host),
			pretty: id.source_url(&host.host)
		}
	}
}

#[get("/h/<id>")]
fn retrieve_pretty(id: String, host: HostInfo) -> Result<Template, Option<Redirect>> {
	if let Some(mut f) = get_paste(id.clone()) {
		let mut buf = String::new();
		let i = PasteId::from_id(&id).unwrap();
		return match f.read_to_string(&mut buf) {
			Ok(_) => Ok(Template::render("pretty", PrettyCtx::new(i, buf, host))),
			Err(_) => Err(Some(Redirect::to(&i.url(&host.host))))
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
fn create_url(url: String, info: PasteInfo, host: HostInfo, _limit: LimitGuard) -> String {
	let id = UrlId::generate();
	let info = UrlInfo{ expire: info.expire, target: url };
	info.write_to_file(&id.filename());
	id.url(&host.host)
}

#[get("/s/<id>")]
fn redirect_short(id: String) -> Option<Redirect> {
	let i = UrlId::from_id(&id)?;
	let info = UrlInfo::load(&i.filename());
	Some(Redirect::to(&info.target))
}

#[derive(Serialize)]
struct NotFoundCtx {
	request: String
}

#[error(404)]
fn not_found(req: &Request) -> Template {
	Template::render("404", NotFoundCtx{
		request: req.uri().to_string()
	})
}
