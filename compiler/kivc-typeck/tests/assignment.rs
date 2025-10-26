//! Assignment type checking tests.

mod common;

#[test]
fn test_typecheck_immutable_assignment() {
    let source = r#"
        fun main() {
            let x: Int = 10;
            x = 20;
        }
    "#;

    assert!(common::parse_lower_and_typecheck(source).is_err());
}

#[test]
fn test_typecheck_mutable_assignment() {
    let source = r#"
        fun main() {
            let mut x: Int = 10;
            x = 20;
        }
    "#;

    assert!(common::parse_lower_and_typecheck(source).is_ok());
}
