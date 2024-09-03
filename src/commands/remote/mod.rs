use gritty_clap::remote::{Remote, RemoteCommands};

use crate::config::Config;
use crate::error::Result;

pub mod list;

pub use list::list_remotes;

pub async fn remote(remote: Remote, config: &Config) -> Result<()> {
    match remote.subcommand {
        RemoteCommands::List => list_remotes(config).await,
    }
}
