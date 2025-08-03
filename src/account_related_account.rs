use serde::{Deserialize, Serialize};
use crate::data_types::codeable_concept::CodeableConcept;
use crate::data_types::reference::Reference;


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountRelatedAccount {
    #[serde(rename = "relationship")]
    relationship: Option<CodeableConcept>,
    #[serde(rename = "account")]
    account: Reference,
}


impl AccountRelatedAccount {
    pub fn new(account: Reference) -> Self {
        Self {
            relationship: None,
            account,
        }
    }

    pub fn with_relationship(account: Reference, relationship: CodeableConcept) -> Self {
        Self {
            relationship: Some(relationship),
            account,
        }
    }

    pub fn account(&self) -> &Reference {
        &self.account
    }

    pub fn relationship(&self) -> Option<&CodeableConcept> {
        self.relationship.as_ref()
    }

    pub fn set_account(&mut self, account: Reference) {
        self.account = account;
    }

    pub fn set_relationship(&mut self, relationship: Option<CodeableConcept>) {
        self.relationship = relationship;
    }
}
