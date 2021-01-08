use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::str::FromStr;

use flate2::read::ZlibDecoder;
use sha1::{Digest, Sha1};

use crate::{self as gitrs, GitFileMode, GitObject, GitResult};

/// A git blob object.
#[derive(Debug)]
pub struct GitBlob {
    data: String,
    size: usize,
    filename: Option<OsString>,
    filemode: Option<GitFileMode>,
}

impl GitObject for GitBlob {
    fn data(&self) -> &str {
        self.data.as_str()
    }

    fn fmt(&self) -> &'static str {
        "blob"
    }

    fn from_data(data: &str) -> Self
    where
        Self: Sized,
    {
        Self {
            data: data.to_string(),
            size: data.len(),
            filename: None,
            filemode: None,
        }
    }

    fn from_object_file<P: AsRef<Path>>(path: P) -> GitResult<Self> {
        let file = gitrs::to_git_result(File::open(&path), &path)?;
        let mut data = String::new();
        ZlibDecoder::new(&file).read_to_string(&mut data).unwrap();

        // Find delimiters
        let d1 = data.find(' ').unwrap();
        let d2 = data[d1 + 1..].find('\x00').unwrap();

        // Remove header from data
        let data = data[d2 + 1..].to_string();

        // Read file mode
        let mode = gitrs::to_git_result(file.metadata(), &path)?
            .permissions()
            .mode();

        let size = data.len();
        Ok(Self {
            data,
            size,
            filename: Some(OsString::from_str(path.as_ref().to_str().unwrap()).unwrap()),
            filemode: Some(GitFileMode::from(mode)),
        })
    }

    fn serialize(&self) -> String {
        format!("{} {}\x00{}", self.fmt(), self.size, self.data)
    }

    fn size(&self) -> usize {
        self.size
    }

    fn to_sha1(&self) -> String {
        hex::encode(Sha1::digest(self.serialize().as_bytes()))
    }
}
