use std::{
    any::Any,
    env::{self, set_current_dir},
    path::PathBuf,
};

use crate::commands::{Command, Factory};

pub struct Cd {
    path: String,
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
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path_buf = PathBuf::from(&self.path);

        if self.path.is_empty() || !path_buf.is_dir() {
            println!("cd: {}: No such file or directory", self.path);
            return Ok(());
        }

        set_current_dir(path_buf)?;
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
