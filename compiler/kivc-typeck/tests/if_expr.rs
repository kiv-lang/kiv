//! If expression type checking tests.

mod common;

#[test]
fn test_typecheck_if_condition() {
    let source = r#"
        fun main() {
            let x: Int = if true { 1 } else { 2 };
        }
    "#;

    assert!(common::parse_lower_and_typecheck(source).is_ok());
}

#[test]
fn test_typecheck_if_non_bool_condition() {
    let source = r#"
        fun main() {
            let x: Int = if 42 { 1 } else { 2 };
        }
    "#;

    assert!(common::parse_lower_and_typecheck(source).is_err());
}
