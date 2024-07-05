use async_trait::async_trait;
use std::io::Error;
use teatime::{Client, CreateRepoOption, GetCommitsOption};

use super::*;

pub struct GiteaRemote {
    config: RemoteConfig,
    client: Client,
}

#[async_trait]
impl Remote for GiteaRemote {
    async fn new(config: &RemoteConfig) -> Self {
        let token = match config.auth.clone() {
            Auth::Token { token } => token,
            _ => panic!("auth must be a token for Gitea"),
        };

        let client = Client::new(config.url.clone(), token);

        Self {
            config: config.clone(),
            client,
        }
    }

    async fn create_repo(&self, create_info: RepoCreateInfo) -> Result<String, Error> {
        let cr = CreateRepoOption {
            name: create_info.name,
            description: create_info.description.unwrap_or_default(),
            private: create_info.private,
            ..Default::default()
        };
        let repo = self
            .client
            .user_create_repository(&cr)
            .await
            .map_err(map_error)?;
        Ok(repo.clone_url)
    }

    async fn get_repo_info(&self, name: &str) -> Result<Repository, Error> {
        let owner = &self.config.username;
        let repo = self
            .client
            .get_repository(owner, name)
            .await
            .map_err(map_error)?;

        // disable stats, verification, and files to speed up the request.
        // We only care about the commit messages.
        let commit_option = GetCommitsOption {
            stat: false,
            verification: false,
            files: false,
            limit: Some(25),
            ..Default::default()
        };
        let commits = self
            .client
            .get_commits(owner, name, &commit_option)
            .await
            .map_err(map_error)?;
        let last_commits = commits
            .into_iter()
            .map(|c| Commit {
                sha: c.sha,
                message: c.commit.message,
                author: c.commit.author.name,
                date: c.commit.author.date,
            })
            .collect();

        Ok(Repository {
            name: name.to_string(),
            description: Some(repo.description),
            private: repo.private,
            ssh_url: repo.ssh_url,
            clone_url: repo.clone_url,
            last_commits,
        })
    }

    async fn delete_repo(&self, name: &str) -> Result<(), Error> {
        self.client
            .delete_repository(&self.config.username, name)
            .await
            .map_err(map_error)
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
            .status()?;

        if !status.success() {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Failed to clone repository '{}'", name),
            ));
        }

        Ok(())
    }
}