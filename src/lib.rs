//! This crate provides a vector supporting multiple non-overlapping writers
//! or multiple readers.
//!
//! Developed for radiant-rs and **currently not suitable for general purpose use**!

mod avec;

pub use avec::{AVec, AVecReadGuard, AVecMapGuard};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
