use regex::Regex;
use std::io::{self, Write};
use std::{error::Error, process::exit};

fn main() -> Result<(), Box<dyn Error>> {
    let mut command = String::new();

    loop {
        print!("$ ");
        io::stdout().flush()?;

        command.clear();
        io::stdin().read_line(&mut command)?;

        evaluate_command(&command)?;
    }
}

fn evaluate_command(command: &str) -> Result<(), Box<dyn Error>> {
    let parts: Vec<&str> = Regex::new(r"\s+")?.split(command.trim()).collect();

    match parts[0] {
        "exit" => exit(parts[1].parse::<i32>()?),
        _ => println!("{}: command not found", command.trim()),
    }

    Ok(())
}
