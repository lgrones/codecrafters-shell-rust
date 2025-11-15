use std::any::Any;

use crate::commands::{Command, Factory};

pub struct Redirect {
    args: String,
}

impl Factory for Redirect {
    fn new(args: Vec<String>) -> impl Command {
        Redirect {
            args: args.join(" "),
        }
    }
}

impl Command for Redirect {
    fn execute(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        println!("{}", self.args);
        Ok(None)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
