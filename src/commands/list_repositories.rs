use crate::args::List;
use crate::error::Result;
use crate::log::{self, Highlight};
use crate::remote::ListReposInfo;
use chrono::{DateTime, Local};

use super::load_remote;

pub async fn list_repositories(args: List, config: &Option<String>) -> Result<()> {
    let remote = &args.remote;
    println!(
        "Listing repositories on remote {}...",
        Highlight::Remote(remote)
    );

    let remote = load_remote(remote, config).await?;
    let list_info = ListReposInfo {
        private: args.private,
        forks: args.forks,
    };
    let repos = remote.list_repos(list_info).await?;
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

        log::print(Highlight::Repo(log::leftpad(&repo.name, longest_name)));

        if repo.last_commits.is_empty() {
            print!(" - no commits");
        } else {
            let last = &repo.last_commits[0];
            let date: DateTime<Local> = last.date.into();
            let sha = last.sha.split_at(8).0;
            let message = last.message.split('\n').next().unwrap_or(&last.message);
            print!(
                " - {}: {} - {}",
                Highlight::Date(date),
                Highlight::Commit(sha),
                Highlight::CommitMsg(message)
            );
        }
        println!();
    }
    Ok(())
}
