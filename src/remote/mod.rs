//! The remote module provides an interface for interacting with remote repositories.
//!
//! To acquire a remote, use the [create_remote] function, which returns a remote for the given
//! [Provider]. Support for the following providers is built-in:
//! - [github::GitHubRemote]
//! - [gitlab::GitlabRemote]
//! - [gitea::GiteaRemote]
//!
//! The [Remote] trait provides a common interface for interacting with remotes:
//! ```
//! # async fn run() {
//! use gritty::remote::{self, Remote, RemoteConfig, Provider, Auth, CloneProtocol};
//! use gritty::remote::github::GitHubRemote;
//!
//! let config = RemoteConfig {
//!     username: "octocat".to_string(),
//!     clone_protocol: CloneProtocol::HTTPS,
//!     url: "https://github.com".to_string(),
//!     auth: Auth::Token { token: "your-gh-token".to_string() },
//! };
//!
//! let remote = remote::create_remote(&config, Provider::GitHub).await.unwrap();
//! # }
//! ```
//!
//! The [remote::Remote] trait provides methods for interacting with repositories:
//! ```
//! # async fn run() {
//! # use gritty::remote::{
//!       self, Remote, RemoteConfig, Provider, Auth, CloneProtocol,
//!       RepoCreateInfo, ListReposInfo, RepoForkOption,
//!   };
//! # use gritty::remote::github::GitHubRemote;
//! # let config = RemoteConfig {
//! #     username: "octocat".to_string(),
//! #     clone_protocol: CloneProtocol::HTTPS,
//! #     url: "https://github.com".to_string(),
//! #     auth: Auth::Token { token: "your-gh-token".to_string() },
//! # };
//! # let remote = remote::create_remote(&config, Provider::GitHub).await.unwrap();
//!
//! // Check if we're authenticated
//! let auth = remote.check_auth().await.unwrap();
//! if !auth {
//!     println!("We're not authenticated :(");
//!     return;
//! }
//!
//! // Get information about octocat/hello-world
//! let repo = remote.get_repo_info("hello-world").await.unwrap();
//!
//! // Create a new repository
//! let repo_create_info = RepoCreateInfo {
//!     name: "a-new-repo".to_string(),
//!     description: Some("A new repository for testing".to_string()),
//!     license: Some("MIT".to_string()),
//!     private: false,
//!     init: false,
//! };
//! let new_repo = remote.create_repo(repo_create_info).await.unwrap();
//!
//! // List all repositories, including private ones
//! let list_repos_info = ListReposInfo {
//!     private: true, // Include private repositories
//!     forks: false, // Exclude forked repositories
//! };
//! let private_repos = remote.list_repos(list_repos_info).await.unwrap();
//!
//! // Fork octocat/hello-world to the authenticated user
//! let repo_fork_info = RepoForkOption {
//!     owner: "octocat".to_string(),
//!     repo: "hello-world".to_string(),
//!     ..Default::default()
//! };
//! let forked_repo = remote.create_fork(repo_fork_info).await.unwrap();
//! # }
//! ```

use std::fmt::Debug;

use crate::error::{Error, Result};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod gitea;
pub mod github;
pub mod gitlab;

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

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    pub description: Option<String>,
    pub private: bool,
    pub fork: bool,
    pub default_branch: Option<String>,
    pub ssh_url: String,
    pub clone_url: String,
    pub last_commits: Vec<Commit>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub message: String,
    pub author: String,
    pub date: DateTime<Utc>,
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

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepoForkOption {
    /// The owner of the repository to fork.
    pub owner: String,
    /// The name of the repository to fork.
    pub repo: String,
    /// The name of the fork.
    pub name: Option<String>,
    /// Organization name, if forking into an organization.
    /// If not provided, the fork will be created in the user's account.
    pub organization: Option<String>,
    /// Only clone the default branch.
    pub default_branch_only: Option<bool>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListReposInfo {
    /// Whether to include private repositories in the list.
    pub private: bool,
    /// Whether to include forked repositories in the list.
    pub forks: bool,
}

pub static COMMIT_COUNT: u8 = 25;

#[async_trait]
pub trait Remote: Sync {
    /// Create a new remote with the given configuration.
    async fn new(config: &RemoteConfig) -> Result<Self>
    where
        Self: Sized;

    /// Check if the remote is authenticated.
    async fn check_auth(&self) -> Result<bool>;
    /// Create a new repository on the remote.
    /// Returns the new repository.
    async fn create_repo(&self, create_info: RepoCreateInfo) -> Result<Repository>;
    /// Fork a repository.
    /// Returns the new newly-created fork.
    async fn create_fork(&self, options: RepoForkOption) -> Result<Repository>;
    /// List all repositories.
    async fn list_repos(&self, list_info: ListReposInfo) -> Result<Vec<Repository>>;
    /// Get the information of a repository.
    async fn get_repo_info(&self, name: &str) -> Result<Repository>;
    /// Delete a repository.
    /// WARNING: Operation does not prompt for confirmation and is irreversible.
    async fn delete_repo(&self, name: &str) -> Result<()>;
    /// Get the configuration of the remote.
    fn get_config(&self) -> &RemoteConfig;
    /// Clone a repository to the given path.
    async fn clone_repo(&self, name: &str, path: &str, recursive: bool) -> Result<()> {
        let config = self.get_config();
        let url = self.clone_url(&config.username, name);

        let mut cmd = std::process::Command::new("git");
        cmd.args(["clone", &url, path]);
        if recursive {
            cmd.arg("--recursive");
        }

        let cmd = cmd.status()?;
        if !cmd.success() {
            return Err(Error::other(format!("Failed to clone repository {}", name)));
        }

        Ok(())
    }
    /// Add a remote to the local git repository. If the current directory is not a git repository,
    /// it will be initialized as one.
    async fn add_remote(&self, repo_name: &str, branch: Option<String>) -> Result<()> {
        let config = self.get_config();
        let url = self.clone_url(&config.username, repo_name);

        if !std::path::Path::new(".git").exists() {
            let cmd = std::process::Command::new("git").arg("init").status()?;
            if !cmd.success() {
                return Err(Error::other("Failed to initialize empty git repository"));
            }
        }

        let cmd = std::process::Command::new("git")
            .args(["remote", "add", "origin", &url])
            .status()?;
        if !cmd.success() {
            return Err(Error::other(format!(
                "Failed to add remote 'origin' for repository {}",
                repo_name
            )));
        }

        if let Some(branch) = branch {
            let cmd = std::process::Command::new("git")
                .args(["pull", "origin", &branch])
                .status()?;
            if !cmd.success() {
                return Err(Error::other(format!(
                    "Failed to pull branch {} from repository {}",
                    branch, repo_name
                )));
            }
        }

        Ok(())
    }
    /// Get the clone URL for the given repository.
    fn clone_url(&self, username: &str, repo_name: &str) -> String {
        let config = self.get_config();
        let clean_url = config.url.replace("https://", "").replace("http://", "");
        match config.clone_protocol {
            CloneProtocol::SSH => format!("git@{}:{}/{}.git", clean_url, username, repo_name),
            CloneProtocol::HTTPS => format!("{}/{}/{}.git", config.url, username, repo_name),
        }
    }
}

pub async fn create_remote(config: &RemoteConfig, provider: Provider) -> Result<Box<dyn Remote>> {
    use Provider::*;
    Ok(match provider {
        GitHub => Box::new(github::GitHubRemote::new(config).await?),
        Gitea => Box::new(gitea::GiteaRemote::new(config).await?),
        GitLab => Box::new(gitlab::GitlabRemote::new(config).await?),
    })
}
