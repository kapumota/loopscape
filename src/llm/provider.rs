use super::error::LlmError;

/// Limites simulados para proveedores LLM seguros.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LlmLimits {
    pub max_prompt_tokens: usize,
    pub max_response_tokens: usize,
    pub timeout_ticks: u64,
}

impl LlmLimits {
    pub fn new(max_prompt_tokens: usize, max_response_tokens: usize, timeout_ticks: u64) -> Self {
        Self {
            max_prompt_tokens: max_prompt_tokens.max(1),
            max_response_tokens: max_response_tokens.max(1),
            timeout_ticks: timeout_ticks.max(1),
        }
    }

    pub fn validate(&self) -> Result<(), LlmError> {
        if self.max_prompt_tokens == 0 {
            return Err(LlmError::InvalidLimit(
                "max_prompt_tokens debe ser mayor que cero".to_string(),
            ));
        }

        if self.max_response_tokens == 0 {
            return Err(LlmError::InvalidLimit(
                "max_response_tokens debe ser mayor que cero".to_string(),
            ));
        }

        if self.timeout_ticks == 0 {
            return Err(LlmError::InvalidLimit(
                "timeout_ticks debe ser mayor que cero".to_string(),
            ));
        }

        Ok(())
    }

    pub fn validate_request(&self, request: &LlmRequest) -> Result<(), LlmError> {
        self.validate()?;

        let prompt_tokens = count_tokens(&request.prompt);
        if prompt_tokens > self.max_prompt_tokens {
            return Err(LlmError::TokenLimitExceeded {
                field: "prompt".to_string(),
                limit: self.max_prompt_tokens,
                actual: prompt_tokens,
            });
        }

        if request.max_tokens > self.max_response_tokens {
            return Err(LlmError::TokenLimitExceeded {
                field: "max_tokens".to_string(),
                limit: self.max_response_tokens,
                actual: request.max_tokens,
            });
        }

        if request.simulated_latency_ticks > self.timeout_ticks {
            return Err(LlmError::TimeoutExceeded {
                timeout_ticks: self.timeout_ticks,
                elapsed_ticks: request.simulated_latency_ticks,
            });
        }

        Ok(())
    }

    pub fn validate_response_text(&self, text: &str) -> Result<(), LlmError> {
        let response_tokens = count_tokens(text);
        if response_tokens > self.max_response_tokens {
            return Err(LlmError::TokenLimitExceeded {
                field: "respuesta".to_string(),
                limit: self.max_response_tokens,
                actual: response_tokens,
            });
        }

        Ok(())
    }
}

impl Default for LlmLimits {
    fn default() -> Self {
        Self::new(256, 128, 50)
    }
}

/// Solicitud estable para un proveedor LLM dentro del sandbox.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LlmRequest {
    pub prompt: String,
    pub context: Vec<String>,
    pub max_tokens: usize,
    pub simulated_latency_ticks: u64,
}

impl LlmRequest {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            context: Vec::new(),
            max_tokens: 128,
            simulated_latency_ticks: 1,
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

    pub fn with_simulated_latency_ticks(mut self, simulated_latency_ticks: u64) -> Self {
        self.simulated_latency_ticks = simulated_latency_ticks.max(1);
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
    pub simulated_latency_ticks: u64,
}

impl LlmResponse {
    pub fn new(provider: impl Into<String>, text: impl Into<String>, deterministic: bool) -> Self {
        Self::with_latency(provider, text, deterministic, 1)
    }

    pub fn with_latency(
        provider: impl Into<String>,
        text: impl Into<String>,
        deterministic: bool,
        simulated_latency_ticks: u64,
    ) -> Self {
        let text = text.into();
        let tokens_used = count_tokens(&text);
        Self {
            provider: provider.into(),
            text,
            tokens_used,
            deterministic,
            simulated_latency_ticks: simulated_latency_ticks.max(1),
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
    pub limits: LlmLimits,
}

impl ProviderCapabilities {
    pub fn sandboxed(name: impl Into<String>, replayable: bool) -> Self {
        Self {
            name: name.into(),
            network_enabled: false,
            secrets_required: false,
            replayable,
            limits: LlmLimits::default(),
        }
    }

    pub fn with_limits(mut self, limits: LlmLimits) -> Self {
        self.limits = limits;
        self
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

    capabilities.limits.validate()
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

    #[test]
    fn prompt_token_limit_is_enforced() {
        let limits = LlmLimits::new(2, 10, 5);
        let request = LlmRequest::new("uno dos tres");

        let error = limits
            .validate_request(&request)
            .expect_err("debe rechazar prompt largo");

        assert!(error.to_string().contains("prompt"));
    }

    #[test]
    fn timeout_limit_is_enforced() {
        let limits = LlmLimits::new(10, 10, 3);
        let request = LlmRequest::new("plan")
            .with_max_tokens(10)
            .with_simulated_latency_ticks(4);

        let error = limits
            .validate_request(&request)
            .expect_err("debe rechazar latencia simulada alta");

        assert!(error.to_string().contains("timeout"));
    }
}
