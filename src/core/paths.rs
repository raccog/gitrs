use std::path::PathBuf;

use crate::core::GitRepo;

/// Returns the path to an object from it's Sha1 hash.
#[inline]
pub fn get_object_path(repo: &GitRepo, sha: &str) -> PathBuf {
    repo.gitpath()
        .join("objects")
        .join(&sha[0..2])
        .join(&sha[2..])
}
