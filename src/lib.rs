//! Mitochondria is the powerhouse of the `Cell`.
//!
//! This crate provides additional mutable containers for use cases not
//! covered by the triumvirate of `Cell`, `RefCell` and `UnsafeCell`.

#![deny(missing_docs)]
#![deny(unsafe_code)]

#[path="move.rs"]
mod move_;
mod once;

pub use move_::MoveCell;
pub use once::OnceCell;
