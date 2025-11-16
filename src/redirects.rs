#[derive(PartialEq)]
pub enum CaptureFrom {
    Stdout = 1,
    Stderr = 2,
}

impl CaptureFrom {
    pub fn from_digit(c: &char) -> Self {
        match c.to_digit(10) {
            Some(2) => Self::Stderr,
            _ => Self::Stdout,
        }
    }
}

#[derive(PartialEq)]
pub enum RedirectType {
    Redirect {
        capture_from: CaptureFrom,
        file: String,
    },
    Append {
        capture_from: CaptureFrom,
        file: String,
    },
    None,
}

pub fn get_redirects(args: &mut Vec<String>) -> RedirectType {
    let index = args.iter().position(|x| x.ends_with(">"));

    if index.is_none() {
        return RedirectType::None;
    }

    let redirect_args = args.split_off(index.unwrap());

    let (command, rest) = redirect_args.split_first().unwrap();

    if command.ends_with(">>") {
        return RedirectType::Append {
            capture_from: CaptureFrom::from_digit(&command.chars().next().unwrap()),
            file: rest[0].to_string(),
        };
    }

    RedirectType::Redirect {
        capture_from: CaptureFrom::from_digit(&command.chars().next().unwrap()),
        file: rest[0].to_string(),
    }
}
