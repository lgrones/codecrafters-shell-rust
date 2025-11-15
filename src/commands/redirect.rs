use std::{
    any::Any,
    fmt::Display,
    fs::{create_dir_all, File},
    io::{Error, ErrorKind, Write},
    path::Path,
};

use crate::commands::Command;

pub enum RedirectFrom {
    Stdout = 1,
    Stderr = 2,
}

impl RedirectFrom {
    pub fn from_digit(c: &char) -> Self {
        match c.to_digit(10) {
            Some(2) => Self::Stderr,
            _ => Self::Stdout,
        }
    }
}

pub struct Redirect {
    command: Box<dyn Command>,
    file: String,
}

impl Display for Redirect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Redirect")
    }
}

impl Redirect {
    pub fn new(command: Box<dyn Command>, args: Vec<String>) -> impl Command {
        Redirect {
            command,
            file: args.get(0).unwrap_or(&String::new()).to_string(),
        }
    }
}

impl Command for Redirect {
    fn execute(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let output = self.command.execute()?;

        if output.is_none() {
            return Err(Box::new(Error::new(
                ErrorKind::InvalidInput,
                "Cannot redirect empty output",
            )));
        }

        let path = Path::new(&self.file);
        create_dir_all(path.parent().unwrap_or(Path::new("")))?;

        let mut file = File::create(path)?;
        file.write_all(output.unwrap().as_bytes())?;

        Ok(None)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
