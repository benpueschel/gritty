use crate::error::Result;
use clap::CommandFactory;
use clap_complete::Shell;
use gritty_clap::{
    completions::{self, Completions},
    Args,
};

pub async fn completions(args: Completions) -> Result<()> {
    let shell = match args.shell {
        completions::Shell::Bash => Shell::Bash,
        completions::Shell::Fish => Shell::Fish,
        completions::Shell::Zsh => Shell::Zsh,
        completions::Shell::PowerShell => Shell::PowerShell,
    };
    clap_complete::generate(
        shell,
        &mut Args::command(),
        "gritty",
        &mut std::io::stdout(),
    );
    Ok(())
}
