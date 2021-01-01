use crate::config;
use crate::core::{self, GitRepo, GitResult};

/// Initializes a git repository.
///
/// # Errors
///
/// This function can return all errors that come from [fs::create_dir], [fs::create_dir_all], and
/// [fileio::write_if_new].
pub fn init(repo: &GitRepo) -> GitResult<()> {
    let gitpath = repo.gitpath();
    // create directories
    core::create_dir_if_new(&gitpath)?;
    core::create_dir_if_new(&gitpath.join("hooks"))?;
    core::create_dir_if_new(&gitpath.join("info"))?;
    core::create_dir_all_if_new(&gitpath.join("objects").join("pack"))?;
    let refs_dir = gitpath.join("refs");
    core::create_dir_all_if_new(&refs_dir.join("heads"))?;
    core::create_dir_if_new(&refs_dir.join("tags"))?;

    // write to files
    core::write_if_new(
        &gitpath.join("description"),
        b"Unnamed repository; edit this file 'description' to name the repository.\n",
    )?;
    let head_path = gitpath.join("HEAD");
    if head_path.is_file() {
        println!(
            "Reinitialized existing Git repository in {}",
            repo.worktree().to_str().unwrap()
        );
    } else {
        println!(
            "Initialized empty Git repository in {}",
            repo.worktree().to_str().unwrap()
        );
    }
    core::write_if_new(&head_path, b"ref: refs/heads/master\n")?;
    core::write_if_new(&gitpath.join("config"), config::initial_config().as_bytes())?;

    Ok(())
}
