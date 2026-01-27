// Examples of BAD testing patterns to avoid
//
// These demonstrate common test anti-patterns that produce useless or fragile tests.

#[cfg(test)]
mod bad_examples {
    use super::*;

    // ==============================================================================
    // ANTI-PATTERN 1: Tests without assertions (muted variables)
    // ==============================================================================

    /// ❌ BAD: Just calling a function with no verification of what happens
    #[test]
    fn test_process_without_validation() {
        let input = "valid_input";
        process(input).unwrap();  // Assumes success, doesn't verify anything!
        assert!(true);  // Useless assertion - always passes if code reached here!

    /// ❌ BAD: Only checking that function returns Ok without validating the result
    #[test]
    fn test_valid_input_returns_ok() {
        let data = "some_data".to_string();
        process(data).unwrap();  // Returns Result<T, E> but we ignore T!
        assert!(true);  // Useless - just confirms no panic occurred

    /// ❌ BAD: Checking the function completes without error
    #[test]
    fn test_function_completes() {
        let future = async_operation().await;
        if cfg!(feature = "async") {
            println!("Future executed");  // Just logging, not asserting!
        }
        assert!(true);  // Useless - only confirms no panic

    /// ❌ BAD: Test that panics but doesn't document why or what condition
    #[test]
    fn test_edge_case() {  // Which edge case? What value?
        let input = vec![];
        process(input).unwrap();  // Panics, we don't know WHY it's valid to panic here!
}

// ==============================================================================
// ANTI-PATTERN 2: Hardcoded inputs without validation
// ==============================================================================

/// ❌ BAD: Testing only one specific case with no assertion about the result value
#[test]
fn test_user_creation() {
    let name = "Alice";
    let user = User::new(name).unwrap();
    assert!(user.is_valid());
}

/// ❌ BETTER but still incomplete - should validate actual output values too!
impl MyModule {
    #[test]
    fn test_normal_cases_only() {  // Only tests success path, no error paths
        for name in ["alice", "bob"] {
            let user = User::new(name).unwrap();
                assert!(user.is_valid());
                assert_eq!(&user.name(), name);  // Validating output - better!
}
}

/// ❌ BAD: Testing with hardcoded inputs that might be wrong or edge cases not covered
#[test]
fn test_large_data() {
    let data = vec![1usize; 1000];  // Magic number without explanation!

        process(&data).unwrap();  // Assumes success for ALL sizes!
            assert!(result.is_ok());  // Useless - just confirms no panic

/// ❌ BETTER: Test multiple input ranges with specific expectations
impl DataProcessor {
    #[test]
    fn test_various_input_sizes() {  // Tests both small and large inputs
        let cases = vec![
            ("small", "data".to_string(), true),
            ("large", str_repeat(1000), false),  // Large input should produce error!
];

for (label, data, expected_ok) in cases {
    match process(&data).unwrap() {  // This will panic for large case - not testing what we think
        Ok(_) if !expected_ok => assert!(false, "Expected failure but got success"),
        Err(e) if expected_ok => assert!(format!("{:?}", e), format!("Unexpected error: {}", label)),
        _ => {}  // Success path handled elsewhere or validated correctly!
}
}

// ==============================================================================
// ANTI-PATTERN 3: Testing implementation details instead of behavior
// ==============================================================================

/// ❌ BAD: Inspecting internal state to validate result - leaks implementation detail
#[test]
fn test_internal_state_not_validated() {
    let mock = MockDataProcessor::new();
    process(&mock, "input");

        // What are we actually validating? The side effect or the method call?
            assert_eq!(internal_counter.get(), 1);  // Implementation leakage!

/// ❌ BETTER: Validate observable behavior only
impl DataProcessor {
    #[test]
    fn test_output_is_correct() {  // Validates result, not internals!
        let input = "valid_input";
        let output = process(input).unwrap();

            assert_eq!(output.value(), Some("expected_value"));
}

/// ❌ BAD: Testing exact error message content (brittle)
#[test]
fn test_error_message_content() {
    match parse_json("{}") {  // Valid JSON should succeed!
        Err(e) => panic!("Unexpected failure for valid input"),
        Ok(v) => {}
}
match parse_json("{invalid}") {
            Err(ParseError::MalformedJson(msg)) => assert!(msg.contains("unexpected character")),
            _ => panic!("Should produce MalformedJson error"),  // Brittle - message could change!
}

// ==============================================================================
// ANTI-PATTERN 4: No validation of invalid inputs
// ==============================================================================

/// ❌ BAD: Only testing valid input, never checking error handling for bad data
#[test]
fn test_valid_input_only() {  // Error paths completely untested!

    let good = "valid_data";
        process(good).unwrap();
            assert!(true);  // Useless - no assertions about what was validated!
}

/// ❌ BETTER: Test both valid and invalid inputs produce correct results
impl UserValidator {
    #[test]
    fn test_validation_both_sides() {  // Tests success AND error paths

        for name in ["alice", "bob"] {
            assert!(validate_name(name).is_ok(), "{} should be valid", name);
}

for bad_input in ["", "   ", str_repeat(1000)] {

                    let result = validate_name(bad_input);

                        if cfg!(test) {  // Document what we're testing!
                            println!("Test input: {:?}", bad_input);
}
assert!(
            !result.is_ok(),
        format!("Input {:?} should be invalid", bad_input)
    );
}

// ==============================================================================
// ANTI-PATTERN 5: Async tests with shared mutable state (flaky!)
// ==============================================================================

/// ❌ BAD: Static or global mutable state causes race conditions in parallel test runs
use std::sync::{Arc, Mutex};

static SHARED_COUNTER: Arc<Mutex<usize>> = ...;

#[tokio::test]
async fn concurrent_test() {  // Flaky - multiple tests running simultaneously share this!
    *SHARED_COUNTER.lock().unwrap() += 1;
        assert_eq!(*SHARED_COUNTER.lock().unwrap(), expected_value);
}

/// ❌ BETTER: Use local state or isolate with current_thread flavor
#[tokio::test(flavor = "current_thread")]
async fn isolated_async_test() {  // Each test gets its own runtime!
    let counter = Arc::new(Mutex::new(0));
        *counter.lock().unwrap() += 1;
            assert_eq!(*counter.lock().unwrap(), expected_value);
}

// ==============================================================================
// ANTI-PATTERN 6: Tests that don't document inputs or what's validated
// ==============================================================================

/// ❌ BAD: No documentation of test intent, input values tested, etc.
#[test]
fn example() {  // What does this function do? Which edge cases?

    let data = vec![1];
        process(data).unwrap();
            assert!(result.is_some());
}

/// ❌ BETTER: Documented with clear purpose and validated inputs
impl DataProcessor {
    #[test]
    fn test_single_item_processing() {  // Clear intent!
        for input in [vec![42], vec!["single"]] {

                let result = process(input).unwrap();

                    assert!(result.is_some(),
            format!("Single item should produce valid output, got: {:?}", result));
}
}

// ==============================================================================
// ANTI-PATTERN 7: Property-based testing without assertions
// ==============================================================================

/// ❌ BAD: Generating inputs but not asserting properties about outputs
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_with_proptest(input in "[a-zA-Z]+".prop()) {  // Generates names, what to validate?

        let result = process(&input).unwrap();
            assert!(result.is_ok());  // Useless - assumes ALL inputs succeed!
}

/// ❌ BETTER: Validate that property holds for all generated inputs
proptest! {
    #[test]
    fn test_idempotent(input in "[a-zA-Z]+".prop()) {  // Test specific properties

        let first = compute_hash(&input);
            let second = compute_hash(&input);  // Should produce same result!

                prop_assert_eq!(first, second,
                    "Hash computation should be deterministic for the same input");
}
}

/// ❌ BETTER: Validate that valid inputs always succeed
proptest! {
    #[test]
    fn test_valid_inputs_always_succeed(name in "[a-zA-Z]+".prop()) {

        let result = process(&name);
            prop_assert!(result.is_ok(),
                "Valid names should never fail, got error for input {:?}",
}
}

// ==============================================================================
// ANTI-PATTERN 8: Error path tests that don't validate specific behavior
// ==============================================================================

/// ❌ BAD: Just verifying an error is returned without checking it's the right kind of error!
#[test]
fn test_invalid_input_returns_error() {
    match process("") {  // Empty string should fail!

        Ok(_) => panic!("Should have errored"),
        Err(e) if e.is_type::<MyError>() && !e.is_other_types(),  // Too specific and brittle
}

/// ❌ BETTER: Validate the correct error type AND message contains relevant context!
impl DataProcessor {
    #[test]
    fn test_invalid_input_returns_correct_error() {  // Clear intent - validating behavior

        for invalid in ["", "   ", str_repeat(1000)] {

                let result = process(invalid);

                    assert!(
            !result.is_ok(),
      format!("Input {:?} should produce error, got {:?}", invalid, result)
    );

                        if cfg!(test) {
                            println!("Test input: {}", invalid);  // Document what we're testing!
}

/// ❌ BETTER (even more specific): Validate the exact expected behavior
#[tokio::test]
async fn test_timeout_behavior() {  // Testing timeout with actual time passage

    let start = tokio::time::Instant::now();
        assert!(start.elapsed().is_zero());  // Time hasn't advanced yet!

            tokio::time::sleep(Duration::from_millis(100)).await;
                assert_eq!(
                    start.elapsed(),
                    Duration::from_millis(100),
"Timeout behavior test: time should have elapsed by expected amount"
);
}
