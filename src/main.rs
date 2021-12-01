use std::io::{self, Write};
use std::process;

fn main() {
    loop {
        print_promt();
        let input = read_input();
        if input.len() > 0 && &input[0..1] == "." {
            match do_meta_command(&input) {
                MetaCommandResult::Success => continue,
                MetaCommandResult::Unrecognized(cmd) => {
                    println!("Unrecognized command '{}'.", cmd);
                    continue;
                },
            }
        }

        let statement = prepare_statement(&input);
        match statement {
            Statement::Unrecognized(cmd) => {
                println!("Unrecognized keyword at start of '{}'.", cmd);
                continue;
            }
            _ => execute_statement(&statement),
        }

        println!("Executed.");
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

enum MetaCommandResult {
    Success,
    Unrecognized(String),
}

enum Statement {
    Insert,
    Select,
    Unrecognized(String),
}

fn do_meta_command(command: &str) -> MetaCommandResult {
    if command == ".exit" {
        process::exit(0);
    } else {
        return MetaCommandResult::Unrecognized(String::from(command));
    }
}

fn prepare_statement(command: &str) -> Statement {
    if command.len() >= 6 && &command[0..6] == "insert" {
        return Statement::Insert;
    } else if command.len() >= 6 && &command[0..6] == "select" {
        return Statement::Select;
    }

    return Statement::Unrecognized(String::from(command));
}

fn execute_statement(statement: &Statement) {
    match statement {
        Statement::Insert => println!("This is where we would do an insert."),
        Statement::Select => println!("This is where we would do a select."),
        _ => debug_assert!(false, "Unhandled command"),
    }
}