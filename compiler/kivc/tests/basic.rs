//! Basic integration tests.

mod common;
use kivc::OptLevel;

#[test]
fn test_compile_simple_program() {
    let source = r#"
        fn main() {
            let x = 42;
        }
    "#;

    let output = common::compile_with_config(source, OptLevel::None);

    // Should compile without errors
    assert!(!output.diagnostics.has_errors(), "Compilation should succeed");
}
