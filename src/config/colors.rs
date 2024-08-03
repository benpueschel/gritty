use std::collections::HashMap;

use ansi_term::Style;
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    log::Highlight,
};

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ConfigColorMap(pub HashMap<String, ConfigColor>);

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ConfigColor {
    pub color: Option<Color>,
    pub background: Option<Color>,
    pub bold: Option<bool>,
    pub underline: Option<bool>,
    pub italic: Option<bool>,
    pub inverse: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ColorValue {
    pub color: String,
    pub rgb: Option<String>,
}

macro_rules! map {
    () => {
        HashMap::new()
    };
    ($($key:expr => $value:expr),+ $(,)?) => {{
        let mut map = HashMap::new();
        $( map.insert($key, $value); )*
        map
    }};
}

impl ConfigColorMap {
    pub fn get(&self, key: &str) -> Option<&ConfigColor> {
        self.0.get(key)
    }

    pub fn default_highlights() -> HashMap<Highlight, Style> {
        use ansi_term::Color::*;
        map![
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
            Highlight::Warning => Yellow.normal()
        ]
    }

    pub fn parse_highlights(&self) -> Result<HashMap<Highlight, Style>> {
        let mut map = Self::default_highlights();
        for (k, v) in &self.0 {
            match k.as_str() {
                "important" => {
                    map.insert(Highlight::Important, v.parse()?);
                }
                "special" => {
                    map.insert(Highlight::Special, v.parse()?);
                }
                "repo" => {
                    map.insert(Highlight::Repo, v.parse()?);
                }
                "origin" => {
                    map.insert(Highlight::Origin, v.parse()?);
                }
                "remote" => {
                    map.insert(Highlight::Remote, v.parse()?);
                }
                "username" => {
                    map.insert(Highlight::Username, v.parse()?);
                }
                "path" => {
                    map.insert(Highlight::Path, v.parse()?);
                }
                "protocol" => {
                    map.insert(Highlight::Protocol, v.parse()?);
                }
                "url" => {
                    map.insert(Highlight::Url, v.parse()?);
                }
                "commit" => {
                    map.insert(Highlight::Commit, v.parse()?);
                }
                "date" => {
                    map.insert(Highlight::Date, v.parse()?);
                }
                "author" => {
                    map.insert(Highlight::Author, v.parse()?);
                }
                "commit_msg" => {
                    map.insert(Highlight::CommitMsg, v.parse()?);
                }
                "warning" => {
                    map.insert(Highlight::Warning, v.parse()?);
                }
                _ => {
                    return Err(Error::deserialization(format!(
                        "could not validate color config: '{}' is not a valid highlight. Expected one of {}",
                        k,
                        concat!(
                            "important, special, repo, origin, remote, username, path, protocol, ",
                            "url, commit, date, author, commit_msg, warning."
                        ),
                    )))
                }
            }
        }
        Ok(map)
    }
}

impl ConfigColor {
    pub fn parse(&self) -> Result<ansi_term::Style> {
        let mut color = Style::new();
        if let Some(fg) = &self.color {
            color = color.fg(Result::from(fg)?);
        }
        if let Some(bg) = &self.background {
            color = color.on(Result::from(bg)?);
        }
        if let Some(true) = self.bold {
            color = color.bold();
        }
        if let Some(true) = self.underline {
            color = color.underline();
        }
        if let Some(true) = self.italic {
            color = color.italic();
        }
        if let Some(true) = self.inverse {
            color = color.reverse();
        }
        Ok(color)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Color {
    /// A colour number from 0 to 255, for use in 256-colour terminal
    /// environments.
    ///
    /// - Colours 0 to 7 are the `Black` to `White` variants respectively.
    ///   These colours can usually be changed in the terminal emulator.
    /// - Colours 8 to 15 are brighter versions of the eight colours above.
    ///   These can also usually be changed in the terminal emulator, or it
    ///   could be configured to use the original colours and show the text in
    ///   bold instead. It varies depending on the program.
    /// - Colours 16 to 231 contain several palettes of bright colours,
    ///   arranged in six squares measuring six by six each.
    /// - Colours 232 to 255 are shades of grey from black to white.
    ///
    /// It might make more sense to look at a [colour chart][cc].
    ///
    /// [cc]: https://upload.wikimedia.org/wikipedia/commons/1/15/Xterm_256color_chart.svg
    Fixed(u8),

    /// A 24-bit RGB color, as specified by ISO-8613-3.
    RGB { r: u8, g: u8, b: u8 },

    /// A named color, as defined by the original 3/4-bit ANSI escape codes.
    Named(String),
}

impl From<&Color> for Result<ansi_term::Color> {
    fn from(value: &Color) -> Self {
        use Color::*;
        match value {
            Named(s) => match s.to_lowercase().as_str() {
                "black" => Ok(ansi_term::Color::Black),
                "red" => Ok(ansi_term::Color::Red),
                "green" => Ok(ansi_term::Color::Green),
                "yellow" => Ok(ansi_term::Color::Yellow),
                "blue" => Ok(ansi_term::Color::Blue),
                "purple" => Ok(ansi_term::Color::Purple),
                "cyan" => Ok(ansi_term::Color::Cyan),
                "white" => Ok(ansi_term::Color::White),

                _ => Err(Error::deserialization(format!(
                    "could not parse color: '{}' is not a valid color name. Expected one of: {}",
                    s, "black, red, green, yellow, blue, purple, cyan, white"
                ))),
            },
            Fixed(value) => Ok(ansi_term::Color::Fixed(*value)),
            RGB { r, g, b } => Ok(ansi_term::Color::RGB(*r, *g, *b)),
        }
    }
}
