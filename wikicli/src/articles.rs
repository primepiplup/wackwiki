use std::fs;
use std::io;

pub fn post(path: String, entryname: &String, entrytext: String) -> () {
    let mut title: String = entryname.to_owned();
    title = capitalize(&mut title);
    
    let mut content = "# ".to_owned();
    content += &title;
    content += "\n\n";
    content += &entrytext;
    
    let path = path + "/" + entryname;
    match fs::write(path, content) {
        Ok(_) => println!("Created new article: {}", entryname),
        Err(_) => println!("Something went wrong in the creation of the article"),
    };
}

pub fn list_articles(path: &String) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    let file_iter = match fs::read_dir(path) {
        Ok(files) => files,
        Err(_)   => {
            println!("Directory appears to be empty.");
            return files;
        }
    };

    for file in file_iter {
        let file = match file {
            Ok(file) => file,
            Err(_) => {
                println!("Something went wrong during directory scanning.");
                return files;
            }
        };

        if let Ok(file_type) = file.file_type() {
            if !file_type.is_dir() {
                let filename = match file.file_name().into_string() {
                    Ok(dir_name) => dir_name,
                    Err(_) => {
                        println!("OS file name did not contain valid unicode");
                        return files;
                    }
                };
                files.push(filename);
            }
        }
    };
    return files;
}

pub fn exists(path: &String, entryname: &String) -> bool {
    let articles = list_articles(path);

    for article in &articles {
        if article == entryname {
            return true;
        }
    }
    return false;
}

pub fn get_content(path: &String) -> Result<String, ()> {
    let entry_file = match fs::File::open(path) {
        Ok(file) => file,
        Err(_)   => return Err(()),
    };

    match io::read_to_string(entry_file) {
        Ok(contents) => Ok(contents),
        Err(_)       => return Err(()),
    }
}

pub fn remove_article(path: &String) -> Result<(), ()> {
    match fs::remove_file(path) {
        Ok(_)  => Ok(()),
        Err(_) => Err(()),
    }
}

fn capitalize(string: &mut String) -> String {
    let first = match string.chars().next() {
        Some(c) => c,
        None    => return string.to_owned(),
    };

    string.replacen(first, first.to_ascii_uppercase().to_string().as_str(), 1)
}
