use std::{any::Any, env::current_dir, fmt::Display};

use crate::commands::{Command, Factory, Output};

pub struct Pwd {}

impl Display for Pwd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pwd")
    }
}

impl Factory for Pwd {
    fn new(_: Vec<String>) -> impl Command {
        Pwd {}
    }
}

impl Command for Pwd {
    fn execute(&self) -> Output {
        match current_dir() {
            Ok(res) => Output::ok(res.to_string_lossy().to_string()),
            Err(err) => Output::err(err.to_string()),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
