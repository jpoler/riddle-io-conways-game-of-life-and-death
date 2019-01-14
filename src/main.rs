mod parser;
mod types;

use std::io::{self, BufRead};

use types::{Message, Update};

fn main() {
    let s = io::stdin();
    let br = s.lock();

    let mut parser = parser::Parser::new(br);

    for message in parser.iter() {
        match message {
            Ok(msg) => match msg {
                Message::Action(action) => {
                    eprintln!("passing: {:?}", action);
                    println!("pass");
                }
                msg => eprintln!("message: {:?}", msg),
            },
            Err(err) => eprintln!("error: {:?}", err),
        }
    }
}
