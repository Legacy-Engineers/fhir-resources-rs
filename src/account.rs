use crate::identifier::Identifier;
use crate::data_types::codeable_concept::CodeableConcept;
use crate::data_types::reference::Reference;
use crate::period::Period;
use crate::account_coverage::AccountCoverage;
use crate::account_guarantor::AccountGuarantor;
use crate::account_diagnosis::AccountDiagnosis;
use crate::account_procedure::AccountProcedure;
use crate::account_related_account::AccountRelatedAccount;
use crate::account_balance::AccountBalance;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "resourceType")]
    resource_type: String,
    #[serde(rename = "identifier")]
    identifier: Vec<Identifier>,
    #[serde(rename = "status")]
    status: Option<String>, // active | inactive | entered-in-error | on-hold | unknown
    #[serde(rename = "billingStatus")]
    billing_status: Option<CodeableConcept>,
    #[serde(rename = "type")]
    account_type: Option<CodeableConcept>,
    #[serde(rename = "name")]
    name: Option<String>,
    #[serde(rename = "subject")]
    subject: Vec<Reference>,
    #[serde(rename = "servicePeriod")]
    service_period: Option<Period>,
    #[serde(rename = "coverage")]
    coverage: Vec<AccountCoverage>,
    #[serde(rename = "owner")]
    owner: Option<Reference>,
    #[serde(rename = "description")]
    description: Option<String>,
    #[serde(rename = "guarantor")]
    guarantor: Vec<AccountGuarantor>,
    #[serde(rename = "diagnosis")]
    diagnosis: Vec<AccountDiagnosis>,
    #[serde(rename = "procedure")]
    procedure: Vec<AccountProcedure>,
    #[serde(rename = "relatedAccount")]
    related_account: Vec<AccountRelatedAccount>,
    #[serde(rename = "currency")]
    currency: Option<CodeableConcept>,
    #[serde(rename = "balance")]
    balance: Vec<AccountBalance>,
    #[serde(rename = "calculatedAt")]
    calculated_at: Option<String>,
}


impl Account {
    /// Creates a new Account with default values.
    pub fn new() -> Self {
        Self {
            resource_type: "Account".to_string(),
            identifier: Vec::new(),
            status: None,
            billing_status: None,
            account_type: None,
            name: None,
            subject: Vec::new(),
            service_period: None,
            coverage: Vec::new(),
            owner: None,
            description: None,
            guarantor: Vec::new(),
            diagnosis: Vec::new(),
            procedure: Vec::new(),
            related_account: Vec::new(),
            currency: None,
            balance: Vec::new(),
            calculated_at: None,
        }
    }

    /// Creates a new Account with the specified resource type.
    pub fn with_resource_type(resource_type: String) -> Self {
        Self {
            resource_type,
            identifier: Vec::new(),
            status: None,
            billing_status: None,
            account_type: None,
            name: None,
            subject: Vec::new(),
            service_period: None,
            coverage: Vec::new(),
            owner: None,
            description: None,
            guarantor: Vec::new(),
            diagnosis: Vec::new(),
            procedure: Vec::new(),
            related_account: Vec::new(),
            currency: None,
            balance: Vec::new(),
            calculated_at: None,
        }
    }

    // Getters
    pub fn resource_type(&self) -> &str {
        &self.resource_type
    }

    pub fn identifiers(&self) -> &[Identifier] {
        &self.identifier
    }

    pub fn status(&self) -> Option<&str> {
        self.status.as_deref()
    }

    pub fn billing_status(&self) -> Option<&CodeableConcept> {
        self.billing_status.as_ref()
    }

    pub fn account_type(&self) -> Option<&CodeableConcept> {
        self.account_type.as_ref()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn subject(&self) -> &[Reference] {
        &self.subject
    }

    pub fn service_period(&self) -> Option<&Period> {
        self.service_period.as_ref()
    }

    pub fn coverage(&self) -> &[AccountCoverage] {
        &self.coverage
    }

    pub fn owner(&self) -> Option<&Reference> {
        self.owner.as_ref()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn guarantor(&self) -> &[AccountGuarantor] {
        &self.guarantor
    }

    pub fn diagnosis(&self) -> &[AccountDiagnosis] {
        &self.diagnosis
    }

    pub fn procedure(&self) -> &[AccountProcedure] {
        &self.procedure
    }

    pub fn related_account(&self) -> &[AccountRelatedAccount] {
        &self.related_account
    }

    pub fn currency(&self) -> Option<&CodeableConcept> {
        self.currency.as_ref()
    }

    pub fn balance(&self) -> &[AccountBalance] {
        &self.balance
    }

    pub fn calculated_at(&self) -> Option<&str> {
        self.calculated_at.as_deref()
    }

    // Setters
    pub fn set_resource_type(&mut self, resource_type: String) {
        self.resource_type = resource_type;
    }

    pub fn set_identifiers(&mut self, identifiers: Vec<Identifier>) {
        self.identifier = identifiers;
    }

    pub fn add_identifier(&mut self, identifier: Identifier) {
        self.identifier.push(identifier);
    }

    pub fn set_status(&mut self, status: Option<String>) {
        self.status = status;
    }

    pub fn set_billing_status(&mut self, billing_status: Option<CodeableConcept>) {
        self.billing_status = billing_status;
    }

    pub fn set_account_type(&mut self, account_type: Option<CodeableConcept>) {
        self.account_type = account_type;
    }

    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    pub fn set_subject(&mut self, subject: Vec<Reference>) {
        self.subject = subject;
    }

    pub fn add_subject(&mut self, subject: Reference) {
        self.subject.push(subject);
    }

    pub fn set_service_period(&mut self, service_period: Option<Period>) {
        self.service_period = service_period;
    }

    pub fn set_coverage(&mut self, coverage: Vec<AccountCoverage>) {
        self.coverage = coverage;
    }

    pub fn add_coverage(&mut self, coverage: AccountCoverage) {
        self.coverage.push(coverage);
    }

    pub fn set_owner(&mut self, owner: Option<Reference>) {
        self.owner = owner;
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn set_guarantor(&mut self, guarantor: Vec<AccountGuarantor>) {
        self.guarantor = guarantor;
    }

    pub fn add_guarantor(&mut self, guarantor: AccountGuarantor) {
        self.guarantor.push(guarantor);
    }

    pub fn set_diagnosis(&mut self, diagnosis: Vec<AccountDiagnosis>) {
        self.diagnosis = diagnosis;
    }

    pub fn add_diagnosis(&mut self, diagnosis: AccountDiagnosis) {
        self.diagnosis.push(diagnosis);
    }

    pub fn set_procedure(&mut self, procedure: Vec<AccountProcedure>) {
        self.procedure = procedure;
    }

    pub fn add_procedure(&mut self, procedure: AccountProcedure) {
        self.procedure.push(procedure);
    }

    pub fn set_related_account(&mut self, related_account: Vec<AccountRelatedAccount>) {
        self.related_account = related_account;
    }

    pub fn add_related_account(&mut self, related_account: AccountRelatedAccount) {
        self.related_account.push(related_account);
    }

    pub fn set_currency(&mut self, currency: Option<CodeableConcept>) {
        self.currency = currency;
    }

    pub fn set_balance(&mut self, balance: Vec<AccountBalance>) {
        self.balance = balance;
    }

    pub fn add_balance(&mut self, balance: AccountBalance) {
        self.balance.push(balance);
    }

    pub fn set_calculated_at(&mut self, calculated_at: Option<String>) {
        self.calculated_at = calculated_at;
    }

    /// Converts the Account to JSON string.
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    /// Creates an Account from JSON string.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

impl Default for Account {
    fn default() -> Self {
        Self::new()
    }
}


