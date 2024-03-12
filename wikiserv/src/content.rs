use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn get_html(path: String) -> Result<String, ()> {
    let content = match parse_to_html(path) {
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

fn parse_to_html(path: String) -> Result<String, ()> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_)   => {
            println!("Could not open requested filepath.");
            return Err(());
        }
    };

    let file_reader = BufReader::new(file);
    let mut html: String = "".to_owned();
    for line in file_reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(_)   => {
                println!("Failed while reading a line from wiki entry.");
                return Err(());
            },
        };
        html = html + &line + "</br>";
    }

    return Ok(html);
}
