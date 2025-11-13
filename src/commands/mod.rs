use std::error::Error;

use crate::commands::{echo::Echo, exit::Exit, noop::Noop};

mod echo;
mod exit;
mod noop;

pub trait Command {
    fn execute(&self) -> Result<(), Box<dyn Error>>;
}

pub fn evaluate_command(command: &str) -> Result<(), String> {
    let parts: Vec<&str> = command.trim().split_whitespace().collect();

    let cmd: Box<dyn Command> = match parts.as_slice() {
        ["exit", args @ ..] => Box::new(Exit::new(args)),
        ["echo", args @ ..] => Box::new(Echo::new(args)),
        [args @ ..] => Box::new(Noop::new(args)),
    };

    cmd.execute().map_err(|x| x.to_string())
}
