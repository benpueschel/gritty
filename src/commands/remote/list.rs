use crate::config::Config;
use crate::error::Result;
use crate::log::{self, Highlight, Paint};

pub async fn list_remotes(config: &Config) -> Result<()> {
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
        let name = log::leftpad(name, longest_name).paint(Highlight::Remote);
        let username = log::leftpad(&remote.username, longest_username).paint(Highlight::Username);
        println!(
            "  {name} - username: {username} - url: {}",
            &remote.url.paint(Highlight::Url),
        );
    }
    Ok(())
}
