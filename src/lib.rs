//! # grittyJk w
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
//! use gritty::remote::{Remote, RemoteConfig, Provider, Auth, CloneProtocol};
//! use gritty::remote::github::GitHubRemote;
//!
//! let config = RemoteConfig {
//!     username: "octocat".to_string(),
//!     clone_protocol: CloneProtocol::HTTPS,
//!     url: "https://github.com".to_string(),
//!     auth: Auth::Token { token: "your-gh-token".to_string() },
//! };
//!
//! let remote = Remote::create_remote(Provider::GitHub, config);
//! ```
//!
//! The [remote::Remote] trait provides methods for interacting with repositories:
//! ```
//! # use gritty::remote::{Remote, RemoteConfig, Provider, Auth, CloneProtocol};
//! # use gritty::remote::github::GitHubRemote;
//! # let config = RemoteConfig {
//! #     username: "octocat".to_string(),
//! #     clone_protocol: CloneProtocol::HTTPS,
//! #     url: "https://github.com".to_string(),
//! #     auth: Auth::Token { token: "your-gh-token".to_string() },
//! # };
//! # let remote = Remote::create_remote(Provider::GitHub, config);
//!
//! // Check if we're authenticated
//! let auth = remote.check_auth().await.unwrap();
//! if !auth {
//!     println!("We're not authenticated :(");
//!     return;
//! }
//!
//! // Get information about octocat/hello-world
//! let repo = remote.get_repo_info("octocat", "hello-world").await.unwrap();
//!
//! // Create a new repository
//! let repo_create_info = RepoCreateInfo {
//!     name: "a-new-repo",
//!     description: "A new repository for testing",
//!     private: false,
//!     license: "MIT",
//!     init: false,
//! };
//! let new_repo = remote.create_repository(repo_create_info).await.unwrap();
//!
//! // List all repositories, including private ones
//! let list_repos_info = ListReposInfo {
//!     private: true, // Include private repositories
//!     fork: false, // Exclude forked repositories
//! };
//! let private_repos = remote.list_repositories(list_repos_info).await.unwrap();
//!
//! // Fork octocat/hello-world to the authenticated user
//! let repo_fork_info = RepoForkInfo {
//!     owner: "octocat",
//!     repo: "hello-world",
//!     ..Default::default()
//! };
//! let forked_repo = remote.create_fork(repo_fork_info).await.unwrap();
//! ```

pub mod error;
pub mod remote;
