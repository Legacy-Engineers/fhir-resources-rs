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
}