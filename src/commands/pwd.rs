use std::{any::Any, env::current_dir, fmt::Display};

use crate::commands::{Command, Factory};

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
    fn execute(&self) -> Result<Option<String>, String> {
        let result = current_dir()
            .map_err(|x| x.to_string())?
            .to_str()
            .unwrap_or("unknown wd")
            .to_string();

        Ok(Some(result))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
