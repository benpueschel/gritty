use gritty_clap::remote::add::Add;

use crate::commands::get_input;
use crate::config::{Config, GitRemoteConfig};
use crate::error::{Error, Result};
use crate::log::{Highlight, Paint};
use crate::remote::{CloneProtocol, Provider};

pub async fn add_remote(args: Add, config: &mut Config) -> Result<()> {
    if config.remotes.contains_key(&args.name) {
        return Err(Error::other(format!(
            "A remote named {} already exists",
            args.name
        )));
    }

    let provider = args.provider.map(Result::Ok).unwrap_or_else(|| {
        print!(
            "Enter the {} for the remote ({}): ",
            "provider".paint(Highlight::Special),
            "github/gitea/gitlab".paint(Highlight::Special)
        );
        Ok(get_input()?.as_str().into())
    })?;

    let provider = match provider {
        gritty_clap::remote::add::Provider::GitHub => Provider::GitHub,
        gritty_clap::remote::add::Provider::Gitea => Provider::Gitea,
        gritty_clap::remote::add::Provider::GitLab => Provider::GitLab,
    };

    let username = args.username.map(Result::Ok).unwrap_or_else(|| {
        print!(
            "Enter the {} for the remote: ",
            "username".paint(Highlight::Username)
        );
        get_input()
    })?;

    let clone_protocol = args.clone_protocol.map(Result::Ok).unwrap_or_else(|| {
        print!(
            "Enter the {} for the remote ({}): ",
            "clone protocol".paint(Highlight::Protocol),
            "ssh/https".paint(Highlight::Protocol)
        );
        Ok(get_input()?.as_str().into())
    })?;

    let clone_protocol = match clone_protocol {
        gritty_clap::remote::add::CloneProtocol::Https => CloneProtocol::HTTPS,
        gritty_clap::remote::add::CloneProtocol::Ssh => CloneProtocol::SSH,
    };

    let url = args.url.map(Result::Ok).unwrap_or_else(|| {
        print!("Enter the {} for the remote: ", "URL".paint(Highlight::Url));
        get_input()
    })?;

    let remote = GitRemoteConfig {
        username,
        clone_protocol,
        url,
        provider,
    };

    config.remotes.insert(args.name.clone(), remote);
    config.save()?;

    println!(
        "Remote {} added successfully",
        args.name.paint(Highlight::Remote)
    );
    println!("Use `gritty auth login {}` to authenticate", args.name);

    Ok(())
}
