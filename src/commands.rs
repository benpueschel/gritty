use std::env;
use std::io::{stdin, stdout, Write};

use chrono::{DateTime, Local};

use crate::args::{Auth, Clone, Create, Delete, List};
use crate::config::{AuthConfig, Config, GitRemoteConfig, InlineSecrets, Secrets};
use crate::error::{Error, ErrorKind, Result};
use crate::log::{self, Highlight};
use crate::remote::{self, Remote, RepoCreateInfo};

fn load_config(path: &Option<String>) -> Result<Config> {
    match Config::load_from_file(None) {
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

async fn load_remote(remote_name: &str, config: &Option<String>) -> Result<Box<dyn Remote>> {
    let config = load_config(config)?;
    let provider = config.get_remote_provider(remote_name)?;
    let remote_config = config.get_remote_config(remote_name)?;
    Ok(remote::create_remote(&remote_config, provider).await)
}

fn get_input() -> Result<String> {
    stdout().flush()?;
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub async fn create_config(cfg: &Option<String>) -> Result<()> {
    println!("Welcome to {}!", Highlight::Special("gritty"));
    println!("This command will ask you some questions to create a config file.");
    println!();

    let mut config = Config::default();

    // Config file path

    if let Some(path) = cfg {
        config.path.clone_from(path);
        println!(
            "Using provided config file path: {}",
            Highlight::Path(&config.path)
        );
    } else {
        println!(
            "Enter the path to the config file (default is '{}'):",
            Highlight::Path(&config.path)
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
            Highlight::Special("1."),
            Highlight::Special("highly recommended, default")
        );
        println!("{} In a plaintext secrets file", Highlight::Special("2."));
        println!("{} In the config file", Highlight::Special("3."));
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
            Highlight::Special("1."),
            Highlight::Special("default")
        );
        println!("{} In the config file", Highlight::Special("2."));

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
        Highlight::Remote("github/gitea")
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
        Highlight::Protocol("ssh/https")
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
        println!("Token added to remote '{}'.", Highlight::Remote(&name));
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
        "Enter the path to the secrets file (default is '{}'): ",
        Highlight::Path(&path)
    );
    let input = get_input()?;
    if !input.is_empty() {
        path = input;
    }
    Ok(Secrets::SecretsFile(path))
}

pub async fn clone_repository(args: Clone, config: &Option<String>) -> Result<()> {
    let remote = load_remote(&args.remote, config).await?;
    remote
        .clone_repo(&args.name, &args.name, args.recursive)
        .await?;
    Ok(())
}

pub async fn list_repositories(args: List, config: &Option<String>) -> Result<()> {
    let remote = &args.remote;
    println!(
        "Listing repositories on remote '{}'...",
        Highlight::Remote(remote)
    );

    let remote = load_remote(remote, config).await?;
    let repos = remote.list_repos().await?;
    println!("* denotes private repositories");
    let mut longest_name = 0;
    for repo in &repos {
        if repo.name.len() > longest_name {
            longest_name = repo.name.len();
        }
    }
    for repo in &repos {
        if repo.private {
            print!("* ");
        } else {
            print!("  ");
        }

        let padding = " ".repeat(longest_name - repo.name.len());
        print!("{}{padding}", Highlight::Repo(&repo.name));

        if repo.last_commits.is_empty() {
            print!(" - no commits");
        } else {
            let last = &repo.last_commits[0];
            let date: DateTime<Local> = last.date.into();
            let sha = last.sha.split_at(8).0;
            let message = last.message.split('\n').next().unwrap_or(&last.message);
            print!(
                " - {}: {} - {}",
                Highlight::Date(date),
                Highlight::Commit(sha),
                Highlight::CommitMsg(message)
            );
        }
        println!();
    }
    Ok(())
}

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
        let name_padding = " ".repeat(longest_name - name.len());
        let username_padding = " ".repeat(longest_username - remote.username.len());
        println!(
            "  {}{name_padding} - username: {}{username_padding} - url: {}",
            Highlight::Remote(name),
            Highlight::Username(&remote.username),
            Highlight::Url(&remote.url),
        );
    }
    Ok(())
}

pub async fn create_repository(args: Create, config: &Option<String>) -> Result<()> {
    let Create {
        private,
        clone,
        recursive,
        name,
        description,
        license,
        init,
        remote,
    } = args;
    let remote = load_remote(&remote, config).await?;
    println!("Creating repository '{}'...", Highlight::Repo(&name));
    let info = RepoCreateInfo {
        name: name.clone(),
        description,
        license,
        init,
        private,
    };
    let url = remote.create_repo(info).await?;
    println!("Repository created at: {}", Highlight::Url(&url));
    if clone {
        remote.clone_repo(&name, &name, recursive).await?;
    }
    Ok(())
}

pub async fn delete_repository(args: Delete, config: &Option<String>) -> Result<()> {
    let Delete {
        name,
        remote: remote_name,
    } = &args;
    let remote = load_remote(remote_name, config).await?;
    let repo_info = match remote.get_repo_info(name).await {
        Ok(x) => x,
        Err(_) => {
            // TODO: match the actual error type
            return Err(Error::not_found(format!(
                "Repository '{name}' not found on remote '{remote_name}'."
            )));
        }
    };
    println!(
        "{}: You are about to delete repository '{}' on remote '{}'.",
        Highlight::Important("WARNING"),
        Highlight::Repo(&name),
        Highlight::Remote(&remote_name),
    );

    if let Some(last) = repo_info.last_commits.first() {
        // Only show the first line of the commit message
        let message = last.message.split('\n').next().unwrap_or(&last.message);
        println!(
            "Last commit: {} - {} by {} on {}",
            Highlight::Commit(last.sha.split_at(8).0),
            Highlight::CommitMsg(message),
            Highlight::Author(&last.author),
            Highlight::Date(&last.date),
        );
    }
    log::print(Highlight::Important(
        "Are you sure you want to continue? (y/N): ",
    ));
    let input = get_input()?;
    if !input.eq_ignore_ascii_case("y") {
        println!("{}", Highlight::Special("Operation cancelled."));
        return Ok(());
    }
    remote.delete_repo(name).await?;
    println!(
        "Repository '{}' deleted on remote '{}'.",
        Highlight::Repo(&name),
        Highlight::Remote(&remote_name)
    );
    Ok(())
}

pub async fn auth(args: Auth, config: &Option<String>) -> Result<()> {
    let Auth { remote } = &args;
    println!(
        "Enter your {} (leave blank to use a token): ",
        Highlight::Username(&remote)
    );
    let username = get_input()?;

    println!("Enter your {} ", Highlight::Special("password or token"));
    stdout().flush()?;
    let password = rpassword::read_password()?;

    println!(
        "Adding authentication to remote '{}'...",
        Highlight::Remote(remote)
    );
    let mut config = load_config(config)?;
    if !username.is_empty() {
        todo!("Basic HTTP auth is not yet supported.");
    }
    config.store_token(remote, &password)?;
    println!(
        "Authentication added to remote '{}'.",
        Highlight::Remote(remote)
    );
    Ok(())
}
