use crate::args::{repo::Fork, OutputFormat};
use crate::commands::load_remote;
use crate::config::Config;
use crate::error::Result;
use crate::log::{Highlight, Paint};
use crate::remote::RepoForkOption;

pub async fn fork_repository(args: Fork, config: &Config) -> Result<()> {
    let Fork {
        clone,
        add_remote,
        format,
        target,
        default_branch_only,
        organization,
        owner,
        repository,
        remote,
    } = args;

    let repo = repository;
    let format = format.unwrap_or_default();
    let remote = load_remote(&remote, config).await?;

    if let OutputFormat::Human = format {
        let org = organization
            .as_ref()
            .unwrap_or(&remote.get_config().username);
        let target = target.as_ref().unwrap_or(&repo);
        println!(
            "Forking repository {} into {}...",
            format!("{}/{}", owner, repo).paint(Highlight::Repo),
            format!("{}/{}", org, target).paint(Highlight::Repo)
        );
    }
    let info = RepoForkOption {
        owner,
        repo,
        organization,
        name: target,
        default_branch_only: Some(default_branch_only),
    };

    let repo = remote.create_fork(info).await?;
    if let OutputFormat::Human = format {
        println!(
            "Repository created at: {}",
            repo.clone_url.paint(Highlight::Url)
        );
    }

    if clone {
        // TODO: add recursive option
        // TODO: add organization option - currently only supports the authenticated user
        remote.clone_repo(&repo.name, &repo.name, false).await?;
    } else if add_remote {
        // TODO: add organization option - currently only supports the authenticated user
        remote.add_remote(&repo.name, None).await?;
    }

    if let OutputFormat::Json = format {
        println!("{}", serde_json::to_string_pretty(&repo)?);
    }
    Ok(())
}
