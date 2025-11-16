use std::{any::Any, fmt::Display};

use crate::{
    commands::{
        append::Append, cd::Cd, echo::Echo, exit::Exit, output::Output, pwd::Pwd, r#type::Type,
        redirect::Redirect, run::Run,
    },
    helper::{get_redirects, Params, RedirectType, SplitArgs, PATHS},
};

mod append;
mod cd;
mod echo;
mod exit;
mod output;
mod pwd;
mod redirect;
mod run;
mod r#type;

pub trait Command: Display {
    fn execute(&self) -> Output;
    fn as_any(&self) -> &dyn Any;
}

pub trait Factory {
    fn new(args: Vec<String>) -> impl Command;
}

const COMMANDS: &[(&str, fn(Vec<String>) -> Box<dyn Command>)] = &[
    ("cd", |args| Box::new(Cd::new(args))),
    ("echo", |args| Box::new(Echo::new(args))),
    ("exit", |args| Box::new(Exit::new(args))),
    ("pwd", |args| Box::new(Pwd::new(args))),
    ("type", |args| Box::new(Type::new(args))),
];

pub fn create_command(command: &str) -> Box<dyn Command> {
    let Params { name, mut args } = command.get_args();
    let redirect = get_redirects(&mut args);

    let command = COMMANDS
        .iter()
        .find(|(cmd, _)| *cmd == name)
        .map(|(_, f)| f(args.clone()))
        .unwrap_or_else(|| Box::new(Run::new(name, args)));

    match redirect {
        RedirectType::Redirect { capture_from, file } => {
            Box::new(Redirect::new(command, capture_from, file))
        }
        RedirectType::Append { capture_from, file } => {
            Box::new(Append::new(command, capture_from, file))
        }
        RedirectType::None => command,
    }
}

pub fn autocomplete(prefix: &str) -> Vec<String> {
    let commands = COMMANDS.iter().map(|(cmd, _)| *cmd).filter_map(|cmd| {
        if cmd.starts_with(prefix) {
            Some(cmd.to_string())
        } else {
            None
        }
    });

    let binding = PATHS.lock().unwrap();
    let paths = binding.iter().map(|(cmd, _)| cmd).filter_map(|cmd| {
        if cmd.starts_with(prefix) {
            Some(cmd.to_string())
        } else {
            None
        }
    });

    commands.chain(paths).collect()
}
