use std::{
    any::Any,
    env::{self, set_current_dir},
    fmt::Display,
    path::PathBuf,
};

use crate::commands::{Command, Factory, Output};

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
    fn execute(&self) -> Output {
        let path_buf = PathBuf::from(&self.path);

        if self.path.is_empty() || !path_buf.is_dir() {
            let result = format!("cd: {}: No such file or directory", self.path);
            return Output::err(result);
        }

        match set_current_dir(path_buf) {
            Ok(_) => Output::none(),
            Err(err) => Output::err(err.to_string()),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
