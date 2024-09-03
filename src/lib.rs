//! # gritty
//!
//! Gritty is a library for interacting with different Git providers through a common interface.
//! It provides a main trait, [remote::Remote], which has implementations for different Git
//! providers:
//! - [remote::github::GitHubRemote]
//! - [remote::gitlab::GitlabRemote]
//! - [remote::gitea::GiteaRemote]
//!
//! The [remote::Remote] trait provides a main method `create_remote` which returns a remote for
//! the given [remote::Provider] and configuration:
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

pub mod error;
pub mod remote;
