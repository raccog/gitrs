use std::env::{self, VarError};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use clap::ArgMatches;
use hex;
use sha1::{Digest, Sha1};

use crate::core::{GitError, GitResult};

/// A container for all information about a git repository.
pub struct GitRepo {
    worktree: PathBuf,
    gitpath: PathBuf,
}

impl GitRepo {
    /// Creates a container from existing info about the repository.
    pub fn new(worktree: PathBuf, gitpath: PathBuf) -> Self {
        Self { worktree, gitpath }
    }
    /// Returns the current git repository from command line arguments.
    ///
    /// # Errors
    ///
    /// * GIT_DIR environment variable is invalid unicode
    /// * Arguments contain a path to an invalid directory
    pub fn from_args(matches: &ArgMatches) -> GitResult<GitRepo> {
        // Get GIT_DIR environment variable
        let gitpath = match env::var("GIT_DIR") {
            Ok(dir) => dir,
            Err(VarError::NotPresent) => ".git".to_string(),
            Err(VarError::NotUnicode(dir)) => {
                return Err(GitError::VarInvalidUnicode(OsString::from("GIT_DIR"), dir));
            }
        };

        // TODO: Add -git-dir to command line args and parse directly here

        // Get worktree from 'git init' args.
        let worktree = ".";
        let worktree = if let Some(sub_m) = matches.subcommand_matches("init") {
            if let Some(dir) = sub_m.value_of("directory") {
                dir
            } else {
                worktree
            }
        } else {
            worktree
        };
        let worktree = PathBuf::from(worktree);

        let gitpath = PathBuf::from(gitpath);
        let gitpath = if !gitpath.is_absolute() {
            worktree.join(gitpath)
        } else {
            gitpath
        };

        Ok(GitRepo::new(worktree, gitpath))
    }

    /// Returns a [Path] to the git directory of this repository.
    pub fn gitpath(&self) -> &Path {
        self.gitpath.as_path()
    }

    /// Returns a [Path] to the worktree directory of this repository.
    pub fn worktree(&self) -> &Path {
        self.worktree.as_path()
    }
}

/// A data interface used to serialize and deserialize different types of git objects.
pub trait GitObject {
    /// Returns the data contained in this object without the header.
    fn data(&self) -> &str;

    /// Returns the type of object.
    fn fmt(&self) -> &'static str;

    /// Returns an interface to the object created from data (without the header).
    fn from_data(data: &str) -> Self;

    /// Returns the data contained in this object including the header.
    fn serialize(&self) -> String;

    /// Returns the size of this object.
    fn size(&self) -> usize;

    /// Returns the Sha1 hash for this object.
    fn to_sha1(&self) -> String;
}

/// A git blob object.
pub struct GitBlob {
    data: String,
    size: usize,
}

impl GitObject for GitBlob {
    fn data(&self) -> &str {
        self.data.as_str()
    }

    fn fmt(&self) -> &'static str {
        "blob"
    }

    fn from_data(data: &str) -> Self {
        Self {
            data: data.to_string(),
            size: data.len(),
        }
    }

    fn serialize(&self) -> String {
        format!("{} {}\x00{}", self.fmt(), self.size, self.data)
    }

    fn size(&self) -> usize {
        self.size
    }

    fn to_sha1(&self) -> String {
        hex::encode(Sha1::digest(self.serialize().as_bytes()))
    }
}
