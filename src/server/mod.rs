// https://docs.rs/http/latest/http/
use http::{Request, Response};

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
        todo!(); // log to log file
    }
    pub fn serve_static(&mut self, path: &str) {
        self.log(&format!("Serving static files from directory {}", path), LogLevel::Info);
        todo!();
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
    fn create_server() {
        let server = Server::new(80);
        todo!();
    }
}
