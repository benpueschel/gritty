use crate::args::auth::{Auth, AuthCommands};

use crate::config::Config;
use crate::error::Result;

pub mod login;
pub mod status;

pub async fn auth(args: Auth, config: &mut Config) -> Result<()> {
    match args.subcommand {
        AuthCommands::Login(login) => login::login(login, config).await,
        AuthCommands::Status => status::status(config).await,
    }
}
