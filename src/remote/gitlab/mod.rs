use super::{
    Auth, ListReposInfo, Remote, RemoteConfig, RepoCreateInfo, RepoForkOption, Repository,
};
use crate::{
    error::{Error, Result},
    remote::COMMIT_COUNT,
};
use ::gitlab as gl;
use async_trait::async_trait;
use chrono::DateTime;
use gitlab::api::users::CurrentUser;
use gl::api::{
    self,
    common::VisibilityLevel,
    endpoint_prelude::Method,
    projects::{self, repository::commits::Commits, CreateProject, Projects},
    ApiError, AsyncQuery, Endpoint, Pagination,
};
use serde::{de::IgnoredAny, Deserialize};
use std::{borrow::Cow, str::FromStr};

pub mod error;

pub struct GitlabRemote {
    config: RemoteConfig,
    client: gl::AsyncGitlab,
}

/// A project in Gitlab.
#[derive(Debug, Deserialize)]
struct Project {
    name: String,
    description: Option<String>,
    default_branch: Option<String>,
    forked_from_project: Option<IgnoredAny>,
    ssh_url_to_repo: String,
    http_url_to_repo: String,
    visibility: String,
    empty_repo: bool,
    id: u64,
}

#[async_trait]
impl Remote for GitlabRemote {
    async fn new(config: &RemoteConfig) -> Result<Self> {
        let token = match &config.auth {
            Auth::Token { token } => token,
            _ => {
                return Err(Error::other(
                    "Only token authentication is supported for Gitlab",
                ))
            }
        };
        let client = gl::GitlabBuilder::new(config.url.clone().replace("https://", ""), token)
            .build_async()
            .await?;
        Ok(Self {
            config: config.clone(),
            client,
        })
    }
    #[allow(dead_code)]
    async fn check_auth(&self) -> Result<bool> {
        #[derive(Deserialize)]
        struct User {
            pub id: u64,
        }
        let endpoint = CurrentUser::builder()
            .build()
            .expect("building user should always work");
        let _: User = match endpoint.query_async(&self.client).await {
            Ok(x) => x,
            Err(err) => {
                if let ApiError::GitlabService { status, data: _ } = &err {
                    if *status == 401 {
                        return Ok(false);
                    }
                }
                return Err(Error::other(format!("Failed to check auth: {}", err)));
            }
        };
        Ok(true)
    }
    async fn create_repo(&self, create_info: RepoCreateInfo) -> Result<Repository> {
        let visibility = match create_info.private {
            true => VisibilityLevel::Private,
            false => VisibilityLevel::Public,
        };
        if create_info.license.is_some() {
            println!("License is not supported by Gitlab. Ignoring.");
        }
        let project = CreateProject::builder()
            .name(create_info.name)
            .visibility(visibility)
            .description(create_info.description.unwrap_or_default())
            .initialize_with_readme(create_info.init)
            .build()?;

        let project: Project = project.query_async(&self.client).await?;
        self.get_project_info(project).await
    }
    async fn create_fork(&self, options: RepoForkOption) -> Result<Repository> {
        #[derive(Debug, Deserialize)]
        #[allow(dead_code)]
        struct ForkEndpoint {
            #[serde(skip)]
            id: String,
            /// The name of the fork.
            name: Option<String>,
            /// The path of the fork.
            namespace_path: Option<String>,
        }
        impl Endpoint for ForkEndpoint {
            /// The HTTP method to use for the endpoint.
            fn method(&self) -> Method {
                Method::POST
            }
            /// The path to the endpoint.
            fn endpoint(&self) -> Cow<'static, str> {
                format!("/projects/{}/fork", self.id).into()
            }
        }
        let path = options.repo.replace(' ', "-").to_lowercase();
        let encoded_path = format!(
            "{}/{}",
            urlencoding::encode(&options.owner),
            urlencoding::encode(&path)
        );
        let endpoint = ForkEndpoint {
            id: encoded_path,
            name: options.name,
            namespace_path: options.organization,
        };
        let project = endpoint.query_async(&self.client).await?;
        self.get_project_info(project).await
    }
    async fn list_repos(&self, list_info: ListReposInfo) -> Result<Vec<Repository>> {
        let mut projects = Projects::builder();
        projects.owned(true).include_hidden(list_info.private);
        if !list_info.private {
            projects.visibility(VisibilityLevel::Public);
        }

        let projects = projects.build()?;
        let projects: Vec<Project> = projects.query_async(&self.client).await?;

        let mut futures = Vec::new();
        for project in projects {
            // SAFETY: We are not moving `self` in the closure, self is guaranteed to be valid as
            // long as the closure is running and we're not mutating it, so this is safe.
            let this = unsafe { &*(self as *const Self) };
            futures.push(tokio::spawn(this.get_project_info(project)));
        }
        let mut result = Vec::with_capacity(futures.len());
        for future in futures {
            let f = future.await.unwrap()?;
            // Filter out forks if the user doesn't want them
            if !list_info.forks && f.fork {
                continue;
            }
            result.push(f);
        }
        Ok(result)
    }
    async fn get_repo_info(&self, name: &str) -> Result<Repository> {
        let path = name.replace(' ', "-").to_lowercase();
        let encoded_path = format!(
            "{}/{}",
            urlencoding::encode(&self.config.username),
            urlencoding::encode(&path)
        );
        let project = projects::Project::builder().project(encoded_path).build()?;
        let project: Project = project.query_async(&self.client).await?;
        self.get_project_info(project).await
    }
    async fn delete_repo(&self, name: &str) -> Result<()> {
        let path = name.replace(' ', "-").to_lowercase();
        let encoded_path = format!(
            "{}/{}",
            urlencoding::encode(&self.config.username),
            urlencoding::encode(&path)
        );
        let endpoint = projects::DeleteProject::builder()
            .project(encoded_path)
            .build()?;
        api::ignore(endpoint).query_async(&self.client).await?;
        Ok(())
    }
    fn get_config(&self) -> &RemoteConfig {
        &self.config
    }
}

impl GitlabRemote {
    async fn get_project_info(&self, project: Project) -> Result<Repository> {
        #[derive(Debug, Deserialize)]
        struct Commit {
            id: String,
            message: String,
            author_name: String,
            committed_date: String,
        }

        let mut last_commits;
        if project.empty_repo {
            last_commits = Vec::new();
        } else {
            let commits = Commits::builder().project(project.id).build()?;
            let commits = api::paged(commits, Pagination::Limit(COMMIT_COUNT as usize));
            let commits: Vec<Commit> = commits.query_async(&self.client).await?;

            last_commits = Vec::with_capacity(commits.len());
            for commit in commits {
                last_commits.push(super::Commit {
                    sha: commit.id,
                    message: commit.message,
                    author: commit.author_name,
                    // TODO: handle error
                    date: DateTime::from_str(&commit.committed_date).unwrap(),
                });
            }
        }

        Ok(Repository {
            name: project.name,
            description: project.description,
            private: project.visibility == "private",
            default_branch: project.default_branch,
            fork: project.forked_from_project.is_some(),
            ssh_url: project.ssh_url_to_repo,
            clone_url: project.http_url_to_repo,
            last_commits,
        })
    }
}
