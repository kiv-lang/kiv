//! Basic type checking tests.

mod common;

#[test]
fn test_typecheck_simple() {
    let source = r#"
        fun main() {
            let x: Int = 42;
        }
    "#;

    assert!(common::parse_lower_and_typecheck(source).is_ok());
}

#[test]
fn test_typecheck_binary_op() {
    let source = r#"
        fun main() {
            let x: Int = 1 + 2;
        }
    "#;

    assert!(common::parse_lower_and_typecheck(source).is_ok());
}

#[test]
fn test_typecheck_comparison() {
    let source = r#"
        fun main() {
            let x: Bool = 5 > 3;
        }
    "#;

    assert!(common::parse_lower_and_typecheck(source).is_ok());
}
