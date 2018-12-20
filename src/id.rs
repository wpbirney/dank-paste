use rand::{self, Rng};

use std::path::Path;
use std::fs;

/// Table to retrieve base62 values from.
const BASE62: &'static [u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

pub fn generate_id(size: usize) -> String {
    let mut id = String::with_capacity(size);
    let mut rng = rand::thread_rng();
    for _ in 0..size {
        id.push(BASE62[rng.gen::<usize>() % 62] as char);
    }
    id
}

fn check_for_id(root: &str, id: &str) -> bool {
    Path::new(&format!("{}/{}", root, id)).exists()
}

fn generate_unused(root: &str) -> String {
    let mut len = 2;
    let mut tries = 0;
    let mut id = generate_id(len);
    while check_for_id(&root, &id) {
        if tries > 5 {
            len += 1;
            tries = 0;
        }
        id = generate_id(len);
        tries += 1;
    }
    id
}

pub trait DankId
where
    Self: Sized,
{
    fn generate() -> Self;
    fn from_id(id: &str) -> Option<Self>;
    fn id(&self) -> String;
    fn filename(&self) -> String;
    fn json(&self) -> String;
    fn del(&self) -> String;
    fn delete_all(&self);
}

#[derive(DankId)]
#[Path = "upload"]
pub struct PasteId {
    id: String,
}

impl PasteId {
    //paste specifics
    pub fn url(&self, host: &str) -> String {
        format!("{}://{}/{}", ::proto(), host, self.id)
    }
    pub fn source_url(&self, host: &str) -> String {
        format!("{}://{}/h/{}", ::proto(), host, self.id)
    }
}

#[derive(DankId)]
#[Path = "shorts"]
pub struct UrlId {
    id: String,
}

impl UrlId {
    pub fn url(&self) -> String {
        format!("/s/{}", self.id)
    }
}
