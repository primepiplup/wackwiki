use std::fs;
use std::path::Path;
use std::os::unix::fs::symlink;

use crate::articles::list_articles;

pub fn handle(args: Vec<String>, wikipath: String) -> () {
    if args.len() < 3 {
        print_help();
        return;
    }

    match args[2].as_str() {
        "add" => add(wikipath, args),
        "list" => list(wikipath, args),
        "remove" => remove(wikipath, args),
        _ => print_help(),
    }
}

fn add(wikipath: String, args: Vec<String>) -> () {
    if args.len() < 4 {
        add_help();
        return;
    }

    let filepath = &args[3];

    let (_, filename) = match filepath.rsplit_once("/") {
        Some(split) => split,
        None => {
            println!("It seems your filepath is incorrect.");
            return;
        },
    };

    if !match Path::try_exists(Path::new(filepath)) {
        Ok(exists) => exists,
        Err(_)     => {
            println!("Could not verify existence of file.");
            return;
        }
    } {
        println!("The file you're attempting to link does not exist.");
        return;
    }

    let grouppath = wikipath + "/" + match args.get(4) {
        Some(path) => path,
        None       => "",
    };

    if !match Path::try_exists(Path::new(&grouppath)) {
        Ok(exists) => exists,
        Err(_)     => {
            println!("Could not verify existence of group.");
            return;
        }
    } {
        println!("The group you're attempting create a link for does not exist.");
        return;
    }

    let linkpath = grouppath + "/.link";

    if !match Path::try_exists(Path::new(&linkpath)) {
        Ok(exists) => exists,
        Err(_)     => {
            println!("Was unable to check for link subdirectory in group.");
            return;
        }
    } {
        match fs::create_dir(&linkpath) {
            Ok(_)  => (),
            Err(_) => {
                println!("Failed to create .link subdirectory for group");
                return;
            }
        }
    }

    let linkpath = linkpath + "/" + filename;

    match symlink(filepath, linkpath) {
        Ok(_) => {
            println!("Successfully created link from {}", filename);
        },
        Err(_) => {
            println!("Failed to create link from {}", filename);
        },
    };
}

fn remove(wikipath: String, args: Vec<String>) -> () {
    if args.len() < 4 {
        remove_help();
        return;
    }

    let filename = &args[3];

    let grouppath = wikipath + "/" + match args.get(4) {
        Some(path) => path,
        None       => "",
    };

    if !match Path::try_exists(Path::new(&grouppath)) {
        Ok(exists) => exists,
        Err(_)     => {
            println!("Could not verify existence of group.");
            return;
        }
    } {
        println!("The group you're attempting to remove a link from does not exist.");
        return;
    }

    let filepath = grouppath + "/.link/" + filename;

    if !match Path::try_exists(Path::new(&filepath)) {
        Ok(exists) => exists,
        Err(_)     => {
            println!("Could not verify existence of file.");
            return;
        }
    } {
        println!("The link you're attempting to remove does not exist.");
        return;
    }

    match fs::remove_file(filepath) {
        Ok(_)  => println!("Successfully removed link to {}", filename),
        Err(_) => println!("Was unable to remove link to {}", filename),
    }
}

fn list(wikipath: String, args: Vec<String>) -> () {
    let group: String;
    if args.len() > 3 {
        group = args[3].to_owned();
    } else {
        group = String::new();
    }

    let grouppath = wikipath + "/" + &group + "/.link";

    let elements = list_articles(&grouppath);

    for element in elements {
        println!("{}", element);
    }
}

fn print_help() -> () {
    println!("Usage:");
    println!("wikicli link [subcommand]");
    println!("\nSubcommands:");
    println!("add    -- add a file to a group's relative path");
    println!("help   -- display this help message");
    println!("list   -- list the links contained in a group");
    println!("remove -- remove a file from a group's relative path");
}

fn add_help() -> () {
    println!("Usage:");
    println!("wikicli link add [filepath] [optional:groupname]");
}

fn remove_help() -> () {
    println!("Usage:");
    println!("wikicli link remove [linkname] [optional:groupname]");
}

