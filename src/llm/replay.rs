use super::error::LlmError;
use super::provider::{
    assert_sandboxed, LlmProvider, LlmRequest, LlmResponse, ProviderCapabilities,
};

/// Proveedor que reproduce respuestas predefinidas sin usar red.
pub struct ReplayProvider {
    responses: Vec<String>,
    cursor: usize,
}

impl ReplayProvider {
    pub fn from_responses(responses: Vec<String>) -> Self {
        Self {
            responses,
            cursor: 0,
        }
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
        ProviderCapabilities::sandboxed("replay", true)
    }

    fn complete(&mut self, request: &LlmRequest) -> Result<LlmResponse, LlmError> {
        request.validate()?;
        assert_sandboxed(&self.capabilities())?;

        let Some(response) = self.responses.get(self.cursor).cloned() else {
            return Err(LlmError::ReplayExhausted {
                requested_index: self.cursor,
            });
        };

        self.cursor += 1;
        Ok(LlmResponse::new("replay", response, true))
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
}
