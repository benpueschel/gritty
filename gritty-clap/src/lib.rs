use std::env;

mod auth;
mod clone;
mod create;
mod delete;
mod list;

pub use auth::Auth;
pub use clone::Clone;
pub use create::Create;
pub use delete::Delete;
pub use list::List;

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
    about = "A tool to manage remote git repositories.",
    arg_required_else_help = true,
    styles = styles()
)]
pub struct Args {
    #[command(subcommand)]
    pub subcommand: Commands,
    #[arg(
        short = 'C',
        long,
        help = "Path to the configuration file",
        long_help = "Path to the configuration file.
\
If not provided, the config will be searched for in the following directories:

- $XDG_CONFIG_HOME/gritty/config.toml (~/.config/gritty/config.toml)
- $HOME/.gritty.toml                  (~/.gritty.toml)

On Windows, the following directories will be searched:

- %LOCALAPPDATA%\\gritty\\config.toml   (C:\\Users\\<user>\\AppData\\Local\\gritty\\config.toml)
- %LOCALAPPDATA%\\.gritty.toml          (C:\\Users\\<user>\\AppData\\Local\\.gritty.toml)

If the config file does not exist, it will be created in the specified location,
or ~/.config/gritty/config.toml if not specified.\
"
    )]
    pub config: Option<String>,
}

#[derive(Default, Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "lowercase")]
pub enum OutputFormat {
    #[default]
    Human,
    Json,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    Clone(Clone),
    List(List),
    Create(Create),
    Delete(Delete),
    Auth(Auth),

    #[command(about = "Interactively configure gritty")]
    CreateConfig,
    #[command(about = "List all configured remotes")]
    ListRemotes,
}
