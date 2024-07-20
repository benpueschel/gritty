use args::{Args, Commands};
use clap::Parser;
use error::Result;

pub mod args;
pub mod commands;
pub mod config;
pub mod error;
pub mod log;
pub mod remote;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Execute the sub-command
    let result = match args.subcommand {
        Commands::CreateConfig => commands::create_config().await,
        Commands::ListRemotes => commands::list_remotes().await,
        Commands::Clone(args) => commands::clone_repository(args).await,
        Commands::List(args) => commands::list_repositories(args).await,
        Commands::Create(args) => commands::create_repository(args).await,
        Commands::Delete(args) => commands::delete_repository(args).await,
        Commands::Auth(args) => commands::auth(args).await,
    };

    // Handle errors by printing them to stderr and exiting with status code 1
    let _ = result.map_err(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });
    Ok(())
}
