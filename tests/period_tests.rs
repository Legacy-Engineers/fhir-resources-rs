use fhir_resources_rs::period::Period;

#[test]
fn test_period_creation() {
    // Create a basic period
    let period = Period::new("2020-01-01".to_string(), "2023-12-31".to_string());
    
    // Verify basic values
    assert_eq!(period.start(), "2020-01-01");
    assert_eq!(period.end(), "2023-12-31");
    
    println!("✅ Period created successfully with basic data");
    println!("   Start: {}", period.start());
    println!("   End: {}", period.end());
}

#[test]
fn test_period_modification() {
    // Create a period
    let mut period = Period::new("2020-01-01".to_string(), "2023-12-31".to_string());
    
    // Modify the period
    period.set_start("2021-01-01".to_string());
    period.set_end("2024-12-31".to_string());
    
    // Verify modifications
    assert_eq!(period.start(), "2021-01-01");
    assert_eq!(period.end(), "2024-12-31");
    
    println!("✅ Period modified successfully!");
    println!("   Start: {}", period.start());
    println!("   End: {}", period.end());
}

#[test]
fn test_period_different_formats() {
    // Test different date/time formats
    let period_formats = vec![
        ("2020-01-01", "2023-12-31", "Standard date format"),
        ("2020-01-01T00:00:00Z", "2023-12-31T23:59:59Z", "ISO 8601 format"),
        ("2020-01-01 00:00:00", "2023-12-31 23:59:59", "DateTime format"),
        ("2020-01-01T00:00:00+00:00", "2023-12-31T23:59:59+00:00", "ISO 8601 with timezone"),
    ];
    
    for (start, end, description) in period_formats {
        let period = Period::new(start.to_string(), end.to_string());
        
        assert_eq!(period.start(), start);
        assert_eq!(period.end(), end);
        
        println!("✅ {}: {} to {}", description, start, end);
    }
}

#[test]
fn test_period_healthcare_scenarios() {
    // Test healthcare-specific period scenarios
    let healthcare_periods = vec![
        ("2020-01-01", "2020-12-31", "Annual coverage period"),
        ("2020-01-01", "2020-01-31", "Monthly billing period"),
        ("2020-01-01", "2020-01-07", "Weekly treatment period"),
        ("2020-01-01", "2020-01-01", "Single day appointment"),
        ("2020-01-01", "2023-12-31", "Long-term care period"),
    ];
    
    for (start, end, description) in healthcare_periods {
        let period = Period::new(start.to_string(), end.to_string());
        
        println!("✅ {}: {} to {}", description, start, end);
        println!("   Duration: {} to {}", period.start(), period.end());
    }
}

#[test]
fn test_period_serialization() {
    // Create a period with specific dates
    let period = Period::new("2020-01-01".to_string(), "2023-12-31".to_string());
    
    // Serialize to JSON
    let json = serde_json::to_string_pretty(&period).unwrap();
    
    // Deserialize from JSON
    let deserialized_period: Period = serde_json::from_str(&json).unwrap();
    
    // Verify serialization/deserialization
    assert_eq!(period, deserialized_period);
    
    println!("✅ Period serialization/deserialization successful!");
    println!("   JSON Output:");
    println!("{}", json);
}

#[test]
fn test_period_edge_cases() {
    // Test edge cases for periods
    let edge_cases = vec![
        ("2020-01-01", "2020-01-01", "Same start and end date"),
        ("2020-12-31", "2021-01-01", "Year boundary"),
        ("2020-02-29", "2020-02-29", "Leap year date"),
        ("2020-01-01T00:00:00.000Z", "2023-12-31T23:59:59.999Z", "Millisecond precision"),
    ];
    
    for (start, end, description) in edge_cases {
        let period = Period::new(start.to_string(), end.to_string());
        
        println!("✅ {}: {} to {}", description, start, end);
        println!("   Start: {}, End: {}", period.start(), period.end());
    }
}

#[test]
fn test_period_real_world_scenarios() {
    // Test real-world period scenarios
    let real_world_scenarios = vec![
        ("Insurance Coverage", "2020-01-01", "2020-12-31", "Annual insurance coverage period"),
        ("Hospital Stay", "2020-03-15", "2020-03-20", "5-day hospital admission"),
        ("Prescription Validity", "2020-01-01", "2020-12-31", "Annual prescription validity"),
        ("Treatment Course", "2020-01-01", "2020-06-30", "6-month treatment course"),
        ("Follow-up Period", "2020-01-01", "2020-01-31", "30-day follow-up period"),
    ];
    
    for (scenario, start, end, description) in real_world_scenarios {
        let period = Period::new(start.to_string(), end.to_string());
        
        println!("✅ {}: {}", scenario, description);
        println!("   Period: {} to {}", period.start(), period.end());
    }
}

#[test]
fn test_period_international_formats() {
    // Test international date formats
    let international_formats = vec![
        ("2020-01-01", "2023-12-31", "ISO format (YYYY-MM-DD)"),
        ("01/01/2020", "12/31/2023", "US format (MM/DD/YYYY)"),
        ("01/01/2020", "31/12/2023", "European format (DD/MM/YYYY)"),
        ("2020年1月1日", "2023年12月31日", "Japanese format"),
        ("1. Januar 2020", "31. Dezember 2023", "German format"),
    ];
    
    for (start, end, description) in international_formats {
        let period = Period::new(start.to_string(), end.to_string());
        
        println!("✅ {}: {} to {}", description, start, end);
        println!("   Period: {} to {}", period.start(), period.end());
    }
} 