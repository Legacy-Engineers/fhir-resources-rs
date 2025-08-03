use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HumanName {
    r#use: String,
    text: String,
    family: String,
    given: Vec<String>,
    prefix: Vec<String>,
    suffix: Vec<String>,
    period: Option<String>, // Simplified for now, can be enhanced later
}

impl HumanName {
    /// Creates a new HumanName with the required fields.
    pub fn new(use_value: String, text: String, family: String) -> Self {
        Self {
            r#use: use_value,
            text,
            family,
            given: Vec::new(),
            prefix: Vec::new(),
            suffix: Vec::new(),
            period: None,
        }
    }

    /// Returns the use value.
    pub fn use_value(&self) -> &str {
        &self.r#use
    }

    /// Returns the text value.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns the family name.
    pub fn family(&self) -> &str {
        &self.family
    }

    /// Returns the given names.
    pub fn given(&self) -> &[String] {
        &self.given
    }

    /// Returns the prefixes.
    pub fn prefix(&self) -> &[String] {
        &self.prefix
    }

    /// Returns the suffixes.
    pub fn suffix(&self) -> &[String] {
        &self.suffix
    }

    /// Returns the period.
    pub fn period(&self) -> Option<&str> {
        self.period.as_deref()
    }

    /// Sets the given names.
    pub fn set_given(&mut self, given: Vec<String>) {
        self.given = given;
    }

    /// Sets the prefixes.
    pub fn set_prefix(&mut self, prefix: Vec<String>) {
        self.prefix = prefix;
    }

    /// Sets the suffixes.
    pub fn set_suffix(&mut self, suffix: Vec<String>) {
        self.suffix = suffix;
    }

    /// Sets the period.
    pub fn set_period(&mut self, period: Option<String>) {
        self.period = period;
    }
}
