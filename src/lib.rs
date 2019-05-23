#![warn(missing_docs)]
#![no_std]

//! NewType semantics for single-field tuple structs.
//!

/// Treat a single-field tuple struct as a "newtype"
///
/// This will implement `From`, `Into`, `Deref`, and `DerefMut` for the inner
/// type.
pub use derive_newtype::NewType;