use crate::args::Clone;
use crate::error::Result;
use crate::config::Config;

use super::load_remote;

pub async fn clone_repository(args: Clone, config: &Config) -> Result<()> {
    let remote = load_remote(&args.remote, config).await?;
    remote
        .clone_repo(&args.name, &args.name, args.recursive)
        .await?;
    Ok(())
}
