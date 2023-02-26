use super::server::Handler;
use super::http::{Request, Response, StatusCode, ParseError, Method};
use std::fs;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        WebsiteHandler { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    None
                }
            }
            Err(_) => None
        }

    }
}

impl Handler for WebsiteHandler {
    fn handleRequest(&mut self, request: &Request) -> Response {

        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(
                    StatusCode::Ok, 
                    self.read_file("index.html")
                ),
                "/another-page" => Response::new(
                    StatusCode::Ok, 
                    self.read_file("another-page.html")
                ),
                path => match self.read_file(path) {
                    Some(content) => Response::new(
                        StatusCode::Ok, 
                        Some(content),
                    ),
                    None => Response::new(
                        StatusCode::NotFound, 
                        self.read_file("404.html")
                    )
                }
            }
            _ => Response::new(
                    StatusCode::NotFound, 
                    None
                )
        }
    }
}