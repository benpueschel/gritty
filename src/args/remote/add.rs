use crate::remote::{CloneProtocol, Provider};
use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(aliases = &["new", "create"])]
/// Add a new remote to the configuration
///
/// Gritty will prompt you for any missing information.
pub struct Add {
    /// The protocol to use when cloning from this remote.
    #[arg(short, long)]
    pub clone_protocol: Option<CloneProtocol>,

    /// The provider of the remote.
    #[arg(short, long)]
    pub provider: Option<Provider>,

    /// The username to use when interacting with the remote.
    #[arg(short, long)]
    pub username: Option<String>,

    #[arg(short = 'U', long)]
    /// The URL of the remote
    ///
    /// This is the base URL of the remote. For example, if you are adding a remote for GitHub,
    /// you would use `https://github.com`.
    ///
    /// The URL must be a valid URL, and it must not contain a trailing slash.
    pub url: Option<String>,

    #[arg()]
    /// The name of the remote
    ///
    /// This is the name you will use to refer to the remote in other commands.
    /// The remote name must be unique, but it does not have to be equal to the provider (e.g.
    /// GitHub, Gitea, GitLab).
    pub name: String,
}
