use std::{any::Any, env::current_dir};

use crate::commands::{Command, Factory};

pub struct Pwd {}

impl Factory for Pwd {
    fn new(_: Vec<String>) -> impl Command {
        Pwd {}
    }
}

impl Command for Pwd {
    fn execute(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let result = current_dir()?.to_str().unwrap_or("unknown wd").to_string();
        Ok(Some(result))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
