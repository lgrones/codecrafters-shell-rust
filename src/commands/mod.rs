use std::{any::Any, error::Error};

use crate::{
    commands::{cd::Cd, echo::Echo, exit::Exit, pwd::Pwd, r#type::Type, run::Run},
    helper::SplitArgs,
};

mod cd;
mod echo;
mod exit;
mod pwd;
mod redirect;
mod run;
mod r#type;

pub trait Command {
    fn execute(&self) -> Result<Option<String>, Box<dyn Error>>;
    fn as_any(&self) -> &dyn Any;
}

pub trait Factory {
    fn new(args: Vec<String>) -> impl Command;
}

pub fn create_command(command: &str) -> Box<dyn Command> {
    let (name, args) = command.get_args();

    match name.as_str() {
        "cd" => Box::new(Cd::new(args)),
        "echo" => Box::new(Echo::new(args)),
        "exit" => Box::new(Exit::new(args)),
        "pwd" => Box::new(Pwd::new(args)),
        "type" => Box::new(Type::new(args)),
        _ => Box::new(Run::new(vec![name].into_iter().chain(args).collect())),
    }
}
