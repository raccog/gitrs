mod error;
mod fileio;
mod objects;
mod paths;
pub mod subcommand_functions;

pub use error::{to_git_result, GitError, GitResult};
pub use fileio::{create_dir_all_if_new, create_dir_if_new, read_file, write_if_new};
pub use objects::{GitBlob, GitFileMode, GitObject, GitRepo};
