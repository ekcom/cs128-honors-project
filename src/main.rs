use apache_clone::server::*;

const STATIC_DIRECTORY: &str = "./public/";
fn main() {
    let mut server = Server::new(80);
    server.serve_static(STATIC_DIRECTORY);
}
