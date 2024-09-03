use std::str::FromStr;

use crate::error::{Error, ErrorKind, Result};
use async_trait::async_trait;
use gitea_sdk::{
    error::{TeatimeError, TeatimeErrorKind},
    Client,
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
    async fn new(config: &RemoteConfig) -> Result<Self> {
        let auth = match config.auth.clone() {
            Auth::Token { token } => gitea_sdk::Auth::Token(token),
            Auth::Basic { username, password } => gitea_sdk::Auth::Basic(username, password),
        };

        let client = Client::new(config.url.clone(), auth);

        Ok(Self {
            config: config.clone(),
            client,
        })
    }

    async fn check_auth(&self) -> Result<bool> {
        if let Err(err) = self.client.user().current().send(&self.client).await {
            match err.status_code.as_u16() {
                401 | 403 => return Ok(false),
                _ => {}
            }
            return Err(err.into());
        }
        Ok(true)
    }

    async fn create_repo(&self, create_info: RepoCreateInfo) -> Result<Repository> {
        let repo = self
            .client
            .user()
            .create_repo(create_info.name)
            .auto_init(create_info.init)
            .description(create_info.description.unwrap_or_default())
            .license(create_info.license.unwrap_or_default())
            .private(create_info.private)
            .send(&self.client)
            .await?;
        self.get_repo_info(repo).await
    }

    async fn create_fork(&self, options: RepoForkOption) -> Result<Repository> {
        let mut fork = self.client.repos(options.owner, options.repo).create_fork();
        if let Some(name) = options.name.clone() {
            fork = fork.name(name);
        }
        if let Some(org) = options.organization.clone() {
            fork = fork.organization(org);
        }
        let fork = fork.send(&self.client).await?;
        self.get_repo_info(fork).await
    }

    async fn list_repos(&self, list_info: ListReposInfo) -> Result<Vec<Repository>> {
        let owner = self.client.user().current().send(&self.client).await?;
        let repos = self
            .client
            .search()
            .repos()
            .private(list_info.private)
            .uid(owner.id)
            .limit(100)
            .send(&self.client)
            .await?;
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
        let repo = self
            .client
            .repos(owner, name)
            .get()
            .send(&self.client)
            .await?;
        self.get_repo_info(repo).await
    }

    async fn delete_repo(&self, name: &str) -> Result<()> {
        Ok(self
            .client
            .repos(&self.config.username, name)
            .delete()
            .send(&self.client)
            .await?)
    }

    fn get_config(&self) -> &RemoteConfig {
        &self.config
    }
}

impl GiteaRemote {
    async fn get_repo_info(&self, repo: gitea_sdk::model::repos::Repository) -> Result<Repository> {
        let owner = &self.config.username;
        let name = &repo.name;
        // disable stats, verification, and files to speed up the request.
        // We only care about the commit messages.
        let commits = self
            .client
            .repos(owner, name)
            .get_commits()
            .stat(false)
            .verification(false)
            .files(false)
            .limit(super::COMMIT_COUNT as i64)
            .send(&self.client)
            .await;
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
