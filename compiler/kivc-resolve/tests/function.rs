use kivc_diagnostics::DiagnosticsCollector;
use kivc_hir::{
    FunId, HirBlock, HirExpr, HirExprKind, HirFunDef, HirParam, HirProgram, HirStmt, HirStmtKind,
    TypeId, VarId,
};
use kivc_resolve::resolve_names;
use kivc_span::{SourceFile, Span};

fn dummy_span() -> Span {
    let file = SourceFile::new("test.kiv".to_string(), "test".to_string());
    Span::new(file, 0.into(), 4.into())
}

#[test]
fn test_resolve_function_parameters() {
    let span = dummy_span();
    let mut program = HirProgram {
        functions: vec![HirFunDef {
            fun_id: FunId::new(0),
            name: "test".to_string(),
            params: vec![HirParam {
                name: "x".to_string(),
                var_id: VarId::new(0),
                ty: TypeId::new(0),
                span: span.clone(),
            }],
            return_type: Some(TypeId::new(0)),
            body: HirBlock {
                stmts: vec![HirStmt {
                    span: span.clone(),
                    kind: HirStmtKind::Return {
                        value: Some(HirExpr {
                            span: span.clone(),
                            kind: HirExprKind::Var {
                                var_id: VarId::new(0),
                                name: "x".to_string(),
                            },
                            ty: Some(TypeId::new(0)),
                        }),
                    },
                }],
                span: span.clone(),
            },
            span: span.clone(),
        }],
    };

    let mut diagnostics = DiagnosticsCollector::new();
    assert!(resolve_names(&mut program, &mut diagnostics).is_ok());
    assert!(!diagnostics.has_errors());
}
