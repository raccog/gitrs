use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use super::*;

#[test]
fn test_create_dir_if_new() -> Result<(), Box<dyn Error>> {
    // Setup for first test
    let path = PathBuf::from("/tmp/create_dir_if_new");
    assert!(!path.exists());

    // Should create new directory
    create_dir_if_new(&path)?;
    // Should not do anything and return ok
    create_dir_if_new(&path)?;

    assert!(path.exists());
    fs::remove_dir(path)?;

    // Setup for next test
    let non_existing = PathBuf::from("/tmp/create_dir_if_new");
    let path = non_existing.join("failed_dir");
    assert!(!non_existing.exists());

    // Should return GitError when attempting to create
    // directories recursively
    create_dir_if_new(&path).expect_err(
        "utils::create_dir_if_new should not be able to create 
        directories recursively.",
    );
    assert!(!non_existing.exists());

    // Should return GitError when permission is denied
    create_dir_if_new("/usr/create_dir_if_new").expect_err(
        "utils::create_dir_if_new should return a GitError when 
        permission is denied.",
    );

    // Create blank file for next test
    let file_path = PathBuf::from("/tmp/create_dir_if_new");
    OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&file_path)?;
    let path = file_path.join("failed_dir");

    // Should return GitError when a section of the path is
    // actually a file
    create_dir_if_new(&path).expect_err(
        "utils::create_dir_if_new should return a GitError when a 
        section of the path is actually a file.",
    );
    assert!(!path.exists());

    // Cleanup
    fs::remove_file(&file_path)?;

    Ok(())
}

#[test]
fn test_create_dir_all_if_new() -> Result<(), Box<dyn Error>> {
    // Setup for first test
    let non_existing = PathBuf::from("/tmp/non_existing");
    let path = non_existing.join("testdir");
    assert!(!non_existing.exists());

    // Should create new directories recursively
    create_dir_all_if_new(&path)?;
    // Should not do anything and return ok
    create_dir_all_if_new(&path)?;
    assert!(path.exists());

    // Cleanup
    fs::remove_dir_all(non_existing)?;

    Ok(())
}

#[test]
fn test_read_file() -> Result<(), Box<dyn Error>> {
    // Setup for first test
    let path = PathBuf::from("/tmp/read_file");
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&path)?;
    file.write(b"test")?;

    // Should read 'test' from the new file
    let buf = read_file(&path)?;
    assert_eq!(buf, "test");

    // Cleanup
    fs::remove_file(&path)?;

    Ok(())
}
