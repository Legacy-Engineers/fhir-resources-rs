use std::fmt;

pub struct Identifier {
    system: String,
    value: String,
}

impl Identifier {
    pub fn new(system: String, value: String) -> Self {
        Identifier { system, value }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}|{}", self.system, self.value)
    }
}
