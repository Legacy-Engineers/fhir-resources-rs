use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reference {
    #[serde(rename = "reference")]
    reference: Option<String>,
    #[serde(rename = "type")]
    reference_type: Option<String>,
    #[serde(rename = "identifier")]
    identifier: Option<String>,
    #[serde(rename = "display")]
    display: Option<String>,
}

impl Reference {
    /// Creates a new Reference.
    pub fn new() -> Self {
        Self {
            reference: None,
            reference_type: None,
            identifier: None,
            display: None,
        }
    }

    /// Creates a new Reference with a reference string.
    pub fn with_reference(reference: String) -> Self {
        Self {
            reference: Some(reference),
            reference_type: None,
            identifier: None,
            display: None,
        }
    }

    /// Creates a new Reference with reference and type.
    pub fn with_reference_and_type(reference: String, reference_type: String) -> Self {
        Self {
            reference: Some(reference),
            reference_type: Some(reference_type),
            identifier: None,
            display: None,
        }
    }

    /// Returns the reference.
    pub fn reference(&self) -> Option<&str> {
        self.reference.as_deref()
    }

    /// Returns the reference type.
    pub fn reference_type(&self) -> Option<&str> {
        self.reference_type.as_deref()
    }

    /// Returns the identifier.
    pub fn identifier(&self) -> Option<&str> {
        self.identifier.as_deref()
    }

    /// Returns the display.
    pub fn display(&self) -> Option<&str> {
        self.display.as_deref()
    }

    /// Sets the reference.
    pub fn set_reference(&mut self, reference: Option<String>) {
        self.reference = reference;
    }

    /// Sets the reference type.
    pub fn set_reference_type(&mut self, reference_type: Option<String>) {
        self.reference_type = reference_type;
    }

    /// Sets the identifier.
    pub fn set_identifier(&mut self, identifier: Option<String>) {
        self.identifier = identifier;
    }

    /// Sets the display.
    pub fn set_display(&mut self, display: Option<String>) {
        self.display = display;
    }
}

impl Default for Reference {
    fn default() -> Self {
        Self::new()
    }
} 