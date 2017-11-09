use rand::{self,Rng};

use std::path::Path;

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

fn check_for_id(id: &str) -> bool {
	Path::new(&format!("upload/{}", id)).exists()
}

pub struct PasteId {
	id: String
}

impl PasteId {
	pub fn generate() -> PasteId {
		let mut len = 2;
		let mut tries = 0;
		let mut id = generate_id(len);
		while check_for_id(&id)	{
			if tries > 5 {
				len += 1;
				tries = 0;
			}
			id = generate_id(len);
			tries += 1;
		}
		PasteId{ id: id }
	}
	pub fn id(&self) -> String { self.id.clone() }
	pub fn filename(&self) -> String { format!("upload/{}", self.id) }
	pub fn url(&self) -> String { format!("{}/{}", ::URL, self.id) }
	pub fn source_url(&self) -> String { format!("{}/h/{}", ::URL, self.id) }
}
