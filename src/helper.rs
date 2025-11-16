use is_executable::IsExecutable;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::{env, fs, path::PathBuf};

#[derive(PartialEq)]
pub enum CaptureFrom {
    Stdout = 1,
    Stderr = 2,
}

impl CaptureFrom {
    pub fn from_digit(c: &char) -> Self {
        match c.to_digit(10) {
            Some(2) => Self::Stderr,
            _ => Self::Stdout,
        }
    }
}

#[derive(PartialEq)]
pub enum RedirectType {
    Redirect {
        capture_from: CaptureFrom,
        file: String,
    },
    Append {
        capture_from: CaptureFrom,
        file: String,
    },
    None,
}

pub fn get_redirects(args: &mut Vec<String>) -> RedirectType {
    let index = args.iter().position(|x| x.ends_with(">"));

    if index.is_none() {
        return RedirectType::None;
    }

    let redirect_args = args.split_off(index.unwrap());

    let (command, rest) = redirect_args.split_first().unwrap();

    if command.ends_with(">>") {
        return RedirectType::Append {
            capture_from: CaptureFrom::from_digit(&command.chars().next().unwrap()),
            file: rest[0].to_string(),
        };
    }

    RedirectType::Redirect {
        capture_from: CaptureFrom::from_digit(&command.chars().next().unwrap()),
        file: rest[0].to_string(),
    }
}

pub struct Params {
    pub name: String,
    pub args: Vec<String>,
}

pub trait SplitArgs {
    const QUOTES: [char; 2];
    const ESCAPE: char;
    fn get_args(&self) -> Params;
}

impl SplitArgs for &str {
    const QUOTES: [char; 2] = ['\'', '"'];
    const ESCAPE: char = '\\';

    // Yes, this is a tokenizer
    // No, don't ask me how in the fuck this works
    fn get_args(&self) -> Params {
        let mut args = vec![];
        let mut arg = vec![];
        let mut quote = None;
        let mut escaped = false;

        for char in self.trim().chars() {
            if !escaped
                && <&str as SplitArgs>::QUOTES.contains(&char)
                && quote.is_none_or(|x| x == char)
            {
                quote = match quote {
                    Some(_) => None,
                    None => Some(char),
                };

                continue;
            }

            if !escaped && char == <&str as SplitArgs>::ESCAPE {
                escaped = true;
                continue;
            }

            if quote.is_some_and(|x| x != char) && char != <&str as SplitArgs>::ESCAPE && escaped {
                arg.push(<&str as SplitArgs>::ESCAPE);
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

        let mut iter = args.into_iter();

        Params {
            name: iter.next().unwrap_or(String::new()),
            args: iter.filter(|x| !x.trim().is_empty()).collect(),
        }
    }
}

pub static PATHS: Lazy<Mutex<Vec<(String, PathBuf)>>> = Lazy::new(|| Mutex::new(vec![]));

pub fn search_path(name: &str) -> Option<PathBuf> {
    PATHS.lock().unwrap().iter().find_map(|file| {
        if file.0 == name {
            Some(file.1.clone())
        } else {
            None
        }
    })
}

pub fn precompute_path() {
    let dirs = env::var_os("PATH")
        .into_iter()
        .flat_map(|os| env::split_paths(&os).collect::<Vec<_>>());

    for dir in dirs {
        if let Ok(entries) = fs::read_dir(dir) {
            entries
                .filter_map(Result::ok)
                .filter(|file| file.path().is_executable())
                .for_each(|f| {
                    PATHS
                        .lock()
                        .unwrap()
                        .push((f.file_name().to_string_lossy().to_string(), f.path()))
                });
        }
    }

    PATHS.lock().unwrap().sort();
}
