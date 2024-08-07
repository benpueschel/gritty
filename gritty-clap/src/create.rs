use clap::Parser;

use super::OutputFormat;

#[derive(Debug, Clone, Parser)]
#[command(about = "Create a repository on a remote")]
pub struct Create {
    #[arg(short, long, help = "Create a private repository")]
    pub private: bool,
    #[arg(short, long, help = "Clone the repository after creation")]
    pub clone: bool,
    #[arg(
        short,
        long,
        help = "Initialize and clone all submodules. Only valid with --clone"
    )]
    pub recursive: bool,
    #[arg(
        short,
        long,
        help = "Add the remote to the local git repository as 'origin'. Ignored if --clone is specified",
        long_help = "Add the remote to the local git repository as 'origin'.
If the current directory is not a git repository, it will be initialized as one.
Ignored if --clone is specified.
"
    )]
    pub add_remote: bool,
    #[arg(short, long, help = "Description of the repository")]
    pub description: Option<String>,
    #[arg(short, long, help = "Initialize the repository with a README.md")]
    pub init: bool,
    #[arg(
        short,
        long,
        help = concat!("License to use for the repository (ex: 'MIT'). ",
        "If not provided, or --init is not specified, no license will be addeed.")
        )]
    pub license: Option<String>,
    #[arg(help = "Name of the repository")]
    pub name: String,
    #[arg(help = "Name of the remote as defined in the config (ex: 'github')")]
    pub remote: String,
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
