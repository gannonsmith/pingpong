use std::env;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    //Read arguments
    //let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);

    //let protocol = &args[1];
    //let address = &args[2];

    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection Success!");
        //handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    if String::from_utf8_lossy(&buffer[..]) == "ping" {
        //write pong
    }
    if String::from_utf8_lossy(&buffer[..]) == "pong" {
        //write ping
    }

    //let response = "HTTP/1.1 200 OK\r\n\r\n";

    //stream.write(response.as_bytes()).unwrap();
   // stream.flush().unwrap();
}