use std::any::Any;

use crate::commands::{Command, Factory};

pub struct Echo {
    args: String,
}

impl Factory for Echo {
    fn new(args: Vec<String>) -> impl Command {
        Echo {
            args: args.join(" "),
        }
    }
}

impl Command for Echo {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", self.args);
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
