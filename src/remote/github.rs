use std::{
    fmt::Debug,
    io::{Error, ErrorKind},
};

use async_trait::async_trait;
use chrono::DateTime;
use octocrab::{
    models::repos::{CommitAuthor, GitUserTime},
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

    async fn create_repo(&self, create_info: RepoCreateInfo) -> Result<String, Error> {
        #[derive(Serialize, Deserialize)]
        struct Response {
            clone_url: String,
        }

        let body = serde_json::to_value(&create_info).unwrap();
        let res: Response = self
            .crab
            .post("/user/repos", Some(&body))
            .await
            .map_err(map_error)?;

        Ok(res.clone_url)
    }

    async fn get_repo_info(&self, name: &str) -> Result<Repository, Error> {
        let base = self.crab.repos(self.config.username.clone(), name);

        let repo = base.get().await.map_err(map_error)?;

        let ssh_url = match repo.ssh_url {
            Some(url) => url.to_string(),
            None => format!(
                "git@{}/{}:{}.git",
                self.config.url, self.config.username, name
            ),
        };

        let clone_url = match repo.clone_url {
            Some(url) => url.to_string(),
            None => format!(
                "https://{}/{}/{}.git",
                self.config.url, self.config.username, name
            ),
        };

        let commits = base
            .list_commits()
            .per_page(25)
            .send()
            .await
            .map_err(map_error)?;

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

    async fn delete_repo(&self, name: &str) -> Result<(), Error> {
        self.crab
            .repos(self.config.username.clone(), name)
            .delete()
            .await
            .map_err(map_error)?;

        Ok(())
    }

    async fn clone_repo(&self, name: &str, path: &str) -> Result<(), Error> {
        let url = format!(
            "git@{}/{}:{}.git",
            self.config.url, self.config.username, name
        );

        let status = std::process::Command::new("git")
            .arg("clone")
            .arg(url)
            .arg(path)
            .status()
            .map_err(map_error)?;

        if !status.success() {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Failed to clone repository '{}'", name),
            ));
        }

        Ok(())
    }
}
