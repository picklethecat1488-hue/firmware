// Integration test to verify compile-correctness of the generated sample CLI implementation.

#[allow(dead_code, unused_imports, static_mut_refs, unused_variables)]
mod sample_cli_build_check {
    include!(concat!(env!("OUT_DIR"), "/generated_sample_cli.rs"));
}

#[test]
fn test_sample_cli_compiles() {
    // This is a compilation boundary check. If the generated code fails to compile,
    // this test (and the whole crate build) will fail.
    assert_eq!(2 + 2, 4);
}
