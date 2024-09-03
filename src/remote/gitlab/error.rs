use crate::error::{Error, ErrorKind};
use ::gitlab::{
    api::{
        projects::{
            repository::commits::CommitsBuilderError, CreateProjectBuilderError,
            DeleteProjectBuilderError, ProjectBuilderError, ProjectsBuilderError,
        },
        ApiError,
    },
    GitlabError, RestError,
};
use std::error::Error as StdError;

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
            ApiError::Gitlab { msg } => {
                // NOTE: For some asinine reason, the gitlab crate decides to check auth when
                // building the client, and then doesn't even manage to return the correct error
                // kind (THEY HAVE AN AUTH ERROR KIND, WHY NOT USE IT?). This is literally the only
                // data the gitlab client returns, so we have to check for it here.
                if msg == "401 Unauthorized" {
                    Error::authentication(msg)
                } else {
                    Error::other(msg)
                }
            }
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

impl From<GitlabError> for Error {
    fn from(value: GitlabError) -> Self {
        match value {
            GitlabError::UrlParse { source } => Error::serialization(source),
            GitlabError::AuthError { source } => Error::authentication(source),
            GitlabError::Communication { source } => Error::other(source),
            GitlabError::Http { status } => Error::other(format!("HTTP error: {}", status)),
            GitlabError::GraphQL { message } => Error::other(
                message
                    .iter()
                    .map(|x| x.message.clone())
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
            GitlabError::NoResponse {} => Error::other("No response from server"),
            GitlabError::DataType { source, typename } => Error::deserialization(format!(
                "Failed to deserialize data of type {typename}: {source}"
            )),
            GitlabError::Api { source } => source.into(),
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
