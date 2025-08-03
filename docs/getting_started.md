# Getting Started with FHIR Resources Rust Library

> **⚠️ IMPORTANT**: This is a personal project and is NOT ready for production use.

## Prerequisites

- Rust 1.70 or later
- Basic understanding of Rust
- Familiarity with FHIR concepts (helpful but not required)

## Installation

### Adding to Your Project

Add the library to your `Cargo.toml`:

```toml
[dependencies]
fhir-resources-rs = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Building from Source

```bash
git clone <repository-url>
cd fhir-resources-rs
cargo build
```

## Basic Usage

### 1. Creating a Patient

```rust
use fhir_resources_rs::patient::Patient;
use fhir_resources_rs::human_name::HumanName;
use fhir_resources_rs::identifier::Identifier;
use fhir_resources_rs::data_types::uri::Uri;

fn main() {
    // Create a new patient
    let mut patient = Patient::new();

    // Add an identifier (Medical Record Number)
    let system_uri = Uri::new_unchecked("https://hospital.example.com/patients".to_string());
    let use_uri = Uri::new_unchecked("official".to_string());
    let identifier = Identifier::new(use_uri, system_uri, "MRN12345".to_string());
    patient.add_identifier(identifier);

    // Add a name
    let mut name = HumanName::new(
        "official".to_string(),
        "Dr. John Smith".to_string(),
        "Smith".to_string()
    );
    name.set_given(vec!["John".to_string()]);
    name.set_prefix(vec!["Dr.".to_string()]);
    patient.add_name(name);

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&patient).unwrap();
    println!("{}", json);

    // Note: The JSON output will use camelCase field names (e.g., "resourceType")
    // to follow FHIR conventions, while the Rust code uses snake_case.
}
```

### 2. Working with Human Names

```rust
use fhir_resources_rs::human_name::HumanName;

fn create_professional_name() -> HumanName {
    let mut name = HumanName::new(
        "official".to_string(),
        "Dr. Sarah Johnson, MD, PhD".to_string(),
        "Johnson".to_string()
    );

    // Add given names
    name.set_given(vec!["Sarah".to_string()]);

    // Add professional titles
    name.set_prefix(vec!["Dr.".to_string()]);
    name.set_suffix(vec!["MD".to_string(), "PhD".to_string()]);

    name
}

fn create_international_name() -> HumanName {
    let mut name = HumanName::new(
        "official".to_string(),
        "Tanaka Yuki".to_string(),
        "Tanaka".to_string()
    );

    name.set_given(vec!["Yuki".to_string()]);

    name
}
```

### 3. Creating Identifiers

```rust
use fhir_resources_rs::identifier::Identifier;
use fhir_resources_rs::data_types::uri::Uri;

fn create_healthcare_identifiers() -> Vec<Identifier> {
    let mut identifiers = Vec::new();

    // Medical Record Number
    let mrn_system = Uri::new_unchecked("https://hospital.example.com/patients".to_string());
    let mrn_use = Uri::new_unchecked("official".to_string());
    let mrn = Identifier::new(mrn_use, mrn_system, "MRN12345".to_string());
    identifiers.push(mrn);

    // Social Security Number
    let ssn_system = Uri::new_unchecked("https://ssa.gov/ssn".to_string());
    let ssn_use = Uri::new_unchecked("official".to_string());
    let ssn = Identifier::new(ssn_use, ssn_system, "123-45-6789".to_string());
    identifiers.push(ssn);

    // Insurance Member ID
    let ins_system = Uri::new_unchecked("https://insurance.example.com/member".to_string());
    let ins_use = Uri::new_unchecked("secondary".to_string());
    let insurance = Identifier::new(ins_use, ins_system, "INS98765".to_string());
    identifiers.push(insurance);

    identifiers
}
```

### 4. Working with Periods

```rust
use fhir_resources_rs::period::Period;

fn create_healthcare_periods() -> Vec<Period> {
    vec![
        // Annual coverage period
        Period::new("2020-01-01".to_string(), "2020-12-31".to_string()),

        // Hospital stay
        Period::new("2020-03-15".to_string(), "2020-03-20".to_string()),

        // Treatment course
        Period::new("2020-01-01".to_string(), "2020-06-30".to_string()),
    ]
}
```

## Advanced Examples

### Creating a Comprehensive Patient Record

```rust
use fhir_resources_rs::patient::Patient;
use fhir_resources_rs::human_name::HumanName;
use fhir_resources_rs::identifier::Identifier;
use fhir_resources_rs::data_types::uri::Uri;

fn create_comprehensive_patient() -> Patient {
    let mut patient = Patient::new();

    // Add multiple identifiers
    let identifiers = create_healthcare_identifiers();
    for identifier in identifiers {
        patient.add_identifier(identifier);
    }

    // Add multiple names
    let names = vec![
        create_official_name(),
        create_nickname(),
        create_maiden_name(),
    ];

    for name in names {
        patient.add_name(name);
    }

    patient
}

fn create_official_name() -> HumanName {
    let mut name = HumanName::new(
        "official".to_string(),
        "Dr. John Andrew Smith, MD, PhD".to_string(),
        "Smith".to_string()
    );
    name.set_given(vec!["John".to_string(), "Andrew".to_string()]);
    name.set_prefix(vec!["Dr.".to_string()]);
    name.set_suffix(vec!["MD".to_string(), "PhD".to_string()]);
    name
}

fn create_nickname() -> HumanName {
    let mut name = HumanName::new(
        "nickname".to_string(),
        "Johnny".to_string(),
        "Smith".to_string()
    );
    name.set_given(vec!["Johnny".to_string()]);
    name
}

fn create_maiden_name() -> HumanName {
    let mut name = HumanName::new(
        "maiden".to_string(),
        "Jane Smith".to_string(),
        "Smith".to_string()
    );
    name.set_given(vec!["Jane".to_string()]);
    name
}
```

### Serialization and Deserialization

```rust
use serde_json;

fn serialize_patient(patient: &Patient) -> String {
    serde_json::to_string_pretty(patient).unwrap()
}

fn deserialize_patient(json: &str) -> Result<Patient, serde_json::Error> {
    serde_json::from_str(json)
}

fn example_serialization() {
    let patient = create_comprehensive_patient();

    // Serialize
    let json = serialize_patient(&patient);
    println!("Serialized patient:\n{}", json);

    // Deserialize
    match deserialize_patient(&json) {
        Ok(deserialized_patient) => {
            println!("Successfully deserialized patient");
            assert_eq!(patient, deserialized_patient);
        }
        Err(e) => println!("Deserialization error: {}", e),
    }
}
```

## Error Handling

### Working with Validation

```rust
use fhir_resources_rs::data_types::uri::Uri;

fn create_validated_uri(uri_string: &str) -> Result<Uri, String> {
    match Uri::new(uri_string.to_string()) {
        Ok(uri) => Ok(uri),
        Err(e) => Err(format!("Invalid URI: {}", e)),
    }
}

fn example_validation() {
    // Valid URI
    match create_validated_uri("https://hospital.example.com/patients") {
        Ok(uri) => println!("Valid URI: {}", uri.as_str()),
        Err(e) => println!("Error: {}", e),
    }

    // Invalid URI
    match create_validated_uri("") {
        Ok(uri) => println!("Valid URI: {}", uri.as_str()),
        Err(e) => println!("Error: {}", e),
    }
}
```

## Testing Your Code

### Running the Test Suite

```bash
# Run all tests
cargo test

# Run specific test files with output
cargo test --test patient_tests -- --nocapture
cargo test --test human_name_tests -- --nocapture
cargo test --test period_tests -- --nocapture
cargo test --test identifier_tests -- --nocapture
```

### Writing Your Own Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use fhir_resources_rs::patient::Patient;

    #[test]
    fn test_patient_creation() {
        let patient = Patient::new();
        assert_eq!(patient.resource_type(), "Patient");
        assert_eq!(patient.identifiers().len(), 0);
        assert_eq!(patient.names().len(), 0);
    }

    #[test]
    fn test_patient_with_data() {
        let mut patient = Patient::new();

        // Add identifier
        let system_uri = Uri::new_unchecked("https://hospital.example.com/patients".to_string());
        let use_uri = Uri::new_unchecked("official".to_string());
        let identifier = Identifier::new(use_uri, system_uri, "MRN12345".to_string());
        patient.add_identifier(identifier);

        assert_eq!(patient.identifiers().len(), 1);
        assert_eq!(patient.identifiers()[0].value(), "MRN12345");
    }
}
```

## Common Patterns

### 1. Building Complex Objects

```rust
fn build_patient_with_builder_pattern() -> Patient {
    let mut patient = Patient::new();

    // Add identifiers
    let identifiers = vec![
        ("https://hospital.example.com/patients", "official", "MRN12345"),
        ("https://ssa.gov/ssn", "official", "123-45-6789"),
    ];

    for (system, use_type, value) in identifiers {
        let system_uri = Uri::new_unchecked(system.to_string());
        let use_uri = Uri::new_unchecked(use_type.to_string());
        let identifier = Identifier::new(use_uri, system_uri, value.to_string());
        patient.add_identifier(identifier);
    }

    // Add names
    let names = vec![
        ("official", "Dr. John Smith", "Smith", vec!["John"], vec!["Dr."], vec!["MD"]),
        ("nickname", "Johnny", "Smith", vec!["Johnny"], vec![], vec![]),
    ];

    for (use_type, text, family, given, prefix, suffix) in names {
        let mut name = HumanName::new(use_type.to_string(), text.to_string(), family.to_string());
        name.set_given(given.into_iter().map(|s| s.to_string()).collect());
        name.set_prefix(prefix.into_iter().map(|s| s.to_string()).collect());
        name.set_suffix(suffix.into_iter().map(|s| s.to_string()).collect());
        patient.add_name(name);
    }

    patient
}
```

### 2. Iterating Over Resources

```rust
fn display_patient_info(patient: &Patient) {
    println!("Patient Information:");
    println!("Resource Type: {}", patient.resource_type());

    println!("\nIdentifiers:");
    for (i, identifier) in patient.identifiers().iter().enumerate() {
        println!("  {}. Type: {}, System: {}, Value: {}",
            i + 1,
            identifier.use_value().as_str(),
            identifier.system().as_str(),
            identifier.value()
        );
    }

    println!("\nNames:");
    for (i, name) in patient.names().iter().enumerate() {
        println!("  {}. Type: {}, Full Name: {}",
            i + 1,
            name.use_value(),
            name.text()
        );
    }
}
```

## Next Steps

1. **Explore the API Reference**: See `docs/api_reference.md` for detailed API documentation
2. **Check Examples**: Look at `docs/examples.md` for more usage examples
3. **Run Tests**: Execute the test suite to see the library in action
4. **Review Limitations**: Understand what's not implemented in the current version

## Troubleshooting

### Common Issues

1. **Compilation Errors**: Make sure you have the correct dependencies in `Cargo.toml`
2. **Serialization Errors**: Ensure all required fields are set before serializing
3. **Validation Errors**: Check that URIs and other values meet the required format

### Getting Help

- Check the test files in `tests/` for working examples
- Review the API reference documentation
- Look at the source code for implementation details

---

**⚠️ Remember**: This is a personal development project and should not be used in production healthcare systems without proper validation and testing.
