use std::{any::Any, fmt::Display};

use crate::commands::{Command, Factory};

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
    fn execute(&self) -> Result<Option<String>, String> {
        Ok(Some(self.args.clone()))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
