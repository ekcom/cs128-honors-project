use http::{Request, Response};
// https://docs.rs/http/latest/http/
use std::path::{PathBuf};
use std::fs;
//use std::sync::mpsc;
use std::thread;
use std::net::{TcpListener, TcpStream}; // https://doc.rust-lang.org/std/net/struct.TcpListener.html
use std::io::BufReader;
use std::io::prelude::*;

//use apache_clone::ServerError;
mod error;
use crate::server::error::{ServerError, ServerErrorType};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    // due to implicit discriminants,
    // the log levels increase numerically
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

pub struct Server {
    port: usize,
    log_level: LogLevel,
}

impl Server {
    pub fn new(port: usize) -> Server {
        Server {
            port,
            log_level: LogLevel::Error,
        }
    }
    pub fn set_log_level(&mut self, lvl: LogLevel) {
        self.log_level = lvl;
    }
    //fn log<T>(&self, msg: T, lvl: LogLevel) {
    fn log(&self, msg: &str, lvl: LogLevel) {
        if lvl >= self.log_level {
            println!("{}", msg);
        }
        // todo log to log file
    }
    pub fn serve_static(&mut self, path: &str) {
        // todo return a join handle?
        self.log(&format!("Serving static files from directory {}", path), LogLevel::Info);
        match self.open_listener() {
            Ok(listener) => {
                let path_cpy: String = path.into(); // make String for better lifetime
                let be_a_server = move || {
                    for mut stream_res in listener.incoming() {
                        match stream_res {
                            Ok(stream) => {
                                let serve = handle_static_request(stream, &path_cpy);
                                if serve.is_err() {
                                    //self.log(&format!("{}", serve.unwrap_err()), LogLevel::Error);
                                    // todo: mpsc send error
                                }
                            }
                            Err(_) => {
                                //self.log(&format!("{}", ServerError::new(ServerErrorType::ConnFail)), LogLevel::Error),
                                // todo: mpsc send error
                            }
                        }
                    }
                };
                // todo multithread
                let thread_handle = thread::spawn(be_a_server);
                // keep running while handles are alive
                thread_handle.join().unwrap();
            },
            Err(e) => self.log(&format!("{}", e), LogLevel::Critical),
        }
    }
    fn open_listener(&self) -> Result<TcpListener, ServerError> {
        self.log(&format!("Opened listener on http://localhost:{}", self.port), LogLevel::Info);
        let listener = TcpListener::bind(format!("localhost:{}",self.port));
        match listener {
            Ok(listener) => Ok(listener),
            Err(_) => Err(ServerError::new(ServerErrorType::BadPort)),
        }
    }
}
/// Returns a listing of the files in path as a vector
/// or returns a `ServerError` if there was an error reading any of the files
fn get_files(serve_path: &str) -> Result<Vec<PathBuf>, ServerError> {
    let file_paths = fs::read_dir(serve_path);
    match file_paths {
        Ok(file_paths) => {
            let mut paths = Vec::new();
            for path in file_paths {
                match path {
                    Ok(entry) => {
                        // todo drop the ./public part idiomatically
                        // and return the full path
                        // (string slices are hard)
                        //let path_string: String = entry.path().to_str().unwrap().into();
                        //paths.clone_from_slice(&String::from(entry.path().to_str().unwrap())[serve_path.len()..]);
                        /*let s = String::from(entry.path().to_str().unwrap());
                        let mut s2 = String::new();
                        let _ = &s[serve_path.len()-1..].clone_into(&mut s2);
                        paths.push(s2);*/
                        paths.push(entry.path());
                    },
                    Err(_) => return Err(ServerError::new(ServerErrorType::ReadFail)),
                };
            }
            Ok(paths)
        },
        Err(_) => {
            Err(ServerError::new(ServerErrorType::BadDirectory))
        }
    }
}
// todo make these parameters more consise (e.g. better path useage)
fn handle_static_request(mut stream: TcpStream, serve_path: &str) -> Result<(), ServerError> {
    match get_files(serve_path) {
        Ok(paths) => {
            let buf_reader = BufReader::new(&mut stream);
            let mut lines = buf_reader.lines();
            let first_line = lines.next();
            if  first_line.is_none() {
                let error_400_file_path: PathBuf = PathBuf::from("./public/400.html");
                if let Err(_) = send_file(&mut stream, ServeDetails { path: error_400_file_path, code: 400 }) { // 400 bad request
                    return Err(ServerError::new(ServerErrorType::ReadFail));
                }
                return Ok(());
            }
            let first_line = first_line.unwrap().unwrap();
            let mut requested_resource = parse_requested_path(&first_line);
            // special serves:
            if requested_resource == "/" {
                requested_resource = "/index.html";
            }
            // todo handle the rest of the lines

            for path in paths {
                // todo better way to check this
                // todo better way to get public directory
                if path.to_str().unwrap() == format!("./public{}", requested_resource) {
                    if let Err(_) = send_file(&mut stream, ServeDetails { path, code: 200 }) {
                        return Err(ServerError::new(ServerErrorType::ReadFail));
                    }
                    return Ok(());
                }
            }
            // still here? not found
            // todo programmatically find error_404_file_path
            let error_404_file_path: PathBuf = PathBuf::from("./public/404.html");
            if let Err(_) = send_file(&mut stream, ServeDetails { path: error_404_file_path, code: 404 }) {
                return Err(ServerError::new(ServerErrorType::ReadFail));
            }
            // todo report not found to server

            Ok(())
        }
        Err(e) => Err(e),
    }
}
/// Parses the requested path
/// Only works for HTTP GET
fn parse_requested_path(http_line: &str) -> &str {
    /*let r = Regex::new("GET [\\s]* HTTP/1.1").unwrap();
    &String::from(r.captures(http_line).unwrap()[0]).clone()*/
    const prefix: &str = "GET ";
    const suffix: &str = " HTTP/1.1";
    &http_line[prefix.len()..http_line.len()-suffix.len()]
}
// todo validate status_code and use http library
struct ServeDetails {
    path: PathBuf,
    code: i32,
}
fn send_file(stream: &mut TcpStream, serve_details: ServeDetails) -> Result<(), std::io::Error> { 
    //let full_path = std::path::Path::new(&format!("{}{}", serve_path, path));
    let data_to_send = fs::read_to_string(&serve_details.path).unwrap();
    let res = format!("HTTP/1.1 {} {}\nContent-Length: {}{}\n\n{}",
        serve_details.code,
        error::http_response_from_code(serve_details.code),
        data_to_send.len(),
        get_mime_type_header(serve_details.path.extension().unwrap().to_str().unwrap()),
        data_to_send);
    stream.write_all(res.as_bytes())
}

fn get_mime_type_header(extension: &str) -> String {
    let content_type = match extension {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "text/javascript",
        _ => "" // unknown
    };
    if content_type == "" {
        "".into() // return empty
    } else {
        format!("\nContent-Type: {}", content_type)
    }
}

/*
fn response(req: Request<()>) -> http::Result<Response<()>> {
    match req.uri().path() {
        "/" => index(req),
        "/foo" => foo(req),
        "/bar" => bar(req),
        _ => not_found(req),
    }
}
use http::{HeaderValue, Response, StatusCode};
use http::header::CONTENT_TYPE;

fn add_server_headers<T>(response: &mut Response<T>) {
    response.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("text/html"));
    *response.status_mut() = StatusCode::OK;
}
*/

#[cfg(test)]
mod test {
    use crate::server::*;
    
    #[test]
    fn serve_static() {
        let mut server = Server::new(80);
        server.serve_static("../../public/");
    }
    #[test]
    fn parse_requested_path() {
        assert_eq!(crate::server::parse_requested_path("GET /index.html HTTP/1.1"), "/index.html");
    }
}
