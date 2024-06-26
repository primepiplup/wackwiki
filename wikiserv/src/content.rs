use crate::parser::{self, Status};
use crate::paths::Paths;
use crate::template::Template;
use std::io::{BufReader, BufRead};
use std::fs::{File, read_dir};
use std::mem::discriminant;

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

    let mut previous: Status = Status::Empty;

    let file_reader = BufReader::new(file);
    let mut html: String = String::new();
    for (line_num, line) in file_reader.lines().enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(_)   => {
                println!("Failed while reading a line from wiki entry.");
                return Err(());
            },
        };
       
        let (html_line, status) = parser::line_parse_to_html(line, paths, requestpath, line_num);

        if status != Status::Paragraph && previous == Status::Paragraph {
            html = html + "</p>";
        }

        if status == Status::Paragraph {
            if previous != Status::Paragraph {
                html = html + "<p class=\"wiki-paragraph\">";
            } else {
                html = html + "</br>";
            } 
        }

        if let Status::BlockQuote(level) = previous {
            if discriminant(&status) != discriminant(&Status::BlockQuote(1)) {
                for _ in 0..level {
                    html = html + "</blockquote>";
                }
            }
        }

        if let Status::BlockQuote(level) = status {
            if let Status::BlockQuote(prev_level) = previous {
                if level < prev_level {
                    for _ in level..prev_level {
                        html = html + "</blockquote>";
                    }
                } else if level > prev_level {
                    for _ in prev_level..level {
                        html = html + "<blockquote class=\"wiki-blockquote\">";
                    }
                } else {
                    html = html + "</br>";
                }
            } else {
                html = html + "<blockquote class=\"wiki-blockquote\">";
            }
        }

        if let Status::UnorderedList(level) = previous {
            if discriminant(&status) != discriminant(&Status::UnorderedList(1)) {
                for _ in 0..(level + 1) {
                    html = html + "</ul>";
                }
            }
        }

        if let Status::UnorderedList(level) = status {
            if let Status::UnorderedList(prev_level) = previous {
                if level < prev_level {
                    for _ in level..prev_level {
                        html = html + "</ul>";
                    }
                } else if level > prev_level {
                    for _ in prev_level..level {
                        html = html + "<ul class=\"wiki-ul\">";
                    }
                }
            } else {
                html = html + "<ul class=\"wiki-ul\">";
            }
        }

        if let Status::OrderedList(level) = previous {
            if discriminant(&status) != discriminant(&Status::OrderedList(1)) {
                for _ in 0..(level + 1) {
                    html = html + "</ol>";
                }
            }
        }

        if let Status::OrderedList(level) = status {
            if let Status::OrderedList(prev_level) = previous {
                if level < prev_level {
                    for _ in level..prev_level {
                        html = html + "</ol>";
                    }
                } else if level > prev_level {
                    for _ in prev_level..level {
                        html = html + "<ol class=\"wiki-ol\">";
                    }
                }
            } else {
                html = html + "<ol class=\"wiki-ol\">";
            }
        }

        html = html + &html_line;
        previous = status;
    }

    html = html + &functionality(requestpath);

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

fn functionality(requestpath: &str) -> String {
    return format!(
"<script>
    function clickLocation(lineNum, colNum) {{
        fetch(\"{}\", {{
            method: \"POST\",
            body: `checkbox:${{lineNum}}|${{colNum}}\n`,
            headers: {{
                \"Content-Type\": \"text/plain\",
            }},
        }})
    }}
</script>", requestpath);
}
