use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Period {
    start: String,
    end: String,
}

impl Period {
    pub fn new(start: String, end: String) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> &str {
        &self.start
    }

    pub fn set_start(&mut self, start: String) {
        self.start = start;
    }

    pub fn end(&self) -> &str {
        &self.end
    }

    pub fn set_end(&mut self, end: String) {
        self.end = end;
    }

    /// Converts the Period to a JSON string.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// Converts a JSON string to a Period.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}