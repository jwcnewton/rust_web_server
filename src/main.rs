use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::error::Error;

pub mod reqlib;

static INDEX_PAGE:&'static str = "./src/pages/index.html";
static NOT_FOUND_PAGE:&'static str = "./src/pages/404.html";

fn main() {
    server_on();
}

fn server_on(){
   let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
   
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let request_string = String::from_utf8_lossy(&buffer[..]);
    let request = reqlib::str_to_request(request_string.to_string());
    if request.uri == "/" {
        handle_index(stream, request);
    } else {
        handle_not_found(stream, request);
    }
}

fn handle_not_found(mut stream: TcpStream, reqest:reqlib::HttpReq){
     if reqest.res_type.starts_with("text/html") {
        let index_page_content = get_page_content(NOT_FOUND_PAGE);
        let res = format!("HTTP/1.1 200 Ok \r\n\r\n {}", index_page_content);
        stream.write(&res.as_bytes()).unwrap(); 
        stream.flush().unwrap();
    } else {
        let index_page_content = get_page_resource(reqest.uri);
        if index_page_content.len() == 0 {
            let res = format!("HTTP/1.1 404 \r\n\r\n {}", index_page_content);
            stream.write(&res.as_bytes()).unwrap(); 
            stream.flush().unwrap();
        } else {
            let res = format!("HTTP/1.1 200 Ok \r\n\r\n {}", index_page_content);
            stream.write(&res.as_bytes()).unwrap(); 
            stream.flush().unwrap();
        }
    }
}

fn handle_index(mut stream: TcpStream, reqest: reqlib::HttpReq){
    if reqest.res_type.starts_with("text/html") {
        let index_page_content = get_page_content(INDEX_PAGE);
        let res = format!("HTTP/1.1 200 Ok \r\n\r\n {}", index_page_content);
        stream.write(&res.as_bytes()).unwrap(); 
        stream.flush().unwrap();
    } else {
        println!("{}", &reqest.uri.clone());
        let index_page_content = get_page_resource(reqest.uri);
        let res = format!("HTTP/1.1 200 Ok \r\n\r\n {}", index_page_content);
        stream.write(&res.as_bytes()).unwrap(); 
        stream.flush().unwrap();
    }
}

fn get_page_content(page_ref: &'static str)->String {
    let mut f = File::open(page_ref).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    return contents;
}

fn get_page_resource(page_ref: String)->String {
    let mut file = match File::open(format!(".{}",page_ref)) {
        Err(_) => (return "".to_string()),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(_) => s = "".to_string(),
        Ok(_) => (),
    }
    return s;
}