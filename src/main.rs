use std::error::Error;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn Error>> {
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut command = String::new();
    io::stdin().read_line(&mut command)?;
    println!("{}: command not found", command.trim());

    Ok(())
}
