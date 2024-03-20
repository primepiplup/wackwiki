use std::fs::{self, ReadDir};

pub struct Paths {
    wikipath: String,
    paths: Vec<String>,
    grouppaths: Vec<String>,
}

impl Paths {
    pub fn new(path: String) -> Result<Paths, ()> {
        let mut paths = Vec::new();
        let mut grouppaths = Vec::new();

        let files = match fs::read_dir(&path) {
            Ok(dir_iter) => dir_iter,
            Err(_) => return Err(()),
        };

        eat_entries(files, &path, &"".to_string(), &mut paths, &mut grouppaths);

        Ok(Paths {
            wikipath: path,
            paths,
            grouppaths,
        })
    }

    pub fn wikipath(&self) -> &String {
        return &self.wikipath;
    }

    pub fn contains(&self, path: &String) -> bool {
        return self.paths.contains(path);
    }

    pub fn contains_group(&self, path: &String) -> bool {
        return self.grouppaths.contains(path);
    }
}

fn eat_entries(files: ReadDir, path: &String, relativepath: &String, paths: &mut Vec<String>, grouppaths: &mut Vec<String>) {
    for file in files {
        let file = match file {
            Ok(file) => file,
            Err(_)   => continue,
        };

        if let Ok(filetype) = file.file_type() {
            if filetype.is_dir() {
                let dir_name = match file.file_name().into_string() {
                    Ok(string) => string,
                    Err(_) => continue
                };
                if dir_name.starts_with(".") {
                    return;
                }
                let dirpath = path.clone() + relativepath + "/" + &dir_name;
                println!("reading dir: {}", dirpath);
                let files = match fs::read_dir(dirpath) {
                    Ok(dir_iter) => dir_iter,
                    Err(_) => continue,
                };
                let relativepath = relativepath.to_owned() + "/" + &dir_name;
                eat_entries(files, path, &relativepath, paths, grouppaths);
                grouppaths.push(relativepath);
            } else if filetype.is_file() {
                let file_name = match file.file_name().into_string() {
                    Ok(string) => string,
                    Err(_) => continue
                };
                let filepath = relativepath.to_owned() + "/" + &file_name;
                println!("Found entry: {}", filepath);
                paths.push(filepath);
            }
        }
    }
}
