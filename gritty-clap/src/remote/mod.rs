use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
#[command(about = "Remote operations")]
pub struct Remote {
    #[command(subcommand)]
    pub subcommand: RemoteCommands,
}
#[derive(Debug, Clone, Subcommand)]
pub enum RemoteCommands {
    #[command(about = "List all configured remotes", aliases = &["ls"])]
    List,
}
