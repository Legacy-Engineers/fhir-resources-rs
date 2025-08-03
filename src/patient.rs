use crate::identifier::Identifier;
use crate::human_name::HumanName;
use crate::data_types::contact_point::ContactPoint;
use crate::data_types::address::Address;
use crate::data_types::codeable_concept::CodeableConcept;
use crate::data_types::reference::Reference;
use crate::patient_contact::PatientContact;
use crate::patient_communication::PatientCommunication;
use crate::patient_link::PatientLink;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Patient {
    #[serde(rename = "resourceType")]
    resource_type: String,
    #[serde(rename = "identifier")]
    identifier: Vec<Identifier>,
    #[serde(rename = "active")]
    active: Option<bool>,
    #[serde(rename = "name")]
    name: Vec<HumanName>,
    #[serde(rename = "telecom")]
    telecom: Vec<ContactPoint>,
    #[serde(rename = "gender")]
    gender: Option<String>, // male | female | other | unknown
    #[serde(rename = "birthDate")]
    birth_date: Option<String>,
    #[serde(rename = "deceasedBoolean")]
    deceased_boolean: Option<bool>,
    #[serde(rename = "deceasedDateTime")]
    deceased_date_time: Option<String>,
    #[serde(rename = "address")]
    address: Vec<Address>,
    #[serde(rename = "maritalStatus")]
    marital_status: Option<CodeableConcept>,
    #[serde(rename = "multipleBirthBoolean")]
    multiple_birth_boolean: Option<bool>,
    #[serde(rename = "multipleBirthInteger")]
    multiple_birth_integer: Option<i32>,
    #[serde(rename = "photo")]
    photo: Vec<String>, // Simplified as String for now
    #[serde(rename = "contact")]
    contact: Vec<PatientContact>,
    #[serde(rename = "communication")]
    communication: Vec<PatientCommunication>,
    #[serde(rename = "generalPractitioner")]
    general_practitioner: Vec<Reference>,
    #[serde(rename = "managingOrganization")]
    managing_organization: Option<Reference>,
    #[serde(rename = "link")]
    link: Vec<PatientLink>,
}

impl Patient {
    /// Creates a new Patient with default values.
    pub fn new() -> Self {
        Self {
            resource_type: "Patient".to_string(),
            identifier: Vec::new(),
            active: None,
            name: Vec::new(),
            telecom: Vec::new(),
            gender: None,
            birth_date: None,
            deceased_boolean: None,
            deceased_date_time: None,
            address: Vec::new(),
            marital_status: None,
            multiple_birth_boolean: None,
            multiple_birth_integer: None,
            photo: Vec::new(),
            contact: Vec::new(),
            communication: Vec::new(),
            general_practitioner: Vec::new(),
            managing_organization: None,
            link: Vec::new(),
        }
    }

    /// Creates a new Patient with the specified resource type.
    pub fn with_resource_type(resource_type: String) -> Self {
        Self {
            resource_type,
            identifier: Vec::new(),
            active: None,
            name: Vec::new(),
            telecom: Vec::new(),
            gender: None,
            birth_date: None,
            deceased_boolean: None,
            deceased_date_time: None,
            address: Vec::new(),
            marital_status: None,
            multiple_birth_boolean: None,
            multiple_birth_integer: None,
            photo: Vec::new(),
            contact: Vec::new(),
            communication: Vec::new(),
            general_practitioner: Vec::new(),
            managing_organization: None,
            link: Vec::new(),
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

    // Active status
    pub fn active(&self) -> Option<bool> {
        self.active
    }

    pub fn set_active(&mut self, active: Option<bool>) {
        self.active = active;
    }

    // Telecom
    pub fn telecom(&self) -> &[ContactPoint] {
        &self.telecom
    }

    pub fn add_telecom(&mut self, telecom: ContactPoint) {
        self.telecom.push(telecom);
    }

    pub fn set_telecom(&mut self, telecom: Vec<ContactPoint>) {
        self.telecom = telecom;
    }

    // Gender
    pub fn gender(&self) -> Option<&str> {
        self.gender.as_deref()
    }

    pub fn set_gender(&mut self, gender: Option<String>) {
        self.gender = gender;
    }

    // Birth date
    pub fn birth_date(&self) -> Option<&str> {
        self.birth_date.as_deref()
    }

    pub fn set_birth_date(&mut self, birth_date: Option<String>) {
        self.birth_date = birth_date;
    }

    // Deceased
    pub fn deceased_boolean(&self) -> Option<bool> {
        self.deceased_boolean
    }

    pub fn set_deceased_boolean(&mut self, deceased: Option<bool>) {
        self.deceased_boolean = deceased;
    }

    pub fn deceased_date_time(&self) -> Option<&str> {
        self.deceased_date_time.as_deref()
    }

    pub fn set_deceased_date_time(&mut self, deceased_date_time: Option<String>) {
        self.deceased_date_time = deceased_date_time;
    }

    // Address
    pub fn address(&self) -> &[Address] {
        &self.address
    }

    pub fn add_address(&mut self, address: Address) {
        self.address.push(address);
    }

    pub fn set_address(&mut self, address: Vec<Address>) {
        self.address = address;
    }

    // Marital status
    pub fn marital_status(&self) -> Option<&CodeableConcept> {
        self.marital_status.as_ref()
    }

    pub fn set_marital_status(&mut self, marital_status: Option<CodeableConcept>) {
        self.marital_status = marital_status;
    }

    // Multiple birth
    pub fn multiple_birth_boolean(&self) -> Option<bool> {
        self.multiple_birth_boolean
    }

    pub fn set_multiple_birth_boolean(&mut self, multiple_birth: Option<bool>) {
        self.multiple_birth_boolean = multiple_birth;
    }

    pub fn multiple_birth_integer(&self) -> Option<i32> {
        self.multiple_birth_integer
    }

    pub fn set_multiple_birth_integer(&mut self, multiple_birth: Option<i32>) {
        self.multiple_birth_integer = multiple_birth;
    }

    // Photo
    pub fn photo(&self) -> &[String] {
        &self.photo
    }

    pub fn add_photo(&mut self, photo: String) {
        self.photo.push(photo);
    }

    pub fn set_photo(&mut self, photo: Vec<String>) {
        self.photo = photo;
    }

    // Contact
    pub fn contact(&self) -> &[PatientContact] {
        &self.contact
    }

    pub fn add_contact(&mut self, contact: PatientContact) {
        self.contact.push(contact);
    }

    pub fn set_contact(&mut self, contact: Vec<PatientContact>) {
        self.contact = contact;
    }

    // Communication
    pub fn communication(&self) -> &[PatientCommunication] {
        &self.communication
    }

    pub fn add_communication(&mut self, communication: PatientCommunication) {
        self.communication.push(communication);
    }

    pub fn set_communication(&mut self, communication: Vec<PatientCommunication>) {
        self.communication = communication;
    }

    // General practitioner
    pub fn general_practitioner(&self) -> &[Reference] {
        &self.general_practitioner
    }

    pub fn add_general_practitioner(&mut self, practitioner: Reference) {
        self.general_practitioner.push(practitioner);
    }

    pub fn set_general_practitioner(&mut self, practitioners: Vec<Reference>) {
        self.general_practitioner = practitioners;
    }

    // Managing organization
    pub fn managing_organization(&self) -> Option<&Reference> {
        self.managing_organization.as_ref()
    }

    pub fn set_managing_organization(&mut self, organization: Option<Reference>) {
        self.managing_organization = organization;
    }

    // Link
    pub fn link(&self) -> &[PatientLink] {
        &self.link
    }

    pub fn add_link(&mut self, link: PatientLink) {
        self.link.push(link);
    }

    pub fn set_link(&mut self, link: Vec<PatientLink>) {
        self.link = link;
    }

    /// Converts the Patient to a JSON string.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// Converts a JSON string to a Patient.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

impl Default for Patient {
    fn default() -> Self {
        Self::new()
    }
}
