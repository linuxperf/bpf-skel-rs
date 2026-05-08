//! Rust-facing libbpf skeleton modules generated from third-party BPF programs.
//!
//! The skeleton modules are pre-generated under `src/skel` and include the BPF
//! object bytes inline, so downstream crates can use them without running this
//! repository's BPF generation tooling.

#![deny(unsafe_op_in_unsafe_fn)]

pub mod skel;

pub use skel::*;
