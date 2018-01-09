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
    pub expire: u64,
    pub host: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for RequestInfo {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<RequestInfo, ()> {
        let expire = match request.headers().get_one("expire") {
            Some(req_expire) => {
                let mut exp: u64 = req_expire.parse().unwrap();
                if exp > MAX_AGE {
                    exp = DEFAULT_AGE;
                }
                exp
            }
            None => DEFAULT_AGE,
        };

        let host = request.headers().get_one("Host").unwrap();

        Outcome::Success(RequestInfo {
            expire: expire,
            host: host.to_string(),
        })
    }
}
