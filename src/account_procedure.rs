use serde::{Deserialize, Serialize};
use crate::data_types::codeable_concept::CodeableConcept;
use crate::data_types::reference::Reference;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountProcedure {
    #[serde(rename = "sequence")]
    sequence: Option<String>, // positiveInt
    #[serde(rename = "code")]
    code: Reference, // CodeableReference(Procedure) - simplified as Reference for now
    #[serde(rename = "dateOfService")]
    date_of_service: Option<String>, // dateTime
    #[serde(rename = "type")]
    procedure_type: Vec<CodeableConcept>,
    #[serde(rename = "packageCode")]
    package_code: Vec<CodeableConcept>,
    #[serde(rename = "device")]
    device: Vec<Reference>,
}



impl AccountProcedure {
    pub fn new(code: Reference) -> Self {
        Self {
            sequence: None,
            code,
            date_of_service: None,
            procedure_type: Vec::new(),
            package_code: Vec::new(),
            device: Vec::new(),
        }
    }

    pub fn code(&self) -> &Reference {
        &self.code
    }

    pub fn sequence(&self) -> Option<&str> {
        self.sequence.as_deref()
    }

    pub fn date_of_service(&self) -> Option<&str> {
        self.date_of_service.as_deref()
    }

    pub fn procedure_type(&self) -> &[CodeableConcept] {
        &self.procedure_type
    }

    pub fn package_code(&self) -> &[CodeableConcept] {
        &self.package_code
    }

    pub fn device(&self) -> &[Reference] {
        &self.device
    }

    pub fn set_code(&mut self, code: Reference) {
        self.code = code;
    }

    pub fn set_sequence(&mut self, sequence: Option<String>) {
        self.sequence = sequence;
    }

    pub fn set_date_of_service(&mut self, date_of_service: Option<String>) {
        self.date_of_service = date_of_service;
    }

    pub fn set_procedure_type(&mut self, procedure_type: Vec<CodeableConcept>) {
        self.procedure_type = procedure_type;
    }

    pub fn add_procedure_type(&mut self, procedure_type: CodeableConcept) {
        self.procedure_type.push(procedure_type);
    }

    pub fn set_package_code(&mut self, package_code: Vec<CodeableConcept>) {
        self.package_code = package_code;
    }

    pub fn add_package_code(&mut self, package_code: CodeableConcept) {
        self.package_code.push(package_code);
    }

    pub fn set_device(&mut self, device: Vec<Reference>) {
        self.device = device;
    }

    pub fn add_device(&mut self, device: Reference) {
        self.device.push(device);
    }
}
