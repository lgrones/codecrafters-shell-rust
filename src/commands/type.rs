use std::{any::Any, error::Error};

use crate::commands::{create_command, noop::Noop, Command, Factory};

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

        if !command.as_any().is::<Noop>() {
            println!("{} is a shell builtin", self.name);
        } else {
            println!("{}: not found", self.name);
        }

        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
