//use http::{Request, Response};
use std::path::{PathBuf, Path};
use std::fs::{self, File};
//use std::sync::mpsc;
use std::thread;
use std::net::{TcpListener, TcpStream}; // https://doc.rust-lang.org/std/net/struct.TcpListener.html
use std::io::BufReader;
use std::io::prelude::*;

//use apache_clone::ServerError;
mod error;
use crate::server::error::{ServerError, ServerErrorType};
use error_file_system::ErrorFileSystem;

/// The level of verbosity to write logs with
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    // due to implicit discriminants,
    // the log levels increase numerically
    // (that is, they are ordered)
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}
/// Returns the LogLevel passed in `&str` format
fn get_readable_loglevel(ll: &LogLevel) -> &'static str {
    match ll {
        LogLevel::Debug => "Debug",
        LogLevel::Info => "Info",
        LogLevel::Warning => "Warning",
        LogLevel::Error => "Error",
        LogLevel::Critical => "Critical",
    }
}

/// A static server, which stores relevant configuration information
pub struct Server {
    port: usize,
    log_level: LogLevel,
    log_file: File,
    error_files: ErrorFileSystem,
}

impl Server {
    /// Creates a new server on port `port`
    /// Note: does not start the server upon construction
    pub fn new(port: usize) -> Server {
        let log_file_path = Path::new("logs/server.log");
        if !log_file_path.exists() {
            fs::create_dir_all("logs").unwrap();
            File::create(log_file_path).unwrap();
        }
        let log_file = File::options().append(true).open(log_file_path).unwrap();
        Server {
            port,
            log_level: LogLevel::Error,
            log_file,
            error_files: ErrorFileSystem::new(),
        }
    }
    /// Adjusts the minimum log level of which to console logs to
    pub fn set_log_level(&mut self, lvl: LogLevel) {
        self.log_level = lvl;
    }
    /// Writes a log message to the console if above the log level
    /// And writes a log message to the log file
    //fn log<T>(&self, msg: T, lvl: LogLevel) {
    fn log(&mut self, msg: &str, lvl: LogLevel) {
        if lvl >= self.log_level {
            println!("[{}] {}", get_readable_loglevel(&lvl), msg);
        }
        if let Err(_) = writeln!(self.log_file, "[{}] {}", get_readable_loglevel(&lvl), msg) {
            println!("[Error] Failed to write to log file");
        }
    }
    /// Starts a static server, serving files from `path`.
    /// Will handle any network requests by serving them a file from that relative path,
    /// or a 404 page if not found.
    pub fn serve_static(&mut self, path: &str) {
        // todo return a join handle?
        self.log(&format!("Serving static files from directory {}", path), LogLevel::Info);
        match self.open_listener() {
            Ok(listener) => {
                let path_cpy: String = path.into(); // make String for better lifetime
                let be_a_server = move || {
                    for stream_res in listener.incoming() {
                        match stream_res {
                            Ok(stream) => {
                                // how to pass self?
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
    /// Opens a TCP listener on a port
    fn open_listener(&mut self) -> Result<TcpListener, ServerError> {
        self.log(&format!("Opened listener on http://localhost:{}", self.port), LogLevel::Info);
        let listener = TcpListener::bind(format!("localhost:{}",self.port));
        match listener {
            Ok(listener) => Ok(listener),
            Err(_) => Err(ServerError::new(ServerErrorType::BadPort)),
        }
    }
    /// Adds a custom error file to be served in the static server.
    /// Whenever an error of HTTP code `code` is thrown, the page at
    /// `file_path` will be served to the client.
    pub fn add_custom_error_file(&mut self, code: i32, file_path: &str) {
        self.error_files.add_error_file(code, PathBuf::from(file_path));
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
/// Handles a request to the server
/// by serving a file based on the stream's request.
/// 
/// Parses HTTP network requests to deduce the requested resource
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
            let error_404_file_path: PathBuf = PathBuf::from("./public/404.html"); //self.error_files.get_error_file_for(404)
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
/// Only works for HTTP 1.1 GET requests.
fn parse_requested_path(http_line: &str) -> &str {
    /*let r = Regex::new("GET [\\s]* HTTP/1.1").unwrap();
    &String::from(r.captures(http_line).unwrap()[0]).clone()*/
    const PREFIX: &str = "GET ";
    const SUFFIX: &str = " HTTP/1.1";
    &http_line[PREFIX.len()..http_line.len()-SUFFIX.len()]
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

/// Returns the required MIME type to serve
/// based on the file extension of the resource.
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

mod error_file_system {
    use std::{path::PathBuf, collections::HashMap};
    
    pub struct ErrorFileSystem {
        custom_paths: HashMap<i32, PathBuf>,
    }

    impl ErrorFileSystem {
        pub fn new() -> ErrorFileSystem {
            Self {
                custom_paths: HashMap::new(),
            }
        }
        pub fn add_error_file(&mut self, code: i32, file: PathBuf) {
            self.custom_paths.insert(code, file);
        }
        fn get_error_file_for(&self, status_code: i32) -> PathBuf {
            if let Some(file) = self.custom_paths.get(&status_code) {
                return file.clone();
            }
            get_default_error_file_for(status_code)
        }
    }
    fn get_default_error_file_for(status_code: i32) -> PathBuf {
        PathBuf::from(match status_code {
            404 => "./public/404.html",
            400 => "./public/400.html",
            500 => "./public/500.html",
            _ => "./public/500.html", // unknown
        })
    }
}

#[cfg(test)]
mod test {
    use crate::server::*;
    
    #[test]
    fn serve_static() {
        println!("No error should be thrown opening serve file and log file");
        let mut server = Server::new(80);
        server.serve_static("../../public/");
    }
    #[test]
    fn parse_requested_path() {
        assert_eq!(crate::server::parse_requested_path("GET /index.html HTTP/1.1"), "/index.html");
    }
}
