use std::fs::File;

use rocket::request::{self, Request, FromRequest};
use rocket::Outcome;

use serde_json;

use paste_dog::{MAX_AGE, DEFAULT_AGE};

macro_rules! load_write {
    ($t:tt) => {
        pub fn load(path: &str) -> $t {
            let f = File::open(path).unwrap();
            serde_json::from_reader(f).unwrap()
        }

        pub fn write_to_file(&self, path: &str)   {
            let f = File::create(path).unwrap();
            serde_json::to_writer(f, &self).unwrap();
        }
    };
}

#[derive(Serialize, Deserialize)]
pub struct PasteInfo {
    pub expire: u64,
}

impl PasteInfo {
    pub fn new(secs: u64) -> PasteInfo {
        PasteInfo { expire: secs }
    }
    load_write!(Self);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UrlInfo {
    pub expire: u64,
    pub target: String,
}

impl UrlInfo {
    pub fn new(expire: u64, target: String) -> UrlInfo {
        UrlInfo {
            expire: expire,
            target: target,
        }
    }
    load_write!(Self);
}

pub struct RequestInfo {
    pub expire: Option<u64>,
    pub host: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for RequestInfo {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<RequestInfo, ()> {
        let expire = match request.headers().get_one("expire") {
            Some(v) => {
                let mut a: u64 = v.parse().unwrap();
                if a > MAX_AGE {
                    a = DEFAULT_AGE;
                }
                Some(a)
            }
            None => None,
        };

        let host = request.headers().get_one("Host").unwrap();

        Outcome::Success(RequestInfo {
            expire: expire,
            host: host.to_string(),
        })
    }
}
