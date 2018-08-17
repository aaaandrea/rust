extern crate simple_server;

use simple_server::Server;

fn main() {
    // env_logger::init().unwrap();

    let host = "127.0.0.1";
    let port = "7878";

    let server = Server::new(|request, mut response| {
        println!("Request received. {} {}", request.method(), request.uri());
        Ok(response.body("Hello RustConf, Portland!".as_bytes().to_vec())?)
    });

    server.listen(host, port);
}
