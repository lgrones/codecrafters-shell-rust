use std::{
    env,
    fs::{create_dir_all, File},
    io::{self, ErrorKind, Read, Write},
    path::PathBuf,
    sync::Mutex,
};

use once_cell::sync::Lazy;

use crate::output::Output;

static HISTORY: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(vec![]));
static HISTORY_POINTER: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));
static NEW_HISTORY_POINTER: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));

#[derive(PartialEq)]
pub enum HistoryFlag {
    READ,
    WRITE,
    APPEND,
    NONE,
}

pub fn get_history(n: Option<usize>) -> (usize, Vec<String>) {
    let history = HISTORY.lock().unwrap();

    (
        history.len(),
        history
            .iter()
            .rev()
            .take(n.unwrap_or(history.len()))
            .map(|x| x.to_string())
            .collect(),
    )
}

pub fn push_history(command: String) {
    let mut history = HISTORY.lock().unwrap();
    history.push(command.to_string());
    *HISTORY_POINTER.lock().unwrap() = history.len();
}

pub fn prev_history() -> Option<String> {
    let mut pointer = HISTORY_POINTER.lock().unwrap();

    if *pointer == 0 {
        return None;
    }

    *pointer -= 1;
    HISTORY.lock().unwrap().get(*pointer).map(|x| x.to_string())
}

pub fn next_history() -> Option<String> {
    let history = HISTORY.lock().unwrap();
    let mut pointer = HISTORY_POINTER.lock().unwrap();

    if *pointer >= history.len() - 1 {
        return None;
    }

    *pointer += 1;
    history.get(*pointer).map(|x| x.to_string())
}

pub fn io_history(flag: &HistoryFlag, path: &PathBuf) -> Output {
    match flag {
        HistoryFlag::READ => read_history(path),
        HistoryFlag::WRITE => write_history(path),
        HistoryFlag::APPEND => append_history(path),
        HistoryFlag::NONE => Output::none(),
    }
}

fn read_history(path: &PathBuf) -> Output {
    let result = File::open(path);

    if let Err(err) = result {
        return Output::err(err.to_string());
    }

    let mut buffer = String::new();

    if let Err(err) = result.unwrap().read_to_string(&mut buffer) {
        return Output::err(err.to_string());
    }

    let commands = buffer.split("\n").filter_map(|x| {
        if !x.trim().is_empty() {
            Some(x.to_string())
        } else {
            None
        }
    });

    let mut history = HISTORY.lock().unwrap();
    history.extend(commands);
    *HISTORY_POINTER.lock().unwrap() = history.len();
    *NEW_HISTORY_POINTER.lock().unwrap() = history.len();

    Output::none()
}

fn write_history(path: &PathBuf) -> Output {
    if let Some(parent) = path.parent() {
        if let Err(err) = create_dir_all(parent) {
            return Output::err(err.to_string());
        }
    }

    let result = File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path);

    if let Err(err) = result {
        return Output::err(err.to_string());
    }

    let history = HISTORY.lock().unwrap();
    let content = history.join("\n") + "\n";
    *NEW_HISTORY_POINTER.lock().unwrap() = history.len();

    match result.unwrap().write_all(content.as_bytes()) {
        Ok(_) => Output::none(),
        Err(err) => Output::err(err.to_string()),
    }
}

fn append_history(path: &PathBuf) -> Output {
    let result = File::options().create(true).append(true).open(path);

    if let Err(err) = result {
        return Output::err(err.to_string());
    }

    let history = HISTORY.lock().unwrap();
    let mut new_history = *NEW_HISTORY_POINTER.lock().unwrap();
    let content = history
        .iter()
        .skip(new_history)
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";

    new_history = history.len();

    match result.unwrap().write_all(content.as_bytes()) {
        Ok(_) => Output::none(),
        Err(err) => Output::err(err.to_string()),
    }
}

pub fn save_history() -> io::Result<()> {
    let path = env::var("HISTFILE").map(|x| PathBuf::from(x)).ok();

    if path.is_none() {
        return Ok(());
    }

    let result = append_history(&path.unwrap());

    if let Some(err) = result.stderr {
        return Err(io::Error::new(ErrorKind::Other, err));
    }

    Ok(())
}

pub fn load_history() -> io::Result<()> {
    let path = env::var("HISTFILE").map(|x| PathBuf::from(x)).ok();

    if path.is_none() {
        return Ok(());
    }

    let result = read_history(&path.unwrap());

    if let Some(err) = result.stderr {
        return Err(io::Error::new(ErrorKind::Other, err));
    }

    Ok(())
}
