use std::fs::File;
use std::io::{self, Read};

use flate2::read::ZlibDecoder;

use crate::core::{GitBlob, GitObject, ObjectPath};

/// Deserializes a git object file and strips the header.
pub fn decode_object(path: &ObjectPath) -> io::Result<impl GitObject> {
    // Read file and inflate
    let mut data = String::new();
    ZlibDecoder::new(File::open(path)?)
        .read_to_string(&mut data)
        .expect(&format!(
            "Could not read data from object {}. Invalid formatting.",
            path.to_str().unwrap()
        ));

    // Find delimiters
    let d1 = data.find(' ').unwrap();
    let d2 = data[d1 + 1..].find("\x00").unwrap();

    // Remove header from data
    let data = data[d2 + 1..].to_string();

    Ok(GitBlob::deserialize(&data))
}
