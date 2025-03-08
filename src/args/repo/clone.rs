use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command()]
/// Clone a repository from a remote
pub struct Clone {
    #[arg()]
    /// Name of the repository to clone.
    ///
    /// The repository must be owned by the authenticated user."
    pub name: String,

    #[arg()]
    /// Name of the remote as defined in the config (ex: 'github').
    ///
    /// The remote must be defined in the config file. There are no restrictions on the name,
    /// it does not have to correspond to the remote type (GitHub, GitLab, Gitea)"
    pub remote: String,

    #[arg(short, long)]
    /// Path to clone the repository to.
    ///
    /// If not provided, the repository will be cloned to the current working directory.
    /// The path must be a valid directory.
    pub path: Option<PathBuf>,


    #[arg(short, long)]
    /// Initialize and clone all submodules.
    /// This is equivalent to running `git clone --recurse-submodules` (or `git clone --recursive`)."
    pub recursive: bool,
}
