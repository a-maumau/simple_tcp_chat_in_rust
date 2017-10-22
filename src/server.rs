use std::net::{TcpListener};
use std::thread;
use std::io::{Read, Write};
use std::io;

fn server(open_addr : &str) -> io::Result<()> {
    let lis = try!(TcpListener::bind(open_addr));
    println!("started on {:?}", lis);
    for stream in lis.incoming() {
        let mut stream = try!(stream);
        println!("{:?} connected.", try!(stream.peer_addr()));
        let _: thread::JoinHandle<io::Result<()>> = thread::spawn(move || {
            loop {
                let mut b = [0; 1024];
                let n = try!(stream.read(&mut b));
                if n == 0 {
                    println!("{:?} closed.", try!(stream.peer_addr()));
                    return Ok(());
                }else {
                    println!("{}", String::from_utf8_lossy(&b));
                    //try!(stream.write(&b[0..n]));
                }
            }
        });
    }
    Ok(())
}