use std::fs::File;

use rocket::request::{self, Request, FromRequest};
use rocket::Outcome;

use serde_json;

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
        if let Some(ex) = request.headers().get_one("expire")   {
            Outcome::Success(PasteInfo::new(ex.parse().unwrap()))
        } else {
            Outcome::Success(PasteInfo::new(48))
        }
    }
}

pub struct PastePath(String);

impl PastePath {
	pub fn new(id: String) -> PastePath {
		PastePath(id)
	}

	pub fn data(&self) -> String { format!("upload/{}", &self.0) }
	pub fn json(&self) -> String { format!("upload/{}.json", &self.0) }
	pub fn del(&self) -> String { format!("upload/{}.del", &self.0) }
}
