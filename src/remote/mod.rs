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
pub struct RemoteConfig {
    pub username: String,
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
}

#[async_trait]
pub trait Remote {
    /// Create a new remote with the given configuration.
    async fn new(config: &RemoteConfig) -> Self
    where
        Self: Sized;

    /// Create a new repository on the remote.
    /// Returns the URL of the new repository.
    async fn create_repo(&self, create_info: RepoCreateInfo) -> Result<String, Error>;
    /// Get the information of a repository.
    async fn get_repo_info(&self, name: &str) -> Result<Repository, Error>;
    /// Delete a repository.
    /// Warning: Operation does not prompt for confirmation and is irreversible.
    async fn delete_repo(&self, name: &str) -> Result<(), Error>;
    /// Clone a repository to the given path.
    async fn clone_repo(&self, name: &str, path: &str) -> Result<(), Error>;
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
