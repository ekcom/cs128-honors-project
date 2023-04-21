use ::apache_clone::server::*;

const STATIC_DIRECTORY: &str = "./public/";
fn main() {
    let mut server = Server::new(5000);
    server.set_log_level(LogLevel::Debug);
    server.serve_static(STATIC_DIRECTORY);
}
