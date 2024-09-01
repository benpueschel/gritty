use crate::commands::load_remote;
use crate::config::Config;
use crate::error::{ErrorKind, Result};
use crate::log::{Highlight, Paint};

pub async fn status(config: &Config) -> Result<()> {
    let mut remotes = Vec::with_capacity(config.remotes.len());
    let mut remote_len = 0;
    for name in config.remotes.keys() {
        remotes.push(name);
        remote_len = remote_len.max(name.len() + 2);
    }
    for name in remotes {
        print!("{:remote_len$}", format!("{}: ", name));
        if config.get_auth(&config.secrets, name).is_err() {
            println!("{}", "not configured".paint(Highlight::Warning));
            continue;
        }
        let remote = load_remote(name, config).await;
        let mut authenticated = true;
        if let Err(err) = &remote {
            match err.kind {
                ErrorKind::Authentication => authenticated = false,
                _ => return Err(err.clone()),
            }
        }
        match authenticated && remote.expect("remote must valid").check_auth().await? {
            true => println!("{}", "authenticated".paint(Highlight::Special)),
            false => println!("{}", "not authenticated".paint(Highlight::Important)),
        }
    }
    Ok(())
}
