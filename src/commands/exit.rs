use std::{any::Any, fmt::Display};

use crate::commands::{Command, Factory, Output};

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
    fn execute(&self) -> Output {
        Output::exit(self.code)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
