use rand::{self,Rng};

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

pub fn generate() -> (String, String)  {
    let id = generate_id(3);
    let filename = format!("upload/{}", id);
    let url = format!("https://ganja.ml/{}", id);
    (filename, url)
}
