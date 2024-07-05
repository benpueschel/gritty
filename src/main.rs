use std::error::Error;

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
        #[structopt(help = concat!("Username (or token) to authenticate with.",
                "If using basic HTTP auth, provide the username. Otherwise, provide a token."))]
        username_or_token: String,
        #[structopt(help = "Password to authenticate with if using basic HTTP auth.")]
        password: Option<String>,
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
        Args::List { remote: _ } => {
            println!("Listing repositories...");
            todo!()
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
        Args::Delete { name, remote: remote_name } => {
            println!("Deleting repository '{name}'...");
            let remote = load_remote(&remote_name).await?;
            remote.delete_repo(&name).await?;
            println!("Repository '{name}' deleted on remote '{remote_name}'.");
        }
        Args::Auth {
            remote,
            username_or_token,
            password,
        } => {
            println!("Adding authentication to remote '{remote}'...");
            let mut config = load_config()?;
            if password.is_some() {
                todo!("Basic HTTP auth is not yet supported.");
            }
            config.store_token(&remote, &username_or_token)?;
            config.save()?;
            println!("Authentication added to remote '{remote}'.");
        }
    }

    Ok(())
}
