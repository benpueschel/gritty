use crate::error::{Error, ErrorKind, Result};
use async_trait::async_trait;
use chrono::DateTime;
use octocrab::{
    models::{
        self,
        repos::{CommitAuthor, GitUserTime},
    },
    repos::RepoHandler,
    Octocrab,
};
use serde::{Deserialize, Serialize};

use super::{Commit, ListReposInfo, Remote, RemoteConfig, RepoCreateInfo, Repository};

pub struct GitHubRemote {
    config: RemoteConfig,
    crab: Octocrab,
}

impl From<octocrab::Error> for Error {
    // NOTE: the formatting is quite weird when explicitly ignoring the backtrace field
    // (backtrace: _), so disable the unused_variables warning for this function
    #[allow(unused_variables)]
    fn from(value: octocrab::Error) -> Self {
        use octocrab::Error::*;
        match value {
            GitHub { source, backtrace } => {
                let status = source.status_code.as_u16();
                let kind = match status {
                    404 => ErrorKind::NotFound,
                    401 => ErrorKind::Authentication,
                    _ => ErrorKind::Other,
                };
                Error {
                    message: source.message,
                    status: Some(status),
                    kind,
                }
            }
            UriParse { source, backtrace } => Error::other(source),
            Uri { source, backtrace } => Error::other(source),
            InvalidHeaderValue { source, backtrace } => Error::other(source),
            Http { source, backtrace } => Error::other(source),
            InvalidUtf8 { source, backtrace } => Error::other(source),
            Encoder { source, backtrace } => source.into(),
            Service { source, backtrace } => Error::other(source),
            Hyper { source, backtrace } => Error::other(source),
            SerdeUrlEncoded { source, backtrace } => Error::serialization(source),
            Serde { source, backtrace } => source.into(),
            Json { source, backtrace } => source.into_inner().into(),
            JWT { source, backtrace } => Error::other(source),
            Other { source, backtrace } => Error::other(source),
        }
    }
}

#[async_trait]
impl Remote for GitHubRemote {
    async fn new(config: &RemoteConfig) -> Self {
        let mut crab = Octocrab::builder();

        use super::Auth::*;
        crab = match config.auth.clone() {
            Basic { username, password } => crab.basic_auth(username, password),
            Token { token } => crab.personal_token(token),
        };

        let crab = crab.build().expect("Failed to create Octocrab instance");
        Self {
            crab,
            config: config.clone(), // TODO: remove clone
        }
    }

    fn get_config(&self) -> &RemoteConfig {
        &self.config
    }

    async fn create_repo(&self, create_info: RepoCreateInfo) -> Result<String> {
        #[derive(Serialize, Deserialize)]
        struct Request {
            name: String,
            description: Option<String>,
            private: bool,
            license_template: Option<String>,
            auto_init: bool,
        }
        #[derive(Serialize, Deserialize)]
        struct Response {
            clone_url: String,
        }
        let req = Request {
            name: create_info.name,
            description: create_info.description,
            private: create_info.private,
            license_template: create_info.license,
            auto_init: create_info.init,
        };
        let body = serde_json::to_value(&req).unwrap();
        let res: Response = self.crab.post("/user/repos", Some(&body)).await?;

        Ok(res.clone_url)
    }

    async fn list_repos(&self, list_info: ListReposInfo) -> Result<Vec<Repository>> {
        let repos = self
            .crab
            .search()
            .repositories(&format!("user:{}", &self.config.username))
            .per_page(100)
            .sort("updated")
            .order("desc")
            .send()
            .await?;
        let mut futures = Vec::new();
        for repo in repos {
            // SAFETY: We are not moving `self` in the closure, self is guaranteed to be valid as
            // long as the closure is running and we're not mutating it, so this is safe.
            let this = unsafe { &*(self as *const Self) };
            let base = this
                .crab
                .repos(self.config.username.clone(), repo.name.clone());
            let username = &self.config.username;
            futures.push(tokio::spawn(Self::get_repo_info(
                username.clone(),
                base,
                repo,
            )));
        }
        let mut result = Vec::with_capacity(futures.len());
        for future in futures {
            let f = future.await.unwrap()?;
            // Skip private repositories if the user doesn't want them. We need to do this here
            // because the GitHub search API doesn't seem to support filtering by privacy.
            if !list_info.private && f.private {
                continue;
            }
            result.push(f);
        }
        Ok(result)
    }

    async fn get_repo_info(&self, name: &str) -> Result<Repository> {
        let base = self.crab.repos(self.config.username.clone(), name);
        let repo = base.get().await?;
        Self::get_repo_info(self.config.username.clone(), base, repo).await
    }

    async fn delete_repo(&self, name: &str) -> Result<()> {
        self.crab
            .repos(self.config.username.clone(), name)
            .delete()
            .await?;

        Ok(())
    }
}

impl GitHubRemote {
    async fn get_repo_info(
        username: String,
        base: RepoHandler<'_>,
        repo: models::Repository,
    ) -> Result<Repository> {
        let commits = base
            .list_commits()
            .per_page(super::COMMIT_COUNT)
            .send()
            .await;
        let ssh_url = match repo.ssh_url {
            Some(url) => url.to_string(),
            None => format!("git@github.com/{}:{}.git", &username, repo.name),
        };

        let clone_url = match repo.clone_url {
            Some(url) => url.to_string(),
            None => format!("https://github.com/{}/{}.git", &username, repo.name),
        };

        use octocrab::Error::GitHub;
        let commits = match commits {
            Ok(x) => x,
            Err(err) => {
                if let GitHub {
                    source,
                    backtrace: _,
                } = &err
                {
                    let status = source.status_code.as_u16();
                    // 409 is the status code for a repository with no commits
                    if status == 409 {
                        Default::default()
                    } else {
                        return Err(Error {
                            message: source.message.clone(),
                            kind: ErrorKind::Other,
                            status: Some(status),
                        });
                    }
                } else {
                    return Err(Error::other(format!("{}", err)));
                }
            }
        };

        let last_commits = commits
            .items
            .into_iter()
            .map(|c| {
                let author = c.commit.author.unwrap_or(GitUserTime {
                    date: Some(DateTime::default()),
                    username: None,
                    user: CommitAuthor {
                        date: None,
                        name: "unknown".to_string(),
                        email: "unknown".to_string(),
                    },
                });
                Commit {
                    sha: c.sha,
                    message: c.commit.message,
                    author: author.user.name,
                    date: author.date.unwrap_or_default(),
                }
            })
            .collect();

        Ok(Repository {
            name: repo.name,
            description: repo.description,
            private: repo.private.unwrap_or(false),
            last_commits,
            ssh_url,
            clone_url,
        })
    }
}
