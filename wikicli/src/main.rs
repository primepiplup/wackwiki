mod edit;
mod groups;
mod new;
mod dirs;
mod articles;
mod user;
mod print;
mod remove;
mod list;
mod parser;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let wikipath = match env::var("WIKIPATH") {
        Ok(path) => path,
        Err(_)   => {
            println!("WIKIPATH environment variable is undefined.");
            return;
        }
    };

    if args.len() < 2 {
        print_help();
    } else {
        match args[1].as_str() {
            "edit"   => edit::handle(args, wikipath),
            "groups" => groups::handle(args, wikipath),
            "help"   => print_help(),
            "list"   => list::handle(args, wikipath),
            "new"    => new::handle(args, wikipath),
            "print"  => print::handle(args, wikipath),
            "remove" => remove::handle(args, wikipath),
            _ => {
                println!("Unrecognized command.\n");
                print_help();
            }
        }
    }
}

fn print_help() -> () {
    println!("Usage: ");
    println!("wikicli [subcommand]");
    println!("\nAvailable subcommands:");
    println!("edit   -- edit a wiki entry using your favourite editor");
    println!("groups -- manage groups and subgroups");
    println!("help   -- display this help message");
    println!("list   -- list the articles in the global group or subgroups");
    println!("new    -- create a new wiki entry");
    println!("print  -- print the contents of a wiki entry");
    println!("remove -- remove a wiki entry, either globally or from a (sub)group");
}

