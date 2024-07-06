use std::{
    error::Error,
    io::{stdin, stdout, Write},
};

use config::{Config, ConfigErrorKind};
use remote::{create_remote, Remote, RepoCreateInfo};
use structopt::StructOpt;

pub mod config;
pub mod remote;

#[derive(Debug, Clone, StructOpt)]
#[structopt(name = "gitrc-rs", about = "A tool to manage remote git repositories.")]
pub enum Args {
    #[structopt(about = "List repositories on a remote")]
    List {
        #[structopt(help = "Name of the remote as defined in the config (ex: 'github')")]
        remote: String,
    },
    #[structopt(about = "Create a repository on a remote")]
    Create {
        #[structopt(short, long, help = "Create a private repository")]
        private: bool,
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

fn load_config() -> Result<Config, Box<dyn Error>> {
    match Config::load_from_file(None) {
        Ok(config) => Ok(config),
        Err(err) => match err.kind {
            ConfigErrorKind::ConfigNotFound => {
                eprintln!("{}", err.message);
                println!("Creating default config...");
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

async fn load_remote(remote_name: &str) -> Result<Box<dyn Remote>, Box<dyn Error>> {
    let config = load_config()?;
    let provider = config.get_remote_provider(remote_name)?;
    let remote_config = config.get_remote_config(remote_name)?;
    Ok(create_remote(&remote_config, provider).await)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let command = Args::from_args();

    match command {
        Args::List { remote } => {
            println!("Listing repositories on remote '{remote}'");
            let remote = load_remote(&remote).await?;
            let repos = remote.list_repos().await?;
            println!("* denotes private repositories");
            let mut longest_name = 0;
            for repo in &repos {
                if repo.name.len() > longest_name {
                    longest_name = repo.name.len();
                }
            }
            for repo in &repos {
                if repo.private {
                    print!("* ");
                } else {
                    print!("  ");
                }
                let padding = " ".repeat(longest_name - repo.name.len());
                print!("{}{padding}", repo.name);
                if repo.last_commits.is_empty() {
                    print!(" - no commits");
                } else {
                    let last = &repo.last_commits[0];
                    let date = &last.date;
                    let sha = last.sha.split_at(8).0;
                    let message = last.message.split('\n').next().unwrap_or(&last.message);
                    print!(" - {date}: {sha}: {message}");
                }
                println!();
            }
        }
        Args::Create {
            private,
            init: _,
            license: _,
            name,
            remote,
        } => {
            println!("Creating repository '{name}'...");
            let remote = load_remote(&remote).await?;
            let info = RepoCreateInfo {
                name,
                description: None,
                private,
            };
            let url = remote.create_repo(info).await?;
            println!("Repository created at: {}", url);
        }
        Args::Delete {
            name,
            remote: remote_name,
        } => {
            println!("Deleting repository '{name}'...");
            let remote = load_remote(&remote_name).await?;
            let repo_info = match remote.get_repo_info(&name).await {
                Ok(x) => x,
                Err(x) => {
                    // TODO: match the actual error type
                    eprintln!("Repository '{name}' not found on remote '{remote_name}'.");
                    eprintln!("{}", x);
                    std::process::exit(1);
                }
            };
            println!(
                "WARNING: You are about to delete repository '{name}' on remote '{remote_name}'."
            );
            if let Some(last) = repo_info.last_commits.first() {
                // Only show the first line of the commit message
                let message = last.message.split('\n').next().unwrap_or(&last.message);
                println!(
                    "Last commit: {} - {} by {} on {}",
                    last.sha, message, last.author, last.date
                );
            }
            print!("Are you sure you want to continue? (y/N): ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            if !input.trim().eq_ignore_ascii_case("y") {
                println!("Operation cancelled.");
                std::process::exit(0);
            }
            remote.delete_repo(&name).await?;
            println!("Repository '{name}' deleted on remote '{remote_name}'.");
        }
        Args::Auth { remote } => {
            print!("Enter your username (leave blank to use a token): ");
            stdout().flush()?;
            let mut username = String::new();
            stdin().read_line(&mut username)?;
            username = username.trim().to_string();

            print!("Enter your password or token: ");
            stdout().flush()?;
            let password = rpassword::read_password()?;

            let def = Config {
                secrets: config::Secrets::Keyring,
                ..Default::default()
            };
            println!("{}", toml::to_string_pretty(&def).unwrap());

            println!("Adding authentication to remote '{remote}'...");
            let mut config = load_config()?;
            if !username.is_empty() {
                todo!("Basic HTTP auth is not yet supported.");
            }
            config.store_token(&remote, &password)?;
            println!("Authentication added to remote '{remote}'.");
        }
    }

    Ok(())
}
