use std::error::Error;

use crate::commands::Command;

pub struct Noop {
    name: String,
}

impl Noop {
    pub fn new(args: &[&str]) -> Self {
        Noop {
            name: args.get(0).unwrap_or(&"empty").to_string(),
        }
    }
}

impl Command for Noop {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        println!("{}: command not found", self.name);
        Ok(())
    }
}
