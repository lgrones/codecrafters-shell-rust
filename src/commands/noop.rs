use std::{any::Any, error::Error};

use crate::commands::{Command, Factory};

pub struct Noop {
    name: String,
}

impl Factory for Noop {
    fn new(args: Vec<String>) -> impl Command {
        Noop {
            name: args.get(0).unwrap_or(&String::from("empty")).to_owned(),
        }
    }
}

impl Command for Noop {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        println!("{}: command not found", self.name);
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
