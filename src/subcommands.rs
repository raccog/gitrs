use crate::core::subcommand_functions;
use crate::core::{GitRepo, GitResult};

/// Initializes a git repository.
///
/// # Errors
///
/// This function can return all errors that come from [std::fs::create_dir] and [std::fs::create_dir_all]
/// along with these errors:
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
pub fn init(repo: &GitRepo) -> GitResult<()> {
    subcommand_functions::init(repo)
}

pub mod hash_object {
    use std::path::Path;

    use clap::ArgMatches;

    use crate::core::subcommand_functions;
    use crate::core::{self, GitResult};

    pub fn run(args: &ArgMatches) -> GitResult<()> {
        let hash = from_args(args)?;
        println!("{}", hash);
        Ok(())
    }

    fn from_args(args: &ArgMatches) -> GitResult<String> {
        from_file(
            args.subcommand_matches("hash-object")
                .unwrap()
                .value_of("file")
                .unwrap(),
        )
    }

    #[inline]
    fn from_data(data: &str) -> String {
        subcommand_functions::hash_object(data)
    }

    fn from_file<P: AsRef<Path>>(path: P) -> GitResult<String> {
        let data = core::read_file(path)?;
        Ok(from_data(&data))
    }
}
