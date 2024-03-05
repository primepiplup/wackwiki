use std::fs;

pub fn get_dirs(wikipath: String) -> Vec<String> {
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
