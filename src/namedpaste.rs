use std::fs::File;
use std::path::Path;

use rocket::request::Request;
use rocket::response::{self, Responder};
use rocket::http::ContentType;

use info::PasteInfo;

pub struct NamedPaste {
    name: Option<String>,
    file: File
}

impl NamedPaste {
    pub fn new(file: File, info: &PasteInfo) -> NamedPaste {
        NamedPaste {
            name: info.name.clone(),
            file: file,
        }
    }
}

impl<'r> Responder<'r> for NamedPaste {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        let mut response = self.file.respond_to(req)?;

        if let Some(n) = self.name {
            if let Some(ext) = Path::new(&n).extension() {
                if let Some(ct) = ContentType::from_extension(&ext.to_string_lossy()) {
                    response.set_header(ct);
                }
            }
            response.set_raw_header("Content-Disposition", format!("filename=\"{}\"", n));
        }

        Ok(response)
    }
}
