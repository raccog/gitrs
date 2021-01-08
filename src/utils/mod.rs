mod fileio;
mod paths;
#[cfg(test)]
mod tests;

pub use fileio::{create_dir_all_if_new, create_dir_if_new, read_file, write_if_new};
