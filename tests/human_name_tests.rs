use fhir_resources_rs::human_name::HumanName;

#[test]
fn test_human_name_creation() {
    // Create a basic human name
    let name = HumanName::new(
        "official".to_string(),
        "John Smith".to_string(),
        "Smith".to_string()
    );
    
    // Verify default values
    assert_eq!(name.use_value(), "official");
    assert_eq!(name.text(), "John Smith");
    assert_eq!(name.family(), "Smith");
    assert_eq!(name.given().len(), 0);
    assert_eq!(name.prefix().len(), 0);
    assert_eq!(name.suffix().len(), 0);
    assert_eq!(name.period(), None);
    
    println!("✅ HumanName created successfully with basic data");
    println!("   Use: {}", name.use_value());
    println!("   Text: {}", name.text());
    println!("   Family: {}", name.family());
}

#[test]
fn test_human_name_with_full_details() {
    // Create a comprehensive human name
    let mut name = HumanName::new(
        "official".to_string(),
        "Dr. John Andrew Smith, MD, PhD".to_string(),
        "Smith".to_string()
    );
    
    // Add given names
    name.set_given(vec!["John".to_string(), "Andrew".to_string()]);
    
    // Add prefix and suffix
    name.set_prefix(vec!["Dr.".to_string()]);
    name.set_suffix(vec!["MD".to_string(), "PhD".to_string()]);
    
    // Add period
    name.set_period(Some("2020-01-01 to 2023-12-31".to_string()));
    
    // Verify all details
    assert_eq!(name.use_value(), "official");
    assert_eq!(name.text(), "Dr. John Andrew Smith, MD, PhD");
    assert_eq!(name.family(), "Smith");
    assert_eq!(name.given(), &["John", "Andrew"]);
    assert_eq!(name.prefix(), &["Dr."]);
    assert_eq!(name.suffix(), &["MD", "PhD"]);
    assert_eq!(name.period(), Some("2020-01-01 to 2023-12-31"));
    
    println!("✅ HumanName with full details created successfully!");
    println!("   Use: {}", name.use_value());
    println!("   Text: {}", name.text());
    println!("   Family: {}", name.family());
    println!("   Given: {}", name.given().join(", "));
    println!("   Prefix: {}", name.prefix().join(", "));
    println!("   Suffix: {}", name.suffix().join(", "));
    println!("   Period: {}", name.period().unwrap_or("None"));
}

#[test]
fn test_human_name_different_use_types() {
    // Test different use types for names
    let use_types = vec![
        ("official", "Dr. John Smith"),
        ("nickname", "Johnny"),
        ("maiden", "Jane Smith"),
        ("anonymous", "Anonymous"),
    ];
    
    for (use_type, text) in use_types {
        let name = HumanName::new(
            use_type.to_string(),
            text.to_string(),
            "Smith".to_string()
        );
        
        assert_eq!(name.use_value(), use_type);
        assert_eq!(name.text(), text);
        
        println!("✅ {} name created: {}", use_type, text);
    }
}

#[test]
fn test_human_name_international_formats() {
    // Test international name formats
    let international_names = vec![
        ("Chinese", "official", "Li Wei", "Li", vec!["Wei"]),
        ("Japanese", "official", "Tanaka Yuki", "Tanaka", vec!["Yuki"]),
        ("Korean", "official", "Kim Min-seok", "Kim", vec!["Min-seok"]),
        ("Spanish", "official", "María José García", "García", vec!["María", "José"]),
        ("Arabic", "official", "Ahmed Al-Rashid", "Al-Rashid", vec!["Ahmed"]),
    ];
    
    for (culture, use_type, text, family, given) in international_names {
        let mut name = HumanName::new(
            use_type.to_string(),
            text.to_string(),
            family.to_string()
        );
        name.set_given(given.into_iter().map(|s| s.to_string()).collect());
        
        assert_eq!(name.use_value(), use_type);
        assert_eq!(name.text(), text);
        assert_eq!(name.family(), family);
        
        println!("✅ {} name created: {}", culture, text);
        println!("   Family: {}, Given: {}", family, name.given().join(", "));
    }
}

#[test]
fn test_human_name_professional_titles() {
    // Test various professional titles and credentials
    let professional_names = vec![
        ("Dr. Sarah Johnson, MD", "Johnson", vec!["Sarah"], vec!["Dr."], vec!["MD"]),
        ("Prof. Michael Chen, PhD, MBA", "Chen", vec!["Michael"], vec!["Prof."], vec!["PhD", "MBA"]),
        ("Rev. David Williams", "Williams", vec!["David"], vec!["Rev."], vec![]),
        ("Sir James Bond, KBE", "Bond", vec!["James"], vec!["Sir"], vec!["KBE"]),
        ("Lady Elizabeth Windsor", "Windsor", vec!["Elizabeth"], vec!["Lady"], vec![]),
    ];
    
    for (text, family, given, prefix, suffix) in professional_names {
        let mut name = HumanName::new(
            "official".to_string(),
            text.to_string(),
            family.to_string()
        );
        name.set_given(given.into_iter().map(|s| s.to_string()).collect());
        name.set_prefix(prefix.into_iter().map(|s| s.to_string()).collect());
        name.set_suffix(suffix.into_iter().map(|s| s.to_string()).collect());
        
        println!("✅ Professional name: {}", text);
        println!("   Family: {}, Given: {}", family, name.given().join(", "));
        if !name.prefix().is_empty() {
            println!("   Prefix: {}", name.prefix().join(", "));
        }
        if !name.suffix().is_empty() {
            println!("   Suffix: {}", name.suffix().join(", "));
        }
    }
}

#[test]
fn test_human_name_serialization() {
    // Create a human name with full details
    let mut name = HumanName::new(
        "official".to_string(),
        "Dr. John Andrew Smith, MD, PhD".to_string(),
        "Smith".to_string()
    );
    name.set_given(vec!["John".to_string(), "Andrew".to_string()]);
    name.set_prefix(vec!["Dr.".to_string()]);
    name.set_suffix(vec!["MD".to_string(), "PhD".to_string()]);
    name.set_period(Some("2020-2023".to_string()));
    
    // Serialize to JSON
    let json = serde_json::to_string_pretty(&name).unwrap();
    
    // Deserialize from JSON
    let deserialized_name: HumanName = serde_json::from_str(&json).unwrap();
    
    // Verify serialization/deserialization
    assert_eq!(name, deserialized_name);
    
    println!("✅ HumanName serialization/deserialization successful!");
    println!("   JSON Output:");
    println!("{}", json);
}

#[test]
fn test_human_name_edge_cases() {
    // Test edge cases for names
    let edge_cases = vec![
        ("Single name", "Madonna", "Madonna", vec![], vec![], vec![]),
        ("Multiple given names", "John Paul George Ringo", "Ringo", vec!["John", "Paul", "George"], vec![], vec![]),
        ("Multiple prefixes", "Dr. Prof. Sir James", "James", vec!["James"], vec!["Dr.", "Prof.", "Sir"], vec![]),
        ("Multiple suffixes", "John Smith Jr. Sr. III", "Smith", vec!["John"], vec![], vec!["Jr.", "Sr.", "III"]),
        ("Empty components", "Smith", "Smith", vec![], vec![], vec![]),
    ];
    
    for (description, text, family, given, prefix, suffix) in edge_cases {
        let mut name = HumanName::new(
            "official".to_string(),
            text.to_string(),
            family.to_string()
        );
        name.set_given(given.into_iter().map(|s| s.to_string()).collect());
        name.set_prefix(prefix.into_iter().map(|s| s.to_string()).collect());
        name.set_suffix(suffix.into_iter().map(|s| s.to_string()).collect());
        
        println!("✅ {}: {}", description, text);
        println!("   Family: {}, Given: {}", family, name.given().join(", "));
        if !name.prefix().is_empty() {
            println!("   Prefix: {}", name.prefix().join(", "));
        }
        if !name.suffix().is_empty() {
            println!("   Suffix: {}", name.suffix().join(", "));
        }
    }
} 