use std::env;
use std::process::Command;
use crate::articles;

pub fn handle(args: Vec<String>, wikipath: String) {
    if args.len() < 3 {
        print_help();
        return;
    }

    let wikieditor = get_editor();
    if &wikieditor == "" {
        println!("Both the EDITOR and WIKIEDITOR environment variables are not set.");
        println!("Exiting.");
        return;
    }

    if args.len() < 4 {
        edit_global_entry(args, wikipath, wikieditor);
    } else {
        edit_group_entry(args, wikipath, wikieditor);
    }
}

fn edit_global_entry(args: Vec<String>, wikipath: String, wikieditor: String) {
    let entryname = &args[2];

    if !articles::exists(&wikipath, entryname) {
        println!("Could not find entry");
        return;
    }
    
    let path = wikipath + "/" + entryname;

    Command::new(wikieditor).arg(path).status().expect("Something went wrong when starting the editor.");
}

fn edit_group_entry(args: Vec<String>, wikipath: String, wikieditor: String) {
    let groupname = &args[2];
    let entryname = &args[3];
    let grouppath = wikipath + "/" + groupname;

    if !articles::exists(&grouppath, entryname) {
        println!("Could not find entry");
        return;
    }

    let path = grouppath + "/" + entryname;

    Command::new(wikieditor).arg(path).status().expect("Something went wrong when starting the editor.");
}

fn get_editor() -> String {
    let mut wikieditor = match env::var("WIKIEDITOR") {
        Ok(path) => path,
        Err(_)   => "".to_owned(),
    };

    if &wikieditor == "" {
        wikieditor = match env::var("EDITOR") {
            Ok(path) => path,
            Err(_)   => "".to_owned(),
        };
    }

    return wikieditor;
}

fn print_help() {
    println!("Usage:");
    println!("wikicli edit [optional:groupname] [entryname]");
}
