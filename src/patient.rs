use crate::identifier::Identifier;
use crate::human_name::HumanName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Patient {
    resource_type: String,
    identifier: Vec<Identifier>,
    name: Vec<HumanName>,
}

impl Patient {
    /// Creates a new Patient with default values.
    pub fn new() -> Self {
        Self {
            resource_type: "Patient".to_string(),
            identifier: Vec::new(),
            name: Vec::new(),
        }
    }

    /// Creates a new Patient with the specified resource type.
    pub fn with_resource_type(resource_type: String) -> Self {
        Self {
            resource_type,
            identifier: Vec::new(),
            name: Vec::new(),
        }
    }

    /// Returns the resource type.
    pub fn resource_type(&self) -> &str {
        &self.resource_type
    }

    /// Returns the identifiers.
    pub fn identifiers(&self) -> &[Identifier] {
        &self.identifier
    }

    /// Returns the names.
    pub fn names(&self) -> &[HumanName] {
        &self.name
    }

    /// Adds an identifier.
    pub fn add_identifier(&mut self, identifier: Identifier) {
        self.identifier.push(identifier);
    }

    /// Adds a name.
    pub fn add_name(&mut self, name: HumanName) {
        self.name.push(name);
    }

    /// Sets the resource type.
    pub fn set_resource_type(&mut self, resource_type: String) {
        self.resource_type = resource_type;
    }

    /// Sets the identifiers.
    pub fn set_identifiers(&mut self, identifiers: Vec<Identifier>) {
        self.identifier = identifiers;
    }

    /// Sets the names.
    pub fn set_names(&mut self, names: Vec<HumanName>) {
        self.name = names;
    }
}

impl Default for Patient {
    fn default() -> Self {
        Self::new()
    }
}
