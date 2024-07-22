use crate::args::Delete;
use crate::error::{Error, Result};
use crate::log::{self, Highlight};

use super::{get_input, load_remote};

pub async fn delete_repository(args: Delete, config: &Option<String>) -> Result<()> {
    let Delete {
        name,
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
    println!(
        "{}: You are about to delete repository {} on remote {}.",
        Highlight::Important("WARNING"),
        Highlight::Repo(&name),
        Highlight::Remote(&remote_name),
    );

    if let Some(last) = repo_info.last_commits.first() {
        // Only show the first line of the commit message
        let message = last.message.split('\n').next().unwrap_or(&last.message);
        println!(
            "Last commit: {} - {} by {} on {}",
            Highlight::Commit(last.sha.split_at(8).0),
            Highlight::CommitMsg(message),
            Highlight::Author(&last.author),
            Highlight::Date(&last.date),
        );
    }
    log::print(Highlight::Important(
        "Are you sure you want to continue? (y/N): ",
    ));
    let input = get_input()?;
    if !input.eq_ignore_ascii_case("y") {
        println!("{}", Highlight::Special("Operation cancelled."));
        return Ok(());
    }
    remote.delete_repo(name).await?;
    println!(
        "Repository {} deleted on remote {}.",
        Highlight::Repo(&name),
        Highlight::Remote(&remote_name)
    );
    Ok(())
}
