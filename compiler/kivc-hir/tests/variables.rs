//! Variable lowering tests.

mod common;

#[test]
fn test_lower_variable_resolution() {
    let source = r#"
        fun test() {
            let x: Int = 10;
            let y: Int = x;
        }
    "#;

    let hir = common::parse_and_lower(source);
    let func = &hir.functions[0];

    // Check that both statements are Let
    assert!(matches!(
        func.body.stmts[0].kind,
        kivc_hir::HirStmtKind::Let { .. }
    ));
    assert!(matches!(
        func.body.stmts[1].kind,
        kivc_hir::HirStmtKind::Let { .. }
    ));
}

#[test]
fn test_lower_assignment() {
    let source = r#"
        fun test() {
            let mut x: Int = 10;
            x = 20;
        }
    "#;

    let hir = common::parse_and_lower(source);
    assert_eq!(hir.functions[0].body.stmts.len(), 2);
}

#[test]
fn test_lower_scope_shadowing() {
    let source = r#"
        fun test() {
            let x: Int = 1;
            {
                let x: Int = 2;
            }
        }
    "#;

    // This should succeed - shadowing is allowed
    let hir = common::parse_and_lower(source);
    assert_eq!(hir.functions.len(), 1);
}
