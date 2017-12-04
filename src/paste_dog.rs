/*
    spawns a thread that cycles over the uploaded files and deletes them,
    gives access to a JoinHandle
*/

use std::fs;
use std::path::Path;
use std::time::Duration;
use std::thread::{self, JoinHandle};

use info::{PasteInfo, UrlInfo};
use id::{DankId, PasteId, UrlId};

//maximum allowed age in seconds
pub const MAX_AGE: u64 = 259200;

//seconds to pause between cycles
const INTERVAL: u64 = 10;

//spawn the paste_dog thread
pub fn launch() -> JoinHandle<()> {
    let handle = thread::spawn(move || { paste_dog(); });
    handle
}

//get the age of the file at path in seconds
fn get_age(path: &Path) -> Option<u64> {
    let modified = path.metadata().ok()?.modified().ok()?;
    Some(modified.elapsed().ok()?.as_secs())
}

macro_rules! delete_if_expired {
	($fp:expr, $id:expr, $info:tt) => {
		let age = get_age($fp).unwrap();
		let info = $info::load($fp.to_str().unwrap());

		if info.expire == 0 {
			if age > MAX_AGE {
				$id.delete_all();
			}
		} else {
			if age > info.expire {
				$id.delete_all();
			}
		}
	};
}

fn walk_paste() {
    for path in fs::read_dir("upload").unwrap() {
        let fp = path.unwrap().path();

        if let Some(ext) = fp.extension() {
            let paste = PasteId::from_id(fp.file_stem().unwrap().to_str().unwrap()).unwrap();
            if ext == "del" {
                paste.delete_all();
            } else if ext == "json" {
                delete_if_expired!(&fp, paste, PasteInfo);
            }
        }
    }
}

fn walk_url() {
    for path in fs::read_dir("shorts").unwrap() {
        let fp = path.unwrap().path();
        let url = UrlId::from_id(fp.file_name().unwrap().to_str().unwrap()).unwrap();
        delete_if_expired!(&fp, url, UrlInfo);
    }
}

fn paste_dog() {
    loop {
        thread::sleep(Duration::from_secs(INTERVAL));
        walk_paste();
        walk_url();
    }
}
