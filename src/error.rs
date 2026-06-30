//! Error types for sortcrab operations.
//!
//! All fallible operations in sortcrab return [`SortcrabError`], which
//! provides typed variants for I/O, configuration, classification, and
//! other domain-specific failures.

use std::path::PathBuf;
use thiserror::Error;

/// Unified error type for sortcrab operations.
///
/// Each variant maps to a distinct failure domain:
///
/// | Variant | When it occurs |
/// |---------|----------------|
/// | [`Io`](SortcrabError::Io) | Filesystem read/write failures |
/// | [`TomlParse`](SortcrabError::TomlParse) | Invalid configuration TOML |
/// | [`Config`](SortcrabError::Config) | Missing or misconfigured settings |
/// | [`InvalidPath`](SortcrabError::InvalidPath) | Source path is not a directory |
/// | [`Semester`](SortcrabError::Semester) | Date/time computation failure |
/// | [`UnknownExtension`](SortcrabError::UnknownExtension) | File extension not in rules |
/// | [`Skipped`](SortcrabError::Skipped) | File intentionally not moved |
/// | [`Other`](SortcrabError::Other) | Catch-all for miscellaneous errors |
///
/// # Example
///
/// ```rust
/// use sortcrab::error::SortcrabError;
///
/// let err = SortcrabError::Config("missing field 'root'".into());
/// assert_eq!(err.to_string(), "Config error: missing field 'root'");
/// ```
#[derive(Error, Debug)]
pub enum SortcrabError {
    /// Wraps a standard [`std::io::Error`].
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Wraps a [`toml::de::Error`] from parsing configuration files.
    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),

    /// A configuration-related error with a descriptive message.
    #[error("Config error: {0}")]
    Config(String),

    /// The supplied path is not valid for the requested operation.
    #[error("Invalid path: {0}")]
    InvalidPath(PathBuf),

    /// An error computing the academic semester from a timestamp.
    #[error("Semester error: {0}")]
    Semester(String),

    /// The file's extension is not mapped in the rules table.
    #[error("Unknown extension: {0}")]
    UnknownExtension(String),

    /// The file was deliberately skipped (dotfile, symlink, already organised).
    #[error("Skipped: {0}")]
    Skipped(String),

    /// Catch-all variant for miscellaneous error messages.
    #[error("{0}")]
    Other(String),
}

impl From<&str> for SortcrabError {
    fn from(s: &str) -> Self {
        SortcrabError::Other(s.to_string())
    }
}

impl From<String> for SortcrabError {
    fn from(s: String) -> Self {
        SortcrabError::Other(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_error_display_io() {
        let err = SortcrabError::Io(io::Error::new(io::ErrorKind::NotFound, "file not found"));
        assert!(err.to_string().contains("file not found"));
        assert!(err.to_string().contains("I/O error"));
    }

    #[test]
    fn test_error_display_config() {
        let err = SortcrabError::Config("missing field 'root'".to_string());
        assert_eq!(err.to_string(), "Config error: missing field 'root'");
    }

    #[test]
    fn test_error_display_invalid_path() {
        let err = SortcrabError::InvalidPath(PathBuf::from("/nonexistent"));
        assert_eq!(err.to_string(), "Invalid path: /nonexistent");
    }

    #[test]
    fn test_error_display_semester() {
        let err = SortcrabError::Semester("invalid format".to_string());
        assert_eq!(err.to_string(), "Semester error: invalid format");
    }

    #[test]
    fn test_error_from_str() {
        let err = SortcrabError::from("test error");
        assert_eq!(err.to_string(), "test error");
    }

    #[test]
    fn test_error_from_string() {
        let err: SortcrabError = "owned error".to_string().into();
        assert_eq!(err.to_string(), "owned error");
    }

    #[test]
    fn test_error_from_io() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "permission denied");
        let err: SortcrabError = io_err.into();
        assert!(err.to_string().contains("permission denied"));
    }
}
