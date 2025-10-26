//! MIR function generation tests.

mod common;

#[test]
fn test_mir_simple_function() {
    let source = r#"
        fun main() {
            let x: Int = 42;
        }
    "#;

    let mir = common::parse_lower_typecheck_and_mir(source);
    assert_eq!(mir.functions.len(), 1);
    assert_eq!(mir.functions[0].name, "main");
    assert!(!mir.functions[0].blocks.is_empty());
}

#[test]
fn test_mir_function_params() {
    let source = r#"
        fun add(a: Int, b: Int) : Int {
            return a + b;
        }
    "#;

    let mir = common::parse_lower_typecheck_and_mir(source);
    let func = &mir.functions[0];

    assert_eq!(func.params.len(), 2);
}
