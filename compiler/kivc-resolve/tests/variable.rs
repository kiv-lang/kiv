use kivc_diagnostics::DiagnosticsCollector;
use kivc_hir::{
    FunId, HirBlock, HirExpr, HirExprKind, HirFunDef, HirProgram, HirStmt, HirStmtKind, Literal,
    TypeId, VarId,
};
use kivc_resolve::resolve_names;
use kivc_span::{SourceFile, Span};

fn dummy_span() -> Span {
    let file = SourceFile::new("test.kiv".to_string(), "test".to_string());
    Span::new(file, 0.into(), 4.into())
}

#[test]
fn test_resolve_simple_variable() {
    let span = dummy_span();
    let mut program = HirProgram {
        functions: vec![HirFunDef {
            fun_id: FunId::new(0),
            name: "main".to_string(),
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
    assert!(resolve_names(&mut program, &mut diagnostics).is_ok());
    assert!(!diagnostics.has_errors());
}

#[test]
fn test_resolve_undefined_variable() {
    let span = dummy_span();
    let mut program = HirProgram {
        functions: vec![HirFunDef {
            fun_id: FunId::new(0),
            name: "main".to_string(),
            params: vec![],
            return_type: None,
            body: HirBlock {
                stmts: vec![HirStmt {
                    span: span.clone(),
                    kind: HirStmtKind::Expr {
                        expr: HirExpr {
                            span: span.clone(),
                            kind: HirExprKind::Var {
                                var_id: VarId::new(999),
                                name: "y".to_string(),
                            },
                            ty: None,
                        },
                    },
                }],
                span: span.clone(),
            },
            span: span.clone(),
        }],
    };

    let mut diagnostics = DiagnosticsCollector::new();
    assert!(resolve_names(&mut program, &mut diagnostics).is_err());
    assert!(diagnostics.has_errors());
}

#[test]
fn test_resolve_assign_to_immutable() {
    let span = dummy_span();
    let mut program = HirProgram {
        functions: vec![HirFunDef {
            fun_id: FunId::new(0),
            name: "main".to_string(),
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
                                kind: HirExprKind::Assign {
                                    var_id: VarId::new(0),
                                    target: "x".to_string(),
                                    value: Box::new(HirExpr {
                                        span: span.clone(),
                                        kind: HirExprKind::Literal(Literal::Int(10)),
                                        ty: Some(TypeId::new(0)),
                                    }),
                                },
                                ty: None,
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
    assert!(resolve_names(&mut program, &mut diagnostics).is_err());
    assert!(diagnostics.has_errors());
}
