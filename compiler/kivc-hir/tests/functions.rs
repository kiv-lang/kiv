//! Function lowering tests.

mod common;

#[test]
fn test_lower_simple_function() {
    let source = r#"
        fun main() {
            let x: Int = 42;
        }
    "#;

    let hir = common::parse_and_lower(source);
    assert_eq!(hir.functions.len(), 1);
    assert_eq!(hir.functions[0].name, "main");
    assert_eq!(hir.functions[0].body.stmts.len(), 1);
}

#[test]
fn test_lower_function_params() {
    let source = r#"
        fun add(a: Int, b: Int): Int {
            return a + b;
        }
    "#;

    let hir = common::parse_and_lower(source);
    let func = &hir.functions[0];

    assert_eq!(func.params.len(), 2);
    assert_eq!(func.params[0].name, "a");
    assert_eq!(func.params[1].name, "b");
    assert!(func.return_type.is_some());
}

#[test]
fn test_lower_multiple_functions() {
    let source = r#"
        fun foo() {
            let x: Int = 1;
        }
        
        fun bar() {
            let y: Int = 2;
        }
    "#;

    let hir = common::parse_and_lower(source);
    assert_eq!(hir.functions.len(), 2);
    assert_eq!(hir.functions[0].name, "foo");
    assert_eq!(hir.functions[1].name, "bar");
}
