use std::env;

pub mod auth;
pub mod remote;
pub mod repo;
pub mod completions;

use auth::Auth;
use remote::Remote;
use repo::Repo;
use completions::Completions;

use clap::{
    builder::styling::{AnsiColor, Effects, Styles},
    crate_version, Parser, Subcommand, ValueEnum,
};

fn styles() -> Styles {
    match env::var("NO_COLOR") {
        Ok(_) => Styles::default(),
        Err(_) => Styles::styled()
            .header(AnsiColor::Cyan.on_default() | Effects::BOLD)
            .usage(AnsiColor::Cyan.on_default() | Effects::BOLD)
            .literal(AnsiColor::BrightBlue.on_default())
            .placeholder(AnsiColor::BrightGreen.on_default()),
    }
}

#[derive(Debug, Clone, Parser)]
#[command(
    name = "gritty",
    version = crate_version!(),
    about = r#"

            _ _   _
           (_) | | |
  __ _ _ __ _| |_| |_ _   _
 / _` | '__| | __| __| | | |
| (_| | |  | | |_| |_| |_| |
 \__, |_|  |_|\__|\__|\__, |
  __/ |                __/ |
 |___/                |___/

 Manage your remote repositories with ease"#,
    arg_required_else_help = true,
    styles = styles()
)]
pub struct Args {
    #[command(subcommand)]
    pub subcommand: Commands,

    #[arg(short = 'C', long, global = true)]
    ///Path to the configuration file.
    ///
    /// If not provided, the config will be searched for in the following directories:
    ///
    /// - $XDG_CONFIG_HOME/gritty/config.toml (~/.config/gritty/config.toml)
    /// - $HOME/.gritty.toml                  (~/.gritty.toml)
    ///
    /// On Windows, the following directories will be searched:
    ///
    /// - %LOCALAPPDATA%\gritty\config.toml   (C:\Users\<user>\AppData\Local\gritty\config.toml)
    ///
    /// - %LOCALAPPDATA%\.gritty.toml          (C:\Users\<user>\AppData\Local\.gritty.toml)
    ///
    /// If the config file does not exist, it will be created in the specified location,
    /// or ~/.config/gritty/config.toml if not specified.
    pub config: Option<String>,

    #[arg(long, default_value = "auto", global = true)]
    /// Whether to use color output.
    pub color: Color,
}

#[derive(Default, Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "lowercase")]
pub enum Color {
    #[default]
    /// Use color output when possible.
    /// If the output is not a TTY, or the `NO_COLOR` environment variable is set,
    /// color will be disabled.
    Auto,
    /// Force color output.
    Always,
    /// Disable color output.
    Never,
}

#[derive(Default, Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "lowercase")]
pub enum OutputFormat {
    #[default]
    /// Output in human-readable format.
    Human,
    /// Output in machine-readable JSON format.
    Json,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    Auth(Auth),
    Repo(Repo),
    Remote(Remote),

    Completions(Completions),

    #[command()]
    /// Interactively configure gritty.
    CreateConfig,
}
