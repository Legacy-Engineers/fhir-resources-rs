use std::fmt;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use regex::Regex;

/// A FHIR Code represents a value from a set of controlled strings defined elsewhere.
/// 
/// A code is restricted to a string which has at least one character and no leading 
/// or trailing whitespace, and where there is no whitespace other than single spaces 
/// in the contents. This datatype can be bound to a ValueSet.
/// 
/// Regex pattern: [^\s]+( [^\s]+)*
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Code {
    value: String,
}

#[derive(Debug, thiserror::Error)]
pub enum CodeError {
    #[error("Code cannot be empty")]
    Empty,
    #[error("Code has leading or trailing whitespace")]
    LeadingTrailingWhitespace,
    #[error("Code contains multiple consecutive spaces")]
    MultipleSpaces,
    #[error("Code contains tabs or newlines")]
    InvalidWhitespace,
    #[error("Code does not match required pattern")]
    InvalidPattern,
}

impl Code {
    /// Creates a new Code with validation.
    pub fn new(value: String) -> Result<Self, CodeError> {
        let code = Code { value: value.clone() };
        code.validate()?;
        Ok(code)
    }

    /// Creates a new Code without validation (use with caution).
    pub fn new_unchecked(value: String) -> Self {
        Code { value }
    }

    /// Returns the code value as a string slice.
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Returns the code value as a string.
    pub fn to_string(&self) -> String {
        self.value.clone()
    }

    /// Validates the code according to FHIR specifications.
    pub fn validate(&self) -> Result<(), CodeError> {
        // Check if empty
        if self.value.is_empty() {
            return Err(CodeError::Empty);
        }

        // Check for leading or trailing whitespace
        if self.value != self.value.trim() {
            return Err(CodeError::LeadingTrailingWhitespace);
        }

        // Check for tabs or newlines
        if self.value.chars().any(|c| c == '\t' || c == '\n' || c == '\r') {
            return Err(CodeError::InvalidWhitespace);
        }

        // Check for multiple consecutive spaces
        if self.value.contains("  ") {
            return Err(CodeError::MultipleSpaces);
        }

        // Validate against regex pattern: [^\s]+( [^\s]+)*
        let pattern = Regex::new(r"^[^\s]+( [^\s]+)*$").unwrap();
        if !pattern.is_match(&self.value) {
            return Err(CodeError::InvalidPattern);
        }

        Ok(())
    }

    /// Returns the individual tokens in the code, split by spaces.
    pub fn tokens(&self) -> Vec<&str> {
        self.value.split(' ').collect()
    }

    /// Returns the number of tokens in the code.
    pub fn token_count(&self) -> usize {
        self.tokens().len()
    }

    /// Checks if the code contains only a single token (no spaces).
    pub fn is_single_token(&self) -> bool {
        !self.value.contains(' ')
    }

    /// Checks if the code contains multiple tokens.
    pub fn is_multi_token(&self) -> bool {
        self.value.contains(' ')
    }

    /// Converts the Code to a JSON string.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// Converts a JSON string to a Code.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

impl FromStr for Code {
    type Err = CodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Code::new(s.to_string())
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl AsRef<str> for Code {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl From<String> for Code {
    fn from(value: String) -> Self {
        Code::new_unchecked(value)
    }
}

impl From<&str> for Code {
    fn from(value: &str) -> Self {
        Code::new_unchecked(value.to_string())
    }
}