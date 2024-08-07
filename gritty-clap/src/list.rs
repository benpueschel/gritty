use clap::Parser;

use super::OutputFormat;

#[derive(Debug, Clone, Parser)]
#[command(about = "List repositories on a remote")]
pub struct List {
    #[arg(
        help = "Name of the remote as defined in the config (ex: 'github')",
        long_help = "\
Name of the remote as defined in the config (ex: 'github').
The remote must be defined in the config file. There are no restrictions on the name,
it does not have to correspond to the remote type (GitHub, GitLab, Gitea)"
    )]
    pub remote: String,
    #[arg(short, long, help = "Show private repositories")]
    pub private: bool,
    #[arg(short, long, help = "Show forks")]
    pub forks: bool,
    #[arg(
        long,
        help = "Change the output format to the specified value (can be 'json')",
        long_help = "\
Change the output format to the specified value.
This option is useful for parsing the output of gritty, such as in a script or another
tool integrating with gritty.

When using the 'json' format, gritty will output information about the repositories in JSON."
    )]
    pub format: Option<OutputFormat>,
}
