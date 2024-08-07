use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(about = "Clone a repository from a remote")]
pub struct Clone {
    #[arg(help = "Name of the repository")]
    pub name: String,
    #[arg(help = "Name of the remote as defined in the config (ex: 'github')")]
    pub remote: String,
    #[arg(short, long, help = "Initialize and clone all submodules")]
    pub recursive: bool,
}

