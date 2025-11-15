use std::{
    any::Any,
    env::{self, set_current_dir},
    fmt::Display,
    path::PathBuf,
};

use crate::commands::{Command, Factory};

pub struct Cd {
    path: String,
}

impl Display for Cd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cd")
    }
}

impl Factory for Cd {
    fn new(args: Vec<String>) -> impl Command {
        let mut path = args.get(0).unwrap_or(&String::new()).to_string();

        if path == "~" {
            path = env::var_os("HOME")
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .to_string()
        }

        Cd { path }
    }
}

impl Command for Cd {
    fn execute(&self) -> Result<Option<String>, String> {
        let path_buf = PathBuf::from(&self.path);

        if self.path.is_empty() || !path_buf.is_dir() {
            let result = format!("cd: {}: No such file or directory", self.path);
            return Ok(Some(result));
        }

        set_current_dir(path_buf).map_err(|x| x.to_string())?;
        Ok(None)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
