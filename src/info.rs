use std::fs::File;

use rocket::request::{self, Request, FromRequest};
use rocket::{Data, Outcome::{Failure, Success}};
use rocket::data::{FromData, Outcome, Transform, Transformed};
use rocket::http::Status;

use std::io::Read;

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
    pub name: Option<String>
}

impl PasteInfo {
    pub fn new(secs: u64, name: Option<String>) -> PasteInfo {
        PasteInfo { expire: secs, name: name }
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
    pub name: Option<String>
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
        let name = request.headers().get_one("filename");

        Success(RequestInfo {
            expire: expire,
            host: host.to_string(),
            name: name.map(|x| x.to_string()),
        })
    }
}

pub struct UrlShortRequest {
    pub url: String
}

impl<'a> FromData<'a> for UrlShortRequest {
    type Error = ();
    type Owned = String;
    type Borrowed = str;

    fn transform(_: &Request, data: Data) -> Transform<Outcome<Self::Owned, Self::Error>> {
        let mut stream = data.open().take(2048);
        let mut string = String::with_capacity((2048 / 2) as usize);
        let outcome = match stream.read_to_string(&mut string) {
            Ok(_) => Success(string),
            Err(_) => Failure((Status::InternalServerError, ()))
        };

        Transform::Borrowed(outcome)
    }

    fn from_data(_: &Request, outcome: Transformed<'a, Self>) -> Outcome<Self, Self::Error> {
        let string = outcome.borrowed()?;

        Success(UrlShortRequest{ url: string.into() })
    }

}


