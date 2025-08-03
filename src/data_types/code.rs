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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_single_token() {
        let code = Code::new("active".to_string()).unwrap();
        assert_eq!(code.as_str(), "active");
        assert!(code.is_single_token());
        assert_eq!(code.token_count(), 1);
    }

    #[test]
    fn test_valid_multi_token() {
        let code = Code::new("active status".to_string()).unwrap();
        assert_eq!(code.as_str(), "active status");
        assert!(code.is_multi_token());
        assert_eq!(code.token_count(), 2);
        assert_eq!(code.tokens(), vec!["active", "status"]);
    }

    #[test]
    fn test_empty_code() {
        let result = Code::new("".to_string());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CodeError::Empty));
    }

    #[test]
    fn test_leading_whitespace() {
        let result = Code::new(" active".to_string());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CodeError::LeadingTrailingWhitespace));
    }

    #[test]
    fn test_trailing_whitespace() {
        let result = Code::new("active ".to_string());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CodeError::LeadingTrailingWhitespace));
    }

    #[test]
    fn test_multiple_spaces() {
        let result = Code::new("active  status".to_string());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CodeError::MultipleSpaces));
    }

    #[test]
    fn test_tab_character() {
        let result = Code::new("active\tstatus".to_string());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CodeError::InvalidWhitespace));
    }

    #[test]
    fn test_newline_character() {
        let result = Code::new("active\nstatus".to_string());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CodeError::InvalidWhitespace));
    }

    #[test]
    fn test_complex_valid_code() {
        let code = Code::new("active status with multiple tokens".to_string()).unwrap();
        assert_eq!(code.token_count(), 5);
        assert_eq!(code.tokens(), vec!["active", "status", "with", "multiple", "tokens"]);
    }

    #[test]
    fn test_from_str() {
        let code = Code::from_str("active").unwrap();
        assert_eq!(code.as_str(), "active");
    }

    #[test]
    fn test_display() {
        let code = Code::new("active status".to_string()).unwrap();
        assert_eq!(code.to_string(), "active status");
    }

    #[test]
    fn test_serialization() {
        let code = Code::new("active status".to_string()).unwrap();
        let serialized = serde_json::to_string(&code).unwrap();
        let deserialized: Code = serde_json::from_str(&serialized).unwrap();
        assert_eq!(code, deserialized);
    }

    #[test]
    fn test_regex_pattern_validation() {
        // Valid patterns
        assert!(Code::new("token".to_string()).is_ok());
        assert!(Code::new("token1 token2".to_string()).is_ok());
        assert!(Code::new("token1 token2 token3".to_string()).is_ok());

        // Invalid patterns
        assert!(Code::new(" token".to_string()).is_err()); // leading space
        assert!(Code::new("token ".to_string()).is_err()); // trailing space
        assert!(Code::new("token  token2".to_string()).is_err()); // multiple spaces
        assert!(Code::new("".to_string()).is_err()); // empty
    }
}
