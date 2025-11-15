use std::{any::Any, env::set_current_dir};

use crate::commands::{Command, Factory};

pub struct Cd {
    path: String,
}

impl Factory for Cd {
    fn new(args: Vec<String>) -> impl Command {
        Cd {
            path: args.get(0).unwrap_or(&String::new()).to_string(),
        }
    }
}

impl Command for Cd {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.path.is_empty() {
            set_current_dir(self.path.clone());
            return Ok(());
        }

        println!("cd: {}: No such file or directory", self.path);
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
