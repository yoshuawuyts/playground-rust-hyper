extern crate hyper;

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;

// create a server
pub fn create_server (port: &str) {
  println!("Server started on port {}", port);
  Server::http("127.0.0.1:3000").unwrap().handle(router);
}

// route incoming requests
fn router (_: Request, res: Response) {
  res.send(b"Hello World!").unwrap();
}
