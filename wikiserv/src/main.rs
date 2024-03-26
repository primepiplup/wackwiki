mod content;
mod parser;
mod paths;
mod template;
mod token;
mod images;

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

    let mut counter = 0;
    for line in bufreader.lines() {
        let requestpart = match line {
            Ok(part) => part,
            Err(_)   => {
                println!("Error occurred during parsing of HTTP request. Dropping connection.");
                return;
            }
        };
        if requestpart.is_empty() {
            if counter >= 5 {
                break;
            }
            counter += 1;
        }
        println!("{}", requestpart);

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

    let method: &str = match match http_request.get(0) {
        Some(first_line) => first_line,
        None             => {
            println!("Malformed request. Dropping connection.");
            return;
        }
    }
        .split_whitespace().collect::<Vec<&str>>().get(0) {
        Some(path) => path,
        None       => {
            println!("Malformed request. Dropping connection.");
            return;
        },
    };

    let response: Vec<u8>;
    if method == "GET" {
        response = get_response(paths, requestpath);
    } else if method == "POST" {
        let body: String = match http_request.last() {
            Some(last) => last.to_owned(),
            None   => {
            println!("Malformed request. Dropping connection.");
            return;
            },
        };
        
        response = post_response(paths, requestpath, body);
    } else {
        response = fourhundred();
    }

    match stream.write_all(&response) {
        Ok(_) => (),
        Err(_) => {
            println!("Something went wrong while responding to HTTP request.");
            return;
        }
    }
}

fn post_response(paths: &Paths, requestpath: &str, body: String) -> Vec<u8> {
    let status = "HTTP/1.1 200 OK";
    let mut headers = "";
    let mut content: Vec<u8> = Vec::new();

    println!("{}", body);
    
    return content;
}

fn get_response(paths: &Paths, requestpath: &str) -> Vec<u8> {
    let status = "HTTP/1.1 200 OK";
    let mut headers = "";
    let mut content: Vec<u8>;

    if requestpath.ends_with(".jpg")  {
        content = match images::get_image(paths, requestpath) {
            Ok(content) => content,
            Err(_)      => return four_oh_four(),
        };
        headers = "Content-Type: image/jpeg\r\n";
    } else if requestpath.ends_with(".png") {
        content = match images::get_image(paths, requestpath) {
            Ok(content) => content,
            Err(_)      => return four_oh_four(),
        };
        headers = "Content-Type: image/png\r\n";
    } else {
        content = match content::get_html(paths, requestpath) {
            Ok(content) => content,
            Err(_)      => return four_oh_four(),
        };
    }

    let length = content.len();

    let mut html_head = format!("{status}\r\nContent-Length: {length}\r\n{headers}\r\n").as_bytes().to_vec();

    html_head.append(&mut content);
    return html_head;
}

fn four_oh_four() -> Vec<u8> {
    return "HTTP/1.1 404 Not Found\r\n\r\n".as_bytes().to_vec();
}

fn fourhundred() -> Vec<u8> {
    return "HTTP/1.1 400 Bad Request\r\n\r\n".as_bytes().to_vec();
}
