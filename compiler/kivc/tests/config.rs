//! Configuration integration tests.

mod common;
use kivc::{CompilerConfig, OptLevel, StopAfter, compile_source};

#[test]
fn test_compile_with_optimization() {
    let source = r#"
        fn main() {
            let x = 1 + 2;
        }
    "#;

    let config = CompilerConfig::new(OptLevel::Aggressive);
    let output = compile_source(source, "test.kiv", &config);

    assert!(!output.diagnostics.has_errors(), "Compilation should succeed");
}

#[test]
fn test_stop_after_parse() {
    let source = r#"
        fn main() {
            let x = 42;
        }
    "#;

    let config = CompilerConfig {
        stop_after: Some(StopAfter::Parse),
        ..Default::default()
    };
    let output = compile_source(source, "test.kiv", &config);

    assert!(!output.diagnostics.has_errors());
    assert!(output.ast.is_some(), "AST should be generated");
}

#[test]
fn test_stop_after_typecheck() {
    let source = r#"
        fn main() {
            let x: Int = 42;
        }
    "#;

    let config = CompilerConfig {
        stop_after: Some(StopAfter::TypeCheck),
        ..Default::default()
    };
    let output = compile_source(source, "test.kiv", &config);

    assert!(!output.diagnostics.has_errors());
    assert!(output.ast.is_some());
}
