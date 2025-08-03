use crate::data_types::reference::Reference;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatientLink {
    #[serde(rename = "other")]
    other: Reference,
    #[serde(rename = "type")]
    link_type: String, // replaced-by | replaces | refer | seealso
}

impl PatientLink {
    /// Creates a new PatientLink.
    pub fn new(other: Reference, link_type: String) -> Self {
        Self {
            other,
            link_type,
        }
    }

    /// Returns the other reference.
    pub fn other(&self) -> &Reference {
        &self.other
    }

    /// Returns the link type.
    pub fn link_type(&self) -> &str {
        &self.link_type
    }

    /// Sets the other reference.
    pub fn set_other(&mut self, other: Reference) {
        self.other = other;
    }

    /// Sets the link type.
    pub fn set_link_type(&mut self, link_type: String) {
        self.link_type = link_type;
    }
} 