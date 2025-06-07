use std::io::{self, Write};

fn main() {
    loop{
        print!("$ ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        if input.is_empty(){
            continue;
        }

        if input.trim() == "exit 0"{
            return;
        }
        
        println!("{}: command not found", input);
    }
}
