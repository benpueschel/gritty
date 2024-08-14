use std::io::{stdin, stdout, Write};

use crate::config::Config;
use crate::error::Result;
use crate::remote::{create_remote, Remote};

mod repo;
pub use repo::repo;

mod remote;
pub use remote::remote;

mod auth;
pub use auth::auth;

mod create_config;
pub use create_config::create_config;

async fn load_remote(remote_name: &str, config: &Config) -> Result<Box<dyn Remote>> {
    let provider = config.get_remote_provider(remote_name)?;
    let remote_config = config.get_remote_config(remote_name)?;
    Ok(create_remote(&remote_config, provider).await)
}

fn get_input() -> Result<String> {
    stdout().flush()?;
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
