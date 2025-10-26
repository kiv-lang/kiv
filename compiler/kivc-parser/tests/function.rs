use kivc_parser::Parser;
use kivc_span::SourceFile;

fn parse_source(source: &str) -> kivc_ast::Program {
    let file = SourceFile::new("test.kiv".to_string(), source.to_string());
    let mut parser = Parser::new(file);
    parser.parse_program()
}

#[test]
fn test_parse_simple_function() {
    let source = r#"
        fun main() {
            let x: Int = 42;
        }
    "#;

    let program = parse_source(source);
    assert_eq!(program.functions.len(), 1);
    assert_eq!(program.functions[0].name, "main");
    assert_eq!(program.functions[0].params.len(), 0);
    assert_eq!(program.functions[0].body.stmts.len(), 1);
}

#[test]
fn test_parse_function_with_params() {
    let source = r#"
        fun add(a: Int, b: Int) : Int {
            return a + b;
        }
    "#;

    let program = parse_source(source);
    assert_eq!(program.functions.len(), 1);
    assert_eq!(program.functions[0].params.len(), 2);
    assert!(program.functions[0].ret_ty.is_some());
}

#[test]
fn test_parse_multiple_functions() {
    let source = r#"
        fun foo() {
            let x: Int = 1;
        }
        
        fun bar() {
            let y: Int = 2;
        }
    "#;

    let program = parse_source(source);
    assert_eq!(program.functions.len(), 2);
    assert_eq!(program.functions[0].name, "foo");
    assert_eq!(program.functions[1].name, "bar");
}

#[test]
fn test_parse_return() {
    let source = r#"
        fun get_value() : Int {
            return 42;
        }
    "#;

    let program = parse_source(source);
    assert_eq!(program.functions[0].body.stmts.len(), 1);
}
