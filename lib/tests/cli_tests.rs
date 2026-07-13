use firmware_lib::subcommand_enum;

subcommand_enum! {
    /// Test subcommand enum
    #[derive(Debug)]
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
    assert_eq!(TestSubcommand::try_from("First"), Ok(TestSubcommand::First));
    assert_eq!(TestSubcommand::try_from("first"), Ok(TestSubcommand::First));
    assert_eq!(TestSubcommand::try_from("FIRST"), Ok(TestSubcommand::First));

    // 2. Test custom override matches (case-insensitively)
    assert_eq!(
        TestSubcommand::try_from("custom_second"),
        Ok(TestSubcommand::Second)
    );
    assert_eq!(
        TestSubcommand::try_from("CUSTOM_SECOND"),
        Ok(TestSubcommand::Second)
    );

    // 3. Test that stringified variant name is NOT matched for custom override
    assert!(TestSubcommand::try_from("Second").is_err());

    // 4. Test another default match
    assert_eq!(TestSubcommand::try_from("third"), Ok(TestSubcommand::Third));

    // 5. Test invalid command error message
    assert_eq!(
        TestSubcommand::try_from("invalid"),
        Err("Invalid test subcommand. Expected: first, custom_second, third")
    );
}
