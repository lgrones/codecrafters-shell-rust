use std::{
    any::Any,
    fmt::Display,
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};

use crate::commands::{Command, Output};

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
    redirect_from: RedirectFrom,
    file: String,
}

impl Display for Redirect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Redirect")
    }
}

impl Redirect {
    pub fn new(
        command: Box<dyn Command>,
        redirect_from: RedirectFrom,
        args: Vec<String>,
    ) -> impl Command {
        Redirect {
            command,
            redirect_from,
            file: args.get(0).unwrap_or(&String::new()).to_string(),
        }
    }
}

impl Command for Redirect {
    fn execute(&self) -> Output {
        let output = self.command.execute();

        let (out, other) = match self.redirect_from {
            RedirectFrom::Stdout => (output.stdout, output.stderr),
            RedirectFrom::Stderr => (output.stderr, output.stdout),
        };

        if let Some(content) = out {
            return write_file(Path::new(&self.file), &content)
                .map(|_| Output::none())
                .unwrap_or_else(|e| Output::err(e.to_string()));
        }

        if let Some(msg) = other {
            return Output::ok(msg);
        }

        Output::none()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn write_file(path: &Path, content: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }

    File::create(path)?.write_all(content.as_bytes())?;
    Ok(())
}
