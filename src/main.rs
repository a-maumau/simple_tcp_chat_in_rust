#[macro_use]
extern crate serde_derive;
extern crate docopt;

use docopt::Docopt;
use std::io;

const USAGE: &'static str = "
simple tcp chat program

Usage:
  simple_chat [-s | -c]
  simple_chat (-s | -c) [-i <ip>] [-p <port>]
  simple_chat (-h | --help)

Options:
  -h --help       Show this screen.
  -s --server     Run Server mode.
  -c --client     Run client mode.
  -i <ip>, --ip <ip>       Set IP address.
  -p <port>, --port <port>   Set port.
";

#[derive(Debug, Deserialize)]
struct Args {
	flag_ip: String,
	flag_port: String,
	flag_server: bool,
	flag_client: bool
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.deserialize())
                            .unwrap_or_else(|e| e.exit());

    if !args.flag_server && !args.flag_client {
    	let mut mode = String::new();
    	
    	print!("Choose Mode [server or client, or quit]:");
    	loop{
    		io::stdin().read_line(&mut mode).expect("error");
    		match  &*(mode.trim()) {
    		    "s" | "server" => break,
    		    "c" | "client" => break,
    		    "q" | "quit"   => return,
    		    _ => {}
    		}
    	}
    }
    println!("{:?}", args);
}