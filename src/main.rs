use std::error::Error;

use config::{Config, ConfigErrorKind};
use remote::create_remote;

pub mod config;
pub mod remote;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let remote_name = "gitea".to_string();

    let config = match Config::load_from_file(None) {
        Ok(config) => config,
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
    };

    let provider = config.get_remote_provider(&remote_name)?;
    let remote_config = config.get_remote_config(&remote_name)?;

    let _remote = create_remote(&remote_config, provider).await;

    // TODO: parse args and call appropriate function

    Ok(())
}
