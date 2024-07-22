use crate::args::Auth;
use crate::error::Result;
use crate::log::Highlight;
use std::io::{stdout, Write};

use super::{get_input, load_config};

pub async fn auth(args: Auth, config: &Option<String>) -> Result<()> {
    let Auth { remote } = &args;
    print!(
        "Enter your {} for remote {} (leave blank to use a token): ",
        Highlight::Username("username"), Highlight::Remote(remote)
    );
    stdout().flush()?;
    let username = get_input()?;

    print!("Enter your {}: ", Highlight::Special("password or token"));
    stdout().flush()?;
    let password = rpassword::read_password()?;

    println!(
        "Adding authentication to remote {}...",
        Highlight::Remote(remote)
    );
    let mut config = load_config(config)?;
    if !username.is_empty() {
        todo!("Basic HTTP auth is not yet supported.");
    }
    config.store_token(remote, &password)?;
    println!(
        "Authentication added to remote {}.",
        Highlight::Remote(remote)
    );
    Ok(())
}
