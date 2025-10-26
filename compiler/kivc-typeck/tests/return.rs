//! Return type checking tests.

mod common;

#[test]
fn test_typecheck_return_type() {
    let source = r#"
        fun get_int(): Int {
            return 42;
        }
    "#;

    assert!(common::parse_lower_and_typecheck(source).is_ok());
}

#[test]
fn test_typecheck_return_type_mismatch() {
    let source = r#"
        fun get_int(): Int {
            return true;
        }
    "#;

    assert!(common::parse_lower_and_typecheck(source).is_err());
}
