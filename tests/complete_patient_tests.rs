use fhir_resources_rs::patient::Patient;
use fhir_resources_rs::human_name::HumanName;
use fhir_resources_rs::identifier::Identifier;
use fhir_resources_rs::data_types::uri::Uri;
use fhir_resources_rs::data_types::contact_point::ContactPoint;
use fhir_resources_rs::data_types::address::Address;
use fhir_resources_rs::data_types::codeable_concept::{CodeableConcept, Coding};
use fhir_resources_rs::data_types::reference::Reference;
use fhir_resources_rs::patient_contact::PatientContact;
use fhir_resources_rs::patient_communication::PatientCommunication;
use fhir_resources_rs::patient_link::PatientLink;

#[test]
fn test_complete_patient_creation() {
    let mut patient = Patient::new();
    
    // Verify default values
    assert_eq!(patient.resource_type(), "Patient");
    assert_eq!(patient.identifiers().len(), 0);
    assert_eq!(patient.names().len(), 0);
    assert_eq!(patient.active(), None);
    assert_eq!(patient.telecom().len(), 0);
    assert_eq!(patient.gender(), None);
    assert_eq!(patient.birth_date(), None);
    assert_eq!(patient.address().len(), 0);
    assert_eq!(patient.contact().len(), 0);
    assert_eq!(patient.communication().len(), 0);
    assert_eq!(patient.general_practitioner().len(), 0);
    assert_eq!(patient.link().len(), 0);
    
    println!("✅ Complete Patient created successfully with default values");
    println!("   Resource Type: {}", patient.resource_type());
    println!("   Active: {:?}", patient.active());
    println!("   Gender: {:?}", patient.gender());
    println!("   Birth Date: {:?}", patient.birth_date());
}

#[test]
fn test_complete_patient_with_all_fields() {
    let mut patient = Patient::new();
    
    // Add identifiers
    let system_uri = Uri::new_unchecked("https://hospital.example.com/patients".to_string());
    let use_uri = Uri::new_unchecked("official".to_string());
    let identifier = Identifier::new(use_uri, system_uri, "MRN12345".to_string());
    patient.add_identifier(identifier);
    
    // Set active status
    patient.set_active(Some(true));
    
    // Add names
    let mut name = HumanName::new(
        "official".to_string(),
        "Dr. John Andrew Smith".to_string(),
        "Smith".to_string()
    );
    name.set_given(vec!["John".to_string(), "Andrew".to_string()]);
    name.set_prefix(vec!["Dr.".to_string()]);
    name.set_suffix(vec!["MD".to_string()]);
    patient.add_name(name);
    
    // Add telecom
    let phone = ContactPoint::new("phone".to_string(), "+1-555-123-4567".to_string());
    patient.add_telecom(phone);
    
    let email = ContactPoint::new("email".to_string(), "john.smith@example.com".to_string());
    patient.add_telecom(email);
    
    // Set gender
    patient.set_gender(Some("male".to_string()));
    
    // Set birth date
    patient.set_birth_date(Some("1980-05-15".to_string()));
    
    // Set deceased status
    patient.set_deceased_boolean(Some(false));
    
    // Add addresses
    let mut home_address = Address::new();
    home_address.set_use(Some("home".to_string()));
    home_address.add_line("123 Main Street".to_string());
    home_address.set_city(Some("Anytown".to_string()));
    home_address.set_state(Some("CA".to_string()));
    home_address.set_postal_code(Some("12345".to_string()));
    home_address.set_country(Some("US".to_string()));
    patient.add_address(home_address);
    
    // Set marital status
    let mut marital_coding = Coding::with_system_and_code(
        "http://terminology.hl7.org/CodeSystem/v3-MaritalStatus".to_string(),
        "M".to_string()
    );
    marital_coding.set_display(Some("Married".to_string()));
    let marital_status = CodeableConcept::with_coding(marital_coding);
    patient.set_marital_status(Some(marital_status));
    
    // Set multiple birth
    patient.set_multiple_birth_boolean(Some(false));
    
    // Add photos
    patient.add_photo("data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAAAQ...".to_string());
    
    // Add contacts
    let mut contact = PatientContact::new();
    let mut contact_name = HumanName::new(
        "official".to_string(),
        "Jane Smith".to_string(),
        "Smith".to_string()
    );
    contact_name.set_given(vec!["Jane".to_string()]);
    contact.set_name(Some(contact_name));
    contact.set_gender(Some("female".to_string()));
    
    let contact_phone = ContactPoint::new("phone".to_string(), "+1-555-987-6543".to_string());
    contact.add_telecom(contact_phone);
    patient.add_contact(contact);
    
    // Add communication preferences
    let mut language_coding = Coding::with_system_and_code(
        "urn:ietf:bcp:47".to_string(),
        "en".to_string()
    );
    language_coding.set_display(Some("English".to_string()));
    let language = CodeableConcept::with_coding(language_coding);
    let communication = PatientCommunication::with_preferred(language, true);
    patient.add_communication(communication);
    
    // Add general practitioner
    let practitioner_ref = Reference::with_reference("Practitioner/123".to_string());
    patient.add_general_practitioner(practitioner_ref);
    
    // Set managing organization
    let org_ref = Reference::with_reference("Organization/456".to_string());
    patient.set_managing_organization(Some(org_ref));
    
    // Add links
    let other_patient_ref = Reference::with_reference("Patient/789".to_string());
    let link = PatientLink::new(other_patient_ref, "seealso".to_string());
    patient.add_link(link);
    
    // Verify all fields
    assert_eq!(patient.resource_type(), "Patient");
    assert_eq!(patient.identifiers().len(), 1);
    assert_eq!(patient.active(), Some(true));
    assert_eq!(patient.names().len(), 1);
    assert_eq!(patient.telecom().len(), 2);
    assert_eq!(patient.gender(), Some("male"));
    assert_eq!(patient.birth_date(), Some("1980-05-15"));
    assert_eq!(patient.deceased_boolean(), Some(false));
    assert_eq!(patient.address().len(), 1);
    assert_eq!(patient.contact().len(), 1);
    assert_eq!(patient.communication().len(), 1);
    assert_eq!(patient.general_practitioner().len(), 1);
    assert_eq!(patient.link().len(), 1);
    
    println!("✅ Complete Patient with all fields created successfully!");
    println!("   Resource Type: {}", patient.resource_type());
    println!("   Active: {:?}", patient.active());
    println!("   Gender: {:?}", patient.gender());
    println!("   Birth Date: {:?}", patient.birth_date());
    println!("   Identifiers: {}", patient.identifiers().len());
    println!("   Names: {}", patient.names().len());
    println!("   Telecom: {}", patient.telecom().len());
    println!("   Addresses: {}", patient.address().len());
    println!("   Contacts: {}", patient.contact().len());
    println!("   Communication: {}", patient.communication().len());
    println!("   General Practitioners: {}", patient.general_practitioner().len());
    println!("   Links: {}", patient.link().len());
}

#[test]
fn test_complete_patient_serialization() {
    let mut patient = Patient::new();
    
    // Add basic data
    let system_uri = Uri::new_unchecked("https://hospital.example.com/patients".to_string());
    let use_uri = Uri::new_unchecked("official".to_string());
    let identifier = Identifier::new(use_uri, system_uri, "MRN12345".to_string());
    patient.add_identifier(identifier);
    
    patient.set_active(Some(true));
    patient.set_gender(Some("male".to_string()));
    patient.set_birth_date(Some("1980-05-15".to_string()));
    
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
    
    println!("✅ Complete Patient serialization/deserialization successful!");
    println!("   JSON Output:");
    println!("{}", json);
}

#[test]
fn test_patient_with_complex_data() {
    let mut patient = Patient::new();
    
    // Add multiple identifiers
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
    
    // Add multiple names
    let names = vec![
        ("official", "Dr. John Andrew Smith, MD", "Smith", vec!["John", "Andrew"], vec!["Dr."], vec!["MD"]),
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
    
    // Add multiple telecom
    let telecom_data = vec![
        ("phone", "+1-555-123-4567", "home"),
        ("email", "john.smith@example.com", "work"),
        ("phone", "+1-555-987-6543", "mobile"),
    ];
    
    for (system, value, use_type) in telecom_data {
        let mut telecom = ContactPoint::new(system.to_string(), value.to_string());
        telecom.set_use(Some(use_type.to_string()));
        patient.add_telecom(telecom);
    }
    
    // Add multiple addresses
    let addresses = vec![
        ("home", vec!["123 Main Street"], "Anytown", "CA", "12345", "US"),
        ("work", vec!["456 Business Ave", "Suite 100"], "Worktown", "CA", "54321", "US"),
    ];
    
    for (use_type, lines, city, state, postal_code, country) in addresses {
        let mut address = Address::new();
        address.set_use(Some(use_type.to_string()));
        for line in lines {
            address.add_line(line.to_string());
        }
        address.set_city(Some(city.to_string()));
        address.set_state(Some(state.to_string()));
        address.set_postal_code(Some(postal_code.to_string()));
        address.set_country(Some(country.to_string()));
        patient.add_address(address);
    }
    
    // Add multiple contacts
    let contacts = vec![
        ("Spouse", "Jane Smith", "female"),
        ("Emergency Contact", "Bob Johnson", "male"),
    ];
    
    for (relationship, name_text, gender) in contacts {
        let mut contact = PatientContact::new();
        
        // Add relationship
        let mut relationship_coding = Coding::with_system_and_code(
            "http://terminology.hl7.org/CodeSystem/v2-0131".to_string(),
            relationship.to_string()
        );
        relationship_coding.set_display(Some(relationship.to_string()));
        let relationship_concept = CodeableConcept::with_coding(relationship_coding);
        contact.add_relationship(relationship_concept);
        
        // Add name
        let name_parts: Vec<&str> = name_text.split_whitespace().collect();
        let family_name = name_parts.last().unwrap_or(&"");
        let given_names = name_parts[..name_parts.len()-1].to_vec();
        
        let mut contact_name = HumanName::new(
            "official".to_string(),
            name_text.to_string(),
            family_name.to_string()
        );
        contact_name.set_given(given_names.into_iter().map(|s| s.to_string()).collect());
        contact.set_name(Some(contact_name));
        
        contact.set_gender(Some(gender.to_string()));
        patient.add_contact(contact);
    }
    
    // Add multiple communication preferences
    let languages = vec![
        ("en", "English", true),
        ("es", "Spanish", false),
    ];
    
    for (code, display, preferred) in languages {
        let mut language_coding = Coding::with_system_and_code(
            "urn:ietf:bcp:47".to_string(),
            code.to_string()
        );
        language_coding.set_display(Some(display.to_string()));
        let language = CodeableConcept::with_coding(language_coding);
        let communication = PatientCommunication::with_preferred(language, preferred);
        patient.add_communication(communication);
    }
    
    // Verify complex data
    assert_eq!(patient.identifiers().len(), 3);
    assert_eq!(patient.names().len(), 3);
    assert_eq!(patient.telecom().len(), 3);
    assert_eq!(patient.address().len(), 2);
    assert_eq!(patient.contact().len(), 2);
    assert_eq!(patient.communication().len(), 2);
    
    println!("✅ Patient with complex data created successfully!");
    println!("   Identifiers: {}", patient.identifiers().len());
    println!("   Names: {}", patient.names().len());
    println!("   Telecom: {}", patient.telecom().len());
    println!("   Addresses: {}", patient.address().len());
    println!("   Contacts: {}", patient.contact().len());
    println!("   Communication: {}", patient.communication().len());
    
    // Display detailed information
    println!("\n📋 Complete Patient Details:");
    println!("   Resource Type: {}", patient.resource_type());
    println!("   Active: {:?}", patient.active());
    println!("   Gender: {:?}", patient.gender());
    println!("   Birth Date: {:?}", patient.birth_date());
    
    println!("\n🆔 Identifiers:");
    for (i, identifier) in patient.identifiers().iter().enumerate() {
        println!("   {}. {}: {} ({})", 
            i + 1,
            identifier.use_value().as_str(),
            identifier.value(),
            identifier.system().as_str()
        );
    }
    
    println!("\n👤 Names:");
    for (i, name) in patient.names().iter().enumerate() {
        println!("   {}. {}: {}", 
            i + 1,
            name.use_value(),
            name.text()
        );
    }
    
    println!("\n📞 Telecom:");
    for (i, telecom) in patient.telecom().iter().enumerate() {
        println!("   {}. {}: {} ({})", 
            i + 1,
            telecom.system(),
            telecom.value(),
            telecom.use_value().unwrap_or("unknown")
        );
    }
    
    println!("\n🏠 Addresses:");
    for (i, address) in patient.address().iter().enumerate() {
        println!("   {}. {}: {}", 
            i + 1,
            address.use_value().unwrap_or("unknown"),
            address.text().unwrap_or("No text")
        );
    }
} 