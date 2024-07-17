use std::env;

use clap::{
    builder::styling::{AnsiColor, Effects, Styles},
    Parser,
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
pub enum Args {
    #[command(about = "Interactively configure gritty")]
    CreateConfig,
    #[command(about = "Clone a repository from a remote")]
    Clone {
        #[arg(help = "Name of the repository")]
        name: String,
        #[arg(help = "Name of the remote as defined in the config (ex: 'github')")]
        remote: String,
    },
    #[command(about = "List repositories on a remote")]
    List {
        #[arg(help = "Name of the remote as defined in the config (ex: 'github')")]
        remote: String,
    },
    #[command(about = "List all configured remotes")]
    ListRemotes,
    #[command(about = "Create a repository on a remote")]
    Create {
        #[arg(short, long, help = "Create a private repository")]
        private: bool,
        #[arg(short, long, help = "Clone the repository after creation")]
        clone: bool,
        #[arg(short, long, help = "Initialize the repository with a README.md")]
        init: bool,
        #[arg(
            short,
            long,
            help = concat!("License to use for the repository (ex: 'MIT'). ",
            "If not provided, or --init is not specified, no license will be addeed.")
        )]
        license: Option<String>,
        #[arg(help = "Name of the repository")]
        name: String,
        #[arg(help = "Name of the remote as defined in the config (ex: 'github')")]
        remote: String,
    },
    #[command(about = "Delete a repository on a remote")]
    Delete {
        #[arg(help = "Name of the repository")]
        name: String,
        #[arg(help = "Name of the remote as defined in the config (ex: 'github')")]
        remote: String,
    },
    #[command(about = "Authenticate with a remote")]
    Auth {
        #[arg(help = "Name of the remote as defined in the config (ex: 'github')")]
        remote: String,
    },
}
