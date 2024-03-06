use crate::dirs;

pub fn handle(args: Vec<String>, wikipath: String) -> () {
    if args.len() < 3 {
        print_help();
        return;
    }

    match args[2].as_str() {
        "help" => print_help(),
        _ => create_entry(args, wikipath),
    }
}

fn create_entry(args: Vec<String>, wikipath: String) -> () {
    if args.len() < 4 {
        print_help();
        return;
    }

    let first_element = &args[3];
    if dirs::contains(&wikipath, first_element) {
        create_group_entry(&args, wikipath, first_element);
        return;
    }

    create_global_entry(&args, wikipath, first_element);
}


fn create_group_entry(args: &Vec<String>, wikipath: String, groupname: &String) -> () {

}

fn create_global_entry(args: &Vec<String>, wikipath: String, groupname: &String) -> () {

}

fn print_help() -> () {
    println!("Usage:");
    println!("wikicli new [optional:groupname] [entryname] [optional:entry text]");
    println!("\nExcluding the group name puts the entry into the global group.");
    println!("When putting an entry into the global group it cannot have the same name as an existing group.");
    println!("\nGroup names are treated literally but are also interpreted with the ending 's' removed");
    println!("This means you can write either:");
    println!("\twikicli new ideas newidea");
    println!("\twikicli new idea newidea");
    println!("And achieve the same result");
    println!("\nAfter providing the entryname you can choose to enter the text for the entry.");
    println!("This skips the longer entry creation process and immediately creates the entry.");
}
