use std::io;
use whatlang::detect;

struct Info {
    name: String,
    script: String,
}

impl Info {
    fn new(content: String) -> Option<Self> {
        
        match detect(&content) {

            Some(info) => {
                let name = info.lang().eng_name().to_string();
                let script = info.script().name().to_string();
                Some(
                    Self {
                        name,
                        script,
                    }
                )
            },
            None => None,
        }
    }

}

fn main() {
    let mut content = String::new();
    println!("Enter String:");
    if let Err(e) = io::stdin().read_line(&mut content) {
        eprintln!("Error: {}",e);
    }

    if let Some(info) = Info::new(content) {
        println!("Language : {}",info.name);
        println!("Script   : {}",info.script);
    } else {
        eprintln!("Error: Enter Valid Language");
    }
}