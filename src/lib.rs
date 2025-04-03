//! # apple-utils
//!
//! Provides safe wrappers around Apple platform features using [objc2](https://crates.io/crates/objc2) bindings.
//!
//! ## Features
//!
//! ### iOS
//!
//! - **File Picker**: Access to the native iOS document picker via the [`FilePicker`](ios::file_picker::FilePicker) struct
//! - **File Type Handling**: Work with different file type specifications using the [`FileType`](file_type::FileType) enum
//!
//! ## Safety
//!
//! The underlying objc2 bindings are unsafe. While this crate aims to provide safe wrappers,
//! these wrappers have not been rigorously tested.

#[cfg(target_os = "ios")]
pub mod ios;

#[cfg(target_os = "ios")]
pub mod file_type;
