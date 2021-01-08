use std::env::{self, VarError};
use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

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
    fn from_data(data: &str) -> Self
    where
        Self: Sized;

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

#[derive(Debug)]
pub struct GitFileMode {
    pub file_type: u8,
    pub owner_mode: u8,
    pub group_mode: u8,
    pub other_mode: u8,
}

impl From<u32> for GitFileMode {
    fn from(filemode: u32) -> Self {
        let file_type = (filemode >> 14) as u8;
        let owner_mode = ((filemode & 0x0F00) >> 5) as u8;
        let group_mode = ((filemode & 0x00F0) >> 2) as u8;
        let other_mode = (filemode & 0x000F) as u8;

        Self {
            file_type,
            owner_mode,
            group_mode,
            other_mode,
        }
    }
}

impl Into<u32> for GitFileMode {
    fn into(self) -> u32 {
        let (t, own, grp, oth) = (
            self.file_type as u32,
            self.owner_mode as u32,
            self.group_mode as u32,
            self.other_mode as u32,
        );
        (t << 14) | (own << 5) | (grp << 2) | oth
    }
}

impl From<&str> for GitFileMode {
    fn from(filemode: &str) -> Self {
        assert_eq!(filemode.len(), 6);
        let file_type = filemode[..2].parse::<u8>().unwrap();
        let owner_mode = filemode[2..4].parse::<u8>().unwrap();
        let group_mode = filemode
            .chars()
            .nth(4)
            .unwrap()
            .to_string()
            .parse::<u8>()
            .unwrap();
        let other_mode = filemode
            .chars()
            .nth(5)
            .unwrap()
            .to_string()
            .parse::<u8>()
            .unwrap();
        Self {
            file_type,
            owner_mode,
            group_mode,
            other_mode,
        }
    }
}

/// A git blob object.
#[derive(Debug)]
pub struct GitBlob {
    data: String,
    size: usize,
    filename: Option<OsString>,
    filemode: Option<GitFileMode>,
}

impl GitObject for GitBlob {
    fn data(&self) -> &str {
        self.data.as_str()
    }

    fn fmt(&self) -> &'static str {
        "blob"
    }

    fn from_data(data: &str) -> Self
    where
        Self: Sized,
    {
        Self {
            data: data.to_string(),
            size: data.len(),
            filename: None,
            filemode: None,
        }
    }

    fn from_object_file<P: AsRef<Path>>(path: P) -> GitResult<Self> {
        let file = core::to_git_result(File::open(&path), &path)?;
        let mut data = String::new();
        ZlibDecoder::new(&file).read_to_string(&mut data).unwrap();

        // Find delimiters
        let d1 = data.find(' ').unwrap();
        let d2 = data[d1 + 1..].find("\x00").unwrap();

        // Remove header from data
        let data = data[d2 + 1..].to_string();

        // Read file mode
        let mode = core::to_git_result(file.metadata(), &path)?
            .permissions()
            .mode();

        let size = data.len();
        Ok(Self {
            data,
            size,
            filename: Some(OsString::from_str(path.as_ref().to_str().unwrap()).unwrap()),
            filemode: Some(GitFileMode::from(mode)),
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_file_mode() -> Result<(), Box<dyn Error>> {
        use std::fs::{self, OpenOptions, Permissions};
        use std::os::unix::fs::PermissionsExt;

        let path = "/tmp/test.txt";
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&path)?;
        file.set_permissions(Permissions::from_mode(0o644))?;

        let real_mode = file.metadata()?.permissions().mode();
        let mode = GitFileMode::from(real_mode);
        fs::remove_file(path)?;

        assert_eq!(real_mode, mode.into());

        Ok(())
    }
}
