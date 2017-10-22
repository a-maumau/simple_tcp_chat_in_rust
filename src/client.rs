use std::io::prelude::*;
use std::net::TcpStream;
use std::io;

pub fn client(server_addr : &str) -> io::Result<()>{
    let server = TcpStream::connect(server_addr);
    let mut stream = server.unwrap();
    println!("Connected.");
    print!(">");
    io::stdout().flush();
    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        // trim() for erasing '\n'
        match stream.write(input.trim().as_bytes()){
            Ok(_) => {
                print!(">");
                io::stdout().flush();
            }
            Err(e) => {
                println!("### could not send ###\npipe broken.");
                return Err(e);
            }
        }
    }
}