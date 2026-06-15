use super::command::CommandKind;
use super::error::DslError;
use super::token::{Token, TokenKind};

/// Lexer minimo para convertir scripts DSL en tokens.
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    /// Crea un lexer nuevo para un script DSL.
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Convierte todo el script en una lista de tokens.
    pub fn tokenize(&mut self) -> Result<Vec<Token>, DslError> {
        let mut tokens = Vec::new();

        while let Some(current) = self.peek_char() {
            match current {
                ' ' | '\t' | '\r' => {
                    self.advance_char();
                }
                '\n' => {
                    let line = self.line;
                    let column = self.column;
                    self.advance_char();
                    tokens.push(Token::new(TokenKind::Newline, line, column));
                }
                '#' => tokens.push(self.read_comment()),
                '/' => tokens.push(self.read_command()?),
                '-' if self.peek_next_char() == Some('>') => {
                    let line = self.line;
                    let column = self.column;
                    self.advance_char();
                    self.advance_char();
                    tokens.push(Token::new(TokenKind::Arrow, line, column));
                }
                '0'..='9' => tokens.push(self.read_number()),
                '"' => tokens.push(self.read_string()?),
                character if is_identifier_start(character) => tokens.push(self.read_identifier()),
                character => {
                    return Err(DslError::unexpected_character(
                        character,
                        self.line,
                        self.column,
                    ));
                }
            }
        }

        tokens.push(Token::new(TokenKind::Eof, self.line, self.column));
        Ok(tokens)
    }

    fn read_command(&mut self) -> Result<Token, DslError> {
        let line = self.line;
        let column = self.column;
        let mut value = String::new();

        while let Some(character) = self.peek_char() {
            if value.is_empty() {
                value.push(character);
                self.advance_char();
                continue;
            }

            if is_command_body(character) {
                value.push(character);
                self.advance_char();
            } else {
                break;
            }
        }

        let command = CommandKind::from_keyword(&value)?;
        Ok(Token::new(TokenKind::Command(command), line, column))
    }

    fn read_identifier(&mut self) -> Token {
        let line = self.line;
        let column = self.column;
        let mut value = String::new();

        while let Some(character) = self.peek_char() {
            if is_identifier_body(character) {
                value.push(character);
                self.advance_char();
            } else {
                break;
            }
        }

        Token::new(TokenKind::Identifier(value), line, column)
    }

    fn read_number(&mut self) -> Token {
        let line = self.line;
        let column = self.column;
        let mut value = String::new();

        while let Some(character) = self.peek_char() {
            if character.is_ascii_digit() {
                value.push(character);
                self.advance_char();
            } else {
                break;
            }
        }

        let number = value.parse::<u64>().unwrap_or(0);
        Token::new(TokenKind::Number(number), line, column)
    }

    fn read_string(&mut self) -> Result<Token, DslError> {
        let line = self.line;
        let column = self.column;
        let mut value = String::new();

        self.advance_char();

        while let Some(character) = self.peek_char() {
            match character {
                '"' => {
                    self.advance_char();
                    return Ok(Token::new(TokenKind::StringLiteral(value), line, column));
                }
                '\n' => return Err(DslError::unterminated_string(line, column)),
                '\\' => {
                    self.advance_char();
                    if let Some(escaped) = self.peek_char() {
                        value.push(escaped);
                        self.advance_char();
                    } else {
                        return Err(DslError::unterminated_string(line, column));
                    }
                }
                _ => {
                    value.push(character);
                    self.advance_char();
                }
            }
        }

        Err(DslError::unterminated_string(line, column))
    }

    fn read_comment(&mut self) -> Token {
        let line = self.line;
        let column = self.column;
        let mut value = String::new();

        self.advance_char();

        while let Some(character) = self.peek_char() {
            if character == '\n' {
                break;
            }
            value.push(character);
            self.advance_char();
        }

        Token::new(TokenKind::Comment(value.trim().to_string()), line, column)
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    fn peek_next_char(&self) -> Option<char> {
        let mut chars = self.input[self.position..].chars();
        chars.next()?;
        chars.next()
    }

    fn advance_char(&mut self) -> Option<char> {
        let character = self.peek_char()?;
        self.position += character.len_utf8();

        if character == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        Some(character)
    }
}

/// Convierte un script completo en tokens.
pub fn lex(input: &str) -> Result<Vec<Token>, DslError> {
    Lexer::new(input).tokenize()
}

fn is_command_body(character: char) -> bool {
    character.is_ascii_alphabetic() || character == '_'
}

fn is_identifier_start(character: char) -> bool {
    character.is_ascii_alphabetic() || character == '_'
}

fn is_identifier_body(character: char) -> bool {
    character.is_ascii_alphanumeric() || matches!(character, '_' | '-' | '.')
}

#[cfg(test)]
mod tests {
    use super::lex;
    use crate::dsl::token::TokenKind;
    use crate::dsl::CommandKind;

    fn token_kinds(input: &str) -> Vec<TokenKind> {
        lex(input)
            .expect("debe tokenizar script")
            .into_iter()
            .map(|token| token.kind)
            .collect()
    }

    #[test]
    fn lexer_recognizes_slash_commands() {
        let tokens = token_kinds("/goal rescatar_victimas");

        assert_eq!(tokens[0], TokenKind::Command(CommandKind::Goal));
        assert_eq!(
            tokens[1],
            TokenKind::Identifier("rescatar_victimas".to_string())
        );
    }

    #[test]
    fn lexer_recognizes_plan_arrows() {
        let tokens = token_kinds("/plan buscar -> clasificar -> asistir");

        assert_eq!(tokens[0], TokenKind::Command(CommandKind::Plan));
        assert_eq!(tokens[2], TokenKind::Arrow);
        assert_eq!(tokens[4], TokenKind::Arrow);
    }

    #[test]
    fn lexer_recognizes_numbers_strings_newlines_and_comments() {
        let tokens = token_kinds(
            "# caso base\n/delegate sector_a worker_1\n/verify \"checklist final\"\n/terminate 10",
        );

        assert_eq!(tokens[0], TokenKind::Comment("caso base".to_string()));
        assert_eq!(tokens[1], TokenKind::Newline);
        assert_eq!(tokens[2], TokenKind::Command(CommandKind::Delegate));
        assert_eq!(tokens[6], TokenKind::Command(CommandKind::Verify));
        assert_eq!(
            tokens[7],
            TokenKind::StringLiteral("checklist final".to_string())
        );
        assert_eq!(tokens[10], TokenKind::Number(10));
    }

    #[test]
    fn lexer_reports_unknown_command() {
        let error = lex("/otro valor").expect_err("debe rechazar comando desconocido");

        assert_eq!(error.to_string(), "comando DSL desconocido: /otro");
    }

    #[test]
    fn lexer_reports_unexpected_character() {
        let error = lex("/goal @").expect_err("debe rechazar caracter inesperado");

        assert_eq!(
            error.to_string(),
            "caracter inesperado en linea 1, columna 7: @"
        );
    }

    #[test]
    fn lexer_reports_unterminated_string() {
        let error = lex("/verify \"sin cierre").expect_err("debe rechazar cadena sin cierre");

        assert_eq!(error.to_string(), "cadena sin cierre en linea 1, columna 9");
    }
}
