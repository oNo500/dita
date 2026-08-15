use std::io::IsTerminal;

/// Colour is an accelerator, never the only carrier of meaning: the symbols
/// read the same without it. Off when piped, off when `NO_COLOR` is set — a
/// log file or a CI transcript full of escape codes helps nobody.
#[derive(Clone, Copy)]
pub struct Paint {
    enabled: bool,
}

impl Paint {
    #[must_use]
    pub fn detect() -> Self {
        let enabled = std::env::var_os("NO_COLOR").is_none() && std::io::stdout().is_terminal();
        Self { enabled }
    }

    #[must_use]
    pub const fn off() -> Self {
        Self { enabled: false }
    }

    fn wrap(self, code: &str, text: &str) -> String {
        if self.enabled {
            format!("\x1b[{code}m{text}\x1b[0m")
        } else {
            text.to_string()
        }
    }

    #[must_use]
    pub fn dim(self, text: &str) -> String {
        self.wrap("2", text)
    }

    #[must_use]
    pub fn green(self, text: &str) -> String {
        self.wrap("32", text)
    }

    #[must_use]
    pub fn yellow(self, text: &str) -> String {
        self.wrap("33", text)
    }

    #[must_use]
    pub fn red(self, text: &str) -> String {
        self.wrap("31", text)
    }
}
