use std::io::prelude::*;
use std::fs::File;

use std::io::{Read, Cursor};
use rocket::data::{self, FromData};
use rocket::Data;
use rocket::Outcome;
use rocket::request::Request;
use multipart::server::Multipart;

pub struct MultipartUpload  {
	file: Vec<u8>,
}

impl MultipartUpload    {
	pub fn write_to_file(&self, path: &String)    {
		let mut f = File::create(&path).expect("failed to open file");
		f.write_all(&self.file).expect("failed to write file");
	}
}

impl FromData for MultipartUpload   {
	type Error = ();

	fn from_data(request: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
		// All of these errors should be reported
		let ct = request.headers().get_one("Content-Type").expect("no content-type");
		let idx = ct.find("boundary=").expect("no boundary");
		let boundary = &ct[(idx + "boundary=".len())..];

		let mut d = Vec::new();
		data.stream_to(&mut d).expect("Unable to read");

		let mut mp = Multipart::with_body(Cursor::new(d), boundary);

		// Custom implementation parts
		let mut file = None;

		mp.foreach_entry(|mut entry| {
			match entry.name.as_str() {
				"file" => {
					let mut d = Vec::new();
					let f = entry.data.as_file().expect("not file");
					f.read_to_end(&mut d).expect("cant read");
					file = Some(d);
				},
				other => panic!("No known key {}", other),
			}
		}).expect("Unable to iterate");

		let v = MultipartUpload {
			file: file.expect("file not set"),
		};

		// End custom

		Outcome::Success(v)
	}
}
