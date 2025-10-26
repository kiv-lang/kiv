//! MIR terminator tests.

mod common;

#[test]
fn test_mir_return() {
    let source = r#"
        fun get_value(): Int {
            return 100;
        }
    "#;

    let mir = common::parse_lower_typecheck_and_mir(source);
    let func = &mir.functions[0];

    // Check that there's a Return terminator
    let has_return = func
        .blocks
        .iter()
        .any(|block| matches!(block.terminator, kivc_mir::MirTerminator::Return { .. }));
    assert!(has_return);
}
