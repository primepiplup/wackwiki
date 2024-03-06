use std::io;

pub fn collect_entry() -> Result<String, ()> {
    let stdin = io::stdin();

    println!("Please specify the contents you wish the entry to contain:");
    println!("You can keep entering lines, exit by entering an empty line.\n");


    let mut buffer = String::new();
    let mut read = match stdin.read_line(&mut buffer) {
        Ok(num) => num,
        Err(_)  => {
            println!("Error occurred while reading input.");
            return Err(());
        }
    };

    while read > 1 {
        read = match stdin.read_line(&mut buffer) {
            Ok(num) => num,
            Err(_)  => {
                println!("Error occurred while reading input.");
                return Err(());
            }
        };
    }

    return Ok(buffer);
}
