use crate::human_name::HumanName;
use crate::data_types::contact_point::ContactPoint;
use crate::data_types::address::Address;
use crate::data_types::codeable_concept::CodeableConcept;
use crate::data_types::reference::Reference;
use crate::period::Period;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatientContact {
    #[serde(rename = "relationship")]
    relationship: Vec<CodeableConcept>,
    #[serde(rename = "name")]
    name: Option<HumanName>,
    #[serde(rename = "telecom")]
    telecom: Vec<ContactPoint>,
    #[serde(rename = "address")]
    address: Option<Address>,
    #[serde(rename = "gender")]
    gender: Option<String>, // male | female | other | unknown
    #[serde(rename = "organization")]
    organization: Option<Reference>,
    #[serde(rename = "period")]
    period: Option<Period>,
}

impl PatientContact {
    /// Creates a new PatientContact.
    pub fn new() -> Self {
        Self {
            relationship: Vec::new(),
            name: None,
            telecom: Vec::new(),
            address: None,
            gender: None,
            organization: None,
            period: None,
        }
    }

    /// Creates a new PatientContact with name.
    pub fn with_name(name: HumanName) -> Self {
        Self {
            relationship: Vec::new(),
            name: Some(name),
            telecom: Vec::new(),
            address: None,
            gender: None,
            organization: None,
            period: None,
        }
    }

    /// Returns the relationships.
    pub fn relationship(&self) -> &[CodeableConcept] {
        &self.relationship
    }

    /// Returns the name.
    pub fn name(&self) -> Option<&HumanName> {
        self.name.as_ref()
    }

    /// Returns the telecom.
    pub fn telecom(&self) -> &[ContactPoint] {
        &self.telecom
    }

    /// Returns the address.
    pub fn address(&self) -> Option<&Address> {
        self.address.as_ref()
    }

    /// Returns the gender.
    pub fn gender(&self) -> Option<&str> {
        self.gender.as_deref()
    }

    /// Returns the organization.
    pub fn organization(&self) -> Option<&Reference> {
        self.organization.as_ref()
    }

    /// Returns the period.
    pub fn period(&self) -> Option<&Period> {
        self.period.as_ref()
    }

    /// Sets the relationships.
    pub fn set_relationship(&mut self, relationship: Vec<CodeableConcept>) {
        self.relationship = relationship;
    }

    /// Adds a relationship.
    pub fn add_relationship(&mut self, relationship: CodeableConcept) {
        self.relationship.push(relationship);
    }

    /// Sets the name.
    pub fn set_name(&mut self, name: Option<HumanName>) {
        self.name = name;
    }

    /// Sets the telecom.
    pub fn set_telecom(&mut self, telecom: Vec<ContactPoint>) {
        self.telecom = telecom;
    }

    /// Adds a telecom.
    pub fn add_telecom(&mut self, telecom: ContactPoint) {
        self.telecom.push(telecom);
    }

    /// Sets the address.
    pub fn set_address(&mut self, address: Option<Address>) {
        self.address = address;
    }

    /// Sets the gender.
    pub fn set_gender(&mut self, gender: Option<String>) {
        self.gender = gender;
    }

    /// Sets the organization.
    pub fn set_organization(&mut self, organization: Option<Reference>) {
        self.organization = organization;
    }

    /// Sets the period.
    pub fn set_period(&mut self, period: Option<Period>) {
        self.period = period;
    }
}

impl Default for PatientContact {
    fn default() -> Self {
        Self::new()
    }
} 