use fhir_resources_rs::identifier::Identifier;
use fhir_resources_rs::data_types::uri::Uri;
use fhir_resources_rs::period::Period;

#[test]
fn test_identifier_creation() {
    // Create a basic identifier
    let system_uri = Uri::new_unchecked("https://hospital.example.com/patients".to_string());
    let use_uri = Uri::new_unchecked("official".to_string());
    let identifier = Identifier::new(use_uri, system_uri, "MRN12345".to_string());
    
    // Verify basic values
    assert_eq!(identifier.use_value().as_str(), "official");
    assert_eq!(identifier.system().as_str(), "https://hospital.example.com/patients");
    assert_eq!(identifier.value(), "MRN12345");
    assert_eq!(identifier.period(), None);
    
    println!("✅ Identifier created successfully with basic data");
    println!("   Use: {}", identifier.use_value().as_str());
    println!("   System: {}", identifier.system().as_str());
    println!("   Value: {}", identifier.value());
}

#[test]
fn test_identifier_with_period() {
    // Create an identifier with a period
    let system_uri = Uri::new_unchecked("https://hospital.example.com/patients".to_string());
    let use_uri = Uri::new_unchecked("official".to_string());
    let mut identifier = Identifier::new(use_uri, system_uri, "MRN12345".to_string());
    
    // Add a period
    let period = Period::new("2020-01-01".to_string(), "2023-12-31".to_string());
    identifier.set_period(Some(period));
    
    // Verify with period
    assert_eq!(identifier.use_value().as_str(), "official");
    assert_eq!(identifier.system().as_str(), "https://hospital.example.com/patients");
    assert_eq!(identifier.value(), "MRN12345");
    assert!(identifier.period().is_some());
    
    let period_ref = identifier.period().unwrap();
    assert_eq!(period_ref.start(), "2020-01-01");
    assert_eq!(period_ref.end(), "2023-12-31");
    
    println!("✅ Identifier with period created successfully!");
    println!("   Use: {}", identifier.use_value().as_str());
    println!("   System: {}", identifier.system().as_str());
    println!("   Value: {}", identifier.value());
    println!("   Period: {} to {}", period_ref.start(), period_ref.end());
}

#[test]
fn test_identifier_different_use_types() {
    // Test different use types for identifiers
    let use_types = vec![
        ("official", "Primary identifier"),
        ("secondary", "Secondary identifier"),
        ("temp", "Temporary identifier"),
        ("old", "Old identifier"),
    ];
    
    for (use_type, description) in use_types {
        let system_uri = Uri::new_unchecked("https://example.com/system".to_string());
        let use_uri = Uri::new_unchecked(use_type.to_string());
        let identifier = Identifier::new(use_uri, system_uri, "ID123".to_string());
        
        assert_eq!(identifier.use_value().as_str(), use_type);
        
        println!("✅ {} identifier created: {}", use_type, description);
    }
}

#[test]
fn test_identifier_healthcare_systems() {
    // Test identifiers from different healthcare systems
    let healthcare_systems = vec![
        ("https://hospital.example.com/patients", "official", "MRN12345", "Hospital MRN"),
        ("https://ssa.gov/ssn", "official", "123-45-6789", "Social Security Number"),
        ("https://insurance.example.com/member", "secondary", "INS98765", "Insurance Member ID"),
        ("https://pharmacy.example.com/customer", "secondary", "PHARM123", "Pharmacy Customer ID"),
        ("https://lab.example.com/patient", "secondary", "LAB456", "Laboratory Patient ID"),
    ];
    
    for (system, use_type, value, description) in healthcare_systems {
        let system_uri = Uri::new_unchecked(system.to_string());
        let use_uri = Uri::new_unchecked(use_type.to_string());
        let identifier = Identifier::new(use_uri, system_uri, value.to_string());
        
        assert_eq!(identifier.system().as_str(), system);
        assert_eq!(identifier.use_value().as_str(), use_type);
        assert_eq!(identifier.value(), value);
        
        println!("✅ {}: {}", description, value);
        println!("   System: {}", system);
        println!("   Use: {}", use_type);
    }
}

#[test]
fn test_identifier_international_formats() {
    // Test international identifier formats
    let international_identifiers = vec![
        ("https://nhs.uk/patients", "official", "1234567890", "UK NHS Number"),
        ("https://medicare.gov/beneficiary", "official", "1-2345-6789-01", "US Medicare Number"),
        ("https://ramq.gouv.qc.ca/assure", "official", "ABCD12345678", "Quebec RAMQ"),
        ("https://australia.gov.au/medicare", "official", "1234567890", "Australian Medicare"),
        ("https://canada.ca/health", "official", "1234567890", "Canadian Health Card"),
    ];
    
    for (system, use_type, value, description) in international_identifiers {
        let system_uri = Uri::new_unchecked(system.to_string());
        let use_uri = Uri::new_unchecked(use_type.to_string());
        let identifier = Identifier::new(use_uri, system_uri, value.to_string());
        
        println!("✅ {}: {}", description, value);
        println!("   System: {}", system);
        println!("   Format: {}", use_type);
    }
}

#[test]
fn test_identifier_validation_scenarios() {
    // Test various identifier validation scenarios
    let validation_scenarios = vec![
        ("UUID", "urn:uuid:53fefa32-fcbb-4ff8-8a92-55ee120877b7", "urn:uuid:53fefa32-fcbb-4ff8-8a92-55ee120877b7", "UUID identifier"),
        ("Email", "https://email.example.com/user", "user@example.com", "Email-based identifier"),
        ("Phone", "https://phone.example.com/contact", "+1-555-123-4567", "Phone-based identifier"),
        ("Custom", "https://custom.example.com/id", "CUSTOM-123-ABC", "Custom format identifier"),
    ];
    
    for (format, system, value, description) in validation_scenarios {
        let system_uri = Uri::new_unchecked(system.to_string());
        let use_uri = Uri::new_unchecked("official".to_string());
        let identifier = Identifier::new(use_uri, system_uri, value.to_string());
        
        println!("✅ {}: {}", description, value);
        println!("   System: {}", system);
        println!("   Format: {}", format);
    }
}

#[test]
fn test_identifier_serialization() {
    // Create an identifier with full details
    let system_uri = Uri::new_unchecked("https://hospital.example.com/patients".to_string());
    let use_uri = Uri::new_unchecked("official".to_string());
    let mut identifier = Identifier::new(use_uri, system_uri, "MRN12345".to_string());
    
    // Add a period
    let period = Period::new("2020-01-01".to_string(), "2023-12-31".to_string());
    identifier.set_period(Some(period));
    
    // Serialize to JSON
    let json = serde_json::to_string_pretty(&identifier).unwrap();
    
    // Deserialize from JSON
    let deserialized_identifier: Identifier = serde_json::from_str(&json).unwrap();
    
    // Verify serialization/deserialization
    assert_eq!(identifier, deserialized_identifier);
    
    println!("✅ Identifier serialization/deserialization successful!");
    println!("   JSON Output:");
    println!("{}", json);
}

#[test]
fn test_identifier_edge_cases() {
    // Test edge cases for identifiers
    let edge_cases = vec![
        ("Long identifier", "https://example.com/long", "VERY-LONG-IDENTIFIER-WITH-MANY-CHARACTERS-AND-NUMBERS-1234567890", "Very long identifier value"),
        ("Special characters", "https://example.com/special", "ID-123_ABC@#$%", "Identifier with special characters"),
        ("Numeric only", "https://example.com/numeric", "1234567890", "Numeric identifier"),
        ("Alphanumeric", "https://example.com/alphanumeric", "ABC123DEF456", "Alphanumeric identifier"),
        ("Mixed case", "https://example.com/mixed", "AbC123dEf456", "Mixed case identifier"),
    ];
    
    for (description, system, value, details) in edge_cases {
        let system_uri = Uri::new_unchecked(system.to_string());
        let use_uri = Uri::new_unchecked("official".to_string());
        let identifier = Identifier::new(use_uri, system_uri, value.to_string());
        
        println!("✅ {}: {}", description, value);
        println!("   Details: {}", details);
        println!("   System: {}", system);
    }
}

#[test]
fn test_identifier_real_world_scenarios() {
    // Test real-world identifier scenarios
    let real_world_scenarios = vec![
        ("Patient Admission", "https://hospital.example.com/admission", "official", "ADM-2023-001", "Hospital admission ID"),
        ("Insurance Claim", "https://insurance.example.com/claim", "secondary", "CLM-2023-789", "Insurance claim number"),
        ("Laboratory Test", "https://lab.example.com/test", "secondary", "LAB-2023-456", "Laboratory test ID"),
        ("Prescription", "https://pharmacy.example.com/rx", "secondary", "RX-2023-123", "Prescription number"),
        ("Emergency Contact", "https://emergency.example.com/contact", "temp", "EMG-2023-999", "Emergency contact ID"),
    ];
    
    for (scenario, system, use_type, value, description) in real_world_scenarios {
        let system_uri = Uri::new_unchecked(system.to_string());
        let use_uri = Uri::new_unchecked(use_type.to_string());
        let identifier = Identifier::new(use_uri, system_uri, value.to_string());
        
        println!("✅ {}: {}", scenario, description);
        println!("   Value: {}", value);
        println!("   System: {}", system);
        println!("   Use: {}", use_type);
    }
} 