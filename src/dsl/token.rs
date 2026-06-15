use serde::{Deserialize, Serialize};

use super::command::CommandKind;

/// Rango de origen de un token dentro del script DSL.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TokenSpan {
    pub line: usize,
    pub column: usize,
}

impl TokenSpan {
    /// Crea un rango de origen con linea y columna basadas en uno.
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

/// Tipo de token reconocido por el lexer del DSL.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TokenKind {
    Command(CommandKind),
    Identifier(String),
    Number(u64),
    StringLiteral(String),
    Arrow,
    Newline,
    Comment(String),
    Eof,
}

/// Token con tipo y posicion de origen.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub kind: TokenKind,
    pub span: TokenSpan,
}

impl Token {
    /// Construye un token con su posicion de origen.
    pub fn new(kind: TokenKind, line: usize, column: usize) -> Self {
        Self {
            kind,
            span: TokenSpan::new(line, column),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Token, TokenKind};
    use crate::dsl::CommandKind;

    #[test]
    fn token_stores_span() {
        let token = Token::new(TokenKind::Command(CommandKind::Goal), 2, 4);

        assert_eq!(token.span.line, 2);
        assert_eq!(token.span.column, 4);
    }

    #[test]
    fn token_kind_can_store_values() {
        assert_eq!(
            TokenKind::Identifier("sector_a".to_string()),
            TokenKind::Identifier("sector_a".to_string())
        );
        assert_eq!(TokenKind::Number(10), TokenKind::Number(10));
        assert_eq!(
            TokenKind::StringLiteral("texto".to_string()),
            TokenKind::StringLiteral("texto".to_string())
        );
    }
}
