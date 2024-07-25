use crate::args::Delete;
use crate::error::{Error, Result};
use crate::log::{self, Highlight, Paint};
use crate::remote::Repository;

use super::{get_input, load_remote};

pub async fn delete_repository(args: Delete, config: &Option<String>) -> Result<()> {
    let Delete {
        name,
        force,
        remote: remote_name,
    } = &args;
    let remote = load_remote(remote_name, config).await?;
    let repo_info = match remote.get_repo_info(name).await {
        Ok(x) => x,
        Err(_) => {
            // TODO: match the actual error type
            return Err(Error::not_found(format!(
                "Repository {name} not found on remote {remote_name}."
            )));
        }
    };
    if !force && !ask_for_confirmation(name, remote_name, &repo_info)? {
        println!("{}", "Operation cancelled.".paint(Highlight::Special));
        return Ok(());
    }
    remote.delete_repo(name).await?;
    println!(
        "Repository {} deleted on remote {}.",
        &name.paint(Highlight::Repo),
        &remote_name.paint(Highlight::Remote)
    );
    Ok(())
}

fn ask_for_confirmation(name: &str, remote_name: &str, repo: &Repository) -> Result<bool> {
    println!(
        "{}: You are about to delete repository {} on remote {}.",
        "WARNING".paint(Highlight::Important),
        &name.paint(Highlight::Repo),
        &remote_name.paint(Highlight::Remote),
    );

    if let Some(last) = repo.last_commits.first() {
        // Only show the first line of the commit message
        let message = last.message.split('\n').next().unwrap_or(&last.message);
        println!(
            "Last commit: {} - {} by {} on {}",
            last.sha.split_at(8).0.paint(Highlight::Commit),
            message.paint(Highlight::CommitMsg),
            &last.author.paint(Highlight::Author),
            &last.date.to_string().paint(Highlight::Date),
        );
    }
    log::print("Are you sure you want to continue? (y/N): ".paint(Highlight::Important));
    let input = get_input()?;
    // Only accept "y" or "Y" as confirmation, return false otherwise
    Ok(input.eq_ignore_ascii_case("y"))
}
