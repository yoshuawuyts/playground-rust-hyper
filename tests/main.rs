extern crate hyper;

use hyper::Client;

#[test]
fn get_backend() {
  let client = Client::new();
  let res = client.get("http://localhost:3000/").send().unwrap();
  assert_eq!(res.status, hyper::Ok);
}
