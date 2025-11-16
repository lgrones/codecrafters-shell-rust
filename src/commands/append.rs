use std::{
    any::Any,
    fmt::Display,
    fs::{create_dir_all, OpenOptions},
    io::Write,
    path::Path,
};

use crate::{
    commands::{Command, Output},
    redirects::CaptureFrom,
};

pub struct Append {
    command: Box<dyn Command>,
    capture_from: CaptureFrom,
    file: String,
}

impl Display for Append {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Append")
    }
}

impl Append {
    pub fn new(command: Box<dyn Command>, capture_from: CaptureFrom, file: String) -> impl Command {
        Append {
            command,
            capture_from,
            file,
        }
    }
}

impl Command for Append {
    fn execute(&self) -> Output {
        let output = self.command.execute();

        let (out, other) = match self.capture_from {
            CaptureFrom::Stdout => (output.stdout, output.stderr),
            CaptureFrom::Stderr => (output.stderr, output.stdout),
        };

        let result = write_file(
            Path::new(&self.file),
            &out.map(|x| if x.ends_with("\n") { x } else { x + "\n" })
                .unwrap_or_default(),
        );

        Output::out(
            other,
            result.map(|_| None).unwrap_or_else(|e| Some(e.to_string())),
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn write_file(path: &Path, content: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(path)?;

    file.write_all((content).as_bytes())?;
    Ok(())
}
