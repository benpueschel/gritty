use std::str::FromStr;

use crate::error::{Error, ErrorKind, Result};
use async_trait::async_trait;
use teatime::{
    error::{TeatimeError, TeatimeErrorKind},
    Client, CreateRepoOption, GetCommitsOption,
};

use super::*;

pub struct GiteaRemote {
    config: RemoteConfig,
    client: Client,
}

impl From<TeatimeError> for Error {
    fn from(err: TeatimeError) -> Self {
        let status = err.status_code.as_u16();
        let kind = match err.kind {
            TeatimeErrorKind::HttpError => ErrorKind::Other,
            TeatimeErrorKind::SerializationError => ErrorKind::Serialization,
            TeatimeErrorKind::Other => ErrorKind::Other,
        };
        Error {
            message: err.message,
            status: Some(status),
            kind,
        }
    }
}

#[async_trait]
impl Remote for GiteaRemote {
    async fn new(config: &RemoteConfig) -> Self {
        let auth = match config.auth.clone() {
            Auth::Token { token } => teatime::Auth::Token(token),
            Auth::Basic { username, password } => teatime::Auth::Basic(username, password),
        };

        let client = Client::new(config.url.clone(), auth);

        Self {
            config: config.clone(),
            client,
        }
    }

    async fn create_repo(&self, create_info: RepoCreateInfo) -> Result<Repository> {
        let cr = CreateRepoOption {
            auto_init: create_info.init,
            license: create_info.license.unwrap_or_default(),
            name: create_info.name,
            description: create_info.description.unwrap_or_default(),
            private: create_info.private,
            ..Default::default()
        };
        let repo = self.client.user_create_repository(&cr).await?;
        self.get_repo_info(repo).await
    }

    async fn list_repos(&self, list_info: ListReposInfo) -> Result<Vec<Repository>> {
        let owner = self.client.get_authenticated_user().await?;
        let search_option = teatime::SearchRepositoriesOption {
            private: Some(list_info.private),
            uid: Some(owner.id),
            limit: Some(100),
            ..Default::default()
        };
        let repos = self.client.search_repositories(&search_option).await?;
        let mut futures = Vec::new();
        for repo in repos {
            // SAFETY: We are not moving `self` in the closure, self is guaranteed to be valid as
            // long as the closure is running and we're not mutating it, so this is safe.
            let this = unsafe { &*(self as *const Self) };
            futures.push(tokio::spawn((*this).get_repo_info(repo)));
        }
        let mut result = Vec::with_capacity(futures.len());
        for future in futures {
            let f = future.await.unwrap()?;
            // Filter out forks. We can't filter them out in the search query because the API
            // requires us to make a whole new request to list all forks. I don't want to do that.
            if !list_info.forks && f.fork {
                continue;
            }
            result.push(f);
        }
        Ok(result)
    }

    async fn get_repo_info(&self, name: &str) -> Result<Repository> {
        let owner = &self.config.username;
        let repo = self.client.get_repository(owner, name).await?;
        self.get_repo_info(repo).await
    }

    async fn delete_repo(&self, name: &str) -> Result<()> {
        Ok(self
            .client
            .delete_repository(&self.config.username, name)
            .await?)
    }

    fn get_config(&self) -> &RemoteConfig {
        &self.config
    }
}

impl GiteaRemote {
    async fn get_repo_info(&self, repo: teatime::Repository) -> Result<Repository> {
        let owner = &self.config.username;
        let name = &repo.name;
        // disable stats, verification, and files to speed up the request.
        // We only care about the commit messages.
        let commit_option = GetCommitsOption {
            stat: false,
            verification: false,
            files: false,
            limit: Some(super::COMMIT_COUNT as i64),
            ..Default::default()
        };
        let commits = self.client.get_commits(owner, name, &commit_option).await;
        let commits = match commits {
            Ok(x) => x,
            Err(err) => {
                let status = err.status_code.as_u16();
                // 409 means the repository is empty, so return an empty list of commits.
                if status == 409 {
                    Vec::new()
                } else {
                    return Err(Error {
                        message: err.message,
                        kind: ErrorKind::Other,
                        status: Some(status),
                    });
                }
            }
        };
        let last_commits = commits
            .into_iter()
            .map(|c| Commit {
                sha: c.sha,
                message: c.commit.message,
                author: c.commit.author.name,
                date: DateTime::from_str(&c.commit.author.date).unwrap(),
            })
            .collect();

        Ok(Repository {
            name: name.to_string(),
            description: Some(repo.description),
            default_branch: Some(repo.default_branch),
            private: repo.private,
            fork: repo.fork,
            ssh_url: repo.ssh_url,
            clone_url: repo.clone_url,
            last_commits,
        })
    }
}
