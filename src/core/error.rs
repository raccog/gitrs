use std::error::Error;
use std::ffi::OsString;
use std::fmt::{self, Display, Formatter};
use std::io::{self};
use std::path::PathBuf;

pub type GitResult<T> = Result<T, GitError>;

pub fn to_git_result<T>(result: io::Result<T>) -> GitResult<T> {
    match result {
        Ok(ok) => Ok(ok),
        Err(error) => Err(GitError::IOError(error)),
    }
}

/// All possible errors that can arise while using git.
#[derive(Debug)]
#[non_exhaustive]
pub enum GitError {
    GitDirInvalidUnicode(OsString),
    InvalidDirectory(PathBuf),
    IOError(io::Error),
}

impl Display for GitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            GitError::GitDirInvalidUnicode(data) => write!(
                f,
                "GIT_DIR environment variable: {} is in an invalid unicode format.",
                data.to_string_lossy()
            ),
            GitError::InvalidDirectory(path) => {
                write!(f, "Invalid directory: {}", path.to_str().unwrap())
            }
            GitError::IOError(error) => {
                write!(f, "IO Error: {}", error)
            }
        }
    }
}

impl Error for GitError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            GitError::IOError(error) => Some(error),
            _ => None,
        }
    }
}
