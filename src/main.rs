use std::error::Error;
use std::io::{self, Read, Write};
use std::os::fd::AsRawFd;
use std::process::exit;

use crate::commands::Output;
use crate::raw_terminal::RawTerminal;

mod commands;
mod helper;
mod raw_terminal;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let fd = stdin.as_raw_fd();

    let mut command = String::new();
    let mut buf = [0u8; 8];
    let terminal = RawTerminal::new(fd);

    print!("$ ");
    io::stdout().flush()?;

    loop {
        stdin.lock().read(&mut buf)?;

        match &buf[..] {
            // Enter key
            [b'\n', ..] | [b'\r', ..] => {
                println!("");
                let output = execute(&mut command)?;

                if output.exit_requested {
                    terminal.restore();
                    exit(output.exit_code.unwrap_or_default());
                }
            }
            // Tab key
            [b'\t', ..] => println!("Tab pressed"),
            // Up arrow, Escape sequence: [ A
            [b'[', b' ', b'A', ..] => println!("Up arrow"),
            // Down arrow, Escape sequence: [ B
            [b'[', b' ', b'B', ..] => println!("Down arrow"),
            // Any other char
            _ => {
                command.push(buf[0] as char);
                // Clear current line and reprint prompt + buffer
                print!("\r> {}", command);
                io::stdout().flush()?;
            }
        }
    }
}

fn execute(command: &mut String) -> io::Result<Output> {
    let output = commands::create_command(command).execute();

    if let Some(ref out) = output.stdout {
        println!("{}", out.trim_end_matches("\n"));
    }

    if let Some(ref out) = output.stderr {
        println!("{}", out.trim_end_matches("\n"));
    }

    command.clear();
    print!("$ ");
    io::stdout().flush()?;

    Ok(output)
}
