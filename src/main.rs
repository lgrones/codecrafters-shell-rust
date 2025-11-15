use std::error::Error;
use std::io::{self, Write};

mod commands;
mod helper;

fn main() -> Result<(), Box<dyn Error>> {
    let mut command = String::new();

    loop {
        print!("$ ");
        io::stdout().flush()?;

        command.clear();
        io::stdin().read_line(&mut command)?;

        let output = commands::create_command(&command).execute();

        if let Some(out) = output.stdout {
            println!("{}", out.trim_end_matches("\n"));
        }

        if let Some(out) = output.stderr {
            println!("{}", out.trim_end_matches("\n"));
        }
    }
}
