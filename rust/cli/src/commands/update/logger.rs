use std::fmt::Display;

use colored::Colorize;

pub struct UpdateLogger(String);

impl UpdateLogger {
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        println!("📦 Updating {}\n", name.bold());
        Self(name)
    }

    pub fn warn(&self, message: impl Display) {
        println!("{} {} {}\n", "⚠".yellow().bold(), self.0.bold(), message);
    }

    pub fn success(&self) {
        println!("{} {} updated {}\n", "✔".green().bold(), self.0.bold(), "successfully".green());
    }
}
