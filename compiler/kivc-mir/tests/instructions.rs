//! MIR instruction tests.

mod common;

#[test]
fn test_mir_binary_op() {
    let source = r#"
        fun test() {
            let x: Int = 1 + 2;
        }
    "#;

    let mir = common::parse_lower_typecheck_and_mir(source);
    assert_eq!(mir.functions.len(), 1);

    let func = &mir.functions[0];
    assert!(!func.blocks.is_empty());

    // Check that there's a BinOp instruction
    let has_binop = func.blocks.iter().any(|block| {
        block
            .instructions
            .iter()
            .any(|instr| matches!(instr, kivc_mir::MirInstr::BinOp { .. }))
    });
    assert!(has_binop);
}

#[test]
fn test_mir_assignment() {
    let source = r#"
        fun test() {
            let mut x: Int = 10;
            x = 20;
        }
    "#;

    let mir = common::parse_lower_typecheck_and_mir(source);
    let func = &mir.functions[0];

    // Check for Assign instructions
    let assign_count = func
        .blocks
        .iter()
        .flat_map(|block| &block.instructions)
        .filter(|instr| matches!(instr, kivc_mir::MirInstr::Assign { .. }))
        .count();

    assert!(assign_count >= 2); // At least two assignments
}
