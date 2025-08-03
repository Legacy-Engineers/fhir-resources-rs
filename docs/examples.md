# Examples

> **⚠️ IMPORTANT**: This is a personal project and is NOT ready for production use.

## Table of Contents

- [Basic Examples](#basic-examples)
- [Healthcare Scenarios](#healthcare-scenarios)
- [International Examples](#international-examples)
- [Advanced Patterns](#advanced-patterns)
- [Error Handling](#error-handling)
- [Serialization Examples](#serialization-examples)

## Basic Examples

### Creating a Simple Patient

```rust
use fhir_resources_rs::patient::Patient;
use fhir_resources_rs::human_name::HumanName;
use fhir_resources_rs::identifier::Identifier;
use fhir_resources_rs::data_types::uri::Uri;

fn create_simple_patient() -> Patient {
    let mut patient = Patient::new();
    
    // Add identifier
    let system_uri = Uri::new_unchecked("https://hospital.example.com/patients".to_string());
    let use_uri = Uri::new_unchecked("official".to_string());
    let identifier = Identifier::new(use_uri, system_uri, "MRN12345".to_string());
    patient.add_identifier(identifier);
    
    // Add name
    let mut name = HumanName::new(
        "official".to_string(),
        "John Smith".to_string(),
        "Smith".to_string()
    );
    name.set_given(vec!["John".to_string()]);
    patient.add_name(name);
    
    patient
}
```

### Working with Human Names

```rust
use fhir_resources_rs::human_name::HumanName;

fn create_professional_name() -> HumanName {
    let mut name = HumanName::new(
        "official".to_string(),
        "Dr. Sarah Johnson, MD, PhD".to_string(),
        "Johnson".to_string()
    );
    
    name.set_given(vec!["Sarah".to_string()]);
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

### Creating Multiple Identifiers

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

## Healthcare Scenarios

### Hospital Admission Patient

```rust
fn create_hospital_admission_patient() -> Patient {
    let mut patient = Patient::new();
    
    // Add hospital identifiers
    let identifiers = vec![
        ("https://hospital.example.com/patients", "official", "ADM-2023-001"),
        ("https://hospital.example.com/admission", "secondary", "ADM-2023-001"),
        ("https://ssa.gov/ssn", "official", "123-45-6789"),
    ];
    
    for (system, use_type, value) in identifiers {
        let system_uri = Uri::new_unchecked(system.to_string());
        let use_uri = Uri::new_unchecked(use_type.to_string());
        let identifier = Identifier::new(use_uri, system_uri, value.to_string());
        patient.add_identifier(identifier);
    }
    
    // Add official name
    let mut official_name = HumanName::new(
        "official".to_string(),
        "Dr. John Andrew Smith, MD".to_string(),
        "Smith".to_string()
    );
    official_name.set_given(vec!["John".to_string(), "Andrew".to_string()]);
    official_name.set_prefix(vec!["Dr.".to_string()]);
    official_name.set_suffix(vec!["MD".to_string()]);
    patient.add_name(official_name);
    
    // Add nickname for informal use
    let mut nickname = HumanName::new(
        "nickname".to_string(),
        "Johnny".to_string(),
        "Smith".to_string()
    );
    nickname.set_given(vec!["Johnny".to_string()]);
    patient.add_name(nickname);
    
    patient
}
```

### Insurance Claim Patient

```rust
fn create_insurance_claim_patient() -> Patient {
    let mut patient = Patient::new();
    
    // Add insurance-related identifiers
    let identifiers = vec![
        ("https://insurance.example.com/member", "official", "INS98765"),
        ("https://insurance.example.com/claim", "secondary", "CLM-2023-789"),
        ("https://ssa.gov/ssn", "official", "123-45-6789"),
    ];
    
    for (system, use_type, value) in identifiers {
        let system_uri = Uri::new_unchecked(system.to_string());
        let use_uri = Uri::new_unchecked(use_type.to_string());
        let identifier = Identifier::new(use_uri, system_uri, value.to_string());
        patient.add_identifier(identifier);
    }
    
    // Add official name
    let mut name = HumanName::new(
        "official".to_string(),
        "Sarah Johnson".to_string(),
        "Johnson".to_string()
    );
    name.set_given(vec!["Sarah".to_string()]);
    patient.add_name(name);
    
    patient
}
```

### Laboratory Test Patient

```rust
fn create_laboratory_patient() -> Patient {
    let mut patient = Patient::new();
    
    // Add lab-related identifiers
    let identifiers = vec![
        ("https://lab.example.com/patient", "official", "LAB456"),
        ("https://lab.example.com/test", "secondary", "TEST-2023-456"),
        ("https://hospital.example.com/patients", "secondary", "MRN12345"),
    ];
    
    for (system, use_type, value) in identifiers {
        let system_uri = Uri::new_unchecked(system.to_string());
        let use_uri = Uri::new_unchecked(use_type.to_string());
        let identifier = Identifier::new(use_uri, system_uri, value.to_string());
        patient.add_identifier(identifier);
    }
    
    // Add name
    let mut name = HumanName::new(
        "official".to_string(),
        "Michael Chen".to_string(),
        "Chen".to_string()
    );
    name.set_given(vec!["Michael".to_string()]);
    patient.add_name(name);
    
    patient
}
```

## International Examples

### Chinese Patient

```rust
fn create_chinese_patient() -> Patient {
    let mut patient = Patient::new();
    
    // Add identifiers
    let system_uri = Uri::new_unchecked("https://hospital.example.com/patients".to_string());
    let use_uri = Uri::new_unchecked("official".to_string());
    let identifier = Identifier::new(use_uri, system_uri, "MRN12345".to_string());
    patient.add_identifier(identifier);
    
    // Add Chinese name
    let mut name = HumanName::new(
        "official".to_string(),
        "Li Wei".to_string(),
        "Li".to_string()
    );
    name.set_given(vec!["Wei".to_string()]);
    patient.add_name(name);
    
    patient
}
```

### Japanese Patient

```rust
fn create_japanese_patient() -> Patient {
    let mut patient = Patient::new();
    
    // Add identifiers
    let system_uri = Uri::new_unchecked("https://hospital.example.com/patients".to_string());
    let use_uri = Uri::new_unchecked("official".to_string());
    let identifier = Identifier::new(use_uri, system_uri, "MRN12345".to_string());
    patient.add_identifier(identifier);
    
    // Add Japanese name
    let mut name = HumanName::new(
        "official".to_string(),
        "Tanaka Yuki".to_string(),
        "Tanaka".to_string()
    );
    name.set_given(vec!["Yuki".to_string()]);
    patient.add_name(name);
    
    patient
}
```

### Spanish Patient

```rust
fn create_spanish_patient() -> Patient {
    let mut patient = Patient::new();
    
    // Add identifiers
    let system_uri = Uri::new_unchecked("https://hospital.example.com/patients".to_string());
    let use_uri = Uri::new_unchecked("official".to_string());
    let identifier = Identifier::new(use_uri, system_uri, "MRN12345".to_string());
    patient.add_identifier(identifier);
    
    // Add Spanish name
    let mut name = HumanName::new(
        "official".to_string(),
        "María José García".to_string(),
        "García".to_string()
    );
    name.set_given(vec!["María".to_string(), "José".to_string()]);
    patient.add_name(name);
    
    patient
}
```

## Advanced Patterns

### Builder Pattern for Complex Patients

```rust
struct PatientBuilder {
    patient: Patient,
}

impl PatientBuilder {
    fn new() -> Self {
        Self {
            patient: Patient::new(),
        }
    }
    
    fn with_identifier(mut self, system: &str, use_type: &str, value: &str) -> Self {
        let system_uri = Uri::new_unchecked(system.to_string());
        let use_uri = Uri::new_unchecked(use_type.to_string());
        let identifier = Identifier::new(use_uri, system_uri, value.to_string());
        self.patient.add_identifier(identifier);
        self
    }
    
    fn with_name(mut self, use_type: &str, text: &str, family: &str, given: Vec<&str>) -> Self {
        let mut name = HumanName::new(
            use_type.to_string(),
            text.to_string(),
            family.to_string()
        );
        name.set_given(given.into_iter().map(|s| s.to_string()).collect());
        self.patient.add_name(name);
        self
    }
    
    fn with_professional_name(mut self, use_type: &str, text: &str, family: &str, 
                            given: Vec<&str>, prefix: Vec<&str>, suffix: Vec<&str>) -> Self {
        let mut name = HumanName::new(
            use_type.to_string(),
            text.to_string(),
            family.to_string()
        );
        name.set_given(given.into_iter().map(|s| s.to_string()).collect());
        name.set_prefix(prefix.into_iter().map(|s| s.to_string()).collect());
        name.set_suffix(suffix.into_iter().map(|s| s.to_string()).collect());
        self.patient.add_name(name);
        self
    }
    
    fn build(self) -> Patient {
        self.patient
    }
}

fn create_complex_patient_with_builder() -> Patient {
    PatientBuilder::new()
        .with_identifier("https://hospital.example.com/patients", "official", "MRN12345")
        .with_identifier("https://ssa.gov/ssn", "official", "123-45-6789")
        .with_professional_name(
            "official",
            "Dr. John Andrew Smith, MD, PhD",
            "Smith",
            vec!["John", "Andrew"],
            vec!["Dr."],
            vec!["MD", "PhD"]
        )
        .with_name("nickname", "Johnny", "Smith", vec!["Johnny"])
        .build()
}
```

### Factory Pattern for Different Patient Types

```rust
enum PatientType {
    Hospital,
    Insurance,
    Laboratory,
    International(String), // country code
}

fn create_patient_by_type(patient_type: PatientType) -> Patient {
    match patient_type {
        PatientType::Hospital => create_hospital_admission_patient(),
        PatientType::Insurance => create_insurance_claim_patient(),
        PatientType::Laboratory => create_laboratory_patient(),
        PatientType::International(country) => match country.as_str() {
            "CN" => create_chinese_patient(),
            "JP" => create_japanese_patient(),
            "ES" => create_spanish_patient(),
            _ => create_simple_patient(),
        }
    }
}
```

## Error Handling

### Validation Examples

```rust
use fhir_resources_rs::data_types::uri::Uri;
use fhir_resources_rs::data_types::code::Code;

fn validate_uri_examples() {
    // Valid URI
    match Uri::new("https://hospital.example.com/patients".to_string()) {
        Ok(uri) => println!("✅ Valid URI: {}", uri.as_str()),
        Err(e) => println!("❌ Invalid URI: {}", e),
    }
    
    // Invalid URI (empty)
    match Uri::new("".to_string()) {
        Ok(uri) => println!("✅ Valid URI: {}", uri.as_str()),
        Err(e) => println!("❌ Invalid URI: {}", e),
    }
    
    // UUID URI (must be lowercase)
    match Uri::new("urn:uuid:53fefa32-fcbb-4ff8-8a92-55ee120877b7".to_string()) {
        Ok(uri) => println!("✅ Valid UUID URI: {}", uri.as_str()),
        Err(e) => println!("❌ Invalid UUID URI: {}", e),
    }
}

fn validate_code_examples() {
    // Valid code
    match Code::new("active".to_string()) {
        Ok(code) => println!("✅ Valid code: {}", code.as_str()),
        Err(e) => println!("❌ Invalid code: {}", e),
    }
    
    // Invalid code (leading whitespace)
    match Code::new(" active".to_string()) {
        Ok(code) => println!("✅ Valid code: {}", code.as_str()),
        Err(e) => println!("❌ Invalid code: {}", e),
    }
    
    // Multi-token code
    match Code::new("active status".to_string()) {
        Ok(code) => {
            println!("✅ Valid multi-token code: {}", code.as_str());
            println!("   Tokens: {:?}", code.tokens());
            println!("   Token count: {}", code.token_count());
        }
        Err(e) => println!("❌ Invalid code: {}", e),
    }
}
```

### Graceful Error Handling

```rust
fn create_patient_with_validation(identifiers: Vec<(&str, &str, &str)>) -> Result<Patient, String> {
    let mut patient = Patient::new();
    
    for (system, use_type, value) in identifiers {
        // Validate URI
        let system_uri = match Uri::new(system.to_string()) {
            Ok(uri) => uri,
            Err(e) => return Err(format!("Invalid system URI '{}': {}", system, e)),
        };
        
        let use_uri = match Uri::new(use_type.to_string()) {
            Ok(uri) => uri,
            Err(e) => return Err(format!("Invalid use URI '{}': {}", use_type, e)),
        };
        
        let identifier = Identifier::new(use_uri, system_uri, value.to_string());
        patient.add_identifier(identifier);
    }
    
    Ok(patient)
}

fn example_validation_usage() {
    let valid_identifiers = vec![
        ("https://hospital.example.com/patients", "official", "MRN12345"),
        ("https://ssa.gov/ssn", "official", "123-45-6789"),
    ];
    
    match create_patient_with_validation(valid_identifiers) {
        Ok(patient) => println!("✅ Patient created successfully with {} identifiers", 
                                patient.identifiers().len()),
        Err(e) => println!("❌ Failed to create patient: {}", e),
    }
    
    let invalid_identifiers = vec![
        ("", "official", "MRN12345"), // Invalid empty URI
    ];
    
    match create_patient_with_validation(invalid_identifiers) {
        Ok(patient) => println!("✅ Patient created successfully"),
        Err(e) => println!("❌ Failed to create patient: {}", e),
    }
}
```

## Serialization Examples

### Basic Serialization

```rust
use serde_json;

fn serialize_patient_example() {
    let patient = create_simple_patient();
    
    // Serialize to JSON
    match serde_json::to_string_pretty(&patient) {
        Ok(json) => {
            println!("✅ Serialized patient:");
            println!("{}", json);
        }
        Err(e) => println!("❌ Serialization failed: {}", e),
    }
}
```

### Round-trip Serialization

```rust
fn round_trip_serialization_example() {
    let original_patient = create_complex_patient_with_builder();
    
    // Serialize
    let json = match serde_json::to_string(&original_patient) {
        Ok(json) => json,
        Err(e) => {
            println!("❌ Serialization failed: {}", e);
            return;
        }
    };
    
    // Deserialize
    let deserialized_patient: Patient = match serde_json::from_str(&json) {
        Ok(patient) => patient,
        Err(e) => {
            println!("❌ Deserialization failed: {}", e);
            return;
        }
    };
    
    // Verify round-trip
    if original_patient == deserialized_patient {
        println!("✅ Round-trip serialization successful!");
    } else {
        println!("❌ Round-trip serialization failed - data mismatch");
    }
}
```

### Batch Processing

```rust
fn process_multiple_patients() {
    let patients = vec![
        create_simple_patient(),
        create_hospital_admission_patient(),
        create_insurance_claim_patient(),
        create_laboratory_patient(),
    ];
    
    let mut json_results = Vec::new();
    
    for (i, patient) in patients.iter().enumerate() {
        match serde_json::to_string_pretty(patient) {
            Ok(json) => {
                json_results.push(json);
                println!("✅ Patient {} serialized successfully", i + 1);
            }
            Err(e) => println!("❌ Failed to serialize patient {}: {}", i + 1, e),
        }
    }
    
    println!("Processed {} patients, {} successful", patients.len(), json_results.len());
}
```

### Custom Serialization Format

```rust
fn serialize_with_custom_format(patient: &Patient) -> String {
    let mut output = String::new();
    
    output.push_str(&format!("Patient: {}\n", patient.resource_type()));
    output.push_str("Identifiers:\n");
    
    for (i, identifier) in patient.identifiers().iter().enumerate() {
        output.push_str(&format!("  {}. {}: {} ({})\n", 
            i + 1,
            identifier.use_value().as_str(),
            identifier.value(),
            identifier.system().as_str()
        ));
    }
    
    output.push_str("Names:\n");
    for (i, name) in patient.names().iter().enumerate() {
        output.push_str(&format!("  {}. {}: {}\n", 
            i + 1,
            name.use_value(),
            name.text()
        ));
    }
    
    output
}

fn example_custom_format() {
    let patient = create_complex_patient_with_builder();
    let formatted = serialize_with_custom_format(&patient);
    println!("{}", formatted);
}
```

---

**⚠️ Remember**: This is a personal development project and should not be used in production healthcare systems without proper validation and testing. 