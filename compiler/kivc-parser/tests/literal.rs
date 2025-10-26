use kivc_parser::Parser;
use kivc_span::SourceFile;

fn parse_source(source: &str) -> kivc_ast::Program {
    let file = SourceFile::new("test.kiv".to_string(), source.to_string());
    let mut parser = Parser::new(file);
    parser.parse_program()
}

#[test]
fn test_parse_string_literals() {
    let source = r#"
        fun main() {
            let msg: Text = "Hello, world!";
        }
    "#;

    let program = parse_source(source);
    assert_eq!(program.functions[0].body.stmts.len(), 1);
}

#[test]
fn test_parse_bool_literals() {
    let source = r#"
        fun main() {
            let t: Bool = true;
            let f: Bool = false;
        }
    "#;

    let program = parse_source(source);
    assert_eq!(program.functions[0].body.stmts.len(), 2);
}
