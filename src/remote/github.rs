use std::io::{Error, ErrorKind};

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

use super::{map_error, Commit, Remote, RemoteConfig, RepoCreateInfo, Repository};

pub struct GitHubRemote {
    config: RemoteConfig,
    crab: Octocrab,
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

    async fn create_repo(&self, create_info: RepoCreateInfo) -> Result<String, Error> {
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
        let res: Response = self
            .crab
            .post("/user/repos", Some(&body))
            .await
            .map_err(map_error)?;

        Ok(res.clone_url)
    }

    async fn list_repos(&self) -> Result<Vec<Repository>, Error> {
        let repos = self
            .crab
            .search()
            .repositories(&format!("user:{}", &self.config.username))
            .per_page(100)
            .sort("updated")
            .order("desc")
            .send()
            .await
            .map_err(map_error)?;
        let mut result = Vec::new();
        for repo in repos {
            let base = self
                .crab
                .repos(self.config.username.clone(), repo.name.clone());
            result.push(self.get_repo_info(base, repo).await?);
        }
        Ok(result)
    }

    async fn get_repo_info(&self, name: &str) -> Result<Repository, Error> {
        let base = self.crab.repos(self.config.username.clone(), name);
        let repo = base.get().await.map_err(map_error)?;
        self.get_repo_info(base, repo).await
    }

    async fn delete_repo(&self, name: &str) -> Result<(), Error> {
        self.crab
            .repos(self.config.username.clone(), name)
            .delete()
            .await
            .map_err(map_error)?;

        Ok(())
    }
}

impl GitHubRemote {
    async fn get_repo_info(
        &self,
        base: RepoHandler<'_>,
        repo: models::Repository,
    ) -> Result<Repository, Error> {
        static COMMIT_COUNT: u8 = 25;
        let commits = base.list_commits().per_page(COMMIT_COUNT).send().await;
        let ssh_url = match repo.ssh_url {
            Some(url) => url.to_string(),
            None => format!(
                "git@{}/{}:{}.git",
                self.config.url, self.config.username, repo.name
            ),
        };

        let clone_url = match repo.clone_url {
            Some(url) => url.to_string(),
            None => format!(
                "https://{}/{}/{}.git",
                self.config.url, self.config.username, repo.name
            ),
        };

        let commits = match commits {
            Ok(x) => x,
            Err(err) => {
                if let octocrab::Error::GitHub {
                    source,
                    backtrace: _,
                } = &err
                {
                    // If the repository is empty, return an empty list of commits
                    // TODO: this is a bit hacky, is there a better way to handle this?
                    if source.message == "Git Repository is empty." {
                        Default::default()
                    } else {
                        return Err(Error::new(ErrorKind::Other, source.message.clone()));
                    }
                } else {
                    return Err(Error::new(ErrorKind::Other, format!("{}", err)));
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
                    date: author.date.unwrap_or_default().to_rfc2822(),
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
