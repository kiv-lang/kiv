//! Language feature integration tests.

mod common;
use kivc::OptLevel;

#[test]
fn test_mutable_variable() {
    let source = r#"
        fn main() {
            let mut x = 42;
            x = 10;
        }
    "#;

    let output = common::compile_with_config(source, OptLevel::None);

    assert!(!output.diagnostics.has_errors(), "Mutable assignment should work");
}

#[test]
fn test_function_with_parameters() {
    let source = r#"
        fn add(a: Int, b: Int) -> Int {
            return a + b;
        }
    "#;

    let output = common::compile_with_config(source, OptLevel::None);

    assert!(
        !output.diagnostics.has_errors(),
        "Function with parameters should compile"
    );
}

#[test]
fn test_binary_operations() {
    let source = r#"
        fn main() {
            let a = 1 + 2;
            let b = 3 - 1;
            let c = 2 * 3;
            let d = 10 / 2;
        }
    "#;

    let output = common::compile_with_config(source, OptLevel::None);

    assert!(!output.diagnostics.has_errors(), "Binary operations should compile");
}

#[test]
fn test_if_expression() {
    let source = r#"
        fn main() {
            let x = 5;
            if x > 0 {
                let y = 1;
            } else {
                let z = 2;
            }
        }
    "#;

    let output = common::compile_with_config(source, OptLevel::None);

    assert!(!output.diagnostics.has_errors(), "If expressions should compile");
}

#[test]
fn test_nested_blocks() {
    let source = r#"
        fn main() {
            let x = 1;
            {
                let y = 2;
                {
                    let z = 3;
                }
            }
        }
    "#;

    let output = common::compile_with_config(source, OptLevel::None);

    assert!(!output.diagnostics.has_errors(), "Nested blocks should compile");
}
