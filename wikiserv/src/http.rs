pub fn basic_response(status_code: usize) -> Vec<u8> {
    return response(status_code, String::new(), &mut Vec::new());
}

pub fn response(status_code: usize, headers: String, body: &mut Vec<u8>) -> Vec<u8> {
    let status = build_status(status_code);
    let length = body.len();
    let mut response = format!("{status}\r\nContent-Length: {length}\r\n{headers}\r\n").as_bytes().to_vec();
    response.append(body);
    return response;
}

fn build_status(mut status_code: usize) -> String {
    let status: String = "HTTP/1.1 ".to_string();

    let status_text = match get_text_for_code(status_code) {
        Some(text) => text,
        None       => {
            status_code = fallback_status_code(status_code);
            match get_text_for_code(status_code) {
                Some(text) => text,
                None => {
                    status_code = 500;
                    "Internal Error"
                }
            }
        }
    };

    return status + &status_code.to_string() + " " + status_text;
}

fn get_text_for_code(status_code: usize) -> Option<&'static str> {
    match status_code {
        200 => Some("OK"),
        400 => Some("Bad Request"),
        404 => Some("Not Found"),
        500 => Some("Internal Error"),
        _ => None,
    }
}

fn fallback_status_code(mut status_code: usize) -> usize {
    if status_code > 200 && status_code < 300 {
        status_code = 200;
    } else if status_code > 300 && status_code < 400 {
        status_code = 300;
    } else if status_code > 400 && status_code < 500 {
        status_code = 400;
    } else {
        status_code = 500;
    }

    return status_code;
}
