use std::io::prelude::*;
use std::net::TcpStream;
use std::io;

pub fn client(server_addr : &str){
    if let Ok(mut stream) = TcpStream::connect(server_addr){
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
                Err(_) => {
                    println!("### could not send ###\npipe broken.");
                    break;
                }
            }
        }
    }else{
        println!("Could not find the server.");
    }
}