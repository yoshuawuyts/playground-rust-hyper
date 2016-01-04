#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde_json;
extern crate hyper;
extern crate serde;

use hyper::server::Request;
use hyper::server::Response;
use hyper::Server;
use serde::json;

#[derive(Serialize, Deserialize)]
struct Greeting {
  msg: String
}

// create a server
pub fn create_server (port: &str) {
  println!("Server started on port {}", port);
  Server::http("127.0.0.1:1337").unwrap().handle(router);
}

// route incoming requests
fn router (_: Request, res: Response) {
  res.send(b"Hello World!").unwrap();
}
