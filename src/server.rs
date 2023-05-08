use std::{path::PathBuf, fs::File};

use tiny_http::{Server, Response, Method, Header};

const CONTENT_TYPE_HTML:  &[u8] = b"Content-Type";


fn content_type_from_url(url: &str) -> Option<String> {
    let path = PathBuf::from(url);
    if let Some(extend) = path.extension() {
        if let Some(ext) = extend.to_str() {
            match ext {
               "html" => Some("text/html".to_string()),
               "js" => Some("text/javascript".to_string()),
               "css" => Some("text/css".to_string()),
               "png" => Some("image/png".to_string()),
               "jpg" | "jpeg" => Some("image/jpeg".to_string()),
               _ => None
            }
        } else {
            None
        }
    } else {
        None
    }
}

pub struct Serf {
    server: Server,
    dist: PathBuf,
    address: String
}

impl Serf {
    pub fn new(address: &str, dist: PathBuf) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let server: Server = Server::http(address)?; 
        Ok(Self {server, dist, address: address.to_string()})
    }

    pub fn serve(&mut self) {
        println!("INFO: Serving on {}", self.address);
        for request in self.server.incoming_requests() {
            match request.method() {
                Method::Get => {
                    println!("{}", request.url());
                    if request.url() == "/" {
                        let resp = match self.get_index() {
                            Ok(response) => response,
                            Err(err) => {
                                eprint!("ERROR: reading index.html -> {err}");
                                continue;
                            }
                        };
                        match request.respond(resp) {
                            Ok(_) => println!("responded with index.html"),
                            Err(err) => eprintln!("ERROR: could not build response: {err}")
                        };
                    } else {
                        let resp = match self.get_file(request.url()) {
                            Ok(response) => response,
                            Err(err) => {
                                eprint!("ERROR: reading file at {file} -> {err}", file = request.url());
                                continue;
                            }
                        };
                        match request.respond(resp) {
                            Ok(_) => println!("responded with file", ),
                            Err(err) => eprintln!("ERROR: could not build response: {err}")
                        };
                    }
                },
                _ => {
                    eprintln!("ERROR: Only 'GET' request implemented. Skipping request.");
                    continue;
                }
            }
        }
    }

    pub fn get_index(&self) -> Result<Response<File>, std::io::Error> {
        let mut p = self.dist.clone();
        p.push("index.html");
        let f = File::open(p)?;
        let mut resp = Response::from_file(f);
        resp.add_header(Header::from_bytes(CONTENT_TYPE_HTML, &b"text/html"[..]).unwrap());
        Ok(resp)
    }

    pub fn get_file(&self, url: &str) -> Result<Response<File>, std::io::Error> {
        let mut p = self.dist.clone();
        p.push(&url.to_string()[1..]);
        let f = File::open(p)?;
        let mut resp = Response::from_file(f);
        let mime = content_type_from_url(url).unwrap_or("text/html".to_string());
        resp.add_header(Header::from_bytes(CONTENT_TYPE_HTML, mime.as_bytes()).unwrap());
        Ok(resp)
    }
}


