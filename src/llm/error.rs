use std::fmt;

/// Error controlado para proveedores LLM seguros.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LlmError {
    EmptyPrompt,
    InvalidLimit(String),
    ReplayExhausted {
        requested_index: usize,
    },
    TimeoutExceeded {
        timeout_ticks: u64,
        elapsed_ticks: u64,
    },
    TokenLimitExceeded {
        field: String,
        limit: usize,
        actual: usize,
    },
    UnsafeConfiguration(String),
}

impl fmt::Display for LlmError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LlmError::EmptyPrompt => write!(formatter, "la solicitud LLM no puede tener prompt vacio"),
            LlmError::InvalidLimit(message) => write!(formatter, "limite LLM invalido: {message}"),
            LlmError::ReplayExhausted { requested_index } => write!(
                formatter,
                "el proveedor replay no tiene respuesta para el indice {requested_index}"
            ),
            LlmError::TimeoutExceeded {
                timeout_ticks,
                elapsed_ticks,
            } => write!(
                formatter,
                "timeout simulado excedido: limite={timeout_ticks} ticks, observado={elapsed_ticks} ticks"
            ),
            LlmError::TokenLimitExceeded {
                field,
                limit,
                actual,
            } => write!(
                formatter,
                "limite de tokens excedido en {field}: limite={limit}, observado={actual}"
            ),
            LlmError::UnsafeConfiguration(message) => {
                write!(formatter, "configuracion LLM insegura: {message}")
            }
        }
    }
}

impl std::error::Error for LlmError {}
