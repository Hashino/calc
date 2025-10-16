use std::{
    io::{BufRead, stdin},
    process::exit,
};

use crate::calc::calculator::evaluate;

mod calc {
    pub mod calculator;
    pub mod parser;

    #[cfg(test)]
    mod tests;
}

fn show_help() {
    println!("Available commands:");
    println!("help - Show this help message");
    println!("quit - Exit the program");
    println!();
    println!("Usage:");
    println!("<expression> - Calculate the result of the expression");
    println!("Examples:");
    println!("  2 + 3          -> 5");
    println!("  10 - 4         -> 6");
    println!("  5 * 6          -> 30");
    println!("  8 / 2          -> 4");
    println!("  (2 + 3) * 4    -> 20");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdin = stdin().lock();
    let mut buffer = String::new();

    loop {
        buffer.clear();
        stdin.read_line(&mut buffer)?;

        match buffer.trim() {
            "help" | "h" => show_help(),
            "quit" | "q" => exit(0),
            _ => {
                match evaluate(buffer.clone()) {
                    Ok(res) => println!("{res:?}"),
                    Err(e) => eprintln!("Error: {e}"),
                }
            }
        }
    }
}
