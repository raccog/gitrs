use crate::config;
use crate::core::{self, GitRepo, GitResult};

/// Initializes a git repository.
///
/// # Errors
///
/// This function can return all errors that come from [fs::create_dir], [fs::create_dir_all], and
/// [fileio::write_if_new].
pub fn init(repo: &GitRepo, quiet: bool) -> GitResult<()> {
    let gitpath = repo.gitpath();
    // create directories
    if let Some(worktree) = repo.worktree() {
        core::create_dir_all_if_new(worktree)?;
    }
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
    let gitpath_str = gitpath.canonicalize().unwrap();
    let gitpath_str = gitpath_str.to_str().unwrap();
    if !quiet {
        if head_path.is_file() {
            println!("Reinitialized existing Git repository in {}", gitpath_str);
        } else {
            println!("Initialized empty Git repository in {}", gitpath_str);
        }
    }
    core::write_if_new(&head_path, b"ref: refs/heads/master\n")?;
    core::write_if_new(&gitpath.join("config"), config::initial_config().as_bytes())?;

    Ok(())
}
