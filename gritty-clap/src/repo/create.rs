use clap::Parser;

use crate::OutputFormat;

#[derive(Debug, Clone, Parser)]
#[command(about = "Create a repository on a remote", aliases = &["new"])]
pub struct Create {
    #[arg(short, long, help = "Create a private repository")]
    pub private: bool,
    #[arg(short, long, help = "Clone the repository after creation")]
    pub clone: bool,
    #[arg(
        short,
        long,
        help = "Initialize and clone all submodules. Only valid with --clone",
        long_help = "\
Initialize and clone all submodules.
This is equivalent to running `git clone --recurse-submodules` (or `git clone --recursive`).
Only valid with --clone."
    )]
    pub recursive: bool,
    #[arg(
        short,
        long,
        help = "Add the remote to the local git repository as 'origin'. Ignored if --clone is specified",
        long_help = "\
Add the remote to the local git repository as 'origin'.
If the current directory is not a git repository, it will be initialized as one.
Ignored if --clone is specified."
    )]
    pub add_remote: bool,
    #[arg(short, long, help = "Description of the repository")]
    pub description: Option<String>,
    #[arg(short, long, help = "Initialize the repository with a README.md")]
    pub init: bool,
    #[arg(
        short,
        long,
        help = "\
License to use for the repository (ex: 'MIT').
If not provided, or --init is not specified, no license will be addeed."
    )]
    pub license: Option<String>,
    #[arg(
        help = "Name of the repository",
        long_help = "\
Name of the repository to clone.
The repository must be owned by the authenticated user."
    )]
    pub name: String,
    #[arg(
        help = "Name of the remote as defined in the config (ex: 'github')",
        long_help = "\
Name of the remote as defined in the config (ex: 'github').
The remote must be defined in the config file. There are no restrictions on the name,
it does not have to correspond to the remote type (GitHub, GitLab, Gitea)"
    )]
    pub remote: String,
    #[arg(
        long,
        help = "Change the output format to the specified value (can be 'json')",
        long_help = "\
Change the output format to the specified value.
This option is useful for parsing the output of gritty, such as in a script or another
tool integrating with gritty.

When using the 'json' format, gritty will output information about the newly created repository in JSON."
    )]
    pub format: Option<OutputFormat>,
}
