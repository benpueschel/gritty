use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(about = "Authenticate with a remote")]
pub struct Auth {
    #[arg(
        help = "Name of the remote as defined in the config (ex: 'github')",
        long_help = "\
Name of the remote as defined in the config (ex: 'github').
The remote must be defined in the config file. There are no restrictions on the name,
it does not have to correspond to the remote type (GitHub, GitLab, Gitea)"
    )]
    pub remote: String,
}
