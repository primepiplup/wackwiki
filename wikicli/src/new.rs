use crate::{dirs, articles, user, parser};

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
    let mut first_element = args[2].clone();
    if dirs::contains(&wikipath, &mut first_element) {
        create_group_entry(&args, wikipath, &first_element);
        return;
    }

    create_global_entry(&args, wikipath, &first_element);
}


fn create_group_entry(args: &Vec<String>, wikipath: String, groupname: &String) -> () {
    if args.len() < 4 {
        println!("No entry name was provided.");
        return;
    }

    let entryname = &parser::parse_name(&args[3]);

    let path = wikipath + "/" + groupname;
    if articles::exists(&path, entryname) {
        println!("Article already exists, exiting.");
        return;
    }

    let mut entrytext: String;
    if args.len() > 4 {
        entrytext = text_collect(5, args);
    } else {
        entrytext = match user::collect_entry() {
            Ok(text) => text,
            Err(_)   => {
                println!("Exiting.");
                return;
            }
        };
    }

    entrytext.push('\n');
    
    articles::post(path, entryname, entrytext);
}

fn create_global_entry(args: &Vec<String>, wikipath: String, entryname: &String) -> () {
    let entryname = &parser::parse_name(entryname);
    let path = wikipath;
    if articles::exists(&path, entryname) {
        println!("Article already exists, exiting.");
        return;
    }

    let mut entrytext: String;
    if args.len() > 3 {
        entrytext = text_collect(4, args);
    } else {
        entrytext = match user::collect_entry() {
            Ok(text) => text,
            Err(_)   => {
                println!("Exiting.");
                return;
            }
        };
    }

    entrytext.push('\n');
    
    articles::post(path, entryname, entrytext);
}

fn text_collect(from: usize, args: &Vec<String>) -> String {
    let mut text: String = "".to_owned();
    text += &args[from - 1];
    for i in from..(args.len()) {
        text = text + " " + &args[i];
    }

    return text;
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
    println!("And achieve the same result.");
    println!("\nAfter providing the entryname you can choose to enter the text for the entry.");
    println!("This skips the longer entry creation process and immediately creates the entry.");
}
