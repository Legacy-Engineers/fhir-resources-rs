use crate::data_types::reference::Reference;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountCoverage {
    #[serde(rename = "coverage")]
    coverage: Reference,
    #[serde(rename = "priority")]
    priority: Option<String>, // positiveInt
}

// Implementations for nested structures
impl AccountCoverage {
    pub fn new(coverage: Reference) -> Self {
        Self {
            coverage,
            priority: None,
        }
    }

    pub fn with_priority(coverage: Reference, priority: String) -> Self {
        Self {
            coverage,
            priority: Some(priority),
        }
    }

    pub fn coverage(&self) -> &Reference {
        &self.coverage
    }

    pub fn priority(&self) -> Option<&str> {
        self.priority.as_deref()
    }

    pub fn set_coverage(&mut self, coverage: Reference) {
        self.coverage = coverage;
    }

    pub fn set_priority(&mut self, priority: Option<String>) {
        self.priority = priority;
    }
}
