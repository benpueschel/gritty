use gritty_clap::{repo::List, OutputFormat};
use crate::config::Config;
use crate::error::Result;
use crate::log::{self, Highlight, Paint};
use crate::remote::{ListReposInfo, Repository};
use chrono::{DateTime, Local};

use crate::commands::load_remote;

pub async fn list_repositories(args: List, config: &Config) -> Result<()> {
    let remote = &args.remote;
    let format = args.format.unwrap_or_default();
    if let OutputFormat::Human = format {
        println!(
            "Listing repositories on remote {}...",
            remote.paint(Highlight::Remote)
        );
    }

    let remote = load_remote(remote, config).await?;
    let list_info = ListReposInfo {
        private: args.private,
        forks: args.forks,
    };
    let repos = remote.list_repos(list_info).await?;
    match format {
        OutputFormat::Human => print_human(args, repos),
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&repos)?),
    }
    Ok(())
}

fn print_human(args: List, repos: Vec<Repository>) {
    if args.private {
        println!("* denotes private repositories");
    }
    if args.forks {
        println!("^ denotes forked repositories");
    }
    let mut longest_name = 0;
    for repo in &repos {
        if repo.name.len() > longest_name {
            longest_name = repo.name.len();
        }
    }
    for repo in &repos {
        if repo.private {
            print!("* ");
        } else if repo.fork {
            print!("^ ");
        } else {
            print!("  ");
        }

        log::print(log::leftpad(&repo.name, longest_name).paint(Highlight::Repo));

        if repo.last_commits.is_empty() {
            print!(" - no commits");
        } else {
            let last = &repo.last_commits[0];
            let date: DateTime<Local> = last.date.into();
            let sha = last.sha.split_at(8).0;
            let message = last.message.split('\n').next().unwrap_or(&last.message);
            print!(
                " - {}: {} - {}",
                date.to_string().paint(Highlight::Date),
                sha.paint(Highlight::Commit),
                message.paint(Highlight::CommitMsg)
            );
        }
        println!();
    }
}
