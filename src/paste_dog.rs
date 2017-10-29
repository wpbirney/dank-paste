/*
    spawns a thread that cycles over the uploaded files and deletes them,
    gives access to a tx channel and JoinHandle
*/

use std::fs;
use std::time::Duration;
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{channel, Receiver, Sender};

pub fn launch() -> (JoinHandle<()>, Sender<u8>)  {
    let (tx, rx) = channel::<u8>();
    let handle = thread::spawn(move || {
        paste_dog(rx);
    });
    (handle, tx)
}

fn del_paste(paste: &str)   {
    println!("deleting paste {}", paste);
    fs::remove_file(format!("upload/{}", paste)).unwrap_or(());
    fs::remove_file(format!("upload/{}.json", paste)).unwrap_or(());
    fs::remove_file(format!("upload/{}.del", paste)).unwrap_or(());
}

fn remove_old() {
    for path in fs::read_dir("upload").unwrap() {
        let path = path.unwrap();

        if path.file_name().into_string().unwrap() == "readme" {
            continue;
        }

        let fp = path.path();
        if let Some(ext) = fp.extension() {
            if ext == "del" {
                del_paste(fp.file_stem().unwrap().to_str().unwrap());
            } else if ext == "json" {
                let meta = path.metadata().unwrap();
                let modified = meta.modified().unwrap();
                let age = modified.elapsed().unwrap().as_secs();
                let info = ::PasteInfo::load(fp.to_str().unwrap());

                if info.expire == 0 {
                    if age > 259200 {
                        del_paste(fp.file_stem().unwrap().to_str().unwrap());
                    }
                } else {
                    if age > info.expire {
                        del_paste(fp.file_stem().unwrap().to_str().unwrap());
                    }
                }
            }
        }
    }
}

fn paste_dog(rx: Receiver<u8>)  {
    loop {
        match rx.recv_timeout(Duration::from_secs(5))   {
            Ok(val) => {
                if val == 111   {
                    break;
                }
            }
            Err(_) => thread::sleep(Duration::from_secs(5))
        }
        remove_old();
    }
}
