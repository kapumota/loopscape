use std::fmt;

/// Error controlado para proveedores LLM seguros.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LlmError {
    EmptyPrompt,
    ReplayExhausted { requested_index: usize },
    UnsafeConfiguration(String),
}

impl fmt::Display for LlmError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LlmError::EmptyPrompt => {
                write!(formatter, "la solicitud LLM no puede tener prompt vacio")
            }
            LlmError::ReplayExhausted { requested_index } => write!(
                formatter,
                "el proveedor replay no tiene respuesta para el indice {requested_index}"
            ),
            LlmError::UnsafeConfiguration(message) => {
                write!(formatter, "configuracion LLM insegura: {message}")
            }
        }
    }
}

impl std::error::Error for LlmError {}
