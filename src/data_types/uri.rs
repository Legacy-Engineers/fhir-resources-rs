use std::fmt;
use std::str::FromStr;
use serde::{Deserialize, Serialize};

/// A Uniform Resource Identifier Reference (RFC 3986).
/// 
/// URIs are case sensitive. For UUID (urn:uuid:53fefa32-fcbb-4ff8-8a92-55ee120877b7) 
/// use all lowercase. URIs can be absolute or relative, and may have an optional 
/// fragment identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Uri {
    value: String,
}

#[derive(Debug, thiserror::Error)]
pub enum UriError {
    #[error("URI cannot be empty")]
    Empty,
    #[error("URI contains invalid characters: {0}")]
    InvalidCharacters(String),
    #[error("UUID URI must be lowercase: {0}")]
    UuidNotLowercase(String),
    #[error("Invalid URI format: {0}")]
    InvalidFormat(String),
}

impl Uri {
    /// Creates a new URI with validation.
    pub fn new(value: String) -> Result<Self, UriError> {
        let uri = Uri { value: value.clone() };
        uri.validate()?;
        Ok(uri)
    }

    /// Creates a new URI without validation (use with caution).
    pub fn new_unchecked(value: String) -> Self {
        Uri { value }
    }

    /// Returns the URI value as a string slice.
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Returns the URI value as a string.
    pub fn to_string(&self) -> String {
        self.value.clone()
    }

    /// Validates the URI according to FHIR specifications.
    pub fn validate(&self) -> Result<(), UriError> {
        if self.value.is_empty() {
            return Err(UriError::Empty);
        }

        // Check for invalid characters (basic check)
        if self.value.chars().any(|c| c.is_control()) {
            return Err(UriError::InvalidCharacters(self.value.clone()));
        }

        // Check for UUID URIs - they must be lowercase
        if self.value.starts_with("urn:uuid:") {
            if self.value != self.value.to_lowercase() {
                return Err(UriError::UuidNotLowercase(self.value.clone()));
            }
        }

        // Basic URI format validation
        if !self.is_valid_uri_format() {
            return Err(UriError::InvalidFormat(self.value.clone()));
        }

        Ok(())
    }

    /// Checks if the URI is a UUID URI.
    pub fn is_uuid(&self) -> bool {
        self.value.starts_with("urn:uuid:")
    }

    /// Checks if the URI is absolute.
    pub fn is_absolute(&self) -> bool {
        self.value.contains("://")
    }

    /// Checks if the URI is relative.
    pub fn is_relative(&self) -> bool {
        !self.is_absolute()
    }

    /// Gets the fragment identifier if present.
    pub fn fragment(&self) -> Option<&str> {
        self.value.split('#').nth(1)
    }

    /// Basic URI format validation.
    fn is_valid_uri_format(&self) -> bool {
        // This is a simplified validation - in practice, you might want to use
        // a more comprehensive URI parsing library
        let uri = self.value.trim();
        
        // Check for basic URI components
        if uri.is_empty() {
            return false;
        }

        // Allow relative URIs
        if !uri.contains("://") {
            return true;
        }

        // For absolute URIs, check for scheme
        if let Some(scheme_part) = uri.split("://").next() {
            if scheme_part.is_empty() || !scheme_part.chars().all(|c| c.is_alphanumeric() || c == '+' || c == '-' || c == '.') {
                return false;
            }
        }

        true
    }
}

impl FromStr for Uri {
    type Err = UriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uri::new(s.to_string())
    }
}

impl fmt::Display for Uri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl AsRef<str> for Uri {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl From<String> for Uri {
    fn from(value: String) -> Self {
        Uri::new_unchecked(value)
    }
}

impl From<&str> for Uri {
    fn from(value: &str) -> Self {
        Uri::new_unchecked(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_uri() {
        let uri = Uri::new("https://example.com".to_string()).unwrap();
        assert_eq!(uri.as_str(), "https://example.com");
    }

    #[test]
    fn test_empty_uri() {
        let result = Uri::new("".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_uuid_uri_lowercase() {
        let uri = Uri::new("urn:uuid:53fefa32-fcbb-4ff8-8a92-55ee120877b7".to_string()).unwrap();
        assert!(uri.is_uuid());
    }

    #[test]
    fn test_uuid_uri_uppercase_error() {
        let result = Uri::new("urn:uuid:53FEFA32-FCBB-4FF8-8A92-55EE120877B7".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_relative_uri() {
        let uri = Uri::new("relative/path".to_string()).unwrap();
        assert!(uri.is_relative());
        assert!(!uri.is_absolute());
    }

    #[test]
    fn test_absolute_uri() {
        let uri = Uri::new("https://example.com/path".to_string()).unwrap();
        assert!(uri.is_absolute());
        assert!(!uri.is_relative());
    }

    #[test]
    fn test_uri_with_fragment() {
        let uri = Uri::new("https://example.com#fragment".to_string()).unwrap();
        assert_eq!(uri.fragment(), Some("fragment"));
    }

    #[test]
    fn test_uri_without_fragment() {
        let uri = Uri::new("https://example.com".to_string()).unwrap();
        assert_eq!(uri.fragment(), None);
    }

    #[test]
    fn test_from_str() {
        let uri = Uri::from_str("https://example.com").unwrap();
        assert_eq!(uri.as_str(), "https://example.com");
    }

    #[test]
    fn test_display() {
        let uri = Uri::new("https://example.com".to_string()).unwrap();
        assert_eq!(uri.to_string(), "https://example.com");
    }

    #[test]
    fn test_serialization() {
        let uri = Uri::new("https://example.com".to_string()).unwrap();
        let serialized = serde_json::to_string(&uri).unwrap();
        let deserialized: Uri = serde_json::from_str(&serialized).unwrap();
        assert_eq!(uri, deserialized);
    }
}