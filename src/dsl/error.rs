use serde::{Deserialize, Serialize};

/// Error estructurado del DSL de orquestacion.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DslError {
    UnknownCommand { keyword: String },
    EmptyArguments { command: String },
    InvalidProgram { reason: String },
}

impl DslError {
    /// Crea un error de programa invalido con mensaje en espanol.
    pub fn invalid_program(reason: impl Into<String>) -> Self {
        Self::InvalidProgram {
            reason: reason.into(),
        }
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
}
