use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use rocket::State;
use rocket::http::Status;

use std::time::Instant;
use std::collections::HashMap;
use std::sync::RwLock;

pub struct SenderInfo {
    pub addr: String,
    pub time: Instant,
}

impl SenderInfo {
    pub fn from_request(req: &Request) -> SenderInfo {
        let addr = match req.headers().get_one("real-ip") {
            Some(x) => x.to_string(),
            None => req.remote().unwrap().ip().to_string(),
        };
        SenderInfo {
            addr: addr,
            time: Instant::now(),
        }
    }
}

const INTERVAL: u64 = 10;
const MAX_PER_INTERVAL: u64 = 5;

struct LimitAddr {
    start: Instant,
    count: u64,
}

impl LimitAddr {
    pub fn new() -> LimitAddr {
        LimitAddr {
            start: Instant::now(),
            count: 0,
        }
    }
    pub fn elapsed_secs(&self) -> u64 {
        self.start.elapsed().as_secs()
    }
    pub fn allow_post(&mut self) -> bool {
        if self.count >= MAX_PER_INTERVAL {
            false
        } else {
            self.count += 1;
            true
        }
    }
}

pub struct Limiter {
    map: HashMap<String, LimitAddr>,
}

impl Limiter {
    pub fn new() -> Limiter {
        Limiter { map: HashMap::new() }
    }

    pub fn create_state() -> RwLock<Limiter> {
        RwLock::new(Limiter::new())
    }

    fn remove(&mut self, keys: Vec<String>) {
        for i in keys {
            self.map.remove(&i).unwrap();
        }
    }

    fn get_expired(&self) -> Vec<String> {
        let mut expired = Vec::new();
        for (key, val) in self.map.iter() {
            if val.elapsed_secs() >= INTERVAL {
                expired.push(key.clone());
            }
        }
        expired
    }

    fn handle_expired(&mut self) {
        let x = self.get_expired();
        self.remove(x);
    }

    pub fn can_paste(&mut self, addr: &String) -> bool {
        self.handle_expired();
        match self.map.contains_key(addr) {
            true => self.map.get_mut(addr).unwrap().allow_post(),
            false => {
                self.map.insert(addr.clone(), LimitAddr::new());
                self.map.get_mut(addr).unwrap().allow_post()
            }
        }
    }
}

pub struct LimitGuard();

impl<'a, 'r> FromRequest<'a, 'r> for LimitGuard {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<LimitGuard, ()> {
        let sender = SenderInfo::from_request(&request);
        let state = request.guard::<State<RwLock<Limiter>>>()?;

        let mut limiter = state.write().unwrap();

        match limiter.can_paste(&sender.addr) {
            true => Outcome::Success(LimitGuard()),
            false => Outcome::Failure((Status::raw(429), ())),
        }
    }
}
