use std::{any::Any, env, error::Error, fs};

use is_executable::IsExecutable;

use crate::commands::{echo::Echo, exit::Exit, r#type::Type, run::Run};

mod echo;
mod exit;
mod run;
mod r#type;

pub trait Command {
    fn execute(&self) -> Result<(), Box<dyn Error>>;
    fn as_any(&self) -> &dyn Any;
}

pub trait Factory {
    fn new(args: Vec<String>) -> impl Command;
}

pub fn create_command(command: &str) -> Box<dyn Command> {
    let mut parts = command.trim().split_whitespace();
    let name: &str = parts.next().unwrap_or("empty");
    let args: Vec<String> = parts.map(|s| s.to_string()).collect();

    match name {
        "echo" => Box::new(Echo::new(args)),
        "exit" => Box::new(Exit::new(args)),
        "type" => Box::new(Type::new(args)),
        _ => Box::new(Run::new(
            vec![name.to_string()].into_iter().chain(args).collect(),
        )),
    }
}

pub fn search_path(name: &str) -> Option<std::path::PathBuf> {
    env::var_os("PATH")
        .into_iter() // Option<OsString> -> iterator with 0 or 1 element
        .flat_map(|os| env::split_paths(&os).collect::<Vec<_>>())
        .find_map(|dir| {
            fs::read_dir(dir).ok().and_then(|entries| {
                entries
                    .filter_map(Result::ok)
                    .find(|file| {
                        file.path().is_executable() && file.file_name().to_string_lossy() == name
                    })
                    .map(|file| file.path())
            })
        })
}
