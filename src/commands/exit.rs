use std::{error::Error, process::exit};

use crate::commands::Command;

pub struct Exit {
    code: i32,
}

impl Exit {
    pub fn new(args: &[&str]) -> Self {
        Exit {
            code: args
                .get(0)
                .unwrap_or(&"0")
                .parse::<i32>()
                .unwrap_or_default(),
        }
    }
}

impl Command for Exit {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        exit(self.code)
    }
}
