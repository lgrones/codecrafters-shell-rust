use std::{env, fs, path::PathBuf};

use is_executable::IsExecutable;

use crate::commands::redirect::RedirectFrom;

pub trait SplitArgs {
    fn get_args(&self) -> (String, Vec<String>, Option<RedirectFrom>, Vec<String>);
}

impl SplitArgs for &str {
    // Yes, this is a tokenizer
    // No, don't ask me how in the fuck this works
    fn get_args(&self) -> (String, Vec<String>, Option<RedirectFrom>, Vec<String>) {
        let quotes = ['\'', '"'];
        let escape = '\\';
        let mut args = vec![];
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

            args.push(arg.iter().collect::<String>());
            arg.clear();
        }

        args.push(arg.iter().collect::<String>());
        let iter = args.iter();

        let redirect_option = iter.enumerate().find_map(|(i, x)| {
            if x.ends_with(">") {
                return Some((RedirectFrom::from_digit(&x.chars().next().unwrap()), i));
            }

            None
        });

        let mut redirect_args = vec![];
        let mut redirect = None;

        if let Some((r, index)) = redirect_option {
            let split = args.as_slice().split_at(index);

            redirect_args = split.1.into_iter().skip(1).map(|x| x.to_string()).collect();
            args = split.0.into_iter().map(|x| x.to_string()).collect();

            redirect = Some(r);
        }

        let mut result_iter = args.into_iter();

        (
            result_iter.next().unwrap_or(String::new()),
            result_iter.filter(|x| !x.trim().is_empty()).collect(),
            redirect,
            redirect_args,
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
