use crate::core::{GitBlob, GitObject};

pub fn hash_object(data: &str) -> String {
    let blob = GitBlob::from_data(data);
    blob.to_sha1()
}
