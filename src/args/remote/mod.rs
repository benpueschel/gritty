use clap::{Parser, Subcommand};

pub mod add;

#[derive(Debug, Clone, Parser)]
#[command()]
/// Manage remotes
pub struct Remote {
    #[command(subcommand)]
    pub subcommand: RemoteCommands,
}
#[derive(Debug, Clone, Subcommand)]
pub enum RemoteCommands {
    #[command(aliases = &["ls"])]
    /// List all configured remotes
    List,
    Add(add::Add),
}
