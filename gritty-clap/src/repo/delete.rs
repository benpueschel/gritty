use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(aliases = &["rm"])]
/// Delete a repository on a remote
pub struct Delete {
    #[arg()]
    /// Name of the repository to delete.
    ///
    /// The repository must be owned by the authenticated user.
    pub name: String,

    #[arg()]
    /// Name of the remote as defined in the config (ex: 'github').
    ///
    /// The remote must be defined in the config file. There are no restrictions on the name,
    /// it does not have to correspond to the remote type (GitHub, GitLab, Gitea)"
    pub remote: String,

    #[arg(short, long)]
    /// Force deletion without confirmation. Use with caution!
    pub force: bool,
}
