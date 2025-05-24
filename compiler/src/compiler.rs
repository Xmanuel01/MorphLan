use crate::parser::ASTNode;

pub fn compile(ast: &ASTNode) -> String {
    match ast {
        ASTNode::Number(value) => value.to_string(),
        ASTNode::Identifier(name) => name.clone(),
        ASTNode::BinaryOp { left, op, right } => {
            let left_code = compile(left);
            let right_code = compile(right);
            let op_symbol = match op {
                crate::lexer::MorphToken::Plus => "+",
                crate::lexer::MorphToken::Minus => "-",
                _ => panic!("Unsupported operator"),
            };
            format!("({} {} {})", left_code, op_symbol, right_code)
        }
    }
}

// Compiler Test
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{ASTNode};
    
    #[test]
    fn test_compiler() {
        let ast = ASTNode::BinaryOp {
            left: Box::new(ASTNode::Number(5)),
            op: crate::lexer::MorphToken::Plus,
            right: Box::new(ASTNode::Number(3)),
        };
        assert_eq!(compile(&ast), "(5 + 3)");
    }
}
