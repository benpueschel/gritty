use clap::Parser;

use crate::args::OutputFormat;

#[derive(Debug, Clone, Parser)]
#[command(aliases = &["new"])]
/// Create a repository on a remote
pub struct Create {
    #[arg(short, long)]
    /// Create a private repository
    pub private: bool,

    #[arg(short, long)]
    /// Clone the repository after creation
    pub clone: bool,

    #[arg(short, long)]
    /// Initialize and clone all submodules. Only valid with --clone.
    ///
    /// This is equivalent to running `git clone --recurse-submodules` (or `git clone --recursive`).
    pub recursive: bool,

    #[arg(short, long)]
    /// Add the remote to the local git repository as 'origin'. Ignored if --clone is specified.
    ///
    /// If the current directory is not a git repository, it will be initialized as one.
    pub add_remote: bool,

    #[arg(short, long)]
    /// Description of the repository
    pub description: Option<String>,

    #[arg(short, long)]
    /// Initialize the repository with a README.md
    pub init: bool,

    #[arg(short, long)]
    /// License to use for the repository (ex: 'MIT').
    /// If not provided, or --init is not specified, no license will be addeed.
    pub license: Option<String>,

    #[arg()]
    /// Name of the repository
    ///
    /// The repository must be owned by the authenticated user.
    pub name: String,

    #[arg()]
    /// Name of the remote as defined in the config (ex: 'github').
    /// The remote must be defined in the config file.
    ///
    /// There are no restrictions on the name,
    /// it does not have to correspond to the remote type (GitHub, GitLab, Gitea)
    pub remote: String,

    #[arg(long)]
    /// Change the output format to the specified value.
    ///
    /// This option is useful for parsing the output of gritty, such as in a script or another
    /// tool integrating with gritty.
    ///
    /// When using the 'json' format, gritty will output information about the newly created repository in JSON.
    pub format: Option<OutputFormat>,
}
