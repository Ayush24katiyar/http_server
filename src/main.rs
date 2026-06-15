use std::{
     io::Read, net::{TcpListener, TcpStream}
};

fn read_lines (request : &str)  {
    let mut lines = request.lines();
    let request_line = lines.next();
    match request_line {
        Some(request_line) => {
            let mut parts = request_line.split(' ');
            let method = parts.next();
            let path = parts.next();
            let  version = parts.next();
            match (method , path , version) {
                (Some(method), Some(path) , Some(version)) => {
                    println!("Method : {}", method);
                    println!("Path : {}", path);
                    println!("Version : {}", version);
                }

                
                _ => println!("we found a error"),
            }
        },
        None => println!("the issue is beyond our hold"),
    }

for line in lines {
    println!("{}",line)
}



    
}
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer);
    match n {
        Ok(n) => {
            // println!("the print value is : {:?}", &buffer[0..n]);

            let request = String::from_utf8(buffer[0..n].to_vec());
            match request {
                Ok(request) => {
                    read_lines(&request)
                },
                Err(e) => println!("the error is due to : {e}"),
            }
        }
        Err(e) => println!("error due to some issue listed below : {e}"),
    }
}
fn main() {
    let connect = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("listening on 127.0.0.1:8080");

    for stream in connect.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => println!("connection failed : {e}"),
        }
    }
}
