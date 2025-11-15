use std::{env, fs, path::PathBuf};

use is_executable::IsExecutable;

pub trait SplitArgs {
    fn get_args(&self) -> (String, Vec<String>);
}

impl SplitArgs for &str {
    // Yes, this is a tokenizer
    // No, don't ask me how in the fuck this works
    fn get_args(&self) -> (String, Vec<String>) {
        let quotes = ['\'', '"'];
        let escape = '\\';
        let mut result = vec![];
        let mut arg = vec![];
        let mut quote = None;
        let mut escaped = false;

        for char in self.trim().chars() {
            if !escaped && quotes.contains(&char) && quote.is_none_or(|x| x == char) {
                quote = match quote {
                    Some(_) => None,
                    None => Some(char),
                };

                continue;
            }

            if !escaped && char == escape {
                escaped = true;
                continue;
            }

            if quote.is_some_and(|x| x != char) && char != escape && escaped {
                arg.push(escape);
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
