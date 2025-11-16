use std::{any::Any, collections::HashSet, fmt::Display};

use crate::{
    commands::{
        append::Append, cd::Cd, echo::Echo, exit::Exit, history::History, output::Output, pwd::Pwd,
        r#type::Type, redirect::Redirect, run::Run,
    },
    helper::{get_redirects, push_history, Params, RedirectType, SplitArgs, PATHS},
};

mod append;
mod cd;
mod echo;
mod exit;
mod history;
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
    ("history", |args| Box::new(History::new(args))),
    ("pwd", |args| Box::new(Pwd::new(args))),
    ("type", |args| Box::new(Type::new(args))),
];

pub fn create_command(command: &str) -> Box<dyn Command> {
    push_history(command.to_string());

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

    let mut result: Vec<String> = commands
        .chain(paths)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    if result.len() == 1 {
        return result.iter().map(|x| x.to_string() + " ").collect();
    }

    if let Some(res) = longest_common_prefix(&mut result) {
        if res != prefix {
            result = vec![res];
        }
    }

    result.sort();

    result
}

fn longest_common_prefix(commands: &mut Vec<String>) -> Option<String> {
    if commands.is_empty() {
        return None;
    }

    commands.sort_unstable_by(|a, b| b.len().cmp(&a.len()));

    let mut prefix = commands[0].to_string();

    for command in commands.iter() {
        while !command.starts_with(&prefix) {
            if prefix.is_empty() {
                return None;
            }

            prefix.pop();
        }
    }

    Some(prefix)
}
