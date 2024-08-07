use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(about = "Delete a repository on a remote")]
pub struct Delete {
    #[arg(help = "Name of the repository")]
    pub name: String,
    #[arg(help = "Name of the remote as defined in the config (ex: 'github')")]
    pub remote: String,
    #[arg(
        short,
        long,
        help = "Force deletion without confirmation. Use with caution!"
    )]
    pub force: bool,
}
