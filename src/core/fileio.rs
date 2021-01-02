use std::fs::{self, OpenOptions};
use std::io::{self, ErrorKind, Read, Write};
use std::path::Path;

use crate::core::{self, GitResult};

/// Shorthand for creating a new directory.
///
/// Returns [Ok] if the directory already exists.
///
/// # Errors
///
/// This function returns some errors from [ErrorKind] wrapped in a [GitError::IOError].
///
/// * NotFound: One of the directory components of the directory path does not exist.
/// * PermissionDenied: The user lacks permission to get the specified access rights for the
/// directory.
/// * Other: One of the directory components of the specified directory path was not, in fact, a directory.
#[inline]
pub fn create_dir_if_new<P: AsRef<Path>>(path: P) -> GitResult<()> {
    core::to_git_result(consume_already_exists(fs::create_dir(path)))
}

/// Shorthand for creating a new directory and recursively creating it's parents if they don't
/// exist.
///
/// Converts [io::Error] into [GitError].
///
/// Returns [Ok] if the directory already exists.
///
/// # Errors
///
/// This function returns some errors from [ErrorKind] wrapped in a [GitError::IOError].
///
/// * NotFound: One of the directory components of the directory path does not exist.
/// * PermissionDenied: The user lacks permission to get the specified access rights for the
/// directory.
/// * Other: One of the directory components of the specified directory path was not, in fact, a directory.
#[inline]
pub fn create_dir_all_if_new<P: AsRef<Path>>(path: P) -> GitResult<()> {
    core::to_git_result(consume_already_exists(fs::create_dir_all(path)))
}

pub fn read_file<P: AsRef<Path>>(path: P) -> GitResult<String> {
    let mut file = core::to_git_result(OpenOptions::new().read(true).open(path))?;
    let mut data = String::new();
    core::to_git_result(file.read_to_string(&mut data))?;
    Ok(data)
}

/// Shorthand for creating a new file and writing a buffer into it.
///
/// Returns [Ok] if the file already exists.
///
/// # Errors
///
/// This function returns some errors from [ErrorKind] wrapped in a [GitError::IOError].
///
/// * NotFound: One of the directory components of the file path does not exist.
/// * PermissionDenied: The user lacks permission to get the specified access rights for the file.
/// * PermissionDenied: The user lacks permission to open one of the directory components of the
/// specified path.
/// * Other: One of the directory components of the specified file path was not, in fact, a directory.
/// * Other: Filesystem-level errors: full disk, write permission requested on a read-only file
/// system, exceeded disk quota, too many open files, too long filename, too many symbolic links
/// in the specified path (Unix-like systems only), etc.
#[inline]
pub fn write_if_new<P: AsRef<Path>>(path: P, buf: &[u8]) -> GitResult<()> {
    core::to_git_result(consume_already_exists(write_new(&path, buf)))
}

fn consume_already_exists(result: io::Result<()>) -> io::Result<()> {
    match result {
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => Ok(()),
            _ => Err(error),
        },
        Ok(()) => Ok(()),
    }
}

fn write_new<P: AsRef<Path>>(path: P, buf: &[u8]) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.write_all(buf)
}
