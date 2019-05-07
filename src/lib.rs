#![warn(missing_docs)]

//! NewType semantics for single-field tuple structs.
//!

use std::ops::Deref;

/// Treat a single-field tuple struct as a "newtype"
///
/// This will implement `From`, `Into`, `Deref`, and `DerefMut` for the inner
/// type.
pub use derive_newtype::NewType;

/// Trait for unwrapping the newtype
///
/// Would love to have `DerefMove` so that `*` could do this, but it's [currently
/// in RFC](https://github.com/rust-lang/rfcs/issues/997).
pub trait IntoInner: Deref {
    /// Unwrap the newtype into the inner value
    fn into_inner(self) -> Self::Target;
}
