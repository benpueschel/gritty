use clap::{Parser, Subcommand};

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
}
