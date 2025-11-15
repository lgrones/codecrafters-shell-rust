use std::{
    any::Any,
    fmt::Display,
    process::{self, Stdio},
};

use crate::{
    commands::{Command, Factory, Output},
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

impl Factory for Run {
    fn new(args: Vec<String>) -> impl Command {
        let mut iter = args.into_iter();

        Run {
            name: iter.next().unwrap_or(String::from("empty")).to_owned(),
            args: iter.collect(),
        }
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
            .stdout(Stdio::piped()) // capture stdout
            .stderr(Stdio::piped())
            .spawn();

        if let Err(err) = process {
            return Output::err(err.to_string());
        }

        process
            .unwrap()
            .wait_with_output()
            .map(|res| Output {
                stdout: to_option(res.stdout),
                stderr: to_option(res.stderr),
            })
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
