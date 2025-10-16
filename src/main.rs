use std::{
    io::{BufRead, stdin},
    process::exit,
};

use crate::calc::calculator::evaluate_with_debug;
use crate::log::{Level, log};

mod calc {
    pub mod calculator;
    pub mod parser;

    #[cfg(test)]
    mod tests;
}

mod log;

use clap::Parser;

#[derive(Parser)]
#[command(about, long_about = None)]
struct Cli {
    /// evaluates expression from command line instead of interactive mode
    #[arg(short, long)]
    input: Option<String>,

    /// displays AST for each expression (for debugging)
    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

fn show_help() {
    println!("Available commands:");
    println!("help - Show this help message");
    println!("quit - Exit the program");
    println!();
    println!("Usage:");
    println!("<expression> - Calculate the result of the expression");
    println!("Examples:");
    println!(" 2 + 3          -> 5");
    println!(" 10 - 4         -> 6");
    println!(" 5 * 6          -> 30");
    println!(" 8 / 2          -> 4");
    println!(" (2 + 3) * 4    -> 20");
}

fn format_result(res: f64) -> String {
    match res {
        r if r.fract() == 0.0 => format!("{}", r as i64),
        r if r.is_nan() => "NaN".to_string(),
        r if r.is_infinite() && r.is_sign_positive() => "Infinity".to_string(),
        r if r.is_infinite() && r.is_sign_negative() => "-Infinity".to_string(),
        r => format!("{}", r),
    }
}

fn main() {
    let cli = Cli::parse();

    // If an input expression is provided via CLI, evaluate it and exit
    if let Some(input) = cli.input.as_deref() {
        match evaluate_with_debug(input.to_string(), cli.debug) {
            Ok(res) => {
                if !res.is_nan() {
                    println!("{}", format_result(res));
                    exit(0);
                } else {
                    println!("{}", format_result(res));
                    exit(1);
                }
            }
            Err(e) => {
                log(Level::Error, &format!("{:?}", e));
                exit(1);
            }
        }
    }

    let mut stdin = stdin().lock();
    let mut buffer = String::new();

    loop {
        buffer.clear();
        if let Err(e) = stdin.read_line(&mut buffer) {
            log(Level::Error, &e.to_string());
            exit(1);
        }

        match buffer.trim() {
            "help" | "h" => show_help(),
            "quit" | "q" => exit(0),
            _ => {
                match evaluate_with_debug(buffer.clone(), cli.debug) {
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
}
