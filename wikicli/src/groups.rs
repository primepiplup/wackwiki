use crate::dirs;

pub fn handle(args: Vec<String>, wikipath: String) -> () {
    if args.len() < 3 {
        print_help();
        return;
    }

    match args[2].as_str() {
        "list" => list_groups(wikipath),
        _ => print_help(),
    }
}

fn print_help() -> () {
    println!("Usage:");
    println!("wikicli groups [subcommand]");
    println!("\nSubcommands:");
    println!("help -- display this help message");
    println!("list -- output a list of all groups currently in the wiki");
}

fn list_groups(wikipath: String) -> () {
    let dirs = dirs::get_dirs(wikipath);

    if dirs.len() > 0 {
        println!("Wiki groups:");
        print_vector(dirs);
    }
}

fn print_vector(dirs: Vec<String>) -> () {
    for dir in dirs {
        println!("\t- {}", dir);
    }
}
