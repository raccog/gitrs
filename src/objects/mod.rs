mod blob;
mod file_mode;
mod repo;
#[cfg(test)]
mod tests;

use std::path::Path;

pub use blob::GitBlob;
pub use file_mode::GitFileMode;
pub use repo::GitRepo;

use crate::GitResult;

/// A data interface used to serialize and deserialize different types of git objects.
pub trait GitObject {
    /// Returns the data contained in this object without the header.
    fn data(&self) -> &str;

    /// Returns the type of object.
    fn fmt(&self) -> &'static str;

    /// Returns an object created from data (without the header).
    fn from_data(data: &str) -> Self
    where
        Self: Sized;

    /// Returns an object read from an object file.
    ///
    /// # Errors
    ///
    /// Can return errors obtained when reading a file.
    fn from_object_file<P: AsRef<Path>>(path: P) -> GitResult<Self>
    where
        Self: Sized;

    /// Returns the data contained in this object including the header.
    fn serialize(&self) -> String;

    /// Returns the size of this object.
    fn size(&self) -> usize;

    /// Returns the Sha1 hash for this object.
    fn to_sha1(&self) -> String;
}
