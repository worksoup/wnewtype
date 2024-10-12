#![warn(missing_docs)]
#![no_std]

//! 快捷实现 newtype 模式。
//!

/// 为结构体实现 `newtype` 模式。
///
/// 这将为内含值实现 `From`, `Into`, `Deref` 和 `DerefMut` 特型。
pub use derive_wnewtype::NewType;