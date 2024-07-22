use crate::args::Create;
use crate::error::Result;
use crate::log::Highlight;
use crate::remote::RepoCreateInfo;

use super::load_remote;

pub async fn create_repository(args: Create, config: &Option<String>) -> Result<()> {
    let Create {
        private,
        clone,
        recursive,
        name,
        description,
        license,
        init,
        remote,
    } = args;
    let remote = load_remote(&remote, config).await?;
    println!("Creating repository {}...", Highlight::Repo(&name));
    let info = RepoCreateInfo {
        name: name.clone(),
        description,
        license,
        init,
        private,
    };
    let url = remote.create_repo(info).await?;
    println!("Repository created at: {}", Highlight::Url(&url));
    if clone {
        remote.clone_repo(&name, &name, recursive).await?;
    }
    Ok(())
}
