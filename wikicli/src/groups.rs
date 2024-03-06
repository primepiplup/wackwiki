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
    println!("list   -- output a list of all groups currently in the wiki, or subgroups in a group");
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
    let mut path = wikipath;
    if args.len() < 4 {
        println!("Provide name for new group");
        return;
    }

    let mut groupname = args[3].clone();
    if groupname.contains("/") {
        let pathparts: Vec<&str> = groupname.split("/").collect();
        let mut count = 0;
        while count < pathparts.len() - 1 {
            path = path + "/" + pathparts[count];
            count += 1;
        }
        groupname = pathparts[pathparts.len() - 1].to_owned();
    }

    let dirs = dirs::get_dirs(&path);
    if dirs.contains(&groupname) {
        println!("Group already exists");
        return;
    }

    dirs::create(path, &groupname);
}

fn remove_group(wikipath: String, args: Vec<String>) -> () {
    let mut path = wikipath;
    if args.len() < 4 {
        println!("Provide name for new group");
        return;
    }

    let mut groupname = args[3].clone();
    if groupname.contains("/") {
        let pathparts: Vec<&str> = groupname.split("/").collect();
        let mut count = 0;
        while count < pathparts.len() - 1 {
            path = path + "/" + pathparts[count];
            count += 1;
        }
        groupname = pathparts[pathparts.len() - 1].to_owned();
    }

    let dirs = dirs::get_dirs(&path);
    if !dirs.contains(&groupname) {
        println!("Group does not exist.");
        return;
    }

    dirs::remove(path, &groupname);
}

fn print_vector(dirs: Vec<String>) -> () {
    for dir in dirs {
        println!("\t- {}", dir);
    }
}
