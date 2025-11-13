use std::{any::Any, error::Error};

use crate::commands::{create_command, run::Run, search_path, Command, Factory};

pub struct Type {
    name: String,
}

impl Factory for Type {
    fn new(args: Vec<String>) -> impl Command {
        Type {
            name: args.get(0).unwrap_or(&String::from("empty")).to_owned(),
        }
    }
}

impl Command for Type {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        let command = create_command(&self.name);

        if !command.as_any().is::<Run>() {
            println!("{} is a shell builtin", self.name);
            return Ok(());
        }

        if let Some(path) = search_path(&self.name) {
            println!("{} is {}", self.name, path.to_string_lossy());
            return Ok(());
        }

        println!("{}: not found", self.name);
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
