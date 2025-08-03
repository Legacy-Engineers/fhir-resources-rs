use crate::data_types::codeable_concept::CodeableConcept;
use crate::data_types::reference::Reference;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountDiagnosis {
    #[serde(rename = "sequence")]
    sequence: Option<String>, // positiveInt
    #[serde(rename = "condition")]
    condition: Reference, // CodeableReference(Condition) - simplified as Reference for now
    #[serde(rename = "dateOfDiagnosis")]
    date_of_diagnosis: Option<String>, // dateTime
    #[serde(rename = "type")]
    diagnosis_type: Vec<CodeableConcept>,
    #[serde(rename = "onAdmission")]
    on_admission: Option<bool>,
    #[serde(rename = "packageCode")]
    package_code: Vec<CodeableConcept>,
}




impl AccountDiagnosis {
    pub fn new(condition: Reference) -> Self {
        Self {
            sequence: None,
            condition,
            date_of_diagnosis: None,
            diagnosis_type: Vec::new(),
            on_admission: None,
            package_code: Vec::new(),
        }
    }

    pub fn condition(&self) -> &Reference {
        &self.condition
    }

    pub fn sequence(&self) -> Option<&str> {
        self.sequence.as_deref()
    }

    pub fn date_of_diagnosis(&self) -> Option<&str> {
        self.date_of_diagnosis.as_deref()
    }

    pub fn diagnosis_type(&self) -> &[CodeableConcept] {
        &self.diagnosis_type
    }

    pub fn on_admission(&self) -> Option<bool> {
        self.on_admission
    }

    pub fn package_code(&self) -> &[CodeableConcept] {
        &self.package_code
    }

    pub fn set_condition(&mut self, condition: Reference) {
        self.condition = condition;
    }

    pub fn set_sequence(&mut self, sequence: Option<String>) {
        self.sequence = sequence;
    }

    pub fn set_date_of_diagnosis(&mut self, date_of_diagnosis: Option<String>) {
        self.date_of_diagnosis = date_of_diagnosis;
    }

    pub fn set_diagnosis_type(&mut self, diagnosis_type: Vec<CodeableConcept>) {
        self.diagnosis_type = diagnosis_type;
    }

    pub fn add_diagnosis_type(&mut self, diagnosis_type: CodeableConcept) {
        self.diagnosis_type.push(diagnosis_type);
    }

    pub fn set_on_admission(&mut self, on_admission: Option<bool>) {
        self.on_admission = on_admission;
    }

    pub fn set_package_code(&mut self, package_code: Vec<CodeableConcept>) {
        self.package_code = package_code;
    }

    pub fn add_package_code(&mut self, package_code: CodeableConcept) {
        self.package_code.push(package_code);
    }
}
