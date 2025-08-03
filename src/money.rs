use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Money {
    #[serde(rename = "value")]
    value: Option<f64>,
    #[serde(rename = "currency")]
    currency: Option<String>,
}


impl Money {
    pub fn new() -> Self {
        Self {
            value: None,
            currency: None,
        }
    }

    pub fn with_value(value: f64) -> Self {
        Self {
            value: Some(value),
            currency: None,
        }
    }

    pub fn with_currency(value: f64, currency: String) -> Self {
        Self {
            value: Some(value),
            currency: Some(currency),
        }
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }

    pub fn currency(&self) -> Option<&str> {
        self.currency.as_deref()
    }

    pub fn set_value(&mut self, value: Option<f64>) {
        self.value = value;
    }

    pub fn set_currency(&mut self, currency: Option<String>) {
        self.currency = currency;
    }
}


impl Default for Money {
    fn default() -> Self {
        Self::new()
    }
} 