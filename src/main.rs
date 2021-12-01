use std::io::{self, Write};
use std::process;

fn main() {
    loop {
        print_promt();
        let input = read_input();
        if input == ".exit" {
            process::exit(0);
        } else {
            println!("Unrecognized command '{}'.", input);
        }
    }
}

fn print_promt() {
    print!("sqlite> ");
    io::stdout().flush().unwrap();
}

fn read_input() -> String {
    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Err(error) => {
            eprintln!("Error reading Input({})", error);
            process::exit(1);
        },
        Ok(n) => {
            line.truncate(n-1);
            line
        },
    }
}