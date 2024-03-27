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
    let mut buffer: Vec<u8> = Vec::new();

    let mut requestpath: String = String::new();
    let mut method: String = String::new();
    let mut body: String = String::new();
    let mut size = 0;
    for (i, byte) in bufreader.bytes().enumerate() {
        let byte = match byte {
            Ok(byte) => byte,
            Err(_)   => {
                println!("Reading byte went wrong.");
                return;
            }
        };

        if byte == 0b0 {
            break;
        }

        if byte == b'\n' {
            let requestpart = match String::from_utf8(buffer) {
                Ok(string) => string,
                Err(_)     => {
                    println!("Could not read bytes into utf8 string.");
                    return;
                }
            };

            println!("{}", requestpart);

            if requestpart == "\r" {
                break;
            }

            if requestpart.contains("Content-Length") {
                let content_size = match requestpart.split_once(": ") {
                    Some((_, num)) => num,
                    None => {
                        println!("Something impossible happened during line parsing.");
                        return;
                    }
                };
                size = match content_size.parse() {
                    Ok(num) => num,
                    Err(_)  => {
                        println!("Expected content length number - something went wrong.");
                        return;
                    }
                };
                println!("Parsed length: {}", size);
            }

            http_request.push(requestpart);
            buffer = Vec::new();
            continue;
        }
        buffer.push(byte);
    }

    requestpath = match http_request[0].split_whitespace().collect::<Vec<&str>>().get(1) {
        Some(path) => path.to_string(),
        None       => {
            println!("Could not find requestpath, killing connection.");
            return;
        }
    };

    method = match http_request[0].split_whitespace().collect::<Vec<&str>>().get(0) {
        Some(path) => path.to_string(),
        None       => {
            println!("Could not find requestpath, killing connection.");
            return;
        }
    };

    if method == "POST" {
        println!("Handling post request");

        if size == 0 {
            println!("No content found in POST request.");
            return;
        }

        println!("Trying to read {} bytes.", size);

        let bufreader = BufReader::new(&mut stream);
        for (i, byte) in bufreader.bytes().enumerate() {
            println!("{}", i);
        }

        println!("Body size read from POST request: {}", size);
    }

    let response: Vec<u8>;
    if method == "GET" {
        response = get_response(paths, &requestpath);
    } else if method == "POST" {
        response = post_response(paths, &requestpath, body);
    } else {
        response = fourhundred();
    }

    match stream.write_all(&response) {
        Ok(_) => (),
        Err(_) => {
            println!("Something went wrong while responding to HTTP request.");
            return;
        }
    };
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
