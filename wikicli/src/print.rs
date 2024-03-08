use crate::articles;

pub fn handle(args: Vec<String>, wikipath: String)  -> () {
    if args.len() < 3 {
        print_help();
        return;
    }

    if args.len() < 4 {
        print_global_entry(args, wikipath);
    } else {
        print_group_entry(args, wikipath);
    }
}

fn print_global_entry(args: Vec<String>, wikipath: String) -> () {
    let entryname = &args[2];

    if !articles::exists(&wikipath, entryname) {
        println!("Could not find entry");
        return;
    }
    
    let path = wikipath + "/" + entryname;
    let contents = match articles::get_content(&path) {
        Ok(contents) => contents,
        Err(_)       => {
            println!("Something went wrong in opening or reading the entry file");
            return;
        }
    };

    let contents = contents.trim_end();

    println!("{}", contents);
}

fn print_group_entry(args: Vec<String>, wikipath: String) -> () {
    let groupname = &args[2];
    let entryname = &args[3];
    let grouppath = wikipath + "/" + groupname;

    if !articles::exists(&grouppath, entryname) {
        println!("Could not find entry");
        return;
    }

    let path = grouppath + "/" + entryname;
    let contents = match articles::get_content(&path) {
        Ok(contents) => contents,
        Err(_)       => {
            println!("Something went wrong in opening or reading the entry file");
            return;
        }
    };

    let contents = contents.trim_end();

    println!("{}", contents);
}

fn print_help() -> () {
    println!("Usage:");
    println!("wikicli print [optional:groupname] [entryname]");
}

