#[macro_use]
extern crate serde_derive;
extern crate docopt;

use docopt::Docopt;
use std::io;

mod server;
mod client;

const USAGE: &'static str = "
simple tcp chat program

Usage:
  simple_chat [-s] (-i <ip> -p <port>)
  simple_chat (-h | --help)

Options:
  -h --help                  Show this screen.
  -s --server                Run Server mode.
  -i <ip>, --ip <ip>         Set IP address.
  -p <port>, --port <port>   Set port.
";

#[derive(Debug, Deserialize)]
struct Args {
	flag_ip: String,
	flag_port: String,
	flag_server: bool
}

fn main() {
  let args: Args = Docopt::new(USAGE)
							.and_then(|d| d.deserialize())
							.unwrap_or_else(|e| e.exit());
	if args.flag_server {
		match server::server(&(args.flag_ip+":"+&args.flag_port)){
        Ok(_) => return,
        Err(e) =>println!("{:?}", e)
    }
		println!("yeee");
	}else{
		client::client(&(args.flag_ip+":"+&args.flag_port));
		println!("aaa");
	}
}