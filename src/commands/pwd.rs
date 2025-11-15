use std::{any::Any, env::current_dir};

use crate::commands::{Command, Factory};

pub struct Pwd {}

impl Factory for Pwd {
    fn new(_: Vec<String>) -> impl Command {
        Pwd {}
    }
}

impl Command for Pwd {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", current_dir()?.to_str().unwrap_or("unknown wd"));
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
