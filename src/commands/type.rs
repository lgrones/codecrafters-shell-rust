use std::{any::Any, fmt::Display};

use crate::{
    commands::{create_command, run::Run, Command, Factory, Output},
    helper::search_path,
};

pub struct Type {
    name: String,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Type")
    }
}

impl Factory for Type {
    fn new(args: Vec<String>) -> impl Command {
        Type {
            name: args.get(0).unwrap_or(&String::from("empty")).to_owned(),
        }
    }
}

impl Command for Type {
    fn execute(&self) -> Output {
        let command = create_command(&self.name);

        if !command.as_any().is::<Run>() {
            let result = format!("{} is a shell builtin", self.name);
            return Output::ok(result);
        }

        if let Some(path) = search_path(&self.name) {
            let result = format!("{} is {}", self.name, path.to_string_lossy());
            return Output::ok(result);
        }

        let result = format!("{}: not found", self.name);
        Output::err(result)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
