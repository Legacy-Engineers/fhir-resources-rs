use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Address {
    #[serde(rename = "use")]
    r#use: Option<String>, // home | work | temp | old | billing
    #[serde(rename = "type")]
    address_type: Option<String>, // postal | physical | both
    #[serde(rename = "text")]
    text: Option<String>,
    #[serde(rename = "line")]
    line: Vec<String>,
    #[serde(rename = "city")]
    city: Option<String>,
    #[serde(rename = "district")]
    district: Option<String>,
    #[serde(rename = "state")]
    state: Option<String>,
    #[serde(rename = "postalCode")]
    postal_code: Option<String>,
    #[serde(rename = "country")]
    country: Option<String>,
    #[serde(rename = "period")]
    period: Option<String>,
}

impl Address {
    /// Creates a new Address with basic fields.
    pub fn new() -> Self {
        Self {
            r#use: None,
            address_type: None,
            text: None,
            line: Vec::new(),
            city: None,
            district: None,
            state: None,
            postal_code: None,
            country: None,
            period: None,
        }
    }

    /// Creates a new Address with text.
    pub fn with_text(text: String) -> Self {
        Self {
            r#use: None,
            address_type: None,
            text: Some(text),
            line: Vec::new(),
            city: None,
            district: None,
            state: None,
            postal_code: None,
            country: None,
            period: None,
        }
    }

    /// Returns the use value.
    pub fn use_value(&self) -> Option<&str> {
        self.r#use.as_deref()
    }

    /// Returns the address type.
    pub fn address_type(&self) -> Option<&str> {
        self.address_type.as_deref()
    }

    /// Returns the text.
    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }

    /// Returns the line addresses.
    pub fn line(&self) -> &[String] {
        &self.line
    }

    /// Returns the city.
    pub fn city(&self) -> Option<&str> {
        self.city.as_deref()
    }

    /// Returns the district.
    pub fn district(&self) -> Option<&str> {
        self.district.as_deref()
    }

    /// Returns the state.
    pub fn state(&self) -> Option<&str> {
        self.state.as_deref()
    }

    /// Returns the postal code.
    pub fn postal_code(&self) -> Option<&str> {
        self.postal_code.as_deref()
    }

    /// Returns the country.
    pub fn country(&self) -> Option<&str> {
        self.country.as_deref()
    }

    /// Returns the period.
    pub fn period(&self) -> Option<&str> {
        self.period.as_deref()
    }

    /// Sets the use value.
    pub fn set_use(&mut self, use_value: Option<String>) {
        self.r#use = use_value;
    }

    /// Sets the address type.
    pub fn set_address_type(&mut self, address_type: Option<String>) {
        self.address_type = address_type;
    }

    /// Sets the text.
    pub fn set_text(&mut self, text: Option<String>) {
        self.text = text;
    }

    /// Sets the line addresses.
    pub fn set_line(&mut self, line: Vec<String>) {
        self.line = line;
    }

    /// Adds a line address.
    pub fn add_line(&mut self, line: String) {
        self.line.push(line);
    }

    /// Sets the city.
    pub fn set_city(&mut self, city: Option<String>) {
        self.city = city;
    }

    /// Sets the district.
    pub fn set_district(&mut self, district: Option<String>) {
        self.district = district;
    }

    /// Sets the state.
    pub fn set_state(&mut self, state: Option<String>) {
        self.state = state;
    }

    /// Sets the postal code.
    pub fn set_postal_code(&mut self, postal_code: Option<String>) {
        self.postal_code = postal_code;
    }

    /// Sets the country.
    pub fn set_country(&mut self, country: Option<String>) {
        self.country = country;
    }

    /// Sets the period.
    pub fn set_period(&mut self, period: Option<String>) {
        self.period = period;
    }
}

impl Default for Address {
    fn default() -> Self {
        Self::new()
    }
} 