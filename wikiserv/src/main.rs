mod content;
mod parser;
mod paths;
mod template;
mod token;

use std::{
    env,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use paths::Paths;

fn main() {
    let wikipath = match env::var("WIKIPATH") {
        Ok(path) => path,
        Err(_)   => {
            println!("WIKIPATH environment variable is undefined.");
            return;
        }
    };

    let paths = match paths::Paths::new(wikipath) {
        Ok(paths) => paths,
        Err(_)    => {
            println!("Encountered an unrecoverable error whilst indexing wikipath");
            return;
        }
    };

    let listener = match TcpListener::bind("127.0.0.1:6789") {
        Ok(listener) => listener,
        Err(_)       => {
            println!("An error occurred while binding the TcpListener");
            return;
        }
    };

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(_)     => {
                println!("Error occurred while making TCP connection. Readying again.");
                continue;
            }
        };

        handle_connection(stream, &paths);
    }
}

fn handle_connection(mut stream: TcpStream, paths: &Paths) -> () {
    let bufreader = BufReader::new(&mut stream);
    let mut http_request: Vec<String> = Vec::new();
    for line in bufreader.lines() {
        let requestpart = match line {
            Ok(part) => part,
            Err(_)   => {
                println!("Error occurred during parsing of HTTP request. Dropping connection.");
                return;
            }
        };
        if requestpart == "" {
            break;
        }
        http_request.push(requestpart);
    }

    let requestpath: &str = match match http_request.get(0) {
        Some(first_line) => first_line,
        None             => {
            println!("Malformed request. Dropping connection.");
            return;
        }
    }
        .split_whitespace().collect::<Vec<&str>>().get(1) {
        Some(path) => path,
        None       => {
            println!("Malformed request. Dropping connection.");
            return;
        },
    };

    let response = request_response(paths, requestpath);

    match stream.write_all(response.as_bytes()) {
        Ok(_) => (),
        Err(_) => {
            println!("Something went wrong while responding to HTTP request.");
            return;
        }
    }
}

fn request_response(paths: &Paths, requestpath: &str) -> String {
    let status = "HTTP/1.1 200 OK";
    let headers = "";
    let content = match content::get_html(paths, requestpath) {
        Ok(content) => content,
        Err(_)      => return four_oh_four(),
    };
    let length = content.len();

    format!("{status}\r\nContent-Length: {length}\r\n{headers}\r\n{content}")
}

fn four_oh_four() -> String {
    return "HTTP/1.1 404 Not Found\r\n\r\n".to_owned();
}
