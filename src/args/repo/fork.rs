use clap::Parser;

use crate::args::OutputFormat;

#[derive(Debug, Clone, Parser)]
#[command()]
/// Fork a repository on a remote
pub struct Fork {
    #[arg(short, long)]
    /// Clone the repository after forking
    pub clone: bool,

    #[arg(short, long)]
    /// Add the remote to the local git repository as 'origin'. Ignored if --clone is specified.
    ///
    /// If the current directory is not a git repository, it will be initialized as one.
    pub add_remote: bool,

    #[arg(long)]
    /// Change the output format to the specified value.
    ///
    /// This option is useful for parsing the output of gritty, such as in a script or another
    /// tool integrating with gritty.
    ///
    /// When using the 'json' format, gritty will output information about the newly created repository in JSON.
    pub format: Option<OutputFormat>,

    #[arg(short, long)]
    /// Name of the forked repository
    ///
    /// If not provided, the name of the original repository will be used
    pub target: Option<String>,

    #[arg(short, long)]
    /// Only fork the default branch
    ///
    /// If not provided, all branches will be forked
    pub default_branch_only: bool,

    #[arg(short, long)]
    /// Owner of the forked repository
    ///
    /// If not provided, the authenticated user will be the owner
    pub organization: Option<String>,

    #[arg()]
    /// The owner of the repository to fork
    pub owner: String,

    #[arg()]
    /// Name of the repository to fork
    pub repository: String,

    #[arg()]
    /// Name of the remote as defined in the config (ex: 'github').
    /// The remote must be defined in the config file.
    ///
    /// There are no restrictions on the name,
    /// it does not have to correspond to the remote type (GitHub, GitLab, Gitea)
    pub remote: String,
}
