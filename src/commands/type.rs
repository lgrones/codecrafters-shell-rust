use std::{
    any::Any,
    env,
    error::Error,
    fs::{self},
};

use is_executable::IsExecutable;

use crate::commands::{create_command, noop::Noop, Command, Factory};

pub struct Type {
    name: String,
}

impl Factory for Type {
    fn new(args: Vec<String>) -> impl Command {
        Type {
            name: args.get(0).unwrap_or(&String::from("empty")).to_owned(),
        }
    }
}

impl Command for Type {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        let command = create_command(&self.name);

        if !command.as_any().is::<Noop>() {
            println!("{} is a shell builtin", self.name);
            return Ok(());
        }

        let path_command = env::var_os("PATH")
            .into_iter() // Option<OsString> -> iterator with 0 or 1 element
            .flat_map(|os| env::split_paths(&os).collect::<Vec<_>>())
            .find_map(|dir| {
                fs::read_dir(dir).ok().and_then(|entries| {
                    entries
                        .filter_map(Result::ok)
                        .find(|file| {
                            file.path().is_executable()
                                && file.file_name().to_string_lossy() == self.name
                        })
                        .map(|file| file.path())
                })
            });

        if let Some(path) = path_command {
            println!("{} is {}", self.name, path.to_string_lossy());
            return Ok(());
        }

        println!("{}: not found", self.name);
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
