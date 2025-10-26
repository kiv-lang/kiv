//! Error detection integration tests.

mod common;
use kivc::OptLevel;

#[test]
fn test_compile_with_type_error() {
    let source = r#"
        fn main() {
            let x: Int = true;
        }
    "#;

    let output = common::compile_with_config(source, OptLevel::None);

    // Should have type errors
    assert!(output.diagnostics.has_errors(), "Should have type errors");
}

#[test]
fn test_compile_undefined_variable() {
    let source = r#"
        fn main() {
            let x = y;
        }
    "#;

    let output = common::compile_with_config(source, OptLevel::None);

    // Should have resolution errors
    assert!(output.diagnostics.has_errors(), "Should have undefined variable error");
}

#[test]
fn test_immutable_assignment_error() {
    let source = r#"
        fn main() {
            let x = 42;
            x = 10;
        }
    "#;

    let output = common::compile_with_config(source, OptLevel::None);

    assert!(output.diagnostics.has_errors(), "Should error on immutable assignment");
}
