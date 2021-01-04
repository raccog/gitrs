use std::env::{self, VarError};
use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use clap::ArgMatches;
use flate2::read::ZlibDecoder;
use hex;
use sha1::{Digest, Sha1};

use crate::core::{self, GitError, GitResult};

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

/// A data interface used to serialize and deserialize different types of git objects.
pub trait GitObject {
    /// Returns the data contained in this object without the header.
    fn data(&self) -> &str;

    /// Returns the type of object.
    fn fmt(&self) -> &'static str;

    /// Returns an object created from data (without the header).
    fn from_data(data: &str) -> Self;

    /// Returns an object read from an object file.
    ///
    /// # Errors
    ///
    /// Can return errors obtained when reading a file.
    fn from_object_file<P: AsRef<Path>>(path: P) -> GitResult<Self>
    where
        Self: Sized;

    /// Returns the data contained in this object including the header.
    fn serialize(&self) -> String;

    /// Returns the size of this object.
    fn size(&self) -> usize;

    /// Returns the Sha1 hash for this object.
    fn to_sha1(&self) -> String;
}

/// A git blob object.
#[derive(Debug)]
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

    fn from_object_file<P: AsRef<Path>>(path: P) -> GitResult<Self> {
        let mut data = String::new();
        ZlibDecoder::new(core::to_git_result(File::open(&path), path)?)
            .read_to_string(&mut data)
            .unwrap();

        // Find delimiters
        let d1 = data.find(' ').unwrap();
        let d2 = data[d1 + 1..].find("\x00").unwrap();

        // Remove header from data
        let data = data[d2 + 1..].to_string();

        Ok(GitBlob::from_data(&data))
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
