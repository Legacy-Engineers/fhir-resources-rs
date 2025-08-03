# API Reference

> **⚠️ IMPORTANT**: This is a personal project and is NOT ready for production use.

## Table of Contents

- [Patient](#patient)
- [HumanName](#humanname)
- [Identifier](#identifier)
- [Period](#period)
- [Uri](#uri)
- [Code](#code)

## Patient

The `Patient` resource represents a person receiving healthcare services.

### Struct Definition

```rust
pub struct Patient {
    #[serde(rename = "resourceType")]
    resource_type: String,
    identifier: Vec<Identifier>,
    name: Vec<HumanName>,
}
```

**Note**: The field is named `resource_type` in Rust (snake_case) but serializes to `resourceType` in JSON (camelCase) to follow FHIR conventions.

### Methods

#### `new() -> Patient`

Creates a new Patient with default values.

```rust
let patient = Patient::new();
// resource_type = "Patient"
// identifier = []
// name = []
```

#### `with_resource_type(resource_type: String) -> Patient`

Creates a new Patient with a custom resource type.

```rust
let patient = Patient::with_resource_type("CustomPatient".to_string());
```

#### `resource_type() -> &str`

Returns the resource type.

```rust
let patient = Patient::new();
assert_eq!(patient.resource_type(), "Patient");
```

#### `identifiers() -> &[Identifier]`

Returns a slice of all identifiers.

```rust
let identifiers = patient.identifiers();
for identifier in identifiers {
    println!("Identifier: {}", identifier.value());
}
```

#### `names() -> &[HumanName]`

Returns a slice of all names.

```rust
let names = patient.names();
for name in names {
    println!("Name: {}", name.text());
}
```

#### `add_identifier(&mut self, identifier: Identifier)`

Adds an identifier to the patient.

```rust
let mut patient = Patient::new();
let identifier = Identifier::new(use_uri, system_uri, "MRN12345".to_string());
patient.add_identifier(identifier);
```

#### `add_name(&mut self, name: HumanName)`

Adds a name to the patient.

```rust
let mut patient = Patient::new();
let name = HumanName::new("official".to_string(), "John Smith".to_string(), "Smith".to_string());
patient.add_name(name);
```

#### `set_resource_type(&mut self, resource_type: String)`

Sets the resource type.

```rust
let mut patient = Patient::new();
patient.set_resource_type("CustomPatient".to_string());
```

#### `set_identifiers(&mut self, identifiers: Vec<Identifier>)`

Sets all identifiers at once.

```rust
let mut patient = Patient::new();
let identifiers = vec![identifier1, identifier2];
patient.set_identifiers(identifiers);
```

#### `set_names(&mut self, names: Vec<HumanName>)`

Sets all names at once.

```rust
let mut patient = Patient::new();
let names = vec![name1, name2];
patient.set_names(names);
```

## HumanName

The `HumanName` resource represents a person's name with structured components.

### Struct Definition

```rust
pub struct HumanName {
    r#use: String,
    text: String,
    family: String,
    given: Vec<String>,
    prefix: Vec<String>,
    suffix: Vec<String>,
    period: Option<String>,
}
```

### Methods

#### `new(use_value: String, text: String, family: String) -> HumanName`

Creates a new HumanName with required fields.

```rust
let name = HumanName::new(
    "official".to_string(),
    "Dr. John Smith".to_string(),
    "Smith".to_string()
);
```

#### `use_value() -> &str`

Returns the use value (e.g., "official", "nickname", "maiden").

```rust
assert_eq!(name.use_value(), "official");
```

#### `text() -> &str`

Returns the full text representation of the name.

```rust
assert_eq!(name.text(), "Dr. John Smith");
```

#### `family() -> &str`

Returns the family name.

```rust
assert_eq!(name.family(), "Smith");
```

#### `given() -> &[String]`

Returns a slice of given names.

```rust
let given_names = name.given();
// ["John", "Andrew"]
```

#### `prefix() -> &[String]`

Returns a slice of prefixes (e.g., "Dr.", "Prof.").

```rust
let prefixes = name.prefix();
// ["Dr."]
```

#### `suffix() -> &[String]`

Returns a slice of suffixes (e.g., "MD", "PhD").

```rust
let suffixes = name.suffix();
// ["MD", "PhD"]
```

#### `period() -> Option<&str>`

Returns the period if set.

```rust
match name.period() {
    Some(period) => println!("Period: {}", period),
    None => println!("No period set"),
}
```

#### `set_given(&mut self, given: Vec<String>)`

Sets the given names.

```rust
name.set_given(vec!["John".to_string(), "Andrew".to_string()]);
```

#### `set_prefix(&mut self, prefix: Vec<String>)`

Sets the prefixes.

```rust
name.set_prefix(vec!["Dr.".to_string()]);
```

#### `set_suffix(&mut self, suffix: Vec<String>)`

Sets the suffixes.

```rust
name.set_suffix(vec!["MD".to_string(), "PhD".to_string()]);
```

#### `set_period(&mut self, period: Option<String>)`

Sets the period.

```rust
name.set_period(Some("2020-2023".to_string()));
```

## Identifier

The `Identifier` resource represents a unique identifier for a resource.

### Struct Definition

```rust
pub struct Identifier {
    r#use: Uri,
    system: Uri,
    value: String,
    period: Option<Period>,
}
```

### Methods

#### `new(use_value: Uri, system: Uri, value: String) -> Identifier`

Creates a new Identifier with required fields.

```rust
let use_uri = Uri::new_unchecked("official".to_string());
let system_uri = Uri::new_unchecked("https://hospital.example.com/patients".to_string());
let identifier = Identifier::new(use_uri, system_uri, "MRN12345".to_string());
```

#### `use_value() -> &Uri`

Returns the use value URI.

```rust
let use_value = identifier.use_value();
assert_eq!(use_value.as_str(), "official");
```

#### `system() -> &Uri`

Returns the system URI.

```rust
let system = identifier.system();
assert_eq!(system.as_str(), "https://hospital.example.com/patients");
```

#### `value() -> &str`

Returns the identifier value.

```rust
assert_eq!(identifier.value(), "MRN12345");
```

#### `period() -> Option<&Period>`

Returns the period if set.

```rust
match identifier.period() {
    Some(period) => println!("Period: {} to {}", period.start(), period.end()),
    None => println!("No period set"),
}
```

#### `set_period(&mut self, period: Option<Period>)`

Sets the period.

```rust
let period = Period::new("2020-01-01".to_string(), "2023-12-31".to_string());
identifier.set_period(Some(period));
```

## Period

The `Period` resource represents a time period with start and end dates.

### Struct Definition

```rust
pub struct Period {
    start: String,
    end: String,
}
```

### Methods

#### `new(start: String, end: String) -> Period`

Creates a new Period with start and end dates.

```rust
let period = Period::new("2020-01-01".to_string(), "2023-12-31".to_string());
```

#### `start() -> &str`

Returns the start date.

```rust
assert_eq!(period.start(), "2020-01-01");
```

#### `end() -> &str`

Returns the end date.

```rust
assert_eq!(period.end(), "2023-12-31");
```

#### `set_start(&mut self, start: String)`

Sets the start date.

```rust
period.set_start("2021-01-01".to_string());
```

#### `set_end(&mut self, end: String)`

Sets the end date.

```rust
period.set_end("2024-12-31".to_string());
```

## Uri

The `Uri` resource represents a Uniform Resource Identifier with validation.

### Struct Definition

```rust
pub struct Uri {
    value: String,
}
```

### Methods

#### `new(value: String) -> Result<Self, UriError>`

Creates a new URI with validation.

```rust
match Uri::new("https://hospital.example.com/patients".to_string()) {
    Ok(uri) => println!("Valid URI: {}", uri.as_str()),
    Err(e) => println!("Invalid URI: {}", e),
}
```

#### `new_unchecked(value: String) -> Self`

Creates a new URI without validation (use with caution).

```rust
let uri = Uri::new_unchecked("https://hospital.example.com/patients".to_string());
```

#### `as_str() -> &str`

Returns the URI value as a string slice.

```rust
assert_eq!(uri.as_str(), "https://hospital.example.com/patients");
```

#### `to_string() -> String`

Returns the URI value as a string.

```rust
let uri_string = uri.to_string();
```

#### `validate() -> Result<(), UriError>`

Validates the URI according to FHIR specifications.

```rust
match uri.validate() {
    Ok(()) => println!("URI is valid"),
    Err(e) => println!("URI validation failed: {}", e),
}
```

#### `is_uuid() -> bool`

Checks if the URI is a UUID URI.

```rust
if uri.is_uuid() {
    println!("This is a UUID URI");
}
```

#### `is_absolute() -> bool`

Checks if the URI is absolute.

```rust
if uri.is_absolute() {
    println!("This is an absolute URI");
}
```

#### `is_relative() -> bool`

Checks if the URI is relative.

```rust
if uri.is_relative() {
    println!("This is a relative URI");
}
```

#### `fragment() -> Option<&str>`

Gets the fragment identifier if present.

```rust
match uri.fragment() {
    Some(fragment) => println!("Fragment: {}", fragment),
    None => println!("No fragment"),
}
```

### Error Types

#### `UriError`

```rust
pub enum UriError {
    Empty,
    InvalidCharacters(String),
    UuidNotLowercase(String),
    InvalidFormat(String),
}
```

## Code

The `Code` resource represents a FHIR code value with validation.

### Struct Definition

```rust
pub struct Code {
    value: String,
}
```

### Methods

#### `new(value: String) -> Result<Self, CodeError>`

Creates a new Code with validation.

```rust
match Code::new("active".to_string()) {
    Ok(code) => println!("Valid code: {}", code.as_str()),
    Err(e) => println!("Invalid code: {}", e),
}
```

#### `new_unchecked(value: String) -> Self`

Creates a new Code without validation (use with caution).

```rust
let code = Code::new_unchecked("active".to_string());
```

#### `as_str() -> &str`

Returns the code value as a string slice.

```rust
assert_eq!(code.as_str(), "active");
```

#### `to_string() -> String`

Returns the code value as a string.

```rust
let code_string = code.to_string();
```

#### `validate() -> Result<(), CodeError>`

Validates the code according to FHIR specifications.

```rust
match code.validate() {
    Ok(()) => println!("Code is valid"),
    Err(e) => println!("Code validation failed: {}", e),
}
```

#### `tokens() -> Vec<&str>`

Returns the individual tokens in the code, split by spaces.

```rust
let tokens = code.tokens();
// ["active", "status"]
```

#### `token_count() -> usize`

Returns the number of tokens in the code.

```rust
assert_eq!(code.token_count(), 2);
```

#### `is_single_token() -> bool`

Checks if the code contains only a single token (no spaces).

```rust
if code.is_single_token() {
    println!("Single token code");
}
```

#### `is_multi_token() -> bool`

Checks if the code contains multiple tokens.

```rust
if code.is_multi_token() {
    println!("Multi-token code");
}
```

### Error Types

#### `CodeError`

```rust
pub enum CodeError {
    Empty,
    LeadingTrailingWhitespace,
    MultipleSpaces,
    InvalidWhitespace,
    InvalidPattern,
}
```

## Serialization

All resources implement `Serialize` and `Deserialize` traits for JSON serialization. The library follows FHIR conventions by using camelCase field names in JSON output (e.g., `resourceType`) while maintaining Rust naming conventions (snake_case) in the code.

### Example

```rust
use serde_json;

// Serialize
let json = serde_json::to_string_pretty(&patient).unwrap();
println!("{}", json);

// Deserialize
let deserialized_patient: Patient = serde_json::from_str(&json).unwrap();
```

### JSON Output Example

```json
{
  "resourceType": "Patient",
  "identifier": [
    {
      "use": {
        "value": "official"
      },
      "system": {
        "value": "https://hospital.example.com/patients"
      },
      "value": "MRN12345",
      "period": null
    }
  ],
  "name": [
    {
      "use": "official",
      "text": "Dr. John Smith",
      "family": "Smith",
      "given": ["John"],
      "prefix": ["Dr."],
      "suffix": []
    }
  ]
}
```

## Traits Implemented

### Common Traits

All resources implement these common traits:

- `Debug` - For debugging output
- `Clone` - For cloning objects
- `PartialEq` - For equality comparison
- `Eq` - For equality comparison
- `Serialize` - For JSON serialization
- `Deserialize` - For JSON deserialization

### Additional Traits

#### `Default` (Patient)

```rust
impl Default for Patient {
    fn default() -> Self {
        Self::new()
    }
}
```

#### `FromStr` (Uri, Code)

```rust
use std::str::FromStr;

let uri = Uri::from_str("https://example.com").unwrap();
let code = Code::from_str("active").unwrap();
```

#### `Display` (Uri, Code)

```rust
println!("URI: {}", uri);
println!("Code: {}", code);
```

#### `AsRef<str>` (Uri, Code)

```rust
let uri_str: &str = uri.as_ref();
let code_str: &str = code.as_ref();
```

## Error Handling

### Validation Errors

Most resources provide validation methods that return `Result` types:

```rust
// URI validation
match Uri::new("invalid uri".to_string()) {
    Ok(uri) => println!("Valid URI"),
    Err(e) => println!("Invalid URI: {}", e),
}

// Code validation
match Code::new(" invalid code ".to_string()) {
    Ok(code) => println!("Valid code"),
    Err(e) => println!("Invalid code: {}", e),
}
```

### Serialization Errors

JSON serialization can fail:

```rust
match serde_json::to_string(&patient) {
    Ok(json) => println!("Serialized: {}", json),
    Err(e) => println!("Serialization failed: {}", e),
}
```

## Best Practices

### 1. Use Validation When Possible

```rust
// Prefer validated creation
let uri = Uri::new("https://example.com".to_string())?;

// Only use unchecked when you're certain the input is valid
let uri = Uri::new_unchecked("https://example.com".to_string());
```

### 2. Handle Optional Fields

```rust
// Check for optional fields
if let Some(period) = identifier.period() {
    println!("Period: {} to {}", period.start(), period.end());
}
```

### 3. Use Builder Pattern for Complex Objects

```rust
let mut patient = Patient::new();
patient.add_identifier(identifier);
patient.add_name(name);
// ... add more fields
```

### 4. Validate Before Serialization

```rust
// Ensure all required fields are set
if patient.identifiers().is_empty() {
    return Err("Patient must have at least one identifier");
}
```

---

**⚠️ Remember**: This is a personal development project and should not be used in production healthcare systems without proper validation and testing.
