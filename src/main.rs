use std::error::Error;
use std::io::{self, Write};

mod commands;

fn main() -> Result<(), Box<dyn Error>> {
    let mut command = String::new();

    loop {
        print!("$ ");
        io::stdout().flush()?;

        command.clear();
        io::stdin().read_line(&mut command)?;

        commands::create_command(&command).execute()?;
    }
}
