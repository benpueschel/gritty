use std::env;

use clap::{
    builder::styling::{AnsiColor, Effects, Styles},
    Parser, Subcommand,
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
    about = "A tool to manage remote git repositories.",
    arg_required_else_help = true,
    styles = styles()
)]
pub struct Args {
    #[command(subcommand)]
    pub subcommand: Commands,
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

#[derive(Debug, Clone, Parser)]
#[command(about = "Clone a repository from a remote")]
pub struct Clone {
    #[arg(help = "Name of the repository")]
    pub name: String,
    #[arg(help = "Name of the remote as defined in the config (ex: 'github')")]
    pub remote: String,
}

#[derive(Debug, Clone, Parser)]
#[command(about = "List repositories on a remote")]
pub struct List {
    #[arg(help = "Name of the remote as defined in the config (ex: 'github')")]
    pub remote: String,
}

#[derive(Debug, Clone, Parser)]
#[command(about = "Create a repository on a remote")]
pub struct Create {
    #[arg(short, long, help = "Create a private repository")]
    pub private: bool,
    #[arg(short, long, help = "Clone the repository after creation")]
    pub clone: bool,
    #[arg(short, long, help = "Description of the repository")]
    pub description: Option<String>,
    #[arg(short, long, help = "Initialize the repository with a README.md")]
    pub init: bool,
    #[arg(
        short,
        long,
        help = concat!("License to use for the repository (ex: 'MIT'). ",
        "If not provided, or --init is not specified, no license will be addeed.")
        )]
    pub license: Option<String>,
    #[arg(help = "Name of the repository")]
    pub name: String,
    #[arg(help = "Name of the remote as defined in the config (ex: 'github')")]
    pub remote: String,
}
#[derive(Debug, Clone, Parser)]
#[command(about = "Delete a repository on a remote")]
pub struct Delete {
    #[arg(help = "Name of the repository")]
    pub name: String,
    #[arg(help = "Name of the remote as defined in the config (ex: 'github')")]
    pub remote: String,
}
#[derive(Debug, Clone, Parser)]
#[command(about = "Authenticate with a remote")]
pub struct Auth {
    #[arg(help = "Name of the remote as defined in the config (ex: 'github')")]
    pub remote: String,
}
