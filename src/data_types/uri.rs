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

    /// Converts the URI to a JSON string.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// Converts a JSON string to a URI.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
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
