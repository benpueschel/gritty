use gritty_clap::auth::Auth;
use crate::config::Config;
use crate::error::Result;
use crate::log::{Highlight, Paint};
use std::io::{stdout, Write};

use super::get_input;

pub async fn auth(args: Auth, config: &mut Config) -> Result<()> {
    let Auth { remote } = &args;
    print!(
        "Enter your {} for remote {} (leave blank to use a token): ",
        "username".paint(Highlight::Username),
        remote.paint(Highlight::Remote)
    );
    stdout().flush()?;
    let username = get_input()?;

    print!(
        "Enter your {}: ",
        "password or token".paint(Highlight::Special)
    );
    stdout().flush()?;
    let password = rpassword::read_password()?;

    println!(
        "Adding authentication to remote {}...",
        remote.paint(Highlight::Remote)
    );
    if !username.is_empty() {
        todo!("Basic HTTP auth is not yet supported.");
    }
    config.store_token(remote, &password)?;
    println!(
        "Authentication added to remote {}.",
        remote.paint(Highlight::Remote)
    );
    Ok(())
}
