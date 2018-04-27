use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

const RES_200:[u8; 19] = [72, 84, 84, 80, 47, 49, 46, 49, 32, 50, 48, 48, 32, 79, 75, 13, 10, 13, 10];

fn main() {
    server_on();
}

fn server_on(){
   let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
   
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    stream.write(&RES_200).unwrap(); 
    stream.flush().unwrap();
}