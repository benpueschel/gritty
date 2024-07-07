use std::io::{stdin, stdout, Write};

use crate::config::Config;
use crate::error::{Error, ErrorKind, Result};
use crate::log;
use crate::remote::{self, Remote, RepoCreateInfo};

fn load_config() -> Result<Config> {
    match Config::load_from_file(None) {
        Ok(config) => Ok(config),
        Err(err) => match err.kind {
            ErrorKind::NotFound => {
                eprintln!("{}", err.message);
                log::info("Creating default config...");
                log::end_line();
                Config::save_default()?;
                Err(Error::not_found(
                    "Default config created. Please fill in the required fields.",
                ))
            }
            _ => Err(err),
        },
    }
}

async fn load_remote(remote_name: &str) -> Result<Box<dyn Remote>> {
    let config = load_config()?;
    let provider = config.get_remote_provider(remote_name)?;
    let remote_config = config.get_remote_config(remote_name)?;
    Ok(remote::create_remote(&remote_config, provider).await)
}

fn get_input() -> Result<String> {
    stdout().flush()?;
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub async fn create_config() -> Result<()> {
    log::println("Creating default config...");
    Config::save_default()?;
    Ok(())
}

pub async fn list_repositories(remote: &str) -> Result<()> {
    log::print("Listing repositories on remote '");
    log::info(remote);
    log::println("'...");

    let remote = load_remote(remote).await?;
    let repos = remote.list_repos().await?;
    log::println("* denotes private repositories");
    let mut longest_name = 0;
    for repo in &repos {
        if repo.name.len() > longest_name {
            longest_name = repo.name.len();
        }
    }
    for repo in &repos {
        if repo.private {
            log::print("* ");
        } else {
            log::print("  ");
        }
        let padding = " ".repeat(longest_name - repo.name.len());
        log::info(&format!("{}{padding}", repo.name));
        if repo.last_commits.is_empty() {
            log::print(" - no commits");
        } else {
            let last = &repo.last_commits[0];
            let date = &last.date;
            let sha = last.sha.split_at(8).0;
            let message = last.message.split('\n').next().unwrap_or(&last.message);
            log::print(&format!(" - {date}: "));
            log::alt_info(sha);
            log::print(" - ");
            log::info(message);
        }
        log::end_line();
    }
    Ok(())
}

pub async fn create_repository(
    private: bool,
    clone: bool,
    init: bool,
    license: Option<String>,
    name: String,
    remote: String,
) -> Result<()> {
    let remote = load_remote(&remote).await?;
    log::highlight("Creating repository '", &name, "'...");
    let info = RepoCreateInfo {
        name: name.clone(),
        description: None,
        license,
        init,
        private,
    };
    let url = remote.create_repo(info).await?;
    log::print("Repository created at: ");
    log::info(&url);
    log::end_line();
    if clone {
        remote.clone_repo(&name, &name).await?;
    }
    Ok(())
}

pub async fn delete_repository(name: &str, remote_name: &str) -> Result<()> {
    let remote = load_remote(remote_name).await?;
    let repo_info = match remote.get_repo_info(name).await {
        Ok(x) => x,
        Err(_) => {
            // TODO: match the actual error type
            return Err(Error::not_found(format!(
                "Repository '{name}' not found on remote '{remote_name}'."
            )));
        }
    };
    log::important("WARNING: ");
    log::warning("You are about to delete repository '");
    log::important(name);
    log::warning("' on remote '");
    log::important(remote_name);
    log::println("'.");

    if let Some(last) = repo_info.last_commits.first() {
        // Only show the first line of the commit message
        let message = last.message.split('\n').next().unwrap_or(&last.message);
        log::print("Last commit: ");
        log::alt_info(last.sha.split_at(8).0);
        log::print(" - ");
        log::info(message);
        log::println(&format!(" by {} on {}", last.author, last.date));
    }
    log::important("Are you sure you want to continue? (y/N): ");
    let input = get_input()?;
    if !input.eq_ignore_ascii_case("y") {
        log::info("Operation cancelled.");
        log::end_line();
        return Ok(());
    }
    remote.delete_repo(name).await?;
    log::print("Repository '");
    log::info(name);
    log::print("' deleted on remote '");
    log::info(remote_name);
    log::println("'.");
    Ok(())
}

pub async fn auth(remote: &str) -> Result<()> {
    log::info("Enter your username (leave blank to use a token): ");
    let username = get_input()?;

    log::info("Enter your password or token: ");
    stdout().flush()?;
    let password = rpassword::read_password()?;

    log::highlight("Adding authentication to remote '", remote, "'...");
    let mut config = load_config()?;
    if !username.is_empty() {
        todo!("Basic HTTP auth is not yet supported.");
    }
    config.store_token(remote, &password)?;
    log::highlight("Authentication added to remote '", remote, "'.");
    Ok(())
}
