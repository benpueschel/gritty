use args::Args;
use error::Result;
use structopt::StructOpt;

pub mod args;
pub mod commands;
pub mod config;
pub mod error;
pub mod log;
pub mod remote;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let command = Args::from_args();

    // Execute the sub-command
    let result = match command {
        Args::CreateConfig => commands::create_config().await,
        Args::List { remote } => commands::list_repositories(&remote).await,
        Args::Create {
            private,
            clone,
            init,
            license,
            name,
            remote,
        } => commands::create_repository(private, clone, init, license, name, remote).await,
        Args::Delete { name, remote } => commands::delete_repository(&name, &remote).await,
        Args::Auth { remote } => commands::auth(&remote).await,
    };

    // Handle errors by printing them to stderr and exiting with status code 1
    let _ = result.map_err(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });
    Ok(())
}
