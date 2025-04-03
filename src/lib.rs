//! # apple-utils
//!
//! Utilities for interacting with Apple platforms.
//!
//! This crate provides safe wrappers around the bindings provided by the [objc2](https://crates.io/crates/objc2) crates.
//!
//! Currently only iOS file picker functionality is implemented.
//!
//! ## Features
//!
//! ### iOS
//!
//! - **File Picker**: Access to the native iOS document picker via the [`FilePicker`](ios::file_picker::FilePicker) struct
//! - **File Type Handling**: Utilities for working with different file type specifications using the [`FileType`](file_type::FileType) enum
//!
//! ## Safety
//!
//! The underlying objc2 bindings are unsafe. While this crate aims to provide safe wrappers,
//! these wrappers have not been rigorously tested. Use with caution.

#[cfg(target_os = "ios")]
pub mod ios;

#[cfg(target_os = "ios")]
pub mod file_type;
