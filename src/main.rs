use std::fmt::Display;

use gritty_clap::{Args, Commands};
use clap::Parser;
use config::Config;
use error::{Error, ErrorKind, Result};
use log::{Highlight, Paint};

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

/// If we fail before successfully loading the colors, we can't use them to print the error
/// message. In that case, we print the error message without colors and exit with status code 1.
fn exit_no_color(err: impl Display) -> ! {
    eprintln!("{}", err);
    std::process::exit(1);
}

async fn execute_command(args: Args) -> Result<()> {
    if let Commands::CreateConfig = args.subcommand {
        if let Err(err) = log::load_default_colors() {
            exit_no_color(err);
        }
        return commands::create_config(&args.config).await;
    }

    let mut config = match load_config(&args.config) {
        Ok(x) => x,
        Err(err) => exit_no_color(err),
    };
    if let Err(err) = log::load_colors(&config) {
        exit_no_color(err);
    }

    // Execute the sub-command
    match args.subcommand {
        Commands::Remote(remote) => commands::remote(remote, &config).await,
        Commands::Repo(repo) => commands::repo(repo, &config).await,
        Commands::Auth(args) => commands::auth(args, &mut config).await,
        Commands::CreateConfig => unreachable!(),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    human_panic::setup_panic!(human_panic::Metadata::new(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    )
    .authors(env!("CARGO_PKG_AUTHORS").replace(':', ", "))
    .support("- Open an issue at https://github.com/benpueschel/gritty/issues/new"));

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
