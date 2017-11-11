use rocket::request::{self,FromRequest,Request};
use rocket::Outcome;
use rocket::State;
use rocket::http::Status;

use std::time::Instant;
use std::collections::HashMap;
use std::sync::RwLock;

pub struct SenderInfo	{
	pub ip: String,
	pub time: Instant
}

impl SenderInfo {
	pub fn from_request(req: &Request) -> SenderInfo {
		let ip = match req.headers().get_one("real-ip")	{
			Some(x)	=> x.to_string(),
			None => req.remote().unwrap().ip().to_string()
		};
		SenderInfo{
			ip: ip,
			time: Instant::now()
		}
	}
}

pub struct Limiter {
	map: HashMap<String, Instant>
}

impl Limiter {
	pub fn new() -> Limiter {
		Limiter{ map: HashMap::new() }
	}

	pub fn create_state() -> RwLock<Limiter> {
		RwLock::new(Limiter::new())
	}

	fn remove(&mut self, keys: Vec<String>)	{
		for i in keys {
			self.map.remove(&i).unwrap();
		}
	}

	fn get_expired(&self) -> Vec<String>	{
		let mut expired = Vec::new();
		for (key,val) in self.map.iter() {
			if val.elapsed().as_secs() >= 10	{
				expired.push(key.clone());
			}
		}
		expired
	}

	fn handle_expired(&mut self)	{
		let x = self.get_expired();
		self.remove(x);
	}

	pub fn can_paste(&mut self, ip: &String) -> bool {
		self.handle_expired();
		match self.map.contains_key(ip)	{
			true => false,
			false => {
				self.map.insert(ip.clone(), Instant::now());
				true
			}
		}
	}
}

pub struct LimitGuard();

impl <'a,'r>FromRequest<'a,'r> for LimitGuard  {
	type Error = ();
	fn from_request(request: &'a Request<'r>) -> request::Outcome<LimitGuard, ()>  {
		let sender = SenderInfo::from_request(&request);
		let state = request.guard::<State<RwLock<Limiter>>>()?;

		let mut limiter = state.write().unwrap();

		match limiter.can_paste(&sender.ip)	{
			true => Outcome::Success(LimitGuard()),
			false => Outcome::Failure((Status::raw(429), ()))
		}
    }
}
