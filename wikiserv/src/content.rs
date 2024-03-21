use crate::parser::{self, Status};
use crate::paths::Paths;
use crate::template::Template;
use std::io::{BufReader, BufRead};
use std::fs::{File, read_dir};

static TEMPLATE: Template = Template::new();

pub fn get_html(paths: &Paths, mut requestpath: &str) -> Result<Vec<u8>, ()> {
    TEMPLATE.sync();
    if requestpath.len() > 1 && requestpath.ends_with("/") {
        requestpath = match requestpath.rsplit_once("/") { Some((left, _)) => left, None => requestpath};
    }
    
    if requestpath == "/" || paths.contains_group(&requestpath.to_string()) {
        let content = link_page(paths, requestpath);
        return Ok(TEMPLATE.encase(content).as_bytes().to_vec());
    }
    
    let content = match parse_to_html(paths, requestpath) {
        Ok(content) => content,
        Err(_)      => return Err(()),
    };

    Ok(TEMPLATE.encase(content).as_bytes().to_vec())
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

    let mut in_paragraph: bool = false;

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
        let line = line.trim().to_string();
        if line.is_empty() && in_paragraph {
            html = html + "</p>";
            in_paragraph = false;
            continue;
        }
        
        let (html_line, status) = parser::line_parse_to_html(line, paths, requestpath);

        if status == Status::Paragraph && !in_paragraph {
            html = html + "<p class=\"wiki-paragraph\">";
            in_paragraph = true;
        } else if status == Status::Paragraph {
            html = html + "</br>";
        } else if status == Status::Header && in_paragraph {
            in_paragraph = false;
            html = html + "</p>";
        }

        html = html + &html_line;
    }

    return Ok(html);
}

fn link_page(paths: &Paths, requestpath: &str) -> String {
    let mut html = String::new();
    let upper = match requestpath.rsplit_once("/") { Some((left, _)) => left, None => requestpath}.to_string() + "/";

    html = html + "<a href=\"" + &upper + "\">" + &upper + "</a>" + "</br>";

    let dir_iter = match read_dir(paths.wikipath().to_string() + requestpath) {
        Ok(dir_iter) => dir_iter,
        Err(_)       => return html,
    };

    for item in dir_iter {
        if let Ok(entry) = item {
            let file_name = match entry.file_name().into_string() {
                Ok(file_name) => file_name,
                Err(_)        => continue,
            };
            if file_name.starts_with(".") {
                continue;
            }
            let elementpath: String;
            if requestpath == "/" {
                elementpath = requestpath.to_string() + &file_name;
            } else {
                elementpath = requestpath.to_string() + "/" + &file_name;
            }
            html = html + "<a href=\"" + &elementpath + "\">" + &elementpath + "</a>" + "</br>";
        }
    }

    return html;
}

