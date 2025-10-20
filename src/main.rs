use std::process::exit;

use crate::calc::calculator::evaluate;
use crate::log::{Level, log};

mod calc {
    pub mod calculator;
    pub mod parser;

    #[cfg(test)]
    mod tests;
}

mod log;

use clap::Parser;
use colored::Colorize;
use rustyline::{self, DefaultEditor, error::ReadlineError};

#[derive(Parser)]
#[command(about, long_about = None)]
struct Cli {
    /// evaluates expression from command line instead of interactive mode
    #[arg(short, long)]
    input: Option<String>,
}

fn show_help() {
    println!("{}", "Available commands:".cyan().bold());
    println!("  {} - {}", "help".green(), "Show this help message");
    println!("  {} - {}", "quit".green(), "Exit the program");
    println!();
    println!("{}", "Usage:".cyan().bold());
    println!(
        "  {} - {}",
        "<expression>".yellow(),
        "Calculate the result of the expression"
    );
    println!();
    println!("{}", "Examples:".cyan().bold());
    println!("  {} {} {}", "2 + 3".yellow(), "->".blue(), "5".green());
    println!("  {} {} {}", "10 - 4".yellow(), "->".blue(), "6".green());
    println!("  {} {} {}", "5 * 6".yellow(), "->".blue(), "30".green());
    println!("  {} {} {}", "8 / 2".yellow(), "->".blue(), "4".green());
    println!(
        "  {} {} {}",
        "(2 + 3) * 4".yellow(),
        "->".blue(),
        "20".green()
    );
    println!("  {} {} {}", "sqrt 16".yellow(), "->".blue(), "4".green());
    println!("  {} {} {}", "floor 3.7".yellow(), "->".blue(), "3".green());
    println!("  {} {} {}", "ceil 3.2".yellow(), "->".blue(), "4".green());
    println!(
        "  {} {} {}",
        "abs (0 - 5)".yellow(),
        "->".blue(),
        "5".green()
    );
    println!("  {} {} {}", "round 3.6".yellow(), "->".blue(), "4".green());
}

fn format_result(res: f64) -> String {
    match res {
        r if r.fract() == 0.0 => format!("{}", r as i64),
        r => format!("{}", r),
    }
}

fn main() {
    let cli = Cli::parse();

    // If an input expression is provided via CLI, evaluate it and exit
    if let Some(input) = cli.input.as_deref() {
        match evaluate(input.to_string()) {
            Ok(res) => {
                println!("{}", format_result(res));
                exit(!res.is_nan() as i32);
            }
            Err(_) => exit(1),
        }
    } else {
        let mut editor = DefaultEditor::new().unwrap();
        let prompt = format!("{} ", ">".purple());

        loop {
            match editor.readline(&prompt) {
                Ok(line) => {
                    editor.add_history_entry(line.as_str()).unwrap();
                    match line.trim() {
                        "help" | "h" => show_help(),
                        "quit" | "q" => exit(0),
                        _ => {
                            match evaluate(line) {
                                Ok(res) => {
                                    println!("{}", format_result(res));
                                }
                                Err(e) => {
                                    log(Level::Error, &format!("{:?}", e));
                                    continue;
                                }
                            };
                        }
                    }
                }
                Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                    exit(0);
                }
                Err(err) => {
                    log(Level::Error, &format!("Error reading input: {err}"));
                    exit(1);
                }
            }
        }
    }
}
