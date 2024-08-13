use gritty_clap::repo::{Repo, RepoCommands};

use crate::{config::Config, error::Result};

mod clone;
mod create;
mod delete;
mod list;

pub async fn repo(repo: Repo, config: &Config) -> Result<()> {
    match repo.subcommand {
        RepoCommands::Clone(clone) => clone::clone_repository(clone, config).await,
        RepoCommands::List(list) => list::list_repositories(list, config).await,
        RepoCommands::Create(create) => create::create_repository(create, config).await,
        RepoCommands::Delete(delete) => delete::delete_repository(delete, config).await,
    }
}
