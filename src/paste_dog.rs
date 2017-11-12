/*
    spawns a thread that cycles over the uploaded files and deletes them,
    gives access to a tx channel and JoinHandle
*/

use std::fs;
use std::path::Path;
use std::time::Duration;
use std::thread::{self, JoinHandle};

use paste_info::PasteInfo;
use paste_id::PasteId;

pub static MAX_AGE: u64 = 259200;

pub fn launch() -> JoinHandle<()> {
    let handle = thread::spawn(move || {
        paste_dog();
    });
    handle
}

fn get_age(path: &Path) -> Option<u64> {
	let modified = path.metadata().ok()?.modified().ok()?;
	Some(modified.elapsed().ok()?.as_secs())
}

fn remove_old() {
	for path in fs::read_dir("upload").unwrap() {
		let path = path.unwrap();

		let fp = path.path();

		if let Some(ext) = fp.extension() {
			let paste = PasteId::from_id(fp.file_stem().unwrap().to_str().unwrap()).unwrap();
			if ext == "del" {
				paste.delete_all();
			} else if ext == "json" {
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
		}
	}
}

fn paste_dog()  {
	loop {
		thread::sleep(Duration::from_secs(5));
		remove_old();
	}
}
