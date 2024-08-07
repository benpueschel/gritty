use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(about = "Clone a repository from a remote")]
pub struct Clone {
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
    #[arg(short, long, help = "Initialize and clone all submodules", long_help = "\
Initialize and clone all submodules.
This is equivalent to running `git clone --recurse-submodules` (or `git clone --recursive`).")]
    pub recursive: bool,
}
