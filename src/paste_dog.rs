/*
    spawns a thread that cycles over the uploaded files and deletes them,
    gives access to a JoinHandle
*/

use std::fs;
use std::path::Path;
use std::time::Duration;
use std::thread::{self, JoinHandle};

use info::{PasteInfo, UrlInfo, DankInfo};
use id::{DankId, PasteId, UrlId};

//maximum allowed age in seconds
pub const MAX_AGE: u64 = 259200;

//this should probably be moved? DEFAULT_AGE is not refrenced in paste_dog.rs
pub const DEFAULT_AGE: u64 = 86400;

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

//delete id if info expire time is passed
fn delete_if_expired<T: DankInfo, I: DankId>(info: T, id: I) {
    let age = get_age(Path::new(&id.filename())).unwrap();
    if info.expire() == 0 {
        if age > MAX_AGE {
            id.delete_all();
        }
    } else {
        if age > info.expire() {
            id.delete_all();
        }
    }
}

fn walk_paste() {
    for path in fs::read_dir("upload").unwrap() {
        let fp = path.unwrap().path();

        if let Some(ext) = fp.extension() {
            let paste = PasteId::from_id(fp.file_stem().unwrap().to_str().unwrap()).unwrap();
            if ext == "del" {
                paste.delete_all();
            } else if ext == "json" {
                delete_if_expired(PasteInfo::load(fp.to_str().unwrap()), paste);
            }
        }
    }
}

fn walk_url() {
    for path in fs::read_dir("shorts").unwrap() {
        let fp = path.unwrap().path();
        let url = UrlId::from_id(fp.file_name().unwrap().to_str().unwrap()).unwrap();
        delete_if_expired(UrlInfo::load(fp.to_str().unwrap()), url);
    }
}

fn paste_dog() {
    loop {
        thread::sleep(Duration::from_secs(INTERVAL));
        walk_paste();
        walk_url();
    }
}
