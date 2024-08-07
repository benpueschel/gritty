use gritty_clap::{Create, OutputFormat};
use crate::config::Config;
use crate::error::Result;
use crate::log::{Highlight, Paint};
use crate::remote::RepoCreateInfo;

use super::load_remote;

pub async fn create_repository(args: Create, config: &Config) -> Result<()> {
    let Create {
        private,
        clone,
        recursive,
        add_remote,
        name,
        description,
        license,
        init,
        remote,
        format,
    } = args;
    let format = format.unwrap_or_default();
    let remote = load_remote(&remote, config).await?;
    if let OutputFormat::Human = format {
        println!("Creating repository {}...", name.paint(Highlight::Repo));
    }
    let info = RepoCreateInfo {
        name: name.clone(),
        description,
        license,
        init,
        private,
    };
    let repo = remote.create_repo(info).await?;
    if let OutputFormat::Human = format {
        println!("Repository created at: {}", repo.clone_url.paint(Highlight::Url));
    }
    if clone {
        remote.clone_repo(&name, &name, recursive).await?;
    } else if add_remote {
        remote.add_remote(&name, repo.default_branch.clone()).await?;
    }
    if let OutputFormat::Json = format {
        println!("{}", serde_json::to_string_pretty(&repo)?);
    }
    Ok(())
}
