use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::{
    io::{self, Write},
    process::exit,
};

mod commands;
mod helper;

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut command = String::new();

    print!("$ ");
    io::stdout().flush()?;

    loop {
        if let Ok(event) = event::read() {
            match event {
                Event::Paste(text) => {
                    command.push_str(&text);
                    print!("{}", text);
                    io::stdout().flush()?;
                }

                Event::Key(KeyEvent { code, .. }) => match code {
                    KeyCode::Tab => {
                        if let Some(candidate) = commands::autocomplete(&command) {
                            command = String::from(candidate) + " ";
                            print!("\x1b[2K\r$ {command}");
                            io::stdout().flush()?;
                            continue;
                        }

                        print!("\x07");
                        io::stdout().flush()?;
                    }

                    KeyCode::Up => {
                        println!("\r\n[Up triggered for: '{}']\r\n", command);
                        io::stdout().flush()?;
                    }

                    KeyCode::Down => {
                        println!("\r\n[Down triggered for: '{}']\r\n", command);
                        io::stdout().flush()?;
                    }

                    // for some reason code crafter uses j as enter, so yeah, fun debugging
                    KeyCode::Enter | KeyCode::Char('j') => {
                        print!("\r\n");

                        let output = commands::create_command(&command).execute();

                        if let Some(out) = output.stdout {
                            println!("{}", out.trim_end_matches("\n").replace("\n", "\r\n"));
                        }

                        if let Some(out) = output.stderr {
                            println!("{}", out.trim_end_matches("\n").replace("\n", "\r\n"));
                        }

                        if let Some(code) = output.exit_code {
                            disable_raw_mode()?;
                            exit(code)
                        }

                        command.clear();
                        print!("\r$ ");
                        io::stdout().flush()?;
                    }

                    KeyCode::Char(c) => {
                        command.push(c);
                        print!("{}", c);
                        io::stdout().flush()?;
                    }

                    KeyCode::Backspace => {
                        if !command.is_empty() {
                            command.pop();
                            print!("\x08 \x08");
                            io::stdout().flush()?;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

// fn main() -> Result<(), Box<dyn Error>> {
//     let mut command = String::new();

//     loop {
//         print!("$ ");
//         io::stdout().flush()?;

//         command.clear();
//         io::stdin().read_line(&mut command)?;

//         let output = commands::create_command(&command).execute();

//         if let Some(out) = output.stdout {
//             println!("{}", out.trim_end_matches("\n"));
//         }

//         if let Some(out) = output.stderr {
//             println!("{}", out.trim_end_matches("\n"));
//         }

//         if let Some(code) = output.exit_code {
//             exit(code)
//         }
//     }
// }
