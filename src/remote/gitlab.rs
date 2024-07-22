use super::{Auth, ListReposInfo, Remote, RemoteConfig, RepoCreateInfo, Repository};
use crate::{
    error::{Error, ErrorKind, Result},
    remote::COMMIT_COUNT,
};
use ::gitlab as gl;
use async_trait::async_trait;
use chrono::DateTime;
use gl::{
    api::{
        self,
        common::VisibilityLevel,
        projects::{
            self,
            repository::commits::{Commits, CommitsBuilderError},
            CreateProject, CreateProjectBuilderError, DeleteProjectBuilderError,
            ProjectBuilderError, Projects, ProjectsBuilderError,
        },
        ApiError, AsyncQuery, Pagination,
    },
    RestError,
};
use serde::{de::IgnoredAny, Deserialize};
use std::{error::Error as StdError, str::FromStr};

pub struct GitlabRemote {
    config: RemoteConfig,
    client: gl::AsyncGitlab,
}

/// A project in Gitlab.
#[derive(Debug, Deserialize)]
struct Project {
    name: String,
    description: Option<String>,
    forked_from_project: Option<IgnoredAny>,
    ssh_url_to_repo: String,
    http_url_to_repo: String,
    visibility: String,
    empty_repo: bool,
    id: u64,
}

impl<E: StdError + Send + Sync + 'static> From<ApiError<E>> for Error {
    fn from(value: ApiError<E>) -> Self {
        match value {
            ApiError::Client { source } => Error::other(source),
            ApiError::Auth { source } => Error::authentication(source),
            ApiError::UrlParse { source } => Error::deserialization(source),
            ApiError::Body { source } => Error::deserialization(source),
            ApiError::Json { source } => Error::deserialization(source),
            ApiError::MovedPermanently { location } => match location {
                Some(location) => Error::other(format!(
                    "Requested endpoint moved permanently to {location}"
                )),
                None => Error::other("Requested endpoint moved permanently"),
            },
            ApiError::Gitlab { msg } => Error::other(msg),
            ApiError::GitlabService { status, data: _ } => Error {
                message: "Gitlab service error".to_string(),
                kind: ErrorKind::Other,
                status: Some(status.into()),
            },
            ApiError::GitlabObject { obj } => Error::other(format!("Gitlab object error: {obj}")),
            ApiError::GitlabUnrecognized { obj } => {
                Error::other(format!("Gitlab unrecognized object: {obj}"))
            }
            ApiError::DataType { source, typename } => Error::deserialization(format!(
                "Failed to deserialize data of type {typename}: {source}"
            )),
            ApiError::Pagination { source } => {
                Error::other(format!("Failed to paginate data: {source}"))
            }
            ApiError::UnsupportedUrlBase { url_base } => {
                Error::other(format!("Unsupported URL base: {:?}", url_base))
            }

            x => Error::other(x),
        }
    }
}

impl From<DeleteProjectBuilderError> for Error {
    fn from(value: DeleteProjectBuilderError) -> Self {
        match value {
            DeleteProjectBuilderError::UninitializedField(field) => Error::other(format!(
                "Could not delete projects: field {field} is not initialized"
            )),
            DeleteProjectBuilderError::ValidationError(msg) => {
                Error::other(format!("Could not delete projects: {msg}"))
            }
            x => Error::other(format!("Could not delete projects: {x}")),
        }
    }
}
impl From<ProjectBuilderError> for Error {
    fn from(value: ProjectBuilderError) -> Self {
        match value {
            ProjectBuilderError::UninitializedField(field) => Error::other(format!(
                "Could not get project: field {field} is not initialized"
            )),
            ProjectBuilderError::ValidationError(msg) => {
                Error::other(format!("Could not get project: {msg}"))
            }
            x => Error::other(format!("Could not get project: {x}")),
        }
    }
}
impl From<CommitsBuilderError> for Error {
    fn from(value: CommitsBuilderError) -> Self {
        match value {
            CommitsBuilderError::UninitializedField(field) => Error::other(format!(
                "Could not get commits: field {field} is not initialized"
            )),
            CommitsBuilderError::ValidationError(msg) => {
                Error::other(format!("Could not get commits: {msg}"))
            }
            x => Error::other(format!("Could not get commits: {x}")),
        }
    }
}
impl From<ProjectsBuilderError> for Error {
    fn from(value: ProjectsBuilderError) -> Self {
        match value {
            ProjectsBuilderError::UninitializedField(field) => Error::other(format!(
                "Could not list projects: field {field} is not initialized"
            )),
            ProjectsBuilderError::ValidationError(msg) => {
                Error::other(format!("Could not list projects: {msg}"))
            }
            x => Error::other(format!("Could not list projects: {x}")),
        }
    }
}
impl From<CreateProjectBuilderError> for Error {
    fn from(value: CreateProjectBuilderError) -> Self {
        match value {
            CreateProjectBuilderError::UninitializedField(field) => Error::other(format!(
                "Could not create project: field {field} is not initialized"
            )),
            CreateProjectBuilderError::ValidationError(msg) => {
                Error::other(format!("Could not create project: {msg}"))
            }
            x => Error::other(format!("Could not create project: {x}")),
        }
    }
}

impl From<RestError> for Error {
    fn from(value: RestError) -> Self {
        match value {
            RestError::AuthError { source } => Error::authentication(source),
            x => Error::other(x),
        }
    }
}

#[async_trait]
impl Remote for GitlabRemote {
    async fn new(config: &RemoteConfig) -> Self {
        let token = match &config.auth {
            Auth::Token { token } => token,
            _ => panic!("auth must be a token for Gitlab"),
        };
        let client = gl::GitlabBuilder::new(config.url.clone().replace("https://", ""), token)
            .build_async()
            .await
            .expect("Failed to create Gitlab client");
        Self {
            config: config.clone(),
            client,
        }
    }
    async fn create_repo(&self, create_info: RepoCreateInfo) -> Result<String> {
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
        Ok(project.http_url_to_repo)
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
            fork: project.forked_from_project.is_some(),
            ssh_url: project.ssh_url_to_repo,
            clone_url: project.http_url_to_repo,
            last_commits,
        })
    }
}
