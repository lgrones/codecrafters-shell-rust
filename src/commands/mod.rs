use std::{any::Any, fmt::Display};

use crate::{
    commands::{
        cd::Cd, echo::Echo, exit::Exit, pwd::Pwd, r#type::Type, redirect::Redirect, run::Run,
    },
    helper::SplitArgs,
};

mod cd;
mod echo;
mod exit;
mod pwd;
pub(crate) mod redirect;
mod run;
mod r#type;

pub trait Command: Display {
    fn execute(&self) -> Result<Option<String>, String>;
    fn as_any(&self) -> &dyn Any;
}

pub trait Factory {
    fn new(args: Vec<String>) -> impl Command;
}

pub fn create_command(command: &str) -> Box<dyn Command> {
    let (name, args, redirect_from, redirect_args) = command.get_args();

    let command = match name.as_str() {
        "cd" => Box::new(Cd::new(args)) as Box<dyn Command>,
        "echo" => Box::new(Echo::new(args)),
        "exit" => Box::new(Exit::new(args)),
        "pwd" => Box::new(Pwd::new(args)),
        "type" => Box::new(Type::new(args)),
        _ => Box::new(Run::new(vec![name].into_iter().chain(args).collect())),
    };

    if redirect_from.is_none() {
        return command;
    }

    Box::new(Redirect::new(
        command,
        redirect_from.unwrap(),
        redirect_args,
    ))
}
