//! Unit tests for the conditional tracing facade.

use platform::tracing;

#[tracing::instrument(level = "debug")]
fn test_instrumented_fn(x: u32) -> u32 {
    tracing::debug!("Inside instrumented function with x = {}", x);
    x + 1
}

#[test]
fn test_tracing_facade_execution() {
    let result = test_instrumented_fn(42);
    assert_eq!(result, 43);
}
