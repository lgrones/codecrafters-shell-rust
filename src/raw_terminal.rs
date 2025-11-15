use std::sync::Arc;

pub struct RawTerminal {
    fd: i32,
    old: libc::termios,
}

impl RawTerminal {
    pub fn new(fd: i32) -> Arc<Self> {
        let terminal = Arc::new(RawTerminal {
            fd,
            old: unsafe {
                let mut old = std::mem::zeroed();
                libc::tcgetattr(fd, &mut old);
                old
            },
        });

        // Put terminal into raw mode
        unsafe {
            let mut raw = terminal.old;
            raw.c_lflag &= !(libc::ICANON | libc::ECHO);
            libc::tcsetattr(fd, libc::TCSANOW, &raw);
        }

        // Panic hook
        let terminal_clone = Arc::clone(&terminal);
        std::panic::set_hook(Box::new(move |info| {
            terminal_clone.restore();
            eprintln!("{}", info);
        }));

        // Ctrl+C handler
        let terminal_clone = Arc::clone(&terminal);
        ctrlc::set_handler(move || {
            terminal_clone.restore();
            std::process::exit(0);
        })
        .expect("Error setting Ctrl-C handler");

        terminal
    }

    pub fn restore(&self) {
        unsafe { libc::tcsetattr(self.fd, libc::TCSANOW, &self.old) };
    }
}

impl Drop for RawTerminal {
    fn drop(&mut self) {
        self.restore()
    }
}
