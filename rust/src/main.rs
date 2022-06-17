use std::{env, thread};
use std::io::prelude::*;
use std::net::{Shutdown, TcpListener, TcpStream};

fn main() {
    //Read arguments
    //let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);

    //let protocol = &args[1];
    //let address = &args[2];

    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("Server listening on port 8000");

    for stream in listener.incoming() {
        println!("i");
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

   while match stream.read(&mut buffer) {
       Ok(size) => {
           println!("{}::{}", String::from_utf8_lossy(&buffer[..]).replace("\n", ""), String::from("ping"));
           if String::from_utf8_lossy(&buffer[..]).replace("\n", "") == String::from("ping") {
               stream.write("pong".as_bytes()).unwrap();
           } else if String::from_utf8_lossy(&buffer[..]).replace("\n", "") == String::from("pong") {
               stream.write("ping".as_bytes()).unwrap();
           } else {
               stream.write("error".as_bytes()).unwrap();
           }
           true
       },
       Err(_) => {
           println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
           stream.shutdown(Shutdown::Both).unwrap();
           false
       }
   } {}
}