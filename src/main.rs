use args::{Args, Commands};
use clap::Parser;
use error::Result;
use log::{Highlight, Paint};

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
    let config = &args.config;

    // Execute the sub-command
    let result = match args.subcommand {
        Commands::CreateConfig => commands::create_config(config).await,
        Commands::ListRemotes => commands::list_remotes(config).await,
        Commands::Clone(args) => commands::clone_repository(args, config).await,
        Commands::List(args) => commands::list_repositories(args, config).await,
        Commands::Create(args) => commands::create_repository(args, config).await,
        Commands::Delete(args) => commands::delete_repository(args, config).await,
        Commands::Auth(args) => commands::auth(args, config).await,
    };

    // Handle errors by printing them to stderr and exiting with status code 1
    let _ = result.map_err(|e| {
        eprintln!("{}: {}", "Error".paint(Highlight::Important), e);
        std::process::exit(1);
    });
    Ok(())
}
