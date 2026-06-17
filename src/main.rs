use std::{
    collections::HashMap, io::Read, net::{TcpListener, TcpStream}
};


#[derive(Debug)]
struct Request {
    method: String,
    path: String,
    version: String,
    header: HashMap<String,String>,

}

fn read_lines (request : &str) -> Option<Request> {
    let mut lines = request.lines();
    let mut random = HashMap::new();
    let request_line = lines.next();
    let mut meth = String::new();
    let mut pat= String::new();
    let mut  ver= String::new();

    match request_line {
        Some(request_line) => {
            let mut parts = request_line.split(' ');
            let method = parts.next();
            let path = parts.next();
            let  version = parts.next();
            match (method , path , version) {
                (Some(method), Some(path) , Some(version)) => {
                    meth = method.to_string();
                    pat = path.to_string();
                    ver = version.to_string();                    
                    
                    
                    // println!("Method : {}", method);
                    // println!("Path : {}", path);
                    // println!("Version : {}", version);
                }

                
                _ => return None,
            }
        },
        None => return None,
    }

for line in lines {
    let mut xyz = line.split(':');
    let host = xyz.next();
    let value = xyz.next();
    match (host , value) {
        (Some(host) ,Some(value)) => {
            random.insert(host.to_string(), value.trim().to_string());
            
        }
        _ =>  return None
    }

}

// println!("{:?}",random); 

let request1 = Request {
    method: meth,
    path: pat,
    version: ver,
    header: random  ,    
};

// println!("{:#?}",request1);

Some(request1) 
    
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
                    match  read_lines(&request) {
                        Some(request) => println!("{:#?}",request),
                        None => println!("error"),
                    }
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
