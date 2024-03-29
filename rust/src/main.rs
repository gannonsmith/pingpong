use std::{env, thread};
use std::io::prelude::*;
use std::net::{Shutdown, TcpListener, TcpStream, UdpSocket};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Please enter: cargo run [protocol] [address:port]");
        return ();
    }
    println!("{:?}", args);

    let protocol = &args[1];
    let address = &args[2];

    if address.len() != 14 {
        println!("Please enter: cargo run [protocol] [address:port]");
        return ();
    }

    if protocol == "tcp" {
        tcp_listener(address)
    } else if protocol == "udp" {
        udp_listener(address)
    } else {
        println!("Please enter: cargo run [protocol] [address:port]");
        return ();
    }
}


fn udp_listener(address: &str) {
    let socket = UdpSocket::bind(address).expect("Binding failed...");
    println!("Socket binded to {}", address);

    let mut buffer: [u8; 1024] = [0; 1024];

    println!("Awaiting responses...");
    match socket.recv_from(&mut buffer) {
        Ok(new_addr) => {
            println!("Message received");
            let message = String::from_utf8_lossy(&buffer[..]);
            if message.contains("ping") {
                socket.send_to("pong\n".as_bytes(), new_addr.1).expect("Send failed...");
            } else if message.contains("pong") {
                socket.send_to("ping\n".as_bytes(), new_addr.1).expect("Send failed...");
            } else {
                socket.send_to("error\n".as_bytes(), new_addr.1).expect("Send failed...");
            }
        },
        Err(e) => {
            println!("Receiving error: {}", e);
        }
    }
}


fn tcp_listener(address: &str) {
    let listener = TcpListener::bind(address).expect("Binding Failed...");
    println!("Server listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().expect("Connection failed..."));
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
    let mut count = 0;

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
           count += 1;
           if count == 3 {
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