use std::env::{self, VarError};
use std::path::{Path, PathBuf};

use clap::ArgMatches;

use crate::core::{GitError, GitResult};

/// A [Path] to the current worktree directory.
pub type WorktreePath = Path;
/// A [PathBuf] to the current worktree directory.
pub type WorktreePathBuf = PathBuf;

/// A [Path] to the current git directory.
pub type GitPath = Path;
/// A [PathBuf] to the current git directory.
pub type GitPathBuf = PathBuf;

/// A [Path] to an object file.
pub type ObjectPath = Path;
/// A [PathBuf] to an object file.
pub type ObjectPathBuf = PathBuf;

/// A container for all information about a git repository.
pub struct GitRepo {
    worktree: WorktreePathBuf,
    gitpath: GitPathBuf,
}

impl GitRepo {
    /// Creates a container from existing info about the repository.
    pub fn new(worktree: WorktreePathBuf, gitpath: GitPathBuf) -> Self {
        Self { worktree, gitpath }
    }
    /// Returns the current git repository from command line arguments.
    ///
    /// # Errors
    ///
    /// * GIT_DIR environment variable is invalid unicode
    /// * Arguments contain a path to an invalid directory
    pub fn from_args(args: &ArgMatches) -> GitResult<GitRepo> {
        // Get GIT_DIR environment variable
        let default = match env::var("GIT_DIR") {
            Ok(dir) => dir,
            Err(VarError::NotPresent) => ".".to_string(),
            Err(VarError::NotUnicode(dir)) => {
                return Err(GitError::GitDirInvalidUnicode(dir));
            }
        };

        // Check args for path changes
        let worktree = WorktreePathBuf::from(match args.subcommand() {
            ("init", Some(sub_m)) => sub_m.value_of("DIRECTORY").unwrap(),
            _ => &default,
        });

        if !worktree.is_dir() {
            return Err(GitError::InvalidDirectory(worktree));
        }
        let gitpath = worktree.join(".git");

        Ok(GitRepo::new(worktree, gitpath))
    }

    /// Returns a [Path] to the git directory of this repository.
    pub fn gitpath(&self) -> &GitPath {
        self.gitpath.as_path()
    }

    /// Returns a [Path] to the worktree directory of this repository.
    pub fn worktree(&self) -> &WorktreePath {
        self.worktree.as_path()
    }
}

/// Used to serialize and deserialize different types of git objects.
pub trait GitObject {
    /// Deserializes the data from an object file.
    ///
    /// The file should be stripped of its header before being deserialized.
    fn deserialize(data: &str) -> Self;

    /// Returns the type of object.
    fn fmt(&self) -> &'static str;

    /// Returns the serialized data of this object.
    ///
    /// A header needs to be added to this data before it's writtent to a file.
    fn serialize(&self) -> &str;

    /// Returns the size of this object.
    fn size(&self) -> usize;
}

/// A git blob object.
pub struct GitBlob {
    data: String,
    size: usize,
}

impl GitObject for GitBlob {
    fn deserialize(data: &str) -> Self {
        Self {
            data: data.to_string(),
            size: data.len(),
        }
    }

    fn fmt(&self) -> &'static str {
        "blob"
    }

    fn serialize(&self) -> &str {
        self.data.as_str()
    }

    fn size(&self) -> usize {
        self.size
    }
}
