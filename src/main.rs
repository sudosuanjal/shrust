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
           ["exit", _rest] => break,
           ["echo", rest @..]=>println!("{}", rest.join(" ")),
           ["type", rest @..]=>{
                if rest.len() == 0{
                    return;
                }
                match rest[0] {
                    "type"|"echo"|"exit" => println!("{} is a shell builtin",{rest[0]}),
                    _ => println!("{}: not found", rest[0]),
                }
           },
           [other] => println!("{}: command not found", other),
           _ => println!("invalid command"),
       }
    }
}