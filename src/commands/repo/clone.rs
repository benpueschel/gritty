use crate::args::repo::Clone;
use crate::commands::load_remote;
use crate::config::Config;
use crate::error::Result;

pub async fn clone_repository(args: Clone, config: &Config) -> Result<()> {
    let remote = load_remote(&args.remote, config).await?;
    let path = args.path.as_ref().map(|x| x.to_str().unwrap()).unwrap_or(&args.name);
    remote
        .clone_repo(&args.name, path, args.recursive)
        .await?;
    Ok(())
}
