use crate::articles;

pub fn handle(args: Vec<String>, wikipath: String)  -> () {
    if args.len() < 3 {
        print_help();
        return;
    }

    if args.len() < 4 {
        remove_global_entry(args, wikipath);
    } else {
        remove_group_entry(args, wikipath);
    }
}

fn remove_global_entry(args: Vec<String>, wikipath: String) -> () {
    let entryname = &args[2];

    if !articles::exists(&wikipath, entryname) {
        println!("Could not find entry");
        return;
    }
    
    let path = wikipath + "/" + entryname;
    match articles::remove_article(&path) {
        Ok(_) => println!("Successfully removed entry: {}", entryname),
        Err(_)       => println!("Was unable to remove entry."),
    };
}

fn remove_group_entry(args: Vec<String>, wikipath: String) -> () {
    let groupname = &args[2];
    let entryname = &args[3];
    let grouppath = wikipath + "/" + groupname;

    if !articles::exists(&grouppath, entryname) {
        println!("Could not find entry.");
        return;
    }

    let path = grouppath + "/" + entryname;
    match articles::remove_article(&path) {
        Ok(_) => println!("Successfully removed entry: {}", entryname),
        Err(_)       => println!("Was unable to remove entry."),
    };
}

fn print_help() -> () {
    println!("Usage:");
    println!("wikicli remove [optional:groupname] [entryname]");
}
