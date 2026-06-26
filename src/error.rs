// sortcrab — error types

use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SortcrabError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Invalid path: {0}")]
    InvalidPath(PathBuf),

    #[error("Semester error: {0}")]
    Semester(String),

    #[error("Unknown extension: {0}")]
    UnknownExtension(String),

    #[error("Skipped: {0}")]
    Skipped(String),

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
