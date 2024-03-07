use crate::dirs;

pub fn handle(args: Vec<String>, wikipath: String) -> () {
    if args.len() < 3 {
        print_help();
        return;
    }

    match args[2].as_str() {
        "list" => list_groups(wikipath, args),
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

fn list_groups(wikipath: String, args: Vec<String>) -> () {
    let mut path = wikipath;

    if args.len() > 3 {
        let (returnpath, groupname) = separate_group_path_and_name(path, &args);
        path = returnpath + "/" + &groupname;
    }
    
    let dirs = dirs::get_dirs(&path);

    if dirs.len() > 0 {
        println!("Groups:");
        print_vector(dirs);
    }
}

fn create_group(wikipath: String, args: Vec<String>) -> () {
    if args.len() < 4 {
        println!("Provide name for new group");
        return;
    }

    let (path, groupname) = separate_group_path_and_name(wikipath, &args);

    let dirs = dirs::get_dirs(&path);
    if dirs.contains(&groupname) {
        println!("Group already exists");
        return;
    }

    dirs::create(path, &groupname);
}

fn remove_group(wikipath: String, args: Vec<String>) -> () {
    if args.len() < 4 {
        println!("Provide name for new group");
        return;
    }

    let (path, groupname) = separate_group_path_and_name(wikipath, &args);

    let dirs = dirs::get_dirs(&path);
    if !dirs.contains(&groupname) {
        println!("Group does not exist.");
        return;
    }

    dirs::remove(path, &groupname);
}

fn separate_group_path_and_name(wikipath: String, args: &Vec<String>) -> (String, String) {
    let mut path = wikipath;
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

    return (path, groupname);
}

fn print_vector(dirs: Vec<String>) -> () {
    for dir in dirs {
        println!("\t- {}", dir);
    }
}
