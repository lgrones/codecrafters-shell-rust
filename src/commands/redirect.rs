use std::{
    any::Any,
    fmt::Display,
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};

use crate::commands::Command;

#[derive(PartialEq)]
pub enum RedirectFrom {
    Stdout = 1,
    Stderr = 2,
}

// impl Display for RedirectFrom {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.to_string())
//     }
// }

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
    fn execute(&self) -> Result<Option<String>, String> {
        let (redirect_from, output) = match self.command.execute() {
            Ok(out) => (RedirectFrom::Stdout, out),
            Err(err) => (RedirectFrom::Stderr, Some(err)),
        };

        if redirect_from != self.redirect_from {
            return match redirect_from {
                RedirectFrom::Stdout => Ok(output),
                RedirectFrom::Stderr => Err(output.unwrap_or_default()),
            };
        }

        if output.is_none() {
            return Err(String::from("Cannot redirect empty output"));
        }

        let path = Path::new(&self.file);
        create_dir_all(path.parent().unwrap_or(Path::new(""))).map_err(|x| x.to_string())?;

        let mut file = File::create(path).map_err(|x| x.to_string())?;
        file.write_all(output.unwrap().as_bytes())
            .map_err(|x| x.to_string())?;

        Ok(None)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
