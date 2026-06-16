use super::error::LlmError;
use super::provider::{
    assert_sandboxed, LlmLimits, LlmProvider, LlmRequest, LlmResponse, ProviderCapabilities,
};

/// Proveedor simulado determinista.
pub struct MockProvider {
    response_prefix: String,
    limits: LlmLimits,
}

impl MockProvider {
    pub fn new() -> Self {
        Self {
            response_prefix: "respuesta simulada".to_string(),
            limits: LlmLimits::default(),
        }
    }

    pub fn with_prefix(prefix: impl Into<String>) -> Self {
        Self {
            response_prefix: prefix.into(),
            limits: LlmLimits::default(),
        }
    }

    pub fn with_limits(mut self, limits: LlmLimits) -> Self {
        self.limits = limits;
        self
    }
}

impl Default for MockProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl LlmProvider for MockProvider {
    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities::sandboxed("mock", true).with_limits(self.limits.clone())
    }

    fn complete(&mut self, request: &LlmRequest) -> Result<LlmResponse, LlmError> {
        request.validate()?;
        let capabilities = self.capabilities();
        assert_sandboxed(&capabilities)?;
        capabilities.limits.validate_request(request)?;

        let normalized_prompt = request.prompt.trim();
        let response = format!("{}: {}", self.response_prefix, normalized_prompt);
        capabilities.limits.validate_response_text(&response)?;

        Ok(LlmResponse::with_latency(
            "mock",
            response,
            true,
            request.simulated_latency_ticks,
        ))
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

    #[test]
    fn mock_provider_rejects_response_over_limit() {
        let request = LlmRequest::new("planificar rescate").with_max_tokens(2);
        let limits = LlmLimits::new(20, 2, 10);
        let mut provider = MockProvider::new().with_limits(limits);

        let error = provider
            .complete(&request)
            .expect_err("debe rechazar respuesta larga");

        assert!(error.to_string().contains("respuesta"));
    }
}
