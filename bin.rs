extern crate getopts;
extern crate server;

use server::create_server;
use getopts::Options;
use std::env;

fn main () {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut opts = Options::new();
  opts.optopt("p", "port", "set output file name", "PORT");
  opts.optflag("h", "help", "print this help menu");

  let matches = match opts.parse(&args[1..]) {
    Ok(m) => { m }
    Err(f) => { panic!(f.to_string()) }
  };

  if matches.opt_present("h") {
    usage(&program, opts);
    return;
  }

  let port = "1337";

  create_server(port)
}

// print usage
fn usage(program: &str, opts: Options) {
  let brief = format!("Usage: {} FILE [options]", program);
  print!("{}", opts.usage(&brief));
}
