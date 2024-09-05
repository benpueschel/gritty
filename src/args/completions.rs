use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Parser)]
#[command()]
/// Generate shell completions for the specified shell.
pub struct Completions {
    /// The shell for which to generate completions.
    #[arg()]
    pub shell: Shell,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Shell {
    Bash,
    Fish,
    Zsh,
    PowerShell,
}
