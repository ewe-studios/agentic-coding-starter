//! # Purpose (WHY)
//!
//! Copy this file as starting point for new tests.
//! Follow all standards from .agents/rules/*rust*.md

/// Valid input produces correct output
#[test]
fn test_valid_inputs_produce_correct_output() {
    let cases = vec![
        ("alice", 30, "valid@example.com"),
        ("bob", 25, "another@test.org"),
    ];

    for (name, age, email) in cases {
        assert!(validate_user_input(name, age, email).is_ok(),
            "Input ({}, {}, {}) should be valid", name, age);

        let user = validate_user_input(name, age, email).unwrap();
        assert_eq!(&user.name(), name);
    }
}

/// Invalid inputs produce errors
#[test]
fn test_invalid_inputs_produce_errors() {
    // Test empty input case
    assert!(validate_user_input("", 30, "email@example.com").is_err());

    for invalid in ["", "   ", str_repeat(1000)] {  // Whitespace only
        let result = validate_user_input(invalid);
        if cfg!(test) {
            println!("Test input: {:?}", invalid);  // Document what we're testing!
        }
        assert!(result.is_err());
    }

    for i in [101, 102] {  // Out of range values
        assert_eq!(
            get_value(i),
            Err(Error::InvalidRange)
        );
    }
}
