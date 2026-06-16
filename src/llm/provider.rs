use super::error::LlmError;

/// Solicitud estable para un proveedor LLM dentro del sandbox.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LlmRequest {
    pub prompt: String,
    pub context: Vec<String>,
    pub max_tokens: usize,
}

impl LlmRequest {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            context: Vec::new(),
            max_tokens: 128,
        }
    }

    pub fn with_context(mut self, context: Vec<String>) -> Self {
        self.context = context;
        self
    }

    pub fn with_max_tokens(mut self, max_tokens: usize) -> Self {
        self.max_tokens = max_tokens.max(1);
        self
    }

    pub fn validate(&self) -> Result<(), LlmError> {
        if self.prompt.trim().is_empty() {
            return Err(LlmError::EmptyPrompt);
        }
        Ok(())
    }
}

/// Respuesta estable producida por un proveedor LLM del sandbox.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LlmResponse {
    pub provider: String,
    pub text: String,
    pub tokens_used: usize,
    pub deterministic: bool,
}

impl LlmResponse {
    pub fn new(provider: impl Into<String>, text: impl Into<String>, deterministic: bool) -> Self {
        let text = text.into();
        let tokens_used = count_tokens(&text);
        Self {
            provider: provider.into(),
            text,
            tokens_used,
            deterministic,
        }
    }
}

/// Capacidades declaradas por un proveedor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderCapabilities {
    pub name: String,
    pub network_enabled: bool,
    pub secrets_required: bool,
    pub replayable: bool,
}

impl ProviderCapabilities {
    pub fn sandboxed(name: impl Into<String>, replayable: bool) -> Self {
        Self {
            name: name.into(),
            network_enabled: false,
            secrets_required: false,
            replayable,
        }
    }
}

/// Interfaz minima para proveedores LLM seguros.
pub trait LlmProvider {
    fn capabilities(&self) -> ProviderCapabilities;
    fn complete(&mut self, request: &LlmRequest) -> Result<LlmResponse, LlmError>;
}

/// Verifica que un proveedor no requiera red ni secretos.
pub fn assert_sandboxed(capabilities: &ProviderCapabilities) -> Result<(), LlmError> {
    if capabilities.network_enabled {
        return Err(LlmError::UnsafeConfiguration(format!(
            "el proveedor {} declara uso de red",
            capabilities.name
        )));
    }

    if capabilities.secrets_required {
        return Err(LlmError::UnsafeConfiguration(format!(
            "el proveedor {} declara uso de secretos",
            capabilities.name
        )));
    }

    Ok(())
}

pub fn count_tokens(text: &str) -> usize {
    text.split_whitespace().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_prompt_is_rejected() {
        let request = LlmRequest::new("   ");
        assert_eq!(request.validate(), Err(LlmError::EmptyPrompt));
    }

    #[test]
    fn sandboxed_capabilities_do_not_require_network_or_secrets() {
        let capabilities = ProviderCapabilities::sandboxed("mock", true);
        assert!(assert_sandboxed(&capabilities).is_ok());
    }
}
