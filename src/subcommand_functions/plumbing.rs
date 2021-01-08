use crate::{GitBlob, GitObject /*, GitRepo*/};

pub fn hash_object(data: &str) -> String {
    let blob = GitBlob::from_data(data);
    blob.to_sha1()
}

//pub fn cat_file(repo: &GitRepo, object: &str) -> String {}
