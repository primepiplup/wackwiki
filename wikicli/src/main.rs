mod groups;
mod new;
mod dirs;
mod articles;
mod user;

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
            "help" => print_help(),
            "groups" => groups::handle(args, wikipath),
            "new" => new::handle(args, wikipath),
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
    println!("help   -- display this help message");
    println!("groups -- manage groups and subgroups");
    println!("new    -- create a new wiki entry");
}

