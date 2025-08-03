use fhir_resources_rs::patient::Patient;
use fhir_resources_rs::human_name::HumanName;
use fhir_resources_rs::identifier::Identifier;
use fhir_resources_rs::data_types::uri::Uri;

#[test]
fn test_patient_creation() {
    // Create a new patient
    let patient = Patient::new();
    
    // Verify default values
    assert_eq!(patient.resource_type(), "Patient");
    assert_eq!(patient.identifiers().len(), 0);
    assert_eq!(patient.names().len(), 0);
    
    println!("✅ Patient created successfully with default values");
    println!("   Resource Type: {}", patient.resource_type());
    println!("   Identifiers: {}", patient.identifiers().len());
    println!("   Names: {}", patient.names().len());
}

#[test]
fn test_patient_with_demo_data() {
    // Create URIs for identifiers
    let system_uri = Uri::new_unchecked("https://hospital.example.com/patients".to_string());
    let use_uri = Uri::new_unchecked("official".to_string());
    
    // Create an identifier
    let identifier = Identifier::new(use_uri, system_uri, "MRN12345".to_string());
    
    // Create a human name
    let mut human_name = HumanName::new(
        "official".to_string(),
        "Dr. John A. Smith".to_string(),
        "Smith".to_string()
    );
    
    // Add given names
    human_name.set_given(vec!["John".to_string(), "Andrew".to_string()]);
    
    // Add prefix and suffix
    human_name.set_prefix(vec!["Dr.".to_string()]);
    human_name.set_suffix(vec!["MD".to_string(), "PhD".to_string()]);
    
    // Create patient and add data
    let mut patient = Patient::new();
    patient.add_identifier(identifier);
    patient.add_name(human_name);
    
    // Verify the patient data
    assert_eq!(patient.resource_type(), "Patient");
    assert_eq!(patient.identifiers().len(), 1);
    assert_eq!(patient.names().len(), 1);
    
    // Verify identifier details
    let first_identifier = &patient.identifiers()[0];
    assert_eq!(first_identifier.use_value().as_str(), "official");
    assert_eq!(first_identifier.system().as_str(), "https://hospital.example.com/patients");
    assert_eq!(first_identifier.value(), "MRN12345");
    
    // Verify name details
    let first_name = &patient.names()[0];
    assert_eq!(first_name.use_value(), "official");
    assert_eq!(first_name.text(), "Dr. John A. Smith");
    assert_eq!(first_name.family(), "Smith");
    assert_eq!(first_name.given(), &["John", "Andrew"]);
    assert_eq!(first_name.prefix(), &["Dr."]);
    assert_eq!(first_name.suffix(), &["MD", "PhD"]);
    
    println!("✅ Patient with demo data created successfully!");
    println!("   Resource Type: {}", patient.resource_type());
    println!("   Identifiers: {}", patient.identifiers().len());
    println!("   Names: {}", patient.names().len());
    
    // Display detailed information
    println!("\n📋 Patient Details:");
    println!("   Resource Type: {}", patient.resource_type());
    
    println!("\n🆔 Identifiers:");
    for (i, identifier) in patient.identifiers().iter().enumerate() {
        println!("   {}. Use: {}, System: {}, Value: {}", 
            i + 1, 
            identifier.use_value().as_str(),
            identifier.system().as_str(),
            identifier.value()
        );
    }
    
    println!("\n👤 Names:");
    for (i, name) in patient.names().iter().enumerate() {
        println!("   {}. Use: {}", i + 1, name.use_value());
        println!("      Text: {}", name.text());
        println!("      Family: {}", name.family());
        println!("      Given: {}", name.given().join(", "));
        println!("      Prefix: {}", name.prefix().join(", "));
        println!("      Suffix: {}", name.suffix().join(", "));
    }
}

#[test]
fn test_patient_multiple_identifiers_and_names() {
    // Create multiple identifiers
    let mut patient = Patient::new();
    
    // Add first identifier (MRN)
    let mrn_system = Uri::new_unchecked("https://hospital.example.com/patients".to_string());
    let mrn_use = Uri::new_unchecked("official".to_string());
    let mrn_identifier = Identifier::new(mrn_use, mrn_system, "MRN12345".to_string());
    patient.add_identifier(mrn_identifier);
    
    // Add second identifier (SSN)
    let ssn_system = Uri::new_unchecked("https://ssa.gov/ssn".to_string());
    let ssn_use = Uri::new_unchecked("official".to_string());
    let ssn_identifier = Identifier::new(ssn_use, ssn_system, "123-45-6789".to_string());
    patient.add_identifier(ssn_identifier);
    
    // Add first name (official)
    let mut official_name = HumanName::new(
        "official".to_string(),
        "Dr. John Andrew Smith".to_string(),
        "Smith".to_string()
    );
    official_name.set_given(vec!["John".to_string(), "Andrew".to_string()]);
    official_name.set_prefix(vec!["Dr.".to_string()]);
    patient.add_name(official_name);
    
    // Add second name (nickname)
    let mut nickname = HumanName::new(
        "nickname".to_string(),
        "Johnny".to_string(),
        "Smith".to_string()
    );
    nickname.set_given(vec!["Johnny".to_string()]);
    patient.add_name(nickname);
    
    // Verify multiple identifiers and names
    assert_eq!(patient.identifiers().len(), 2);
    assert_eq!(patient.names().len(), 2);
    
    println!("✅ Patient with multiple identifiers and names created!");
    println!("   Total Identifiers: {}", patient.identifiers().len());
    println!("   Total Names: {}", patient.names().len());
    
    // Display all identifiers
    println!("\n🆔 All Identifiers:");
    for (i, identifier) in patient.identifiers().iter().enumerate() {
        println!("   {}. Type: {}, System: {}, Value: {}", 
            i + 1,
            identifier.use_value().as_str(),
            identifier.system().as_str(),
            identifier.value()
        );
    }
    
    // Display all names
    println!("\n👤 All Names:");
    for (i, name) in patient.names().iter().enumerate() {
        println!("   {}. Type: {}, Full Name: {}", 
            i + 1,
            name.use_value(),
            name.text()
        );
    }
}

#[test]
fn test_patient_serialization() {
    // Create a patient with demo data
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
    
    // Serialize to JSON
    let json = serde_json::to_string_pretty(&patient).unwrap();
    
    // Deserialize from JSON
    let deserialized_patient: Patient = serde_json::from_str(&json).unwrap();
    
    // Verify serialization/deserialization
    assert_eq!(patient, deserialized_patient);
    
    println!("✅ Patient serialization/deserialization successful!");
    println!("   JSON Output:");
    println!("{}", json);
}

#[test]
fn test_patient_real_world_scenario() {
    // Create a realistic patient scenario
    let mut patient = Patient::new();
    
    // Add multiple identifiers for different systems
    let identifiers = vec![
        ("https://hospital.example.com/patients", "official", "MRN12345"),
        ("https://ssa.gov/ssn", "official", "123-45-6789"),
        ("https://insurance.example.com/member", "secondary", "INS98765"),
    ];
    
    for (system, use_type, value) in identifiers {
        let system_uri = Uri::new_unchecked(system.to_string());
        let use_uri = Uri::new_unchecked(use_type.to_string());
        let identifier = Identifier::new(use_uri, system_uri, value.to_string());
        patient.add_identifier(identifier);
    }
    
    // Add multiple names for different purposes
    let names = vec![
        ("official", "Dr. John Andrew Smith", "Smith", vec!["John", "Andrew"], vec!["Dr."], vec!["MD", "PhD"]),
        ("nickname", "Johnny", "Smith", vec!["Johnny"], vec![], vec![]),
        ("maiden", "Jane Smith", "Smith", vec!["Jane"], vec![], vec![]),
    ];
    
    for (use_type, text, family, given, prefix, suffix) in names {
        let mut name = HumanName::new(use_type.to_string(), text.to_string(), family.to_string());
        name.set_given(given.into_iter().map(|s| s.to_string()).collect());
        name.set_prefix(prefix.into_iter().map(|s| s.to_string()).collect());
        name.set_suffix(suffix.into_iter().map(|s| s.to_string()).collect());
        patient.add_name(name);
    }
    
    // Verify the comprehensive patient data
    assert_eq!(patient.identifiers().len(), 3);
    assert_eq!(patient.names().len(), 3);
    
    println!("✅ Real-world patient scenario created successfully!");
    println!("   Resource Type: {}", patient.resource_type());
    println!("   Total Identifiers: {}", patient.identifiers().len());
    println!("   Total Names: {}", patient.names().len());
    
    // Display comprehensive patient information
    println!("\n🏥 Patient Information:");
    println!("   Resource Type: {}", patient.resource_type());
    
    println!("\n🆔 Patient Identifiers:");
    for (i, identifier) in patient.identifiers().iter().enumerate() {
        println!("   {}. Type: {}", i + 1, identifier.use_value().as_str());
        println!("      System: {}", identifier.system().as_str());
        println!("      Value: {}", identifier.value());
    }
    
    println!("\n👤 Patient Names:");
    for (i, name) in patient.names().iter().enumerate() {
        println!("   {}. Type: {}", i + 1, name.use_value());
        println!("      Full Name: {}", name.text());
        println!("      Family: {}", name.family());
        println!("      Given: {}", name.given().join(", "));
        if !name.prefix().is_empty() {
            println!("      Prefix: {}", name.prefix().join(", "));
        }
        if !name.suffix().is_empty() {
            println!("      Suffix: {}", name.suffix().join(", "));
        }
    }
} 