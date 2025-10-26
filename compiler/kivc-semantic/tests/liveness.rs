use kivc_diagnostics::DiagnosticsCollector;
use kivc_hir::{
    FunId, HirBlock, HirExpr, HirExprKind, HirFunDef, HirProgram, HirStmt, HirStmtKind, Literal,
    TypeId, VarId,
};
use kivc_semantic::analyze_semantics;
use kivc_span::{SourceFile, Span};

fn dummy_span() -> Span {
    let file = SourceFile::new("test.kiv".to_string(), "test".to_string());
    Span::new(file, 0.into(), 4.into())
}

#[test]
fn test_unused_variable_warning() {
    let span = dummy_span();
    let program = HirProgram {
        functions: vec![HirFunDef {
            fun_id: FunId::new(0),
            name: "test".to_string(),
            params: vec![],
            return_type: None,
            body: HirBlock {
                stmts: vec![HirStmt {
                    span: span.clone(),
                    kind: HirStmtKind::Let {
                        name: "x".to_string(),
                        var_id: VarId::new(0),
                        mutable: false,
                        ty: None,
                        init: HirExpr {
                            span: span.clone(),
                            kind: HirExprKind::Literal(Literal::Int(42)),
                            ty: Some(TypeId::new(0)),
                        },
                    },
                }],
                span: span.clone(),
            },
            span: span.clone(),
        }],
    };

    let mut diagnostics = DiagnosticsCollector::new();
    assert!(analyze_semantics(&program, &mut diagnostics).is_ok());
    assert!(diagnostics.has_warnings());
}

#[test]
fn test_used_variable_no_warning() {
    let span = dummy_span();
    let program = HirProgram {
        functions: vec![HirFunDef {
            fun_id: FunId::new(0),
            name: "test".to_string(),
            params: vec![],
            return_type: None,
            body: HirBlock {
                stmts: vec![
                    HirStmt {
                        span: span.clone(),
                        kind: HirStmtKind::Let {
                            name: "x".to_string(),
                            var_id: VarId::new(0),
                            mutable: false,
                            ty: None,
                            init: HirExpr {
                                span: span.clone(),
                                kind: HirExprKind::Literal(Literal::Int(42)),
                                ty: Some(TypeId::new(0)),
                            },
                        },
                    },
                    HirStmt {
                        span: span.clone(),
                        kind: HirStmtKind::Expr {
                            expr: HirExpr {
                                span: span.clone(),
                                kind: HirExprKind::Var {
                                    var_id: VarId::new(0),
                                    name: "x".to_string(),
                                },
                                ty: Some(TypeId::new(0)),
                            },
                        },
                    },
                ],
                span: span.clone(),
            },
            span: span.clone(),
        }],
    };

    let mut diagnostics = DiagnosticsCollector::new();
    assert!(analyze_semantics(&program, &mut diagnostics).is_ok());
    assert!(!diagnostics.has_warnings());
}
