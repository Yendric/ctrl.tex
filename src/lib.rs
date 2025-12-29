pub mod ast;
pub mod lexer;
pub mod parser;
pub mod renderer;
pub mod token;

pub fn convert_latex_to_unicode(input: &str) -> String {
    let lexer = lexer::Lexer::new(input);
    let mut parser = parser::Parser::new(lexer);
    let ast = parser.parse();
    let renderer = renderer::Renderer::new();
    renderer.render(&ast)
}
