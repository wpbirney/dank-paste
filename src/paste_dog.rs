/*
    spawns a thread that cycles over the uploaded files and deletes them,
    gives access to a JoinHandle
*/

use std::fs;
use std::path::Path;
use std::time::Duration;
use std::thread::{self, JoinHandle};

use paste_info::PasteInfo;
use id::{DankId,PasteId};

//maximum allowed age in seconds
pub const MAX_AGE: u64 = 259200;

//seconds to pause between cycles
const INTERVAL: u64 = 10;

//spawn the paste_dog thread
pub fn launch() -> JoinHandle<()> {
    let handle = thread::spawn(move || {
        paste_dog();
    });
    handle
}

//get the age of the file at path in seconds
fn get_age(path: &Path) -> Option<u64> {
	let modified = path.metadata().ok()?.modified().ok()?;
	Some(modified.elapsed().ok()?.as_secs())
}

//delete the paste at fp if expired
fn delete_if_expired(fp: &Path, paste: &PasteId)	{
	let age = get_age(&fp).unwrap();
	let info = PasteInfo::load(fp.to_str().unwrap());

	if info.expire == 0 {
		if age > MAX_AGE {
			paste.delete_all();
		}
	} else {
		if age > info.expire {
			paste.delete_all();
		}
	}
}

fn walk() {
	for path in fs::read_dir("upload").unwrap() {
		let path = path.unwrap();

		let fp = path.path();

		if let Some(ext) = fp.extension() {
			let paste = PasteId::from_id(fp.file_stem().unwrap().to_str().unwrap()).unwrap();
			if ext == "del" {
				paste.delete_all();
			} else if ext == "json" {
				delete_if_expired(&fp, &paste);
			}
		}
	}
}

fn paste_dog()  {
	loop {
		thread::sleep(Duration::from_secs(INTERVAL));
		walk();
	}
}
