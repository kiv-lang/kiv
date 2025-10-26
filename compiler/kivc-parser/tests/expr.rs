use kivc_parser::Parser;
use kivc_span::SourceFile;

fn parse_source(source: &str) -> kivc_ast::Program {
    let file = SourceFile::new("test.kiv".to_string(), source.to_string());
    let mut parser = Parser::new(file);
    parser.parse_program()
}

#[test]
fn test_parse_binary_operators() {
    let source = r#"
        fun test() {
            let a: Int = 1 + 2 * 3;
            let b: Bool = 5 > 3;
            let c: Bool = 10 == 10;
        }
    "#;

    let program = parse_source(source);
    assert_eq!(program.functions[0].body.stmts.len(), 3);
}

#[test]
fn test_parse_if_expression() {
    let source = r#"
        fun test() {
            let x: Int = if true { 1 } else { 2 };
        }
    "#;

    let program = parse_source(source);
    assert_eq!(program.functions[0].body.stmts.len(), 1);
}

#[test]
fn test_parse_function_call() {
    let source = r#"
        fun main() {
            let result: Int = add(1, 2);
        }
    "#;

    let program = parse_source(source);
    assert_eq!(program.functions[0].body.stmts.len(), 1);
}

#[test]
fn test_parse_operator_precedence() {
    let source = r#"
        fun test() {
            let x: Int = 1 + 2 * 3;
            let y: Int = (1 + 2) * 3;
        }
    "#;

    let program = parse_source(source);
    assert_eq!(program.functions[0].body.stmts.len(), 2);
}
