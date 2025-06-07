#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop{
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().split_whitespace().collect::<Vec<&str>>().as_slice() {
           ["exit"] => break,
           ["exit", rest] => break,
           ["echo", rest @..]=>println!("{}", rest.join(" ")),
           [other] => println!("{}: command not found", other),
           _ => println!("invalid command"),
       }
    }
}

