mod client;

use std::env;
use std::net::{TcpListener, TcpStream};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 || (args[1] != "tcp" && args[1] != "udp") || args[2].len() != 14{
        println!("Enter files in format: cargo run [protocol] [ip]");
        return;
    }

    let protocol = &args[1];
    let ip = &args[2];
    println!("Using protocol {} on {}", protocol, ip);
    connect(ip);


}

fn connect(ip: &String) {
    let mut stream = TcpStream::connect(ip).expect("Connection failed...")?;
    let mut buf :[u8];
    stream.read(&mut buf);

}


