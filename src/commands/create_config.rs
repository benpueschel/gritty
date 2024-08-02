use std::env;
use std::io::{stdout, Write};

use crate::config::{AuthConfig, Config, GitRemoteConfig, InlineSecrets, Secrets};
use crate::error::{Error, Result};
use crate::log::{self, Highlight, Paint};
use crate::remote;

use super::get_input;

pub async fn create_config(cfg: &Option<String>) -> Result<()> {
    println!("Welcome to {}!", "gritty".paint(Highlight::Special));
    println!("This command will ask you some questions to create a config file.");
    println!();

    let mut config = Config::default();

    // Config file path

    if let Some(path) = cfg {
        config.path.clone_from(path);
        println!(
            "Using provided config file path: {}",
            &config.path.paint(Highlight::Path)
        );
    } else {
        println!(
            "Enter the path to the config file (default is {}):",
            &config.path.paint(Highlight::Path)
        );
        let path = get_input()?;
        if !path.is_empty() {
            config.path = path;
        }
        println!();
    }

    // Token storage

    println!("How do you want to store your tokens? (leave blank for default)");
    #[cfg(feature = "keyring")]
    let secrets = {
        println!(
            "{} Use the system keyring ({})",
            "1.".paint(Highlight::Special),
            "highly recommended, default".paint(Highlight::Special)
        );
        println!(
            "{} In a plaintext secrets file",
            "2.".paint(Highlight::Special)
        );
        println!("{} In the config file", "3.".paint(Highlight::Special));
        log::print("> ");

        let num = get_input()?;
        match num.as_str() {
            "2" => ask_for_secrets_file()?,
            "3" => Secrets::Plaintext(InlineSecrets::default()),
            _ => Secrets::Keyring,
        }
    };
    #[cfg(not(feature = "keyring"))]
    let secrets = {
        println!(
            "{} In a plaintext secrets file ({})",
            "1.".paint(Highlight::Special),
            "default".paint(Highlight::Special)
        );
        println!("{} In the config file", "2.".paint(Highlight::Special));

        log::print("> ");
        let num = get_input()?;
        match num.as_str() {
            "2" => Secrets::Plaintext(InlineSecrets::default()),
            _ => ask_for_secrets_file()?,
        }
    };

    config.secrets = secrets;
    println!("Token storage method set.");
    println!();

    // Remotes

    log::print("Do you want to add a remote? (y/N): ");
    loop {
        let input = get_input()?;
        if input.eq_ignore_ascii_case("y") {
            let (name, remote, token) = ask_for_remote()?;

            config.remotes.insert(name.clone(), remote);
            if let Some(token) = token {
                // TODO: don't unwrap, also check for basic auth if ever supported
                config.store_token(&name, &token.token.unwrap())?;
            }

            println!("Remote added.");
            log::print("Do you want to add another remote? (y/N): ");
            continue;
        }
        // If the user didn't enter 'y', assume they meant 'n' and exit the loop
        break;
    }

    println!("Saving config...");
    config.save()?;
    Ok(())
}

fn ask_for_remote() -> Result<(String, GitRemoteConfig, Option<AuthConfig>)> {
    log::print("Enter the name of the remote: ");
    let name = get_input()?;
    if name.is_empty() {
        return Err(Error::other("Remote name cannot be empty."));
    }
    print!(
        "Enter the name of the remote ({}): ",
        "github/gitea".paint(Highlight::Remote)
    );
    // we need to flush stdout, this is the cleanest way to do it
    log::print("");
    let provider = get_input()?;
    if provider.is_empty() {
        return Err(Error::other("Remote provider cannot be empty."));
    }
    let provider = match provider.to_lowercase().as_str() {
        "github" => remote::Provider::GitHub,
        "gitea" => remote::Provider::Gitea,
        _ => {
            return Err(Error::other(
                "Remote provider must be either 'github' or 'gitea'.",
            ));
        }
    };

    log::print("Enter the URL of the remote: ");
    let url = get_input()?;
    if url.is_empty() {
        return Err(Error::other("Remote URL cannot be empty."));
    }
    log::print("Enter the username for the remote: ");
    let username = get_input()?;
    if username.is_empty() {
        return Err(Error::other("Remote username cannot be empty."));
    }

    print!(
        "Enter the clone protocol ({}): ",
        "ssh/https".paint(Highlight::Protocol)
    );
    log::print("");
    let clone_protocol = get_input()?;
    if clone_protocol.is_empty() {
        return Err(Error::other("Clone protocol cannot be empty."));
    }
    let clone_protocol = match clone_protocol.to_lowercase().as_str() {
        "ssh" => remote::CloneProtocol::SSH,
        "https" => remote::CloneProtocol::HTTPS,
        _ => {
            return Err(Error::other(
                "Clone protocol must be either 'ssh' or 'https'.",
            ));
        }
    };

    // Authentication
    log::print("Do you want to add authentication to this remote? (y/N): ");
    let auth = get_input()?;
    let auth = if auth.eq_ignore_ascii_case("y") {
        log::print("Enter token: ");
        stdout().flush()?;
        let token = rpassword::read_password()?;
        println!("Token added to remote {}.", &name.paint(Highlight::Remote));
        Some(AuthConfig {
            username: None,
            password: None,
            token: Some(token),
        })
    } else {
        None
    };

    Ok((
        name,
        GitRemoteConfig {
            clone_protocol,
            provider,
            username,
            url,
        },
        auth,
    ))
}

fn ask_for_secrets_file() -> Result<Secrets> {
    let home = env::var("HOME").unwrap();
    let xdg_config = env::var("XDG_CONFIG_HOME").unwrap_or(format!("{home}/.config"));

    let mut path = format!("{xdg_config}/gritty/secrets.toml");
    print!(
        "Enter the path to the secrets file (default is {}): ",
        path.paint(Highlight::Path)
    );
    let input = get_input()?;
    if !input.is_empty() {
        path = input;
    }
    Ok(Secrets::SecretsFile { file: path })
}
