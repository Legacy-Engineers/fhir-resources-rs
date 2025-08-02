use crate::identifier::Identifier;

struct Patient {
    resource_type: String,
    identifier: Vec<Identifier>,
    name: Vec<HumanName>,
}
