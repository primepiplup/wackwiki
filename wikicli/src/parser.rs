use chrono::{Local, Days};

pub fn parse_name(text: &String) -> String {
    let mut parts: Vec<String> = Vec::new();
    let mut string: &str = text.as_str();
    while string.contains("(") {
        let (left, right) = match string.split_once("(") {
            Some(split) => split,
            None        => panic!("This should have been impossible. String did not contain '('."),
        };
        parts.push(left.to_string());
        let (content, remainder) = match right.split_once(")") {
            Some(split) => split,
            None        => (right, right),
        };
        string = remainder;
        parts.push(parse_format_string(content));
    }

    let mut return_string: String = String::new();
    for part in parts {
        return_string = return_string + part.as_str();
    }
    return return_string;
}

fn parse_format_string(string: &str) -> String {
    match string {
        "now"       => now(),
        "today"     => today(),
        "yesterday" => yesterday(),
        "tomorrow"  => tomorrow(),
        "test"      => "whatever".to_string(),
        _           => "FALLBACK".to_string(),
    }
}

fn now() -> String {
    let now = Local::now();
    now.format("%Y-%m-%d-%H-%M-%S").to_string()
}

fn today() -> String {
    let now = Local::now();
    now.format("%Y-%m-%d").to_string()
}

fn yesterday() -> String {
    let now = Local::now();
    let yesterday = match now.checked_sub_days(Days::new(1)) {
        Some(yesterday) => yesterday,
        None => now,
    };
    yesterday.format("%Y-%m-%d").to_string()
}

fn tomorrow() -> String {
    let now = Local::now();
    let tomorrow = match now.checked_add_days(Days::new(1)) {
        Some(tomorrow) => tomorrow,
        None => now,
    };
    tomorrow.format("%Y-%m-%d").to_string()
}

