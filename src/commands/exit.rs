use std::{any::Any, error::Error, fmt::Display, process::exit};

use crate::commands::{Command, Factory};

pub struct Exit {
    code: i32,
}

impl Display for Exit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Exit")
    }
}

impl Factory for Exit {
    fn new(args: Vec<String>) -> impl Command {
        Exit {
            code: args
                .get(0)
                .unwrap_or(&String::from("0"))
                .parse::<i32>()
                .unwrap_or_default(),
        }
    }
}

impl Command for Exit {
    fn execute(&self) -> Result<Option<String>, Box<dyn Error>> {
        exit(self.code)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
