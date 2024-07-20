use std::{
    fmt::{self, Display},
    io::{IsTerminal, Write},
};

use ansi_term::Style;

pub fn is_color() -> bool {
    // only colorize if NO_COLOR is not set and stdout is a tty
    std::env::var("NO_COLOR").is_err() && std::io::stdout().is_terminal()
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Highlight<D: ToString> {
    Important(D),
    Special(D),
    Repo(D),
    Origin(D),
    Remote(D),
    Username(D),
    Path(D),
    Protocol(D),
    Url(D),
    Commit(D),
    Date(D),
    Author(D),
    CommitMsg(D),
    Warning(D),
}

/// Print a string to stdout, flushing the buffer immediately.
pub fn print(data: impl ToString) {
    print!("{}", data.to_string());
    std::io::stdout().flush().unwrap();
}

impl<D: ToString> Highlight<D> {
    pub fn get_style(&self) -> Style {
        use ansi_term::Color::*;

        if !is_color() {
            return Style::new();
        }
        match self {
            Highlight::Important(_) => Red.bold(),
            Highlight::Special(_) => Cyan.normal(),
            Highlight::Repo(_) => Cyan.normal(),
            Highlight::Origin(_) => Green.normal(),
            Highlight::Remote(_) => Green.normal(),
            Highlight::Username(_) => Green.normal(),
            Highlight::Path(_) => Cyan.normal(),
            Highlight::Protocol(_) => Cyan.normal(),
            Highlight::Url(_) => Purple.normal(),
            Highlight::Commit(_) => Green.normal(),
            Highlight::Date(_) => Style::new(),
            Highlight::Author(_) => Style::new(),
            Highlight::CommitMsg(_) => Cyan.normal(),
            Highlight::Warning(_) => Yellow.normal(),
        }
    }
    pub fn get_data(&self) -> &D {
        match self {
            Highlight::Important(d) => d,
            Highlight::Special(d) => d,
            Highlight::Repo(d) => d,
            Highlight::Origin(d) => d,
            Highlight::Remote(d) => d,
            Highlight::Username(d) => d,
            Highlight::Path(d) => d,
            Highlight::Protocol(d) => d,
            Highlight::Url(d) => d,
            Highlight::Commit(d) => d,
            Highlight::Date(d) => d,
            Highlight::Author(d) => d,
            Highlight::CommitMsg(d) => d,
            Highlight::Warning(d) => d,
        }
    }
}

impl<D: ToString> Display for Highlight<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let style = self.get_style();
        let data = self.get_data().to_string();
        write!(f, "{}", style.paint(data))
    }
}
