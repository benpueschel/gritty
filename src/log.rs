use std::{
    fmt::{self, Display},
    io::{IsTerminal, Write},
};

use ansi_term::Style;

pub fn leftpad(s: &str, width: usize) -> String {
    format!("{s}{}", " ".repeat(width.saturating_sub(s.len())))
}

pub fn is_color() -> bool {
    // only colorize if NO_COLOR is not set and stdout is a tty
    std::env::var("NO_COLOR").is_err() && std::io::stdout().is_terminal()
}

pub struct StyledString(pub String, pub Highlight);
pub trait Paint {
    fn paint(&self, highlight: Highlight) -> StyledString;
}

impl Paint for dyn ToString {
    fn paint(&self, highlight: Highlight) -> StyledString {
        StyledString(self.to_string(), highlight)
    }
}
impl Paint for String {
    fn paint(&self, highlight: Highlight) -> StyledString {
        StyledString(self.clone(), highlight)
    }
}
impl Paint for &str {
    fn paint(&self, highlight: Highlight) -> StyledString {
        StyledString(self.to_string(), highlight)
    }
}

#[derive(Copy, Hash, Clone, PartialEq, Eq)]
pub enum Highlight {
    Important,
    Special,
    Repo,
    Origin,
    Remote,
    Username,
    Path,
    Protocol,
    Url,
    Commit,
    Date,
    Author,
    CommitMsg,
    Warning,
}

/// Print a string to stdout, flushing the buffer immediately.
pub fn print(data: impl ToString) {
    print!("{}", data.to_string());
    std::io::stdout().flush().unwrap();
}

impl Highlight {
    pub fn get_style(&self) -> Style {
        use ansi_term::Color::*;

        if !is_color() {
            return Style::new();
        }
        match self {
            Highlight::Important => Red.bold(),
            Highlight::Special => Cyan.normal(),
            Highlight::Repo => Cyan.normal(),
            Highlight::Origin => Green.normal(),
            Highlight::Remote => Green.normal(),
            Highlight::Username => Green.normal(),
            Highlight::Path => Cyan.normal(),
            Highlight::Protocol => Cyan.normal(),
            Highlight::Url => Purple.normal(),
            Highlight::Commit => Green.normal(),
            Highlight::Date => Style::new(),
            Highlight::Author => Style::new(),
            Highlight::CommitMsg => Cyan.normal(),
            Highlight::Warning => Yellow.normal(),
        }
    }
}

impl Display for StyledString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let style = self.1.get_style();
        let data = &self.0;
        write!(f, "{}", style.paint(data))
    }
}
