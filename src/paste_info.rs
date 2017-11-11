use std::fs::File;

use rocket::request::{self, Request, FromRequest};
use rocket::Outcome;

use serde_json;

use paste_dog::MAX_AGE;

#[derive(Serialize, Deserialize)]
pub struct PasteInfo    {
    pub expire: u64
}

impl PasteInfo  {
    pub fn new(secs: u64) -> PasteInfo   {
        PasteInfo{expire: secs}
    }

    pub fn load(path: &str) -> PasteInfo {
        let f = File::open(path).unwrap();
        serde_json::from_reader(f).unwrap()
    }

    pub fn write_to_file(&self, path: &str)   {
        let f = File::create(path).unwrap();
        serde_json::to_writer(f, &self).unwrap();
    }
}

impl <'a,'r>FromRequest<'a,'r> for PasteInfo  {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<PasteInfo, ()>  {
		let mut age: u64 = MAX_AGE;
		if let Some(ex) = request.headers().get_one("expire")   {
			age = ex.parse().unwrap();
		}
		Outcome::Success(PasteInfo::new(age))
    }
}
