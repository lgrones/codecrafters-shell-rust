use std::{any::Any, fmt::Display, path::PathBuf};

use crate::{
    commands::{Command, Factory, Output},
    history::{get_history, io_history, HistoryFlag},
};

pub struct History {
    entries: Option<usize>,
    flag: HistoryFlag,
    path: Option<PathBuf>,
}

impl Display for History {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "History")
    }
}

impl Factory for History {
    fn new(args: Vec<String>) -> impl Command {
        let arg = args.get(0).unwrap_or(&String::new()).to_owned();
        let path = args.get(1).map(|x| PathBuf::from(x));

        let flag = match arg.as_str() {
            "-r" => HistoryFlag::READ,
            "-w" => HistoryFlag::WRITE,
            "-a" => HistoryFlag::APPEND,
            _ => HistoryFlag::NONE,
        };

        History {
            entries: arg.parse::<usize>().ok(),
            flag,
            path,
        }
    }
}

impl Command for History {
    fn execute(&self) -> Output {
        if self.flag != HistoryFlag::NONE {
            return io_history(&self.flag, &self.path.clone().unwrap());
        }

        let (length, history) = get_history(self.entries);

        let commands = history
            .iter()
            .enumerate()
            .map(|(index, command)| format!("    {}  {command}\r\n", length - index))
            .rev()
            .collect::<Vec<_>>()
            .join("");

        Output::ok(commands)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl History {}
