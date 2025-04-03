use objc2::rc::Retained;
use objc2_foundation::{NSString, ns_string};
use objc2_uniform_type_identifiers::UTType;

/// Represents different ways to specify a file type.
#[derive(Debug, Clone)]
pub enum FileType {
    /// Matches any file type.
    Any,
    /// A file type specified by its extension (e.g., "pdf", "txt").
    Extension(String),
    /// A file type specified by its MIME type (e.g., "application/pdf").
    MimeType(String),
    /// A file type specified by its Uniform Type Identifier (e.g., "public.image").
    UniformTypeIdentifier(String),
}

impl FileType {
    pub(crate) fn to_uttype(&self) -> Option<Retained<UTType>> {
        unsafe {
            match self {
                FileType::Any => UTType::typeWithIdentifier(ns_string!("public.item")),
                FileType::Extension(ext) => {
                    UTType::typeWithFilenameExtension(&NSString::from_str(ext))
                }
                FileType::MimeType(mime_type) => {
                    UTType::typeWithMIMEType(&NSString::from_str(mime_type))
                }
                FileType::UniformTypeIdentifier(uti) => {
                    UTType::typeWithIdentifier(&NSString::from_str(uti))
                }
            }
        }
    }
}
