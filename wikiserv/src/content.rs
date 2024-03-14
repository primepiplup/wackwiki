use crate::parser;
use crate::paths::Paths;
use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn get_html(paths: &Paths, requestpath: &str) -> Result<String, ()> {
    let content = match parse_to_html(paths, requestpath) {
        Ok(content) => content,
        Err(_)      => return Err(()),
    };

    Ok(format!(
"<html>
    <head>
    </head>
    <body>
        {content}
    </body>
</html>"))
}

fn parse_to_html(paths: &Paths, requestpath: &str) -> Result<String, ()> {
    let filepath = paths.wikipath().to_owned() + requestpath;
    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(_)   => {
            println!("Could not open requested filepath.");
            return Err(());
        }
    };

    let file_reader = BufReader::new(file);
    let mut html: String = String::new();
    for line in file_reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(_)   => {
                println!("Failed while reading a line from wiki entry.");
                return Err(());
            },
        };
        let html_line = parser::line_parse_to_html(line, paths, requestpath);
        html = html + &html_line + "</br>";
    }

    return Ok(html);
}

