use super::error::LlmError;
use super::provider::{
    assert_sandboxed, LlmProvider, LlmRequest, LlmResponse, ProviderCapabilities,
};

/// Proveedor simulado determinista.
pub struct MockProvider {
    response_prefix: String,
}

impl MockProvider {
    pub fn new() -> Self {
        Self {
            response_prefix: "respuesta simulada".to_string(),
        }
    }

    pub fn with_prefix(prefix: impl Into<String>) -> Self {
        Self {
            response_prefix: prefix.into(),
        }
    }
}

impl Default for MockProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl LlmProvider for MockProvider {
    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities::sandboxed("mock", true)
    }

    fn complete(&mut self, request: &LlmRequest) -> Result<LlmResponse, LlmError> {
        request.validate()?;
        assert_sandboxed(&self.capabilities())?;

        let normalized_prompt = request.prompt.trim();
        let response = format!("{}: {}", self.response_prefix, normalized_prompt);
        Ok(LlmResponse::new("mock", response, true))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_provider_is_deterministic() {
        let request = LlmRequest::new("planificar rescate");
        let mut first = MockProvider::new();
        let mut second = MockProvider::new();

        let first_response = first.complete(&request).expect("respuesta mock valida");
        let second_response = second.complete(&request).expect("respuesta mock valida");

        assert_eq!(first_response, second_response);
        assert!(first_response.deterministic);
    }
}
