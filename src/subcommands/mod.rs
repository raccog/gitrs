pub use hash_object_utils::run as hash_object;

mod init_helper;
mod plumbing;

use clap::ArgMatches;

use self::init_helper::init as init_helper;
use crate::{GitRepo, GitResult};

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
pub fn init(matches: &ArgMatches) -> GitResult<()> {
    assert_eq!(matches.subcommand_name(), Some("init"));

    let quiet = matches
        .subcommand_matches("init")
        .unwrap()
        .is_present("quiet");
    init_helper(&GitRepo::from_args(&matches)?, quiet)
}

pub mod hash_object_utils {
    use std::path::Path;

    use clap::ArgMatches;

    use super::plumbing::hash_object as hash_object_helper;
    use crate::{utils, GitResult};

    pub fn run(matches: &ArgMatches) -> GitResult<()> {
        let hash = from_args(matches)?;
        println!("{}", hash);
        Ok(())
    }

    fn from_args(matches: &ArgMatches) -> GitResult<String> {
        from_file(
            matches
                .subcommand_matches("hash-object")
                .unwrap()
                .value_of("file")
                .unwrap(),
        )
    }

    #[inline]
    fn from_data(data: &str) -> String {
        hash_object_helper(data)
    }

    fn from_file<P: AsRef<Path>>(path: P) -> GitResult<String> {
        let data = utils::read_file(path)?;
        Ok(from_data(&data))
    }
}
