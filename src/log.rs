use std::fmt::{Display, Formatter, Result};

use colored::Colorize;

#[derive(Debug, Clone, Copy)]
pub enum Level {
    Warning,
    Error,
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Level::Warning => write!(f, "{}", "WARNING".yellow()),
            Level::Error => write!(f, "{}", "ERROR".red()),
        }
    }
}

pub fn log<T: Display>(level: Level, message: T) {
    eprintln!("{level}: {message}");
}
