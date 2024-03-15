use crate::parser;
use crate::paths::Paths;
use crate::template::Template;
use std::io::{BufReader, BufRead};
use std::fs::{File, read_dir};

static TEMPLATE: Template = Template::new();

pub fn get_html(paths: &Paths, mut requestpath: &str) -> Result<String, ()> {
    if requestpath.len() > 1 && requestpath.ends_with("/") {
        requestpath = match requestpath.rsplit_once("/") { Some((left, _)) => left, None => requestpath};
    }
    
    if requestpath == "/" || paths.contains_group(&requestpath.to_string()) {
        println!("Trying to create a link page");
        return Ok(link_page(paths, requestpath));
    }
    
    let content = match parse_to_html(paths, requestpath) {
        Ok(content) => content,
        Err(_)      => return Err(()),
    };

    TEMPLATE.sync();
    Ok(TEMPLATE.encase(content))
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
