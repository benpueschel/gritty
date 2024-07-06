use std::{
    fmt::Debug,
    io::{Error, ErrorKind},
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod gitea;
/// The remote module provides an interface for interacting with remote repositories.
/// To acquire a remote, use the [create_remote] function.
/// The [Remote] trait provides a common interface for interacting with remotes.
pub mod github;

/// The supported providers for remotes.
/// Each provider has its own implementation of the [Remote] trait.
/// The [create_remote] function will return a remote for the given provider.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Provider {
    GitHub,
    GitLab,
    Gitea,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Auth {
    /// Authenticate using a username and password.
    Basic { username: String, password: String },
    /// Authenticate using a token.
    Token { token: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CloneProtocol {
    #[serde(rename = "ssh")]
    SSH,
    #[serde(rename = "https")]
    HTTPS,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoteConfig {
    pub username: String,
    pub clone_protocol: CloneProtocol,
    pub url: String,
    pub auth: Auth,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Repository {
    pub name: String,
    pub description: Option<String>,
    pub private: bool,
    pub ssh_url: String,
    pub clone_url: String,
    pub last_commits: Vec<Commit>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Commit {
    pub sha: String,
    pub message: String,
    pub author: String,
    pub date: String,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepoCreateInfo {
    /// The name of the repository.
    pub name: String,
    /// An optional description of the repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the repository is private. Default is false.
    pub private: bool,
    /// An optional license for the repository.
    pub license: Option<String>,
    /// Whether to initialize the repository with a README and (optional) license. Default is false.
    pub init: bool,
}

pub static COMMIT_COUNT: u8 = 25;

#[async_trait]
pub trait Remote: Sync {
    /// Create a new remote with the given configuration.
    async fn new(config: &RemoteConfig) -> Self
    where
        Self: Sized;

    /// Create a new repository on the remote.
    /// Returns the URL of the new repository.
    async fn create_repo(&self, create_info: RepoCreateInfo) -> Result<String, Error>;
    /// List all repositories.
    async fn list_repos(&self) -> Result<Vec<Repository>, Error>;
    /// Get the information of a repository.
    async fn get_repo_info(&self, name: &str) -> Result<Repository, Error>;
    /// Delete a repository.
    /// Warning: Operation does not prompt for confirmation and is irreversible.
    async fn delete_repo(&self, name: &str) -> Result<(), Error>;
    /// Get the configuration of the remote.
    fn get_config(&self) -> &RemoteConfig;
    /// Clone a repository to the given path.
    async fn clone_repo(&self, name: &str, path: &str) -> Result<(), Error> {
        let config = self.get_config();
        let username = &config.username;
        let clean_url = config.url.replace("https://", "").replace("http://", "");
        let url = match config.clone_protocol {
            CloneProtocol::SSH => format!("git@{}:{}/{}.git", clean_url, username, name),
            CloneProtocol::HTTPS => format!("{}/{}/{}.git", config.url, username, name),
        };

        let status = std::process::Command::new("git")
            .arg("clone")
            .arg(url)
            .arg(path)
            .status()?;

        if !status.success() {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Failed to clone repository '{}'", name),
            ));
        }

        Ok(())
    }
}

pub async fn create_remote(config: &RemoteConfig, provider: Provider) -> Box<dyn Remote> {
    use Provider::*;
    match provider {
        GitHub => Box::new(github::GitHubRemote::new(config).await),
        Gitea => Box::new(gitea::GiteaRemote::new(config).await),
        GitLab => unimplemented!("GitLab not implemented"),
    }
}

fn map_error(e: impl Debug) -> Error {
    Error::new(ErrorKind::Other, format!("{:?}", e))
}
