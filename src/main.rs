use args::{Args, Commands};
use clap::Parser;
use config::Config;
use error::{ErrorKind, Result, Error};
use log::{Highlight, Paint};

pub mod args;
pub mod commands;
pub mod config;
pub mod error;
pub mod log;
pub mod remote;

fn load_config(path: &Option<String>) -> Result<Config> {
    match Config::load_from_file(path.clone()) {
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

async fn execute_command(args: Args) -> Result<()> {
    if let Commands::CreateConfig = args.subcommand {
        log::load_default_colors()?;
        return commands::create_config(&args.config).await;
    }

    let mut config = load_config(&args.config)?;
    log::load_colors(&config)?;

    // Execute the sub-command
    match args.subcommand {
        Commands::ListRemotes => commands::list_remotes(&config).await,
        Commands::Clone(args) => commands::clone_repository(args, &config).await,
        Commands::List(args) => commands::list_repositories(args, &config).await,
        Commands::Create(args) => commands::create_repository(args, &config).await,
        Commands::Delete(args) => commands::delete_repository(args, &config).await,
        Commands::Auth(args) => commands::auth(args, &mut config).await,
        Commands::CreateConfig => unreachable!(),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    human_panic::setup_panic!();
    // Parse command line arguments
    let args = Args::parse();

    let result = execute_command(args).await;

    // Handle errors by printing them to stderr and exiting with status code 1
    let _ = result.map_err(|e| {
        eprintln!("{}: {}", "Error".paint(Highlight::Important), e);
        std::process::exit(1);
    });
    Ok(())
}
