use crate::paths::Paths;
use std::io::{prelude::*, BufReader, BufWriter};
use std::fs::{File, self};

pub fn handle(paths: &Paths, requestpath: &str, command: &str, command_info: &str) -> Result<Vec<u8>, usize> {
    match command {
        "checkbox" => checkbox(paths, requestpath, command_info),
        _ => Err(400),
    }
}

fn checkbox(paths: &Paths, requestpath: &str, info: &str) -> Result<Vec<u8>, usize> {
    let (line_number, column_number) = match info.split_once("|") {
        Some(split) => split,
        None => return Err(400),
    };

    let line_number: usize = match line_number.trim().parse() {
        Ok(num) => num,
        Err(_)  => {
            println!("Failed to parse line number. Returning 400.");
            return Err(400);
        },
    };

    let column_number: usize = match column_number.trim().parse() {
        Ok(num) => num,
        Err(_)  => {
            println!("Failed to parse column number. Returning 400.");
            return Err(400);
        },
    };

    let path = paths.wikipath().to_string() + "/" + requestpath;
    let temppath = path.clone() + ".temp";

    let entry = match File::open(&path) {
        Ok(file) => file,
        Err(_)   => {
            println!("Was unable to open file mentioned in POST request.");
            return Err(400);
        }
    };

    let temp = match File::create(&temppath) {
        Ok(file) => file,
        Err(_)   => {
            println!("Could not create temporary file to write new information to");
            return Err(500);
        },
    };

    let entrybuffer = BufReader::new(entry);
    let mut tempbuffer = BufWriter::new(temp);

    let mut line_index = 0;
    for line in entrybuffer.lines() {
        line_index += 1;
        let mut line = match line {
            Ok(line) => line,
            Err(_)   => {
                println!("Something went wrong when reading the wiki file.");
                return Err(500);
            },
        };

        if line_index == line_number {
            let mut linechars: Vec<char> = line.chars().collect();
            let boxpos = column_number - 1;
            if linechars[boxpos - 1] != '[' && linechars[boxpos + 1] != ']' {
                println!("Incorrect position given for checkbox.");
                return Err(400);
            }
            if linechars[boxpos] == ' ' {
                linechars[boxpos] = 'X';
            } else {
                linechars[boxpos] = ' ';
            }
            line = linechars.into_iter().collect();
        }

        line = line + "\n";

        match tempbuffer.write_all(line.as_bytes()) {
            Ok(_)  => (),
            Err(_) => {
                println!("Was unable to write to temporary file. Exiting.");
                return Err(500);
            }
        };
    }

    match fs::rename(temppath, path) {
        Ok(_)  => (),
        Err(_) => {
            println!("Was unable to replace old file with new temporary file.");
            return Err(500);
        }
    }

    return Ok(Vec::new());
}
