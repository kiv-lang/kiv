//! Function parameter type checking tests.

mod common;

#[test]
fn test_typecheck_function_params() {
    let source = r#"
        fun add(a: Int, b: Int): Int {
            return a + b;
        }
    "#;

    assert!(common::parse_lower_and_typecheck(source).is_ok());
}
