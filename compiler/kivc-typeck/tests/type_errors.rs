//! Type error detection tests.

mod common;

#[test]
fn test_typecheck_type_mismatch() {
    let source = r#"
        fun main() {
            let x: Int = true;
        }
    "#;

    assert!(common::parse_lower_and_typecheck(source).is_err());
}

#[test]
fn test_typecheck_type_inference() {
    let source = r#"
        fun main() {
            let x = 42;
            let y = x + 10;
        }
    "#;

    assert!(common::parse_lower_and_typecheck(source).is_ok());
}
