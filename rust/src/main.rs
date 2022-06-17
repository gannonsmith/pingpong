use std::{/*env,*/ thread};
use std::io::prelude::*;
use std::net::{Shutdown, TcpListener, TcpStream, UdpSocket};

fn main() {
    //Read arguments
    //let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);

    //let protocol = &args[1];
    //let address = &args[2];

    let address = "127.0.0.1:8000";
    tcp_listener(address);

}

fn tcp_listener(address: &str) {
    let listener = TcpListener::bind(String::from(address)).unwrap();
    println!("Server listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    handle_tcp_connection(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}

fn handle_tcp_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut counter = 0;

   'reading_stream: while match stream.read(&mut buffer) {

       Ok(_size) => {
           let message = String::from_utf8_lossy(&buffer[..]);
           if message.contains("ping") {
               stream.write("pong\n".as_bytes()).unwrap();
           } else if message.contains("pong") {
               stream.write("ping\n".as_bytes()).unwrap();
           } else {
               stream.write("error\n".as_bytes()).unwrap();
           }
           counter += 1;
           if counter == 3 {
               println!("Three messages transmitted, closing client's stream.");
               stream.write("Three messages transmitted, closing client's stream.\n".as_bytes()).unwrap();
               match stream.shutdown(Shutdown::Both) {
                   Ok(_) => {
                       println!("Shutdown successful.");
                       break 'reading_stream;
                   },
                   Err(_) => {
                       println!("Shutdown unsuccessful.");
                   }
               }
           }
           true
       },
       Err(_) => {
           println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
           stream.shutdown(Shutdown::Both).unwrap();
           break 'reading_stream;
       }
   } {}
}