use clap::{Parser, Subcommand};

pub mod clone;
pub mod create;
pub mod delete;
pub mod fork;
pub mod list;

pub use clone::Clone;
pub use create::Create;
pub use delete::Delete;
pub use fork::Fork;
pub use list::List;

#[derive(Debug, Clone, Parser)]
#[command()]
/// Manage repositories on a remote.
pub struct Repo {
    #[command(subcommand)]
    pub subcommand: RepoCommands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum RepoCommands {
    Clone(Clone),
    List(List),
    Create(Create),
    Fork(Fork),
    Delete(Delete),
}
