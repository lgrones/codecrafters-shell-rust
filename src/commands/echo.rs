use std::{any::Any, fmt::Display};

use crate::commands::{Command, Factory, Output};

pub struct Echo {
    args: String,
}

impl Display for Echo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Echo")
    }
}

impl Factory for Echo {
    fn new(args: Vec<String>) -> impl Command {
        Echo {
            args: args.join(" "),
        }
    }
}

impl Command for Echo {
    fn execute(&self) -> Output {
        Output::ok(self.args.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
