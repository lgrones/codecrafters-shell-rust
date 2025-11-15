use std::{
    any::Any,
    error::Error,
    fmt::Display,
    process::{self, Stdio},
};

use crate::{
    commands::{Command, Factory},
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
    fn execute(&self) -> Result<Option<String>, Box<dyn Error>> {
        if let Some(_) = search_path(&self.name) {
            let process = process::Command::new(&self.name)
                .args(&self.args)
                .stdout(Stdio::piped()) // capture stdout
                .stderr(Stdio::piped())
                .spawn()?;
            let output = process.wait_with_output()?;
            let result = format!(
                "{}",
                String::from_utf8(output.stdout)?.trim_end_matches('\n')
            );
            return Ok(Some(result));
        }

        let result = format!("{}: command not found", self.name);
        Ok(Some(result))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
