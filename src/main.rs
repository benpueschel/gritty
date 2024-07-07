use std::io::{stdin, stdout, Write};

use config::Config;
use error::{ErrorKind, Result};
use remote::{create_remote, Remote, RepoCreateInfo};
use structopt::StructOpt;

pub mod config;
pub mod error;
pub mod log;
pub mod remote;

#[derive(Debug, Clone, StructOpt)]
#[structopt(name = "gritty", about = "A tool to manage remote git repositories.")]
pub enum Args {
    #[structopt(about = "Create a default config")]
    CreateConfig,
    #[structopt(about = "List repositories on a remote")]
    List {
        #[structopt(help = "Name of the remote as defined in the config (ex: 'github')")]
        remote: String,
    },
    #[structopt(about = "Create a repository on a remote")]
    Create {
        #[structopt(short, long, help = "Create a private repository")]
        private: bool,
        #[structopt(short, long, help = "Clone the repository after creation")]
        clone: bool,
        #[structopt(short, long, help = "Initialize the repository with a README.md")]
        init: bool,
        #[structopt(
            short,
            long,
            help = concat!("License to use for the repository (ex: 'MIT'). ",
            "If not provided, or --init is not specified, no license will be addeed.")
        )]
        license: Option<String>,
        #[structopt(help = "Name of the repository")]
        name: String,
        #[structopt(help = "Name of the remote as defined in the config (ex: 'github')")]
        remote: String,
    },
    #[structopt(about = "Delete a repository on a remote")]
    Delete {
        #[structopt(help = "Name of the repository")]
        name: String,
        #[structopt(help = "Name of the remote as defined in the config (ex: 'github')")]
        remote: String,
    },
    #[structopt(about = "Authenticate with a remote")]
    Auth {
        #[structopt(help = "Name of the remote as defined in the config (ex: 'github')")]
        remote: String,
    },
}

fn load_config() -> Result<Config> {
    match Config::load_from_file(None) {
        Ok(config) => Ok(config),
        Err(err) => match err.kind {
            ErrorKind::NotFound => {
                eprintln!("{}", err.message);
                log::info("Creating default config...");
                log::end_line();
                Config::save_default()?;
                std::process::exit(1);
            }
            _ => {
                eprintln!("{}", err.message);
                std::process::exit(1);
            }
        },
    }
}

async fn load_remote(remote_name: &str) -> Result<Box<dyn Remote>> {
    let config = load_config()?;
    let provider = config.get_remote_provider(remote_name)?;
    let remote_config = config.get_remote_config(remote_name)?;
    Ok(create_remote(&remote_config, provider).await)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let command = Args::from_args();

    match command {
        Args::CreateConfig => {
            log::println("Creating default config...");
            Config::save_default()?;
            return Ok(());
        }
        Args::List { remote } => {
            log::print("Listing repositories on remote '");
            log::info(&remote);
            log::println("'...");

            let remote = load_remote(&remote).await?;
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
        }
        Args::Create {
            private,
            clone,
            init,
            license,
            name,
            remote,
        } => {
            log::print("Creating repository '");
            log::info(&name);
            log::println("'...");
            let remote = load_remote(&remote).await?;
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
        }
        Args::Delete {
            name,
            remote: remote_name,
        } => {
            let remote = load_remote(&remote_name).await?;
            let repo_info = match remote.get_repo_info(&name).await {
                Ok(x) => x,
                Err(_) => {
                    // TODO: match the actual error type
                    eprintln!("Repository '{name}' not found on remote '{remote_name}'.");
                    std::process::exit(1);
                }
            };
            log::important("WARNING: ");
            log::warning("You are about to delete repository '");
            log::important(&name);
            log::warning("' on remote '");
            log::important(&remote_name);
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
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            if !input.trim().eq_ignore_ascii_case("y") {
                log::info("Operation cancelled.");
                log::end_line();
                std::process::exit(0);
            }
            remote.delete_repo(&name).await?;
            log::print("Repository '");
            log::info(&name);
            log::print("' deleted on remote '");
            log::info(&remote_name);
            log::println("'.");
        }
        Args::Auth { remote } => {
            log::info("Enter your username (leave blank to use a token): ");
            stdout().flush()?;
            let mut username = String::new();
            stdin().read_line(&mut username)?;
            username = username.trim().to_string();

            log::info("Enter your password or token: ");
            stdout().flush()?;
            let password = rpassword::read_password()?;

            log::highlight("Adding authentication to remote '", &remote, "'...");
            let mut config = load_config()?;
            if !username.is_empty() {
                todo!("Basic HTTP auth is not yet supported.");
            }
            config.store_token(&remote, &password)?;
            log::highlight("Authentication added to remote '", &remote, "'.");
        }
    }

    Ok(())
}
