use std::fs;

pub fn get_dirs(wikipath: &String) -> Vec<String> {
    let mut dirs: Vec<String> = Vec::new();
    let dir_iter = match fs::read_dir(wikipath) {
        Ok(dirs) => dirs,
        Err(_)   => {
            println!("wiki directory appears to be empty");
            return dirs;
        }
    };

    for dir in dir_iter {
        let dir = match dir {
            Ok(dir) => dir,
            Err(_) => {
                println!("Something went wrong during directory scanning");
                return dirs;
            }
        };

        if let Ok(file_type) = dir.file_type() {
            if file_type.is_dir() {
                let filename = match dir.file_name().into_string() {
                    Ok(dir_name) => dir_name,
                    Err(_) => {
                        println!("OS dir name did not contain valid unicode");
                        return dirs;
                    }
                };
                dirs.push(filename);
            }
        }
    };
    return dirs;
}

pub fn create(wikipath: String, name: &String) -> () {
    let newpath = wikipath + "/" + name;
    match fs::create_dir(newpath) {
        Ok(_) => println!("Created new group: {}", name),
        Err(_) => println!("Failed to create directory for new group"),
    };
}

pub fn remove(wikipath: String, name: &String) -> () {
    let removepath = wikipath + "/" + name;
    match fs::remove_dir(removepath) {
        Ok(_) => println!("Succesfully removed group: {}", name),
        Err(_) => println!("Unable to remove group, something went wrong"),
    }
}

pub fn contains(wikipath: &String, name: &String) -> bool {
    let dirs = get_dirs(&wikipath);

    for dir in &dirs {
        if dir == name {
            return true;
        }
    }

    let pluralized = name.to_owned() + "s";
    for dir in &dirs {
        if dir == &pluralized {
            return true;
        }
    }

    return false;
}