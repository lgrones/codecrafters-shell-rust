use std::{any::Any, fmt::Display};

use crate::{
    commands::{Command, Factory, Output},
    helper::get_history,
};

pub struct History {
    entries: Option<usize>,
}

impl Display for History {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "History")
    }
}

impl Factory for History {
    fn new(args: Vec<String>) -> impl Command {
        History {
            entries: args.get(0).and_then(|x| x.parse::<usize>().ok()),
        }
    }
}

impl Command for History {
    fn execute(&self) -> Output {
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
