use std::{
    any::Any,
    fmt::Display,
    process::{self, Stdio},
};

use crate::{
    commands::{Command, Output},
    helper::search_path,
};

pub struct Run {
    name: String,
    args: Vec<String>,
}

impl Display for Run {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Run")
    }
}

impl Run {
    pub fn new(name: String, args: Vec<String>) -> impl Command {
        Run { name, args }
    }
}

impl Command for Run {
    fn execute(&self) -> Output {
        if search_path(&self.name).is_none() {
            let result = format!("{}: command not found", self.name);
            return Output::err(result);
        }

        let process = process::Command::new(&self.name)
            .args(&self.args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        if let Err(err) = process {
            return Output::err(err.to_string());
        }

        process
            .unwrap()
            .wait_with_output()
            .map(|res| Output::out(to_option(res.stdout), to_option(res.stderr)))
            .unwrap_or_else(|err| Output::err(err.to_string()))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn to_option(s: Vec<u8>) -> Option<String> {
    let s = String::from_utf8_lossy(&s).to_string();

    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}
