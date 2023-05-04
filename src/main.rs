use ::apache_clone::server::*;

const STATIC_DIRECTORY: &str = "./public/";
fn main() {
    // start up a server and serve the files!
    let mut server = Server::new(5000);
    server.set_log_level(LogLevel::Debug);
    server.serve_static(STATIC_DIRECTORY);
}
