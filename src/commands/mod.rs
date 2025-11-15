use std::{any::Any, fmt::Display};

use crate::{
    commands::{
        append::Append, cd::Cd, echo::Echo, exit::Exit, pwd::Pwd, r#type::Type, redirect::Redirect,
        run::Run,
    },
    helper::{get_redirects, Params, RedirectType, SplitArgs},
};

mod append;
mod cd;
mod echo;
mod exit;
mod pwd;
mod redirect;
mod run;
mod r#type;

pub struct Output {
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub exit_code: Option<i32>,
}

impl Output {
    pub fn none() -> Self {
        Output {
            stdout: None,
            stderr: None,
            exit_code: None,
        }
    }

    pub fn ok(msg: String) -> Self {
        Output {
            stdout: Some(msg.to_string()),
            stderr: None,
            exit_code: None,
        }
    }

    pub fn err(msg: String) -> Self {
        Output {
            stdout: None,
            stderr: Some(msg.to_string()),
            exit_code: None,
        }
    }

    pub fn out(out: Option<String>, err: Option<String>) -> Self {
        Output {
            stdout: out,
            stderr: err,
            exit_code: None,
        }
    }

    pub fn exit(code: i32) -> Self {
        Output {
            stdout: None,
            stderr: None,
            exit_code: Some(code),
        }
    }
}

pub trait Command: Display {
    fn execute(&self) -> Output;
    fn as_any(&self) -> &dyn Any;
}

pub trait Factory {
    fn new(args: Vec<String>) -> impl Command;
}

pub fn create_command(command: &str) -> Box<dyn Command> {
    let Params { name, mut args } = command.get_args();
    let redirect = get_redirects(&mut args);

    let command = match name.as_str() {
        "cd" => Box::new(Cd::new(args)) as Box<dyn Command>,
        "echo" => Box::new(Echo::new(args)),
        "exit" => Box::new(Exit::new(args)),
        "pwd" => Box::new(Pwd::new(args)),
        "type" => Box::new(Type::new(args)),
        _ => Box::new(Run::new(name, args)),
    };

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
