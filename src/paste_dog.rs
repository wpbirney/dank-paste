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

fn remove_old() {
    for path in fs::read_dir("upload").unwrap() {
        let path = path.unwrap();

        if path.file_name().into_string().unwrap() == "readme" {
            continue;
        }

        let meta = path.metadata().unwrap();
        let modified = meta.modified().unwrap();
        let hours = modified.elapsed().unwrap().as_secs() / 60 / 60;

        if hours >= 24 {
            fs::remove_file(path.path()).unwrap();
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
