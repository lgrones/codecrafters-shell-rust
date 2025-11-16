pub struct Output {
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub exit_code: Option<i32>,
}

impl Output {
    pub fn none() -> Self {
        Output {
            stdout: None,
            stderr: None,
            exit_code: None,
        }
    }

    pub fn ok(msg: String) -> Self {
        Output {
            stdout: Some(msg.to_string()),
            stderr: None,
            exit_code: None,
        }
    }

    pub fn err(msg: String) -> Self {
        Output {
            stdout: None,
            stderr: Some(msg.to_string()),
            exit_code: None,
        }
    }

    pub fn out(out: Option<String>, err: Option<String>) -> Self {
        Output {
            stdout: out,
            stderr: err,
            exit_code: None,
        }
    }

    pub fn exit(code: i32) -> Self {
        Output {
            stdout: None,
            stderr: None,
            exit_code: Some(code),
        }
    }
}
