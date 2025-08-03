use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeableConcept {
    #[serde(rename = "coding")]
    coding: Vec<Coding>,
    #[serde(rename = "text")]
    text: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Coding {
    #[serde(rename = "system")]
    system: Option<String>,
    #[serde(rename = "version")]
    version: Option<String>,
    #[serde(rename = "code")]
    code: Option<String>,
    #[serde(rename = "display")]
    display: Option<String>,
    #[serde(rename = "userSelected")]
    user_selected: Option<bool>,
}

impl CodeableConcept {
    /// Creates a new CodeableConcept.
    pub fn new() -> Self {
        Self {
            coding: Vec::new(),
            text: None,
        }
    }

    /// Creates a new CodeableConcept with text.
    pub fn with_text(text: String) -> Self {
        Self {
            coding: Vec::new(),
            text: Some(text),
        }
    }

    /// Creates a new CodeableConcept with coding.
    pub fn with_coding(coding: Coding) -> Self {
        Self {
            coding: vec![coding],
            text: None,
        }
    }

    /// Returns the coding.
    pub fn coding(&self) -> &[Coding] {
        &self.coding
    }

    /// Returns the text.
    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }

    /// Sets the coding.
    pub fn set_coding(&mut self, coding: Vec<Coding>) {
        self.coding = coding;
    }

    /// Adds a coding.
    pub fn add_coding(&mut self, coding: Coding) {
        self.coding.push(coding);
    }

    /// Sets the text.
    pub fn set_text(&mut self, text: Option<String>) {
        self.text = text;
    }
}

impl Coding {
    /// Creates a new Coding.
    pub fn new() -> Self {
        Self {
            system: None,
            version: None,
            code: None,
            display: None,
            user_selected: None,
        }
    }

    /// Creates a new Coding with code.
    pub fn with_code(code: String) -> Self {
        Self {
            system: None,
            version: None,
            code: Some(code),
            display: None,
            user_selected: None,
        }
    }

    /// Creates a new Coding with system and code.
    pub fn with_system_and_code(system: String, code: String) -> Self {
        Self {
            system: Some(system),
            version: None,
            code: Some(code),
            display: None,
            user_selected: None,
        }
    }

    /// Returns the system.
    pub fn system(&self) -> Option<&str> {
        self.system.as_deref()
    }

    /// Returns the version.
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Returns the code.
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    /// Returns the display.
    pub fn display(&self) -> Option<&str> {
        self.display.as_deref()
    }

    /// Returns the user selected flag.
    pub fn user_selected(&self) -> Option<bool> {
        self.user_selected
    }

    /// Sets the system.
    pub fn set_system(&mut self, system: Option<String>) {
        self.system = system;
    }

    /// Sets the version.
    pub fn set_version(&mut self, version: Option<String>) {
        self.version = version;
    }

    /// Sets the code.
    pub fn set_code(&mut self, code: Option<String>) {
        self.code = code;
    }

    /// Sets the display.
    pub fn set_display(&mut self, display: Option<String>) {
        self.display = display;
    }

    /// Sets the user selected flag.
    pub fn set_user_selected(&mut self, user_selected: Option<bool>) {
        self.user_selected = user_selected;
    }
}

impl Default for CodeableConcept {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Coding {
    fn default() -> Self {
        Self::new()
    }
} 