# FHIR Resources Rust Library

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/Status-Personal%20Project-red.svg)]()

> **⚠️ IMPORTANT DISCLAIMER**  
> This is a **personal project** and is **NOT ready for production use**. This library is being developed for educational purposes and personal healthcare applications. Do not use this in production healthcare systems without thorough testing and validation.

## Overview

A Rust library for working with FHIR (Fast Healthcare Interoperability Resources) data structures. This library provides type-safe implementations of common FHIR resources and data types used in healthcare applications.

## 🚨 Project Status

- **Status**: Personal Development Project
- **Production Ready**: ❌ No - Not suitable for production use
- **Testing**: ✅ Comprehensive test suite included
- **Documentation**: ✅ Basic documentation available
- **FHIR Compliance**: 🔄 Partial - Not fully FHIR compliant

## Features

### ✅ Implemented Resources

- **Patient**: Core patient resource with identifiers and names
- **HumanName**: Structured human names with international support
- **Identifier**: Healthcare identifiers (MRN, SSN, etc.)
- **Period**: Time periods for healthcare events
- **Uri**: FHIR-compliant URI implementation
- **Code**: FHIR code values with validation

### 🌍 International Support

- Multiple name formats (Chinese, Japanese, Korean, Spanish, Arabic)
- Professional titles and credentials
- International identifier formats (NHS, Medicare, etc.)
- Various date/time formats

### 🔧 Technical Features

- **Type Safety**: Strongly typed Rust implementations
- **Serialization**: JSON serialization/deserialization support
- **Validation**: Basic FHIR validation rules
- **Testing**: Comprehensive test suite with demo data
- **Documentation**: Detailed usage examples

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fhir-resources-rs = "0.1.0"
```

## Quick Start

```rust
use fhir_resources_rs::patient::Patient;
use fhir_resources_rs::human_name::HumanName;
use fhir_resources_rs::identifier::Identifier;
use fhir_resources_rs::data_types::uri::Uri;

// Create a patient
let mut patient = Patient::new();

// Add an identifier
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
```

## Running Tests

```bash
# Run all tests
cargo test

# Run specific test files
cargo test --test patient_tests -- --nocapture
cargo test --test human_name_tests -- --nocapture
cargo test --test period_tests -- --nocapture
cargo test --test identifier_tests -- --nocapture
```

## Project Structure

```
fhir-resources-rs/
├── src/
│   ├── lib.rs              # Main library entry point
│   ├── patient.rs          # Patient resource implementation
│   ├── human_name.rs       # HumanName implementation
│   ├── identifier.rs       # Identifier implementation
│   ├── period.rs           # Period implementation
│   └── data_types/
│       ├── mod.rs          # Data types module
│       ├── uri.rs          # URI implementation
│       └── code.rs         # Code implementation
├── tests/                  # Integration tests
│   ├── patient_tests.rs    # Patient resource tests
│   ├── human_name_tests.rs # HumanName tests
│   ├── period_tests.rs     # Period tests
│   └── identifier_tests.rs # Identifier tests
└── docs/                   # Documentation
    ├── getting_started.md  # Getting started guide
    ├── api_reference.md    # API reference
    └── examples.md         # Usage examples
```

## Limitations

### ❌ Current Limitations

- **Not FHIR Compliant**: This is not a complete FHIR implementation
- **Limited Resources**: Only basic resources implemented
- **No Validation**: Limited FHIR validation rules
- **No Extensions**: FHIR extensions not supported
- **No References**: Resource references not implemented
- **No Search**: No search or query capabilities
- **No Server**: No FHIR server implementation

### 🔄 Planned Features

- [ ] Complete FHIR R4 compliance
- [ ] More resource types (Observation, Medication, etc.)
- [ ] FHIR validation rules
- [ ] Resource references
- [ ] FHIR extensions support
- [ ] Search and query capabilities
- [ ] FHIR server implementation

## Contributing

This is a personal project, but suggestions and feedback are welcome. Please note that this is not intended for production use.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Disclaimer

This library is provided "as is" without warranty of any kind. It is not intended for use in production healthcare systems. Always validate healthcare data according to your organization's requirements and applicable regulations.

## Author

This is a personal project developed for educational purposes and personal healthcare applications.

---

**⚠️ Remember**: This is a personal development project and should not be used in production healthcare systems without proper validation and testing.
