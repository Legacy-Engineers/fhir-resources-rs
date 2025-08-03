use fhir_resources_rs::account::*;
use fhir_resources_rs::account_coverage::AccountCoverage;
use fhir_resources_rs::account_guarantor::AccountGuarantor;
use fhir_resources_rs::account_diagnosis::AccountDiagnosis;
use fhir_resources_rs::account_procedure::AccountProcedure;
use fhir_resources_rs::account_related_account::AccountRelatedAccount;
use fhir_resources_rs::account_balance::AccountBalance;
use fhir_resources_rs::money::Money;
use fhir_resources_rs::identifier::Identifier;
use fhir_resources_rs::data_types::codeable_concept::CodeableConcept;
use fhir_resources_rs::data_types::reference::Reference;
use fhir_resources_rs::data_types::uri::Uri;
use fhir_resources_rs::period::Period;

#[test]
fn test_account_new() {
    let account = Account::new();
    assert_eq!(account.resource_type(), "Account");
    assert!(account.identifiers().is_empty());
    assert!(account.status().is_none());
    assert!(account.billing_status().is_none());
    assert!(account.account_type().is_none());
    assert!(account.name().is_none());
    assert!(account.subject().is_empty());
    assert!(account.service_period().is_none());
    assert!(account.coverage().is_empty());
    assert!(account.owner().is_none());
    assert!(account.description().is_none());
    assert!(account.guarantor().is_empty());
    assert!(account.diagnosis().is_empty());
    assert!(account.procedure().is_empty());
    assert!(account.related_account().is_empty());
    assert!(account.currency().is_none());
    assert!(account.balance().is_empty());
    assert!(account.calculated_at().is_none());
}

#[test]
fn test_account_with_resource_type() {
    let account = Account::with_resource_type("CustomAccount".to_string());
    assert_eq!(account.resource_type(), "CustomAccount");
}

#[test]
fn test_account_setters_and_getters() {
    let mut account = Account::new();
    
    // Test identifier
    let use_uri = Uri::new_unchecked("official".to_string());
    let system_uri = Uri::new_unchecked("http://example.com/identifier".to_string());
    let identifier = Identifier::new(use_uri, system_uri, "12345".to_string());
    account.add_identifier(identifier);
    assert_eq!(account.identifiers().len(), 1);
    
    // Test status
    account.set_status(Some("active".to_string()));
    assert_eq!(account.status(), Some("active"));
    
    // Test name
    account.set_name(Some("Test Account".to_string()));
    assert_eq!(account.name(), Some("Test Account"));
    
    // Test subject
    let reference = Reference::new();
    account.add_subject(reference);
    assert_eq!(account.subject().len(), 1);
    
    // Test description
    account.set_description(Some("Test description".to_string()));
    assert_eq!(account.description(), Some("Test description"));
    
    // Test calculated_at
    account.set_calculated_at(Some("2023-01-01T00:00:00Z".to_string()));
    assert_eq!(account.calculated_at(), Some("2023-01-01T00:00:00Z"));
}

#[test]
fn test_account_coverage() {
    let reference = Reference::new();
    let coverage = AccountCoverage::new(reference.clone());
    assert!(coverage.priority().is_none());
    
    let coverage_with_priority = AccountCoverage::with_priority(reference, "1".to_string());
    assert_eq!(coverage_with_priority.priority(), Some("1"));
}

#[test]
fn test_account_guarantor() {
    let reference = Reference::new();
    let guarantor = AccountGuarantor::new(reference);
    assert!(guarantor.on_hold().is_none());
    assert!(guarantor.period().is_none());
    
    let mut guarantor = guarantor;
    guarantor.set_on_hold(Some(true));
    assert_eq!(guarantor.on_hold(), Some(true));
}

#[test]
fn test_account_diagnosis() {
    let reference = Reference::new();
    let diagnosis = AccountDiagnosis::new(reference);
    assert!(diagnosis.sequence().is_none());
    assert!(diagnosis.date_of_diagnosis().is_none());
    assert!(diagnosis.diagnosis_type().is_empty());
    assert!(diagnosis.on_admission().is_none());
    assert!(diagnosis.package_code().is_empty());
    
    let mut diagnosis = diagnosis;
    diagnosis.set_sequence(Some("1".to_string()));
    diagnosis.set_date_of_diagnosis(Some("2023-01-01T00:00:00Z".to_string()));
    diagnosis.set_on_admission(Some(true));
    
    assert_eq!(diagnosis.sequence(), Some("1"));
    assert_eq!(diagnosis.date_of_diagnosis(), Some("2023-01-01T00:00:00Z"));
    assert_eq!(diagnosis.on_admission(), Some(true));
}

#[test]
fn test_account_procedure() {
    let reference = Reference::new();
    let procedure = AccountProcedure::new(reference);
    assert!(procedure.sequence().is_none());
    assert!(procedure.date_of_service().is_none());
    assert!(procedure.procedure_type().is_empty());
    assert!(procedure.package_code().is_empty());
    assert!(procedure.device().is_empty());
    
    let mut procedure = procedure;
    procedure.set_sequence(Some("1".to_string()));
    procedure.set_date_of_service(Some("2023-01-01T00:00:00Z".to_string()));
    
    assert_eq!(procedure.sequence(), Some("1"));
    assert_eq!(procedure.date_of_service(), Some("2023-01-01T00:00:00Z"));
}

#[test]
fn test_account_related_account() {
    let reference = Reference::new();
    let related_account = AccountRelatedAccount::new(reference.clone());
    assert!(related_account.relationship().is_none());
    
    let codeable_concept = CodeableConcept::new();
    let related_account_with_relationship = AccountRelatedAccount::with_relationship(reference, codeable_concept);
    assert!(related_account_with_relationship.relationship().is_some());
}

#[test]
fn test_account_balance() {
    let money = Money::with_value(100.0);
    let balance = AccountBalance::new(money);
    assert!(balance.aggregate().is_none());
    assert!(balance.term().is_none());
    assert!(balance.estimate().is_none());
    assert_eq!(balance.amount().value(), Some(100.0));
    
    let mut balance = balance;
    balance.set_estimate(Some(true));
    assert_eq!(balance.estimate(), Some(true));
}

#[test]
fn test_money() {
    let money = Money::new();
    assert!(money.value().is_none());
    assert!(money.currency().is_none());
    
    let money_with_value = Money::with_value(100.0);
    assert_eq!(money_with_value.value(), Some(100.0));
    assert!(money_with_value.currency().is_none());
    
    let money_with_currency = Money::with_currency(100.0, "USD".to_string());
    assert_eq!(money_with_currency.value(), Some(100.0));
    assert_eq!(money_with_currency.currency(), Some("USD"));
    
    let mut money = Money::new();
    money.set_value(Some(50.0));
    money.set_currency(Some("EUR".to_string()));
    assert_eq!(money.value(), Some(50.0));
    assert_eq!(money.currency(), Some("EUR"));
}

#[test]
fn test_account_json_serialization() {
    let mut account = Account::new();
    account.set_name(Some("Test Account".to_string()));
    account.set_status(Some("active".to_string()));
    
    let json = account.to_json();
    assert!(json.contains("Test Account"));
    assert!(json.contains("active"));
    assert!(json.contains("Account"));
    
    // Test deserialization
    let deserialized = Account::from_json(&json);
    assert!(deserialized.is_ok());
    let deserialized = deserialized.unwrap();
    assert_eq!(deserialized.name(), Some("Test Account"));
    assert_eq!(deserialized.status(), Some("active"));
}

#[test]
fn test_account_complex_json() {
    let mut account = Account::new();
    account.set_name(Some("Complex Account".to_string()));
    account.set_status(Some("active".to_string()));
    
    // Add identifier
    let use_uri = Uri::new_unchecked("official".to_string());
    let system_uri = Uri::new_unchecked("http://example.com/identifier".to_string());
    let identifier = Identifier::new(use_uri, system_uri, "12345".to_string());
    account.add_identifier(identifier);
    
    // Add subject
    let reference = Reference::new();
    account.add_subject(reference);
    
    // Add coverage
    let coverage_reference = Reference::new();
    let coverage = AccountCoverage::with_priority(coverage_reference, "1".to_string());
    account.add_coverage(coverage);
    
    // Add balance
    let money = Money::with_currency(1000.0, "USD".to_string());
    let balance = AccountBalance::new(money);
    account.add_balance(balance);
    
    let json = account.to_json();
    assert!(json.contains("Complex Account"));
    assert!(json.contains("active"));
    assert!(json.contains("USD"));
    assert!(json.contains("1000"));
    
    // Test deserialization
    let deserialized = Account::from_json(&json);
    assert!(deserialized.is_ok());
    let deserialized = deserialized.unwrap();
    assert_eq!(deserialized.name(), Some("Complex Account"));
    assert_eq!(deserialized.status(), Some("active"));
    assert_eq!(deserialized.coverage().len(), 1);
    assert_eq!(deserialized.balance().len(), 1);
    assert_eq!(deserialized.balance()[0].amount().currency(), Some("USD"));
    assert_eq!(deserialized.balance()[0].amount().value(), Some(1000.0));
} 