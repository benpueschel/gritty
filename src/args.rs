use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
#[structopt(name = "gritty", about = "A tool to manage remote git repositories.")]
pub enum Args {
    #[structopt(about = "Create a default config")]
    CreateConfig,
    #[structopt(about = "List repositories on a remote")]
    List {
        #[structopt(help = "Name of the remote as defined in the config (ex: 'github')")]
        remote: String,
    },
    #[structopt(about = "Create a repository on a remote")]
    Create {
        #[structopt(short, long, help = "Create a private repository")]
        private: bool,
        #[structopt(short, long, help = "Clone the repository after creation")]
        clone: bool,
        #[structopt(short, long, help = "Initialize the repository with a README.md")]
        init: bool,
        #[structopt(
            short,
            long,
            help = concat!("License to use for the repository (ex: 'MIT'). ",
            "If not provided, or --init is not specified, no license will be addeed.")
        )]
        license: Option<String>,
        #[structopt(help = "Name of the repository")]
        name: String,
        #[structopt(help = "Name of the remote as defined in the config (ex: 'github')")]
        remote: String,
    },
    #[structopt(about = "Delete a repository on a remote")]
    Delete {
        #[structopt(help = "Name of the repository")]
        name: String,
        #[structopt(help = "Name of the remote as defined in the config (ex: 'github')")]
        remote: String,
    },
    #[structopt(about = "Authenticate with a remote")]
    Auth {
        #[structopt(help = "Name of the remote as defined in the config (ex: 'github')")]
        remote: String,
    },
}

