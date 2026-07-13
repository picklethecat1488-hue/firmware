use embedded_cli::arguments::FromArgument;
use firmware_lib::subcommand_enum;

subcommand_enum! {
    /// Test subcommand enum
    pub enum TestSubcommand {
        /// Default variant
        First,
        /// Custom string override
        Second = "custom_second",
        /// Another default
        Third,
    }
    "Invalid test subcommand. Expected: first, custom_second, third"
}

#[test]
fn test_subcommand_enum_parsing() {
    // 1. Test exact matches
    assert_eq!(
        TestSubcommand::from_arg("First").ok(),
        Some(TestSubcommand::First)
    );
    assert_eq!(
        TestSubcommand::from_arg("first").ok(),
        Some(TestSubcommand::First)
    );
    assert_eq!(
        TestSubcommand::from_arg("FIRST").ok(),
        Some(TestSubcommand::First)
    );

    // 2. Test custom override matches (case-insensitively)
    assert_eq!(
        TestSubcommand::from_arg("custom_second").ok(),
        Some(TestSubcommand::Second)
    );
    assert_eq!(
        TestSubcommand::from_arg("CUSTOM_SECOND").ok(),
        Some(TestSubcommand::Second)
    );

    // 3. Test that stringified variant name is NOT matched for custom override
    assert!(TestSubcommand::from_arg("Second").is_err());

    // 4. Test another default match
    assert_eq!(
        TestSubcommand::from_arg("third").ok(),
        Some(TestSubcommand::Third)
    );

    // 5. Test invalid command error message
    let err = TestSubcommand::from_arg("invalid").unwrap_err();
    assert_eq!(err.value, "invalid");
    assert_eq!(
        err.expected,
        "Invalid test subcommand. Expected: first, custom_second, third"
    );
}
