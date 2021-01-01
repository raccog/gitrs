use crate::core::subcommand_functions;
use crate::core::{GitRepo, GitResult};

/// Initializes a git repository.
///
/// # Errors
///
/// This function can return all errors that come from [fs::create_dir], [fs::create_dir_all], and
/// [fileio::write_if_new].
pub fn init(repo: &GitRepo) -> GitResult<()> {
    subcommand_functions::init(repo)
}

pub mod hash_object {
    use crate::core::ObjectPath;

    pub fn from_data(data: &str) {}

    pub fn from_file(path: &ObjectPath) {}
}
