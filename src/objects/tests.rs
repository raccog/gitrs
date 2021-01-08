use super::*;
use std::error::Error;

#[test]
fn test_file_mode() -> Result<(), Box<dyn Error>> {
    use std::fs::{self, OpenOptions, Permissions};
    use std::os::unix::fs::PermissionsExt;

    let path = "/tmp/test.txt";
    let file = OpenOptions::new()
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
