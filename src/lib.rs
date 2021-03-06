//! # Gitrs
//!
//! `gitrs` is a re-implementation of some git features in native rust with no [libgit2](https://libgit2.org)  bindings.

mod config;
mod error;
mod objects;
mod subcommands;
mod utils;

pub use crate::error::{to_git_result, GitError, GitResult};
pub use crate::objects::{GitBlob, GitFileMode, GitObject, GitRepo};
pub use crate::subcommands::{hash_object, init};
