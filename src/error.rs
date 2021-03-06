use std::error::Error;
use std::ffi::OsString;
use std::fmt::{self, Display, Formatter};
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};

/// A result that could contain a [GitError].
pub type GitResult<T> = Result<T, GitError>;

/// Maps an [io::Result] to a [GitResult] and maps any [io::Error]s into [GitError]s.
pub fn to_git_result<T, P: AsRef<Path>>(result: io::Result<T>, path: P) -> GitResult<T> {
    match result {
        Ok(ok) => Ok(ok),
        Err(error) => {
            let path = path.as_ref().to_path_buf();
            Err(GitError::IOError { error, path })
        }
    }
}

/// All possible errors that can arise while using git.
///
/// Every git subcommand function returns a [GitResult] possibly containing this error.
#[derive(Debug)]
#[non_exhaustive]
pub enum GitError {
    /// An environment variable that was read contains an invalid UTF-8 sequence.
    VarInvalidUnicode { var: OsString, data: OsString },
    /// Any error coming from [io::Error].
    IOError { error: io::Error, path: PathBuf },
}

impl Display for GitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            GitError::VarInvalidUnicode { var, data } => write!(
                f,
                "Environment variable {}: {} is an invalid byte sequence and cannot be read.",
                var.to_str().unwrap(),
                data.to_string_lossy()
            ),
            GitError::IOError { error, path } => {
                // TODO: Make path absolute for console output without using fs::canonicalize.
                let path = path.to_str().unwrap();
                let msg = match error.kind() {
                    ErrorKind::NotFound => {
                        format!("fatal: Invalid path '{}': No such file or directory.", path)
                    }
                    ErrorKind::PermissionDenied => {
                        format!("fatal: Permission denied when accessing {}.", path)
                    }
                    ErrorKind::ConnectionRefused => {
                        format!("fatal: Connection refused when accessing {}.", path)
                    }
                    ErrorKind::ConnectionReset => {
                        format!("fatal: Connection reset when accessing {}.", path)
                    }
                    ErrorKind::ConnectionAborted => {
                        format!("fatal: Connection aborted when accessing {}.", path)
                    }
                    ErrorKind::NotConnected => format!(
                        "fatal: Failed to access {} as network is not connected.",
                        path
                    ),
                    ErrorKind::AddrInUse => format!(
                        "fatal: Failed to access {} as the socket is already in use elsewhere",
                        path
                    ),
                    ErrorKind::AddrNotAvailable => format!(
                        "fatal: Failed to access {} as the address is not available.",
                        path
                    ),
                    ErrorKind::BrokenPipe => {
                        format!("fatal: Broken pipe while accessing {}.", path)
                    }
                    ErrorKind::AlreadyExists => format!("fatal: {} already exists.", path),
                    ErrorKind::WouldBlock => format!(
                        "fatal: The operation on {} needs to block to complete,
                        but the blocking operation was requested to not occur.",
                        path
                    ),
                    ErrorKind::InvalidInput => {
                        format!("fatal: Invalid input when accessing {}.", path)
                    }
                    ErrorKind::InvalidData => {
                        format!("fatal: Invalid byte sequence read from {}.", path)
                    }
                    ErrorKind::TimedOut => format!("fatal: Timed out when accessing {}.", path),
                    ErrorKind::WriteZero => {
                        "FATAL FOR PROGRAMMERS: I don't understand when this error is used."
                            .to_string()
                    }
                    ErrorKind::Interrupted => {
                        format!("fatal: Operation inturrupted when accessing {}.", path)
                    }
                    ErrorKind::Other => format!(
                        "FATAL FOR PROGRAMMERS: Unknown error occurred when acessing {}.",
                        path
                    ),
                    ErrorKind::UnexpectedEof => format!(
                        "fatal: Reached unexpected end of file when reading {}.",
                        path
                    ),
                    _ => format!(
                        "FATAL FOR PROGRAMMERS: Unknown ErrorKind when accessing {}.",
                        path
                    ),
                };
                write!(f, "{}", msg)
            }
        }
    }
}

impl Error for GitError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            GitError::IOError { error, path: _ } => Some(error),
            _ => None,
        }
    }
}
