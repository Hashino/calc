// Main module for the calculator CLI application
use std::{
    io::{BufRead, stdin},
    process::exit,
};

use crate::calc::calculator::evaluate;

// Module declaration for the calculator logic
mod calc {
    pub mod calculator;
    pub mod parser;

    #[cfg(test)]
    mod tests;
}

// Using clap for command-line argument parsing
use clap::Parser;

// Command-line interface structure
#[derive(Parser)]
#[command(about, long_about = None)]
struct Cli {
    /// evaluates expression from command line instead of interactive mode
    #[arg(short, long)]
    input: Option<String>,
}

// Function to display help information for the interactive mode
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

// Main entry point of the application
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let cli = Cli::parse();

    // If an input expression is provided via CLI, evaluate it and exit
    if let Some(input) = cli.input.as_deref() {
        match evaluate(input.to_string()) {
            Ok(res) => {
                println!("{res:?}");
                exit(0);
            }
            Err(e) => {
                eprintln!("Error: {e}");
                exit(1);
            }
        }
    }

    // Interactive mode: read lines from stdin and evaluate expressions
    let mut stdin = stdin().lock();
    let mut buffer = String::new();

    loop {
        buffer.clear();
        stdin.read_line(&mut buffer)?;

        match buffer.trim() {
            "help" | "h" => show_help(),
            "quit" | "q" => exit(0),
            _ => match evaluate(buffer.clone()) {
                Ok(res) => println!("{res:?}"),
                Err(e) => eprintln!("Error: {e}"),
            },
        }
    }
}
