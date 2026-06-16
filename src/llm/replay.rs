use super::error::LlmError;
use super::provider::{
    assert_sandboxed, LlmLimits, LlmProvider, LlmRequest, LlmResponse, ProviderCapabilities,
};

/// Proveedor que reproduce respuestas predefinidas sin usar red.
pub struct ReplayProvider {
    responses: Vec<String>,
    cursor: usize,
    limits: LlmLimits,
}

impl ReplayProvider {
    pub fn from_responses(responses: Vec<String>) -> Self {
        Self {
            responses,
            cursor: 0,
            limits: LlmLimits::default(),
        }
    }

    pub fn with_limits(mut self, limits: LlmLimits) -> Self {
        self.limits = limits;
        self
    }

    pub fn consumed(&self) -> usize {
        self.cursor
    }

    pub fn remaining(&self) -> usize {
        self.responses.len().saturating_sub(self.cursor)
    }
}

impl LlmProvider for ReplayProvider {
    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities::sandboxed("replay", true).with_limits(self.limits.clone())
    }

    fn complete(&mut self, request: &LlmRequest) -> Result<LlmResponse, LlmError> {
        request.validate()?;
        let capabilities = self.capabilities();
        assert_sandboxed(&capabilities)?;
        capabilities.limits.validate_request(request)?;

        let Some(response) = self.responses.get(self.cursor).cloned() else {
            return Err(LlmError::ReplayExhausted {
                requested_index: self.cursor,
            });
        };

        capabilities.limits.validate_response_text(&response)?;

        self.cursor += 1;
        Ok(LlmResponse::with_latency(
            "replay",
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
    fn replay_provider_consumes_scripted_responses() {
        let request = LlmRequest::new("siguiente paso");
        let mut provider =
            ReplayProvider::from_responses(vec!["buscar".to_string(), "clasificar".to_string()]);

        let first = provider
            .complete(&request)
            .expect("respuesta replay valida");
        let second = provider
            .complete(&request)
            .expect("respuesta replay valida");

        assert_eq!(first.text, "buscar");
        assert_eq!(second.text, "clasificar");
        assert_eq!(provider.consumed(), 2);
        assert_eq!(provider.remaining(), 0);
    }

    #[test]
    fn replay_provider_rejects_timeout_before_consuming_response() {
        let request = LlmRequest::new("siguiente paso")
            .with_max_tokens(20)
            .with_simulated_latency_ticks(9);
        let limits = LlmLimits::new(20, 20, 3);
        let mut provider =
            ReplayProvider::from_responses(vec!["buscar".to_string()]).with_limits(limits);

        let error = provider
            .complete(&request)
            .expect_err("debe rechazar timeout simulado");

        assert!(error.to_string().contains("timeout"));
        assert_eq!(provider.consumed(), 0);
    }
}
