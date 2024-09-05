use gritty_clap::remote::{Remote, RemoteCommands};

use crate::config::Config;
use crate::error::Result;

pub mod list;
pub mod add;

pub use list::list_remotes;

pub async fn remote(remote: Remote, config: &mut Config) -> Result<()> {
    match remote.subcommand {
        RemoteCommands::List => list_remotes(config).await,
        RemoteCommands::Add(add) => add::add_remote(add, config).await,
    }
}
