//! Board configuration library.
//!
//! Exposes and routes specific board module support implementations.

#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![deny(missing_docs)]

pub mod cat_detector;
pub use cat_detector::*;
