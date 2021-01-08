use crate::config;
use crate::{utils, GitRepo, GitResult};

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
        utils::create_dir_all_if_new(worktree)?;
    }
    utils::create_dir_if_new(&gitpath)?;
    utils::create_dir_if_new(&gitpath.join("hooks"))?;
    utils::create_dir_if_new(&gitpath.join("info"))?;
    utils::create_dir_all_if_new(&gitpath.join("objects").join("pack"))?;
    let refs_dir = gitpath.join("refs");
    utils::create_dir_all_if_new(&refs_dir.join("heads"))?;
    utils::create_dir_if_new(&refs_dir.join("tags"))?;

    // write to files
    utils::write_if_new(
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
    utils::write_if_new(&head_path, b"ref: refs/heads/master\n")?;
    utils::write_if_new(&gitpath.join("config"), config::initial_config().as_bytes())?;

    Ok(())
}
