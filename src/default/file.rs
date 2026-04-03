//! File generator - generates random file extensions and mime types

use crate::base::sample;
use crate::locale::{fetch_locale_with_context, sample_with_resolve};

/// Generate a random file extension (e.g. "pdf")
pub fn extension() -> String {
    fetch_locale_with_context("file.extension", "en", Some("file"))
        .map(|v| sample_with_resolve(&v, Some("file")))
        .unwrap_or_else(|| sample(FALLBACK_EXTENSIONS).to_string())
}

/// Generate a random mime type (e.g. "application/pdf")
pub fn mime_type() -> String {
    fetch_locale_with_context("file.mime_type", "en", Some("file"))
        .map(|v| sample_with_resolve(&v, Some("file")))
        .unwrap_or_else(|| sample(FALLBACK_MIME_TYPES).to_string())
}

/// Generate a random file name
pub fn file_name(name: Option<&str>, extension: Option<&str>) -> String {
    let name = name
        .map(|s| s.to_string())
        .unwrap_or_else(crate::lorem::word);
    let extension = extension
        .map(|s| s.to_string())
        .unwrap_or_else(self::extension);
    format!("{}.{}", name, extension)
}

// Fallback data
const FALLBACK_EXTENSIONS: &[&str] = &["jpg", "png", "pdf", "docx", "zip", "mp3"];
const FALLBACK_MIME_TYPES: &[&str] = &[
    "image/jpeg",
    "image/png",
    "application/pdf",
    "application/zip",
    "audio/mpeg",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension() {
        assert!(!extension().is_empty());
    }

    #[test]
    fn test_file_name() {
        let name = file_name(None, None);
        assert!(name.contains('.'));
    }
}
