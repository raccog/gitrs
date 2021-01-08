use std::env::{self, VarError};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use clap::ArgMatches;

use crate::{GitError, GitResult};

/// A container for all information about a git repository.
#[derive(Debug)]
pub struct GitRepo {
    worktree: Option<PathBuf>,
    gitpath: PathBuf,
}

impl GitRepo {
    /// Creates a container from existing info about the repository.
    pub fn new(worktree: PathBuf, gitpath: PathBuf) -> Self {
        let worktree = Some(worktree);
        Self { worktree, gitpath }
    }
    /// Returns the current git repository from command line arguments.
    ///
    /// # Errors
    ///
    /// * [GitError::VarInvalidUnicode]: GIT_DIR environment variable is invalid unicode
    /// * [GitError::IOError]: Arguments contain a path to an invalid directory
    pub fn from_args(matches: &ArgMatches) -> GitResult<GitRepo> {
        // Get GIT_DIR environment variable
        let gitpath = match env::var("GIT_DIR") {
            Ok(dir) => dir,
            Err(VarError::NotPresent) => ".git".to_string(),
            Err(VarError::NotUnicode(dir)) => {
                return Err(GitError::VarInvalidUnicode {
                    var: OsString::from("GIT_DIR"),
                    data: dir,
                });
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
    pub fn worktree(&self) -> Option<&Path> {
        self.worktree.as_ref().map(|p| p.as_path())
    }
}
