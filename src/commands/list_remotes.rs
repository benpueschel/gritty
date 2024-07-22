use crate::error::Result;
use crate::log::{self, Highlight};

use super::load_config;

pub async fn list_remotes(config: &Option<String>) -> Result<()> {
    let config = load_config(config)?;
    println!("Configured remotes:");
    let mut longest_name = 0;
    let mut longest_username = 0;
    for (name, remote) in &config.remotes {
        if name.len() > longest_name {
            longest_name = name.len();
        }
        if remote.username.len() > longest_username {
            longest_username = remote.username.len();
        }
    }
    for (name, remote) in &config.remotes {
        let name = Highlight::Remote(log::leftpad(name, longest_name));
        let username = Highlight::Username(log::leftpad(&remote.username, longest_username));
        println!(
            "  {name} - username: {username} - url: {}",
            Highlight::Url(&remote.url),
        );
    }
    Ok(())
}

