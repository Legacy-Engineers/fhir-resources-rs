use crate::data_types::codeable_concept::CodeableConcept;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatientCommunication {
    #[serde(rename = "language")]
    language: CodeableConcept,
    #[serde(rename = "preferred")]
    preferred: Option<bool>,
}

impl PatientCommunication {
    /// Creates a new PatientCommunication with required language.
    pub fn new(language: CodeableConcept) -> Self {
        Self {
            language,
            preferred: None,
        }
    }

    /// Creates a new PatientCommunication with language and preferred flag.
    pub fn with_preferred(language: CodeableConcept, preferred: bool) -> Self {
        Self {
            language,
            preferred: Some(preferred),
        }
    }

    /// Returns the language.
    pub fn language(&self) -> &CodeableConcept {
        &self.language
    }

    /// Returns the preferred flag.
    pub fn preferred(&self) -> Option<bool> {
        self.preferred
    }

    /// Sets the language.
    pub fn set_language(&mut self, language: CodeableConcept) {
        self.language = language;
    }

    /// Sets the preferred flag.
    pub fn set_preferred(&mut self, preferred: Option<bool>) {
        self.preferred = preferred;
    }
} 