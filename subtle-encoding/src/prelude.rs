//! Use `std` or `alloc` prelude depending on selected cargo features

#[cfg(all(feature = "alloc", not(feature = "std")))]
pub use alloc::{string::String, vec::Vec};

#[cfg(feature = "std")]
pub use std::prelude::v1::*;
