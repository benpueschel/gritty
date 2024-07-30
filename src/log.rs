use std::{
    collections::HashMap,
    fmt::{self, Display},
    io::{IsTerminal, Write},
};

use ansi_term::Style;
use tokio::sync::OnceCell;

use crate::config::{colors::ConfigColorMap, Config};
use crate::error::Result;

static STYLES: OnceCell<HashMap<Highlight, Style>> = OnceCell::const_new();

pub fn load_default_colors() -> Result<()> {
    let map = ConfigColorMap::default().parse_highlights()?;
    STYLES.set(map).map_err(|e| e.to_string()).unwrap();
    Ok(())
}
pub fn load_colors(config: &Config) -> Result<()> {
    let map = config
        .colors
        .clone()
        .unwrap_or_default()
        .parse_highlights()?;

    STYLES.set(map).map_err(|e| e.to_string()).unwrap();
    Ok(())
}

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
        if is_color() {
            STYLES.get().unwrap()[self]
        } else {
            Style::new()
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
