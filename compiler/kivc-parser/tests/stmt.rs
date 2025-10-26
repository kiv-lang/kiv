use kivc_parser::Parser;
use kivc_span::SourceFile;

fn parse_source(source: &str) -> kivc_ast::Program {
    let file = SourceFile::new("test.kiv".to_string(), source.to_string());
    let mut parser = Parser::new(file);
    parser.parse_program()
}

#[test]
fn test_parse_assignment() {
    let source = r#"
        fun main() {
            let mut x: Int = 10;
            x = 20;
        }
    "#;

    let program = parse_source(source);
    assert_eq!(program.functions[0].body.stmts.len(), 2);
}

#[test]
fn test_parse_const() {
    let source = r#"
        fun main() {
            const PI = 3.14;
        }
    "#;

    let program = parse_source(source);
    assert_eq!(program.functions[0].body.stmts.len(), 1);
}
