use std::{any::Any, error::Error, process};

use crate::commands::{search_path, Command, Factory};

pub struct Run {
    name: String,
    args: Vec<String>,
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
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        if let Some(_) = search_path(&self.name) {
            let process = process::Command::new(&self.name).args(&self.args).spawn()?;
            let output = process.wait_with_output()?;
            print!("{}", String::from_utf8(output.stdout)?.trim());
            return Ok(());
        }

        println!("{}: command not found", self.name);
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
