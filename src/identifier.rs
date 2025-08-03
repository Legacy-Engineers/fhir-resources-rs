use crate::data_types::uri::Uri;
use serde::{Deserialize, Serialize};
use crate::period::Period;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Identifier {
    r#use: Uri,
    system: Uri,
    value: String,
    period: Option<Period>, // Using the Period struct
}

impl Identifier {
    /// Creates a new Identifier with the required fields.
    pub fn new(use_value: Uri, system: Uri, value: String) -> Self {
        Self {
            r#use: use_value,
            system,
            value,
            period: None,
        }
    }

    /// Returns the use value.
    pub fn use_value(&self) -> &Uri {
        &self.r#use
    }

    /// Returns the system URI.
    pub fn system(&self) -> &Uri {
        &self.system
    }

    /// Returns the identifier value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the period.
    pub fn period(&self) -> Option<&Period> {
        self.period.as_ref()
    }

    /// Sets the period.
    pub fn set_period(&mut self, period: Option<Period>) {
        self.period = period;
    }

    /// Converts the Identifier to a JSON string.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// Converts a JSON string to an Identifier.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}
