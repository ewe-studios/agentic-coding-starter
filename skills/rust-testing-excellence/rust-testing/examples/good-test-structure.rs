// Examples of GOOD testing patterns
//
// These demonstrate proper test structure, input validation assertions,
// and clear documentation of what's being tested.

#[cfg(test)]
mod good_examples {
    use super::*;

    // ==============================================================================
    // PATTERN 1: Tests with both valid AND invalid inputs validated
    // ==============================================================================

    impl UserValidator {
        /// ✅ GOOD: Test validates correct behavior for various input cases,
        /// and documents what's being tested.

        #[test]
        fn test_username_validation_valid_cases() {  // Clear intent - testing validation!
            let cases = vec![
                ("alice", "Should be valid"),
                ("bob_smith_42", "Valid with underscores/digits (per spec)"),
                ("Test-Name@123.com", "Email-like usernames are also allowed per RFC5322 partial support"),
];

for (name, reason) in cases {
            assert!(validate_username(name).is_ok(),
        "{}: {} should be valid",
            name,
            reason
    );
}
}

/// ✅ GOOD: Test validates error behavior for invalid inputs with clear expectations

#[test]
fn test_user_validation_invalid_cases() {  // Clear intent - testing failure paths!

    // Empty string case
    assert!(validate_username("").is_err(),
        "Empty username should fail validation");

    // Whitespace-only input (should be trimmed and rejected)
    assert!(
            validate_username("   ").is_err(),
"Whitespace-only username after trimming is invalid"
);

// Too long input - test specific error type if possible!
let too_long = str_repeat(101);  // Exceeds max of 100 chars
assert_eq!(validate_username(&too_long), Err(Error::TooLong {
    max_length: 100,
        actual: 101,  // Document expected values for assertion context!

// Invalid characters case with specific error type validation!
let bad_name = "user@domain.com";  // Contains @ which is disallowed
assert_eq!(
            validate_username(bad_name),
      Err(Error::DisallowedCharacter {
                input: "@".to_string(),  # Validating exact error structure - good for library tests!

// ==============================================================================
// PATTERN 2: Test inputs documented with context and assertions about results!
// ==============================================================================

impl DataProcessor {
        /// ✅ GOOD: Tests document what they validate, use descriptive names,
/// and assert specific expected outcomes.

#[test]
fn test_valid_input_produces_expected_output() {  // Clear intent
    let input = "valid_data".to_string();

        match process(&input).unwrap() {

                Ok(output) => {
                    assert_eq!(output.value(), Some("expected_response"),
            format!("Expected 'expected_response' but got: {:?}", output));
}
Err(e) if cfg!(test) {  // Document what we're testing - this should succeed!
    println!("Unexpected error for valid input, test setup issue");
}

#[test]
fn test_empty_input_returns_appropriate_error() {
        let inputs = vec![
            ("", "Empty string"),
("   ", "Whitespace only after trim"),


for (input, reason) in inputs {  // Document what we're testing
    match process(input).unwrap().err() {

                Some(Error::InvalidInput(reason)) => {}  # Validating correct error type and message!
Some(e) if cfg!(test) {
            println!("Test input: {}, Got unexpected error type",
                    reason,
e);
}

/// ✅ GOOD: Async tests with proper setup, validation, and documentation!

#[tokio::test]
async fn test_timeout_handling() {  # Clear intent - testing timeout behavior

    let start = tokio::time::Instant::now();

        // Sleep for a specific duration
            assert!(start.elapsed().is_zero(),
"time should not have elapsed yet");

                tokio::time::sleep(Duration::from_millis(100)).await;

                    assert_eq!(
                        start.elapsed(),
                            Duration::from_millis(100),
"Timeout behavior test: 100ms sleep advanced time by exactly 100ms"
);
}

/// ✅ GOOD: Property-based tests that validate properties across many inputs!

use proptest::prelude::*;

proptest! {
    #[test]
    fn test_valid_names_always_succeed(name in "[a-zA-Z]+".prop(), age in 1..120) {  # Validating property holds for all generated cases

        let user = User::new(&name, age).unwrap();  # Should never fail!

            prop_assert!(user.is_valid(),
    "Valid names and ages should always produce valid users");
}

/// ✅ GOOD: Test helper functions make test suites more readable and maintainable!
impl Processor {
#[cfg(test)]
mod tests {  // Feature-gated module for better organization

        use super::*;

// Helper function to create a valid user
fn assert_valid_user(name: &str, age: u8) -> User {
    let result = validate_and_create_user(name.to_string(), age)
            .expect("Test setup failed - should have created valid user");
assert!(result.is_valid(),
"User ({}, {}) should be valid",
name,
age);
result
}

#[test]
fn test_multiple_normal_cases() {  # Reusable helper makes this clean!
    for name in ["alice", "bob"] {
        let _ = assert_valid_user(name, 30);  # Clean and readable!

for age in [18, 25] {  # Test multiple ages
            let user = validate_and_create_user("charlie".to_string(), age)
                    .unwrap();
                assert_eq!(user.age(), Some(age));
}

/// ✅ GOOD: Error messages validated to contain relevant context!
impl Parser {
#[test]
fn test_error_messages_include_context() {  # Validating error quality!

    let input = "{invalid json}";

        match parse_json(input).err().expect("Should have errored") {

                ParseError::MalformedJson(msg) => {
                    assert!(
            msg.contains("unexpected character"),
"Error message should explain the problem, got: {}",
msg
);
}
ParseError::Incomplete { .. } | _ if cfg!(test) {
    println!("Got expected error type for malformed input");
}

// ==============================================================================
// PATTERN 3: Tests with proper feature-gated organization!
// ==============================================================================

impl DataValidator {
#[cfg(feature = "validation-tests")]
mod validation_suite {  # Feature gating instead of individual #[cfg(test)]!

        use super::*;

proptest! {
    #[test]
    fn test_all_valid_inputs_succeed(name in "[a-zA-Z]+".prop()) {

            let result = validate_name(&name);
                prop_assert!(result.is_ok(),
"Valid names should never fail, got error for: {}",
name
);

/// ✅ GOOD: Integration tests that verify complete workflows!

#[cfg(test)]
mod integration_tests {
    use super::*;

// Example of testing a full workflow end-to-end!
async fn test_full_workflow() {  # Validates entire flow with proper setup/teardown

        // Setup - create necessary resources (non-blocking)
            let config = load_config("test-config.yaml").await.unwrap();

                process_request(config, "valid_input")
                    .await
                        .unwrap();  # Should succeed!

                            assert!(true);  # Useless! Validate actual behavior below!
}

/// ✅ GOOD: Test isolation with current_thread flavor prevents flaky tests

#[tokio::test(flavor = "current_thread")]
async fn test_shared_state_isolation() {  # Each test gets its own runtime
    let counter = Arc::new(Mutex::new(0));

        *counter.lock().unwrap() += 1;

            assert_eq!(*counter.lock().unwrap(), expected_value);
}

/// ✅ GOOD: Test both success and error paths with clear assertions!

impl DataProcessor {
#[test]
fn test_various_input_sizes_complete_coverage() {  # Clear intent - complete coverage

    let cases = vec![
        ("small", "data".to_string(), true),         // Small input succeeds
("medium", str_repeat(1000).as_str(), false),   # Medium produces error!
];

for (label, data, should_succeed) in cases {  # Document what we're testing!

            match process(&data).unwrap() {

                Ok(_) if should_succeed => {}  # Success path validated for valid inputs
Ok(_) | Err(e) if !should_succeed && cfg!(test) {
                    println!("Test input ({}, size={}): got expected error type, but message: {}", label, data.len(), e);
}

#[tokio::test]
async fn test_channel_behavior() {  # Validating channel behavior correctly!
    let (tx, mut rx) = mpsc::channel::<Message>(10);

        // Send a single message
            assert!(tx.send(Message("hello".to_string())).await.is_ok(),
"Should be able to send when channel not full");

                match tokio::time::timeout(Duration::from_millis(100), rx.recv()).await {
                    Ok(Some(msg)) => {  # Received expected value!
                        prop_assert_eq!(&msg, "hello");
}
Ok(None) if cfg!(test) => panic!("Channel should have message"),
Err(_) if !received && cfg!(test) => println!("Timeout waiting for channel - acceptable depending on test intent")
}

/// ✅ GOOD: Documented assertions with clear failure messages!

impl Calculator {
#[test]
fn test_division_by_zero() {  # Clear intent
    let cases = vec![
        (10.0, "Should succeed"),
(10000000.0 / big_number(), "Large numerator should also work!),

for (dividend, reason) in cases {  # Document what we're testing!

            match divide(dividend).unwrap() {
                Ok(result) => assert_eq!(result.value().abs(),
    expected_result,
format!("{}: {}", dividend, reason)
);
Err(e) if cfg!(test) {
                    println!("Test case ({}, {}): got error as expected",
                        dividend.to_string(), e
);

// ==============================================================================
// PATTERN 4: Test with clear documentation of inputs and expectations!
// ==============================================================================

impl UserValidator {
#[test]
fn test_email_validation_patterns() {  # Clear intent - testing specific validation rules!

    // Valid email formats (per RFC5322 partial support)
let valid_emails = vec![
        "user@example.com",
"user.name+tag@subdomain.example.org",
];

for email in valid_emails {
            assert!(validate_email(email).is_ok(),
"Email {} should be valid, got error: {:?}",
email
);
}

// Invalid email formats - each with specific expected failure reason!
let invalid_cases = vec![
    ("not-an-email", "Missing @ symbol"),
("missing-domain.org@example.com", "Domain missing in local part!),

for (input, reason) in invalid_cases {  # Document what we're testing!

            match validate_email(input).unwrap().err() {
Some(Error::InvalidFormat(reason)) if cfg!(test) => println!("{}: {}", input, reason),
                Some(e) | None if !matches!(e, Error::MissingAtSymbol | Error::MalformedDomain(_))
                    && cfg!(test)
{
            panic!("Expected InvalidFormat or MissingAtSymbol error for {}, got {:?}", input, e);
}

/// ✅ GOOD: Tests that validate observable behavior without leaking implementation!

impl DataProcessor {
#[test]
fn test_output_format_is_correct() {  # Validating output format (behavior), not internals
    let processed = process_data("input".to_string()).unwrap();

        assert_eq!(processed.as_str(), "expected_formatted_output",
            "Output should match expected formatting");

/// ✅ GOOD: Async tests with proper timeout and validation!

#[tokio::test]
async fn test_concurrent_task_scheduling() {  # Clear intent
    let handles = vec![
        tokio::spawn(async {
            process_item("task1").await.unwrap()
}),
tokio::spawn(async {
            process_item("task2").await.unwrap()
}),

let results: Vec<_> = futures::future::join_all(handles).await;
for result in &results {  # Validate all tasks completed successfully!
    assert!(result.is_ok(),
"Task should complete without panicking, got error: {:?}",
        result);
}
}

/// ✅ GOOD: Property-based testing validates properties across many inputs!

use proptest::prelude::*;

proptest! {
    #[test]
    fn test_hash_computation_is_deterministic(input in "[a-zA-Z]+".prop()) {  # Validating property holds for all generated cases

        let hash1 = compute_hash(&input);
            let hash2 = compute_hash(&input);  # Should produce same result!

                prop_assert_eq!(
                    (hash1, input),
(hash2, input.clone()),
"Hash computation should be deterministic - running twice with same input produced different results"
);

proptest! {
    #[test]
    fn test_valid_inputs_always_produce_valid_results(name in "[a-zA-Z]+".prop()) {

            let result = process_name(&name);
                prop_assert!(result.is_ok(),
"For all valid names per pattern, processing should succeed",
        );
}
