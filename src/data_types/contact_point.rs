use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContactPoint {
    #[serde(rename = "system")]
    system: String, // phone | fax | email | pager | url | sms | other
    #[serde(rename = "value")]
    value: String,
    #[serde(rename = "use")]
    r#use: Option<String>, // home | work | temp | old | mobile
    #[serde(rename = "rank")]
    rank: Option<i32>,
    #[serde(rename = "period")]
    period: Option<String>,
}

impl ContactPoint {
    /// Creates a new ContactPoint with required fields.
    pub fn new(system: String, value: String) -> Self {
        Self {
            system,
            value,
            r#use: None,
            rank: None,
            period: None,
        }
    }

    /// Creates a new ContactPoint with all fields.
    pub fn new_with_details(
        system: String,
        value: String,
        use_value: Option<String>,
        rank: Option<i32>,
        period: Option<String>,
    ) -> Self {
        Self {
            system,
            value,
            r#use: use_value,
            rank,
            period,
        }
    }

    /// Returns the system.
    pub fn system(&self) -> &str {
        &self.system
    }

    /// Returns the value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the use value.
    pub fn use_value(&self) -> Option<&str> {
        self.r#use.as_deref()
    }

    /// Returns the rank.
    pub fn rank(&self) -> Option<i32> {
        self.rank
    }

    /// Returns the period.
    pub fn period(&self) -> Option<&str> {
        self.period.as_deref()
    }

    /// Sets the system.
    pub fn set_system(&mut self, system: String) {
        self.system = system;
    }

    /// Sets the value.
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    /// Sets the use value.
    pub fn set_use(&mut self, use_value: Option<String>) {
        self.r#use = use_value;
    }

    /// Sets the rank.
    pub fn set_rank(&mut self, rank: Option<i32>) {
        self.rank = rank;
    }

    /// Sets the period.
    pub fn set_period(&mut self, period: Option<String>) {
        self.period = period;
    }
} 