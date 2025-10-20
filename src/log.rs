use colored::Colorize;

#[derive(Debug, Clone, Copy)]
pub enum Level {
    Warning,
    Error,
}

pub fn log(level: Level, message: &str) {
    match level {
        Level::Error => {
            eprintln!("{}", format!("ERROR: {}", message).red());
        }
        Level::Warning => {
            eprintln!("{}", format!("WARN: {}", message).yellow());
        }
    }
}
