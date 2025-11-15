use std::{any::Any, error::Error};

use crate::{
    commands::{create_command, run::Run, Command, Factory},
    helper::search_path,
};

pub struct Type {
    name: String,
}

impl Factory for Type {
    fn new(args: Vec<String>) -> impl Command {
        Type {
            name: args.get(0).unwrap_or(&String::from("empty")).to_owned(),
        }
    }
}

impl Command for Type {
    fn execute(&self) -> Result<Option<String>, Box<dyn Error>> {
        let command = create_command(&self.name);

        if !command.as_any().is::<Run>() {
            let result = format!("{} is a shell builtin", self.name);
            return Ok(Some(result));
        }

        if let Some(path) = search_path(&self.name) {
            let result = format!("{} is {}", self.name, path.to_string_lossy());
            return Ok(Some(result));
        }

        let result = format!("{}: not found", self.name);
        Ok(Some(result))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
