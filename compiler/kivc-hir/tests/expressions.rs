//! Expression lowering tests.

mod common;

#[test]
fn test_lower_binary_expr() {
    let source = r#"
        fun test() {
            let result: Int = 1 + 2 * 3;
        }
    "#;

    let hir = common::parse_and_lower(source);
    assert_eq!(hir.functions[0].body.stmts.len(), 1);
}

#[test]
fn test_lower_if_expr() {
    let source = r#"
        fun test() {
            let x: Int = if true { 1 } else { 2 };
        }
    "#;

    let hir = common::parse_and_lower(source);
    assert_eq!(hir.functions[0].body.stmts.len(), 1);
}

#[test]
fn test_lower_function_call() {
    let source = r#"
        fun main() {
            let result: Int = add(1, 2);
        }
    "#;

    let hir = common::parse_and_lower(source);
    assert_eq!(hir.functions[0].body.stmts.len(), 1);
}
