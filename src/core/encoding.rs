use crate::core::GitObject;

/// Serializes a git object file and adds a header.
pub fn encode_object<O: GitObject>(object: &O) -> String {
    let data = object.serialize();
    let data = format!("{} {}\x00{}", object.fmt(), data.len(), data);

    data
}
