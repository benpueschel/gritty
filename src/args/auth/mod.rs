use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command()]
/// Authentication commands.
pub struct Auth {
    #[command(subcommand)]
    pub subcommand: AuthCommands,
}

#[derive(Debug, Clone, Parser)]
#[command()]
/// Authenticate with a remote.
pub struct Login {
    #[arg()]
    /// Name of the remote as defined in the config (ex: 'github').
    ///
    /// The remote must be defined in the config file. There are no restrictions on the name,
    /// it does not have to correspond to the remote type (GitHub, GitLab, Gitea)"
    pub remote: String,
}

#[derive(Debug, Clone, Parser)]
pub enum AuthCommands {
    Login(Login),
    #[command()]
    /// Show the current authentication status.
    ///
    /// This will show the current authentication status for all remotes.
    /// There are three possible statuses:
    ///
    /// - `authenticated`: The remote is authenticated.
    ///
    /// - `not authenticated`: The remote is not authenticated.
    ///
    /// - `not configured`: The remote has no configured method of authentication.
    Status,
}
