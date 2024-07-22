use std::io::{stdin, stdout, Write};

use crate::config::Config;
use crate::error::{Error, ErrorKind, Result};
use crate::remote::{self, Remote};

mod auth;
pub use auth::auth;

mod create_config;
pub use create_config::create_config;

mod create_repository;
pub use create_repository::create_repository;

mod clone_repository;
pub use clone_repository::clone_repository;

mod list_repositories;
pub use list_repositories::list_repositories;

mod list_remotes;
pub use list_remotes::list_remotes;

mod delete_repository;
pub use delete_repository::delete_repository;

fn load_config(path: &Option<String>) -> Result<Config> {
    match Config::load_from_file(None) {
        Ok(config) => Ok(config),
        Err(err) => match err.kind {
            ErrorKind::NotFound => {
                eprintln!("{}", err.message);
                println!("Creating default config...");
                Config::save_default(path)?;
                Err(Error::not_found(
                    "Default config created. Please fill in the required fields.",
                ))
            }
            _ => Err(err),
        },
    }
}

async fn load_remote(remote_name: &str, config: &Option<String>) -> Result<Box<dyn Remote>> {
    let config = load_config(config)?;
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
