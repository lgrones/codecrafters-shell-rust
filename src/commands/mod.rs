use std::{any::Any, error::Error};

use crate::commands::{echo::Echo, exit::Exit, noop::Noop, r#type::Type};

mod echo;
mod exit;
mod noop;
mod r#type;

pub trait Command {
    fn execute(&self) -> Result<(), Box<dyn Error>>;
    fn as_any(&self) -> &dyn Any;
}

pub trait Factory {
    fn new(args: Vec<String>) -> impl Command;
}

pub fn create_command(command: &str) -> Box<dyn Command> {
    let mut parts = command.trim().split_whitespace();
    let name: &str = parts.next().unwrap_or("empty");
    let args: Vec<String> = parts.map(|s| s.to_string()).collect();

    match name {
        "echo" => Box::new(Echo::new(args)),
        "exit" => Box::new(Exit::new(args)),
        "type" => Box::new(Type::new(args)),
        _ => Box::new(Noop::new(vec![name.to_string()])),
    }
}
