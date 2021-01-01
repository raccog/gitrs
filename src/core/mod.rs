mod decoding;
mod encoding;
mod error;
mod fileio;
mod objects;
mod paths;

pub use error::{to_git_result, GitError, GitResult};
pub use fileio::{create_dir_all_if_new, create_dir_if_new, write_if_new};
pub use objects::{
    GitBlob, GitObject, GitPath, GitPathBuf, GitRepo, ObjectPath, ObjectPathBuf, WorktreePath,
    WorktreePathBuf,
};
