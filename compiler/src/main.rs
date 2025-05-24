mod lexer;
mod parser;
mod compiler;

use lexer::tokenize;
use parser::Parser;
use compiler::compile;

fn main() {
    let source_code = "5 + 3";
    
    // Tokenization
    let tokens = tokenize(source_code);
    println!("Tokens: {:?}", tokens);

    // Parsing
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_expression().expect("Failed to parse");
    println!("AST: {:?}", ast);

    // Compilation
    let compiled_code = compile(&ast);
    println!("Compiled Output: {}", compiled_code);
}
