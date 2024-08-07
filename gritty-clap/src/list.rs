use clap::Parser;

use super::OutputFormat;

#[derive(Debug, Clone, Parser)]
#[command(about = "List repositories on a remote")]
pub struct List {
    #[arg(help = "Name of the remote as defined in the config (ex: 'github')")]
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
Note that the 'create-config' and 'auth' subcommands do not respect this option.
Currently, the only supported format is 'json'."
    )]
    pub format: Option<OutputFormat>,
}

