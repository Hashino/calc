use std::fmt::Display;

use colored::Colorize;

#[derive(Debug, Clone, Copy)]
pub enum Level {
    Warning,
    Error,
}

pub fn log<T: Display>(level: Level, message: T) {
    match level {
        Level::Error => {
            eprintln!("{} {message}", "ERROR:".red());
        }
        Level::Warning => {
            eprintln!("{} {message}", "WARNING:".yellow());
        }
    }
}
