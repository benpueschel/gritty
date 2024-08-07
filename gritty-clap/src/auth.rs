use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(about = "Authenticate with a remote")]
pub struct Auth {
    #[arg(help = "Name of the remote as defined in the config (ex: 'github')")]
    pub remote: String,
}
