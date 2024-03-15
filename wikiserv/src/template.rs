use std::sync::RwLock;

pub struct Template {
    left: RwLock<String>,
    right: RwLock<String>,
    synced: RwLock<bool>,
}

impl Template {
    pub const fn new() -> Template {
        Template {left: RwLock::new(String::new()), right: RwLock::new(String::new()), synced: RwLock::new(false)}
    }
    
    pub fn sync(&self) -> () {
        if match self.synced.read() {
            Ok(val) => *val,
            Err(_)  => return,

        } { return; }
        
        let backup: &str =
            "<html>
                <head>
                </head>
                <body>
                    --wikicontent--
                </body>
            </html>";

        let config_file_path = match std::env::var("HOME") { Ok(dir) => dir, Err(_) => String::new(), } + "/.config/wikiserv/template.html";
        let template: String = match std::fs::read_to_string(config_file_path) {
            Ok(string) => string,
            Err(_)     => {
                println!("Reading the configured template went wrong.");
                backup.to_string()
            },
        };

        let (left, right) = match template.split_once("--wikicontent--") {
            Some(split_template) => split_template,
            None => backup.split_once("--wikicontent--").expect("Builtin backup template broke."),
        };

        *self.left.write().unwrap() = left.to_owned();
        *self.right.write().unwrap() = right.to_owned();
        *self.synced.write().unwrap() = true;
    }

    pub fn encase(&self, content: String) -> String {
        return self.left.read().unwrap().to_string() + &content + &self.right.read().unwrap();
    }
}
