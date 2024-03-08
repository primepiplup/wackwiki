pub fn handle(args: Vec<String>, wikipath: String)  -> () {
    if args.len() < 3 {
        print_help();
        return;
    }
}

fn print_help() -> () {
    println!("Usage:");
    println!("wikicli print [optional:groupname] [entryname]");
}

