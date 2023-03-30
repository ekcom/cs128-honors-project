// https://docs.rs/http/latest/http/
use http::{Request, Response};
use std::path::{PathBuf};
use std::fs;
use std::sync::mpsc;

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
        self.log(&format!("Serving static files from directory {}", path), LogLevel::Info);
        match self.get_file_cache(path) {
            Ok(paths) => {
                // todo multithread
                // todo serve file
                for path in paths {
                    println!("{:?}", path);
                }
            },
            Err(e) => self.log(&format!("{}", e), LogLevel::Critical),
        }
    }
    /// Returns a listing of the files in path as a vector
    /// or returns a `ServerError` if there was an error reading any of the files
    fn get_file_cache(&mut self, path: &str) -> Result<Vec<PathBuf>, ServerError> {
        let file_paths = fs::read_dir(path);
        match file_paths {
            Ok(file_paths) => {
                let mut paths = Vec::new();
                for path in file_paths {
                    match path {
                        Ok(entry) => paths.push(entry.path()),
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
}
