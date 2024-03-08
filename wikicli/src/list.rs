use crate::articles;

pub fn handle(args: Vec<String>, wikipath: String)  -> () {
    if args.len() < 3 {
        list_global_entry(args, wikipath);
    } else {
        list_group_entry(args, wikipath);
    }
}

fn list_global_entry(_args: Vec<String>, wikipath: String) -> () {
    let entries = articles::list_articles(&wikipath);
    println!("Entries in global scope:");
    for entry in entries {
        println!("\t- {}", entry);
    }
}

fn list_group_entry(args: Vec<String>, wikipath: String) -> () {
    let groupname = &args[2];
    let path = wikipath + "/" + groupname;
    let entries = articles::list_articles(&path);
    println!("Entries in {}:", groupname);
    for entry in entries {
        println!("\t- {}", entry);
    }
}

fn _print_help() -> () {
    println!("Usage:");
    println!("wikicli list [optional:groupname]");
}

