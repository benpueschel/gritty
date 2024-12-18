use crate::args::repo::Clone;
use crate::commands::load_remote;
use crate::config::Config;
use crate::error::Result;

pub async fn clone_repository(args: Clone, config: &Config) -> Result<()> {
    let remote = load_remote(&args.remote, config).await?;
    remote
        .clone_repo(&args.name, &args.name, args.recursive)
        .await?;
    Ok(())
}
