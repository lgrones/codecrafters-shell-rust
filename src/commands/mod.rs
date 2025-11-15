use std::{any::Any, env, error::Error, fs, path::PathBuf};

use is_executable::IsExecutable;

use crate::commands::{cd::Cd, echo::Echo, exit::Exit, pwd::Pwd, r#type::Type, run::Run};

mod cd;
mod echo;
mod exit;
mod pwd;
mod run;
mod r#type;

pub trait Command {
    fn execute(&self) -> Result<(), Box<dyn Error>>;
    fn as_any(&self) -> &dyn Any;
}

pub trait Factory {
    fn new(args: Vec<String>) -> impl Command;
}

trait SplitArgs {
    fn get_args(&self) -> (String, Vec<String>);
}

impl SplitArgs for &str {
    fn get_args(&self) -> (String, Vec<String>) {
        let quotes = ['\'', '"'];
        let mut result = vec![];
        let mut quote = None;
        let mut escaped = false;
        let mut arg = vec![];

        for char in self.trim().chars() {
            if !escaped && quotes.contains(&char) && quote.is_none_or(|x| x == char) {
                quote = match quote {
                    Some(_) => None,
                    None => Some(char),
                };

                continue;
            }

            if !escaped && char == '\\' {
                escaped = true;
                continue;
            }

            if quote.is_some_and(|x| x == char) && escaped {
                arg.push(char);
                escaped = false;
                continue;
            }

            if quote.is_some() || escaped || char != ' ' {
                arg.push(char);
                escaped = false;
                continue;
            }

            result.push(arg.iter().collect::<String>());
            arg.clear();
        }

        result.push(arg.iter().collect::<String>());
        let mut iter = result.into_iter();

        (
            iter.next().unwrap_or(String::new()),
            iter.filter(|x| !x.trim().is_empty()).collect(),
        )
    }
}

pub fn create_command(command: &str) -> Box<dyn Command> {
    let (name, args) = command.get_args();

    match name.as_str() {
        "cd" => Box::new(Cd::new(args)),
        "echo" => Box::new(Echo::new(args)),
        "exit" => Box::new(Exit::new(args)),
        "pwd" => Box::new(Pwd::new(args)),
        "type" => Box::new(Type::new(args)),
        _ => Box::new(Run::new(vec![name].into_iter().chain(args).collect())),
    }
}

pub fn search_path(name: &str) -> Option<PathBuf> {
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
