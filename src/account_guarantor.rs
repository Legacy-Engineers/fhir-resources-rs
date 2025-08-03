use crate::data_types::reference::Reference;
use crate::period::Period;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountGuarantor {
    #[serde(rename = "party")]
    party: Reference,
    #[serde(rename = "onHold")]
    on_hold: Option<bool>,
    #[serde(rename = "period")]
    period: Option<Period>,
}



impl AccountGuarantor {
    pub fn new(party: Reference) -> Self {
        Self {
            party,
            on_hold: None,
            period: None,
        }
    }

    pub fn party(&self) -> &Reference {
        &self.party
    }

    pub fn on_hold(&self) -> Option<bool> {
        self.on_hold
    }

    pub fn period(&self) -> Option<&Period> {
        self.period.as_ref()
    }

    pub fn set_party(&mut self, party: Reference) {
        self.party = party;
    }

    pub fn set_on_hold(&mut self, on_hold: Option<bool>) {
        self.on_hold = on_hold;
    }

    pub fn set_period(&mut self, period: Option<Period>) {
        self.period = period;
    }
}
