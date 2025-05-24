use logos::{Logos, Lexer};

#[derive(Debug, Logos, PartialEq)]
pub enum MorphToken {
    // Keywords
    #[token("let")]
    Let,
    #[token("fn")]
    Function,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("return")]
    Return,

    // Operators
    #[token("=")]
    Assign,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Multiply,
    #[token("/")]
    Divide,

    // Symbols
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token(";")]
    Semicolon,

    // Identifiers and Numbers
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", priority = 2)]
    Identifier,
    #[regex(r"[0-9]+", priority = 1)]
    Number,

    // Whitespace (ignored)
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,

    // Catch-All for Invalid Tokens
    #[error]
    Error,
}

// Function to tokenize input source code
pub fn tokenize(input: &str) -> Vec<MorphToken> {
    let lexer = MorphToken::lexer(input);
    lexer.collect()
}

// Lexer Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = "let x = 5 + 3;";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            vec![
                MorphToken::Let,
                MorphToken::Identifier,
                MorphToken::Assign,
                MorphToken::Number,
                MorphToken::Plus,
                MorphToken::Number,
                MorphToken::Semicolon,
            ]
        );
    }
}
