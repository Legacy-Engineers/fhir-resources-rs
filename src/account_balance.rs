use serde::{Deserialize, Serialize};
use crate::data_types::codeable_concept::CodeableConcept;
use crate::money::Money;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountBalance {
    #[serde(rename = "aggregate")]
    aggregate: Option<CodeableConcept>,
    #[serde(rename = "term")]
    term: Option<CodeableConcept>,
    #[serde(rename = "estimate")]
    estimate: Option<bool>,
    #[serde(rename = "amount")]
    amount: Money,
}


impl AccountBalance {
    pub fn new(amount: Money) -> Self {
        Self {
            aggregate: None,
            term: None,
            estimate: None,
            amount,
        }
    }

    pub fn amount(&self) -> &Money {
        &self.amount
    }

    pub fn aggregate(&self) -> Option<&CodeableConcept> {
        self.aggregate.as_ref()
    }

    pub fn term(&self) -> Option<&CodeableConcept> {
        self.term.as_ref()
    }

    pub fn estimate(&self) -> Option<bool> {
        self.estimate
    }

    pub fn set_amount(&mut self, amount: Money) {
        self.amount = amount;
    }

    pub fn set_aggregate(&mut self, aggregate: Option<CodeableConcept>) {
        self.aggregate = aggregate;
    }

    pub fn set_term(&mut self, term: Option<CodeableConcept>) {
        self.term = term;
    }

    pub fn set_estimate(&mut self, estimate: Option<bool>) {
        self.estimate = estimate;
    }
}
