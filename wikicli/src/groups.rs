use crate::dirs;

pub fn handle(args: Vec<String>, wikipath: String) -> () {
    if args.len() < 3 {
        print_help();
        return;
    }

    match args[2].as_str() {
        "list" => list_groups(wikipath),
        "new"  => create_group(wikipath, args),
        "remove" => remove_group(wikipath, args),
        _ => print_help(),
    }
}

fn print_help() -> () {
    println!("Usage:");
    println!("wikicli groups [subcommand]");
    println!("\nSubcommands:");
    println!("help   -- display this help message");
    println!("list   -- output a list of all groups currently in the wiki");
    println!("new    -- create a new group or subgroup");
    println!("remove -- remove a group or subgroup");
}

fn list_groups(wikipath: String) -> () {
    let dirs = dirs::get_dirs(&wikipath);

    if dirs.len() > 0 {
        println!("Wiki groups:");
        print_vector(dirs);
    }
}

fn create_group(wikipath: String, args: Vec<String>) -> () {
    let dirs = dirs::get_dirs(&wikipath);

    if args.len() < 4 {
        println!("Provide name for new group");
        return;
    }

    let groupname = args[3].clone();
    if dirs.contains(&groupname) {
        println!("Group already exists");
        return;
    }

    dirs::create(wikipath, &groupname);
}

fn remove_group(wikipath: String, args: Vec<String>) -> () {
    let dirs = dirs::get_dirs(&wikipath);

    if args.len() < 4 {
        println!("Provide name for the group to delete");
        return;
    }

    let groupname = args[3].clone();
    if !dirs.contains(&groupname) {
        println!("Group does not exist");
        return;
    }

    dirs::remove(wikipath, &groupname);
}

fn print_vector(dirs: Vec<String>) -> () {
    for dir in dirs {
        println!("\t- {}", dir);
    }
}
