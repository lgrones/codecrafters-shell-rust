use std::{
    any::Any,
    fmt::Display,
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};

use crate::{
    commands::{Command, Output},
    helper::CaptureFrom,
};

pub struct Redirect {
    command: Box<dyn Command>,
    capture_from: CaptureFrom,
    file: String,
}

impl Display for Redirect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Redirect")
    }
}

impl Redirect {
    pub fn new(command: Box<dyn Command>, capture_from: CaptureFrom, file: String) -> impl Command {
        Redirect {
            command,
            capture_from,
            file,
        }
    }
}

impl Command for Redirect {
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

        Output {
            stdout: other,
            stderr: result.map(|_| None).unwrap_or_else(|e| Some(e.to_string())),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn write_file(path: &Path, content: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }

    File::create(path)?.write_all(content.as_bytes())?;
    Ok(())
}
