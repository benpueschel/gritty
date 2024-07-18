use std::env;
use std::io::{stdin, stdout, Write};

use crate::config::{AuthConfig, Config, GitRemoteConfig, InlineSecrets, Secrets};
use crate::error::{Error, ErrorKind, Result};
use crate::log;
use crate::remote::{self, Remote, RepoCreateInfo};

fn load_config() -> Result<Config> {
    match Config::load_from_file(None) {
        Ok(config) => Ok(config),
        Err(err) => match err.kind {
            ErrorKind::NotFound => {
                eprintln!("{}", err.message);
                log::info("Creating default config...");
                log::end_line();
                Config::save_default()?;
                Err(Error::not_found(
                    "Default config created. Please fill in the required fields.",
                ))
            }
            _ => Err(err),
        },
    }
}

async fn load_remote(remote_name: &str) -> Result<Box<dyn Remote>> {
    let config = load_config()?;
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

pub async fn create_config() -> Result<()> {
    log::highlight("Welcome to ", "gritty", "!");
    log::println("This command will ask you some questions to create a config file.");
    log::end_line();

    let mut config = Config::default();

    // Config file path

    log::print("Enter the path to the config file (default is '");
    log::info(&config.path);
    log::print("'): ");
    let path = get_input()?;
    if !path.is_empty() {
        config.path = path;
    }
    log::end_line();

    // Token storage

    log::println("How do you want to store your tokens? (leave blank for default)");
    #[cfg(feature = "keyring")]
    let secrets = {
        log::info("1. ");
        log::print("Use the system keyring (");
        log::info("highly recommended, default");
        log::println(")");

        log::highlight("", "2. ", "In a plaintext secrets file");
        log::highlight("", "3. ", "In the config file");

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
        log::info("1. ");
        log::print("In a plaintext secrets file (");
        log::info("default");
        log::println(")");

        log::highlight("", "2. ", "In the config file");

        log::print("> ");
        let num = get_input()?;
        match num.as_str() {
            "2" => Secrets::Plaintext(InlineSecrets::default()),
            _ => ask_for_secrets_file()?,
        }
    };

    config.secrets = secrets;
    log::println("Token storage method set.");
    log::end_line();

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

            log::println("Remote added.");
            log::print("Do you want to add another remote? (y/N): ");
            continue;
        }
        // If the user didn't enter 'y', assume they meant 'n' and exit the loop
        break;
    }

    log::println("Saving config...");
    config.save()?;
    Ok(())
}

fn ask_for_remote() -> Result<(String, GitRemoteConfig, Option<AuthConfig>)> {
    log::print("Enter the name of the remote: ");
    let name = get_input()?;
    if name.is_empty() {
        return Err(Error::other("Remote name cannot be empty."));
    }
    log::print("Enter the provider for the remote (");
    log::info("github/gitea");
    log::print("): ");
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

    log::print("Enter the clone protocol (");
    log::info("ssh/https");
    log::print("): ");
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
        log::highlight("Token added to remote '", &name, "'.");
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
    log::print("Enter the path to the secrets file (default is '");
    log::info(&path);
    log::print("'): ");
    let input = get_input()?;
    if !input.is_empty() {
        path = input;
    }
    Ok(Secrets::SecretsFile(path))
}

pub async fn clone_repository(name: &str, remote: &str) -> Result<()> {
    let remote = load_remote(remote).await?;
    remote.clone_repo(name, name).await?;
    Ok(())
}

pub async fn list_repositories(remote: &str) -> Result<()> {
    log::print("Listing repositories on remote '");
    log::info(remote);
    log::println("'...");

    let remote = load_remote(remote).await?;
    let repos = remote.list_repos().await?;
    log::println("* denotes private repositories");
    let mut longest_name = 0;
    for repo in &repos {
        if repo.name.len() > longest_name {
            longest_name = repo.name.len();
        }
    }
    for repo in &repos {
        if repo.private {
            log::print("* ");
        } else {
            log::print("  ");
        }
        let padding = " ".repeat(longest_name - repo.name.len());
        log::info(&format!("{}{padding}", repo.name));
        if repo.last_commits.is_empty() {
            log::print(" - no commits");
        } else {
            let last = &repo.last_commits[0];
            let date = &last.date;
            let sha = last.sha.split_at(8).0;
            let message = last.message.split('\n').next().unwrap_or(&last.message);
            log::print(&format!(" - {date}: "));
            log::alt_info(sha);
            log::print(" - ");
            log::info(message);
        }
        log::end_line();
    }
    Ok(())
}

pub async fn list_remotes() -> Result<()> {
    let config = load_config()?;
    log::println("Configured remotes:");
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
        log::print("  ");
        log::info(name);
        log::print(&name_padding);
        log::print(" - username: ");
        log::info(&remote.username);
        log::print(&username_padding);
        log::print(" - url: ");
        log::info(&remote.url);
        log::end_line();
    }
    Ok(())
}

pub async fn create_repository(
    private: bool,
    clone: bool,
    description: Option<String>,
    init: bool,
    license: Option<String>,
    name: String,
    remote: String,
) -> Result<()> {
    let remote = load_remote(&remote).await?;
    log::highlight("Creating repository '", &name, "'...");
    let info = RepoCreateInfo {
        name: name.clone(),
        description,
        license,
        init,
        private,
    };
    let url = remote.create_repo(info).await?;
    log::print("Repository created at: ");
    log::info(&url);
    log::end_line();
    if clone {
        remote.clone_repo(&name, &name).await?;
    }
    Ok(())
}

pub async fn delete_repository(name: &str, remote_name: &str) -> Result<()> {
    let remote = load_remote(remote_name).await?;
    let repo_info = match remote.get_repo_info(name).await {
        Ok(x) => x,
        Err(_) => {
            // TODO: match the actual error type
            return Err(Error::not_found(format!(
                "Repository '{name}' not found on remote '{remote_name}'."
            )));
        }
    };
    log::important("WARNING: ");
    log::warning("You are about to delete repository '");
    log::important(name);
    log::warning("' on remote '");
    log::important(remote_name);
    log::println("'.");

    if let Some(last) = repo_info.last_commits.first() {
        // Only show the first line of the commit message
        let message = last.message.split('\n').next().unwrap_or(&last.message);
        log::print("Last commit: ");
        log::alt_info(last.sha.split_at(8).0);
        log::print(" - ");
        log::info(message);
        log::println(&format!(" by {} on {}", last.author, last.date));
    }
    log::important("Are you sure you want to continue? (y/N): ");
    let input = get_input()?;
    if !input.eq_ignore_ascii_case("y") {
        log::info("Operation cancelled.");
        log::end_line();
        return Ok(());
    }
    remote.delete_repo(name).await?;
    log::print("Repository '");
    log::info(name);
    log::print("' deleted on remote '");
    log::info(remote_name);
    log::println("'.");
    Ok(())
}

pub async fn auth(remote: &str) -> Result<()> {
    log::info("Enter your username (leave blank to use a token): ");
    let username = get_input()?;

    log::info("Enter your password or token: ");
    stdout().flush()?;
    let password = rpassword::read_password()?;

    log::highlight("Adding authentication to remote '", remote, "'...");
    let mut config = load_config()?;
    if !username.is_empty() {
        todo!("Basic HTTP auth is not yet supported.");
    }
    config.store_token(remote, &password)?;
    log::highlight("Authentication added to remote '", remote, "'.");
    Ok(())
}
