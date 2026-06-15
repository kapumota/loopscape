use serde::{Deserialize, Serialize};

/// Error estructurado del DSL de orquestacion.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DslError {
    UnknownCommand {
        keyword: String,
    },
    EmptyArguments {
        command: String,
    },
    InvalidProgram {
        reason: String,
    },
    UnexpectedCharacter {
        character: String,
        line: usize,
        column: usize,
    },
    UnterminatedString {
        line: usize,
        column: usize,
    },
}

impl DslError {
    /// Crea un error de programa invalido con mensaje en espanol.
    pub fn invalid_program(reason: impl Into<String>) -> Self {
        Self::InvalidProgram {
            reason: reason.into(),
        }
    }

    /// Crea un error lexico para un caracter no reconocido.
    pub fn unexpected_character(character: char, line: usize, column: usize) -> Self {
        Self::UnexpectedCharacter {
            character: character.to_string(),
            line,
            column,
        }
    }

    /// Crea un error lexico para una cadena sin cierre.
    pub fn unterminated_string(line: usize, column: usize) -> Self {
        Self::UnterminatedString { line, column }
    }
}

impl std::fmt::Display for DslError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownCommand { keyword } => {
                write!(formatter, "comando DSL desconocido: {keyword}")
            }
            Self::EmptyArguments { command } => {
                write!(formatter, "el comando {command} necesita argumentos")
            }
            Self::InvalidProgram { reason } => formatter.write_str(reason),
            Self::UnexpectedCharacter {
                character,
                line,
                column,
            } => write!(
                formatter,
                "caracter inesperado en linea {line}, columna {column}: {character}"
            ),
            Self::UnterminatedString { line, column } => write!(
                formatter,
                "cadena sin cierre en linea {line}, columna {column}"
            ),
        }
    }
}

impl std::error::Error for DslError {}

#[cfg(test)]
mod tests {
    use super::DslError;

    #[test]
    fn unknown_command_has_spanish_message() {
        let error = DslError::UnknownCommand {
            keyword: "/otro".to_string(),
        };

        assert_eq!(error.to_string(), "comando DSL desconocido: /otro");
    }

    #[test]
    fn empty_arguments_has_spanish_message() {
        let error = DslError::EmptyArguments {
            command: "/goal".to_string(),
        };

        assert_eq!(error.to_string(), "el comando /goal necesita argumentos");
    }

    #[test]
    fn unexpected_character_has_spanish_message() {
        let error = DslError::unexpected_character('@', 3, 7);

        assert_eq!(
            error.to_string(),
            "caracter inesperado en linea 3, columna 7: @"
        );
    }

    #[test]
    fn unterminated_string_has_spanish_message() {
        let error = DslError::unterminated_string(5, 2);

        assert_eq!(error.to_string(), "cadena sin cierre en linea 5, columna 2");
    }
}
