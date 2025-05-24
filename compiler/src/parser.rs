use crate::lexer::MorphToken;

#[derive(Debug)]
pub enum ASTNode {
    Number(i64),
    Identifier(String),
    BinaryOp {
        left: Box<ASTNode>,
        op: MorphToken,
        right: Box<ASTNode>,
    },
}

pub struct Parser {
    tokens: Vec<MorphToken>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<MorphToken>) -> Self {
        Self { tokens, position: 0 }
    }

    fn current_token(&self) -> Option<&MorphToken> {
        self.tokens.get(self.position)
    }

    fn eat(&mut self) {
        self.position += 1;
    }

    pub fn parse_expression(&mut self) -> Option<ASTNode> {
        let left = self.parse_primary()?;
        if let Some(op) = self.current_token() {
            if matches!(op, MorphToken::Plus | MorphToken::Minus) {
                self.eat();
                let right = self.parse_primary()?;
                return Some(ASTNode::BinaryOp {
                    left: Box::new(left),
                    op: op.clone(),
                    right: Box::new(right),
                });
            }
        }
        Some(left)
    }

    fn parse_primary(&mut self) -> Option<ASTNode> {
        match self.current_token()? {
            MorphToken::Number => {
                let value = self.tokens[self.position].clone();
                self.eat();
                if let MorphToken::Number(num) = value {
                    return Some(ASTNode::Number(num.parse().unwrap()));
                }
            }
            MorphToken::Identifier => {
                let id = self.tokens[self.position].clone();
                self.eat();
                if let MorphToken::Identifier(name) = id {
                    return Some(ASTNode::Identifier(name));
                }
            }
            _ => return None,
        }
        None
    }
}
