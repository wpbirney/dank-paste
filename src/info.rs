use std::fs::File;

use rocket::request::{self, Request, FromRequest};
use rocket::Outcome;

use serde_json;

use paste_dog::{MAX_AGE, DEFAULT_AGE};

pub trait DankInfo {
    fn load(path: &str) -> Self;
    fn write_to_file(&self, path: &str);
    fn expire(&self) -> u64;
}

#[derive(Serialize, Deserialize, DankInfo)]
pub struct PasteInfo {
    pub expire: u64,
}

impl PasteInfo {
    pub fn new(secs: u64) -> PasteInfo {
        PasteInfo { expire: secs }
    }
}

#[derive(Serialize, Deserialize, Debug, DankInfo)]
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
