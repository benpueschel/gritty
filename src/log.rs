use std::{
    collections::HashMap,
    fmt::{self, Display},
    io::{IsTerminal, Write},
    path::PathBuf,
};

use ansi_term::Style;
use tokio::sync::OnceCell;

use crate::error::Result;
use crate::{
    args,
    config::{colors::ConfigColorMap, Config},
};

static STYLES: OnceCell<HashMap<Highlight, Style>> = OnceCell::const_new();
pub static COLOR_MODE: OnceCell<args::Color> = OnceCell::const_new();

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
    match COLOR_MODE.get().expect("Color mode not set") {
        args::Color::Auto => {
            // only colorize if NO_COLOR is not set and stdout is a tty
            std::env::var("NO_COLOR").is_err() && std::io::stdout().is_terminal()
        }
        args::Color::Always => true,
        args::Color::Never => false,
    }
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
impl Paint for PathBuf {
    fn paint(&self, highlight: Highlight) -> StyledString {
        // Windows paths use backslashes, so we need to replace any forward slashes
        #[cfg(target_os = "windows")]
        let str = self.to_string_lossy().to_string().replace('/', "\\");

        #[cfg(not(target_os = "windows"))]
        let str = self.to_string_lossy().to_string();
        StyledString(str, highlight)
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
