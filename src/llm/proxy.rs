use super::{count_tokens, LlmError, LlmLimits, LlmRequest};

pub const DEFAULT_MAX_BODY_BYTES: usize = 32 * 1024;

/// Politica CORS restringida para el proxy LLM opcional.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProxyCorsPolicy {
    allowed_origins: Vec<String>,
}

impl ProxyCorsPolicy {
    pub fn restricted(origins: Vec<String>) -> Result<Self, LlmError> {
        if origins.is_empty() {
            return Err(LlmError::UnsafeConfiguration(
                "la politica CORS debe declarar al menos un origen permitido".to_string(),
            ));
        }

        let mut allowed_origins = Vec::new();
        for origin in origins {
            let trimmed = origin.trim();
            if trimmed.is_empty() || trimmed == "*" {
                return Err(LlmError::UnsafeConfiguration(
                    "CORS no puede usar origen vacio ni comodin".to_string(),
                ));
            }
            allowed_origins.push(trimmed.to_string());
        }

        Ok(Self { allowed_origins })
    }

    pub fn is_allowed(&self, origin: &str) -> bool {
        self.allowed_origins.iter().any(|item| item == origin)
    }

    pub fn first_origin(&self) -> &str {
        self.allowed_origins
            .first()
            .map(String::as_str)
            .unwrap_or("http://localhost")
    }
}

/// Configuracion segura del proxy HTTP opcional.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LlmProxyConfig {
    pub endpoint: String,
    pub limits: LlmLimits,
    pub max_body_bytes: usize,
    pub cors: ProxyCorsPolicy,
}

impl LlmProxyConfig {
    pub fn new(
        endpoint: impl Into<String>,
        limits: LlmLimits,
        cors: ProxyCorsPolicy,
    ) -> Result<Self, LlmError> {
        let config = Self {
            endpoint: endpoint.into(),
            limits,
            max_body_bytes: DEFAULT_MAX_BODY_BYTES,
            cors,
        };
        config.validate()?;
        Ok(config)
    }

    pub fn with_max_body_bytes(mut self, max_body_bytes: usize) -> Result<Self, LlmError> {
        self.max_body_bytes = max_body_bytes;
        self.validate()?;
        Ok(self)
    }

    pub fn validate(&self) -> Result<(), LlmError> {
        if !self.endpoint.starts_with("http://") && !self.endpoint.starts_with("https://") {
            return Err(LlmError::UnsafeConfiguration(
                "el endpoint del proxy debe usar http o https".to_string(),
            ));
        }

        if self.endpoint.contains("token=") || self.endpoint.contains("api_key=") {
            return Err(LlmError::UnsafeConfiguration(
                "el endpoint del proxy no debe incluir claves en la URL".to_string(),
            ));
        }

        if self.max_body_bytes == 0 {
            return Err(LlmError::UnsafeConfiguration(
                "max_body_bytes debe ser mayor que cero".to_string(),
            ));
        }

        self.limits.validate()?;
        Ok(())
    }
}

/// Solicitud HTTP preparada por el proxy opcional.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProxyHttpRequest {
    pub method: String,
    pub endpoint: String,
    pub origin: String,
    pub timeout_ticks: u64,
    pub body: String,
}

/// Provider proxy que solo se compila con la feature llm-proxy.
pub struct HttpProxyProvider {
    config: LlmProxyConfig,
}

impl HttpProxyProvider {
    pub fn new(config: LlmProxyConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &LlmProxyConfig {
        &self.config
    }

    pub fn build_request(
        &self,
        request: &LlmRequest,
        origin: &str,
    ) -> Result<ProxyHttpRequest, LlmError> {
        self.config.validate()?;
        self.config.limits.validate_request(request)?;

        if !self.config.cors.is_allowed(origin) {
            return Err(LlmError::UnsafeConfiguration(
                "origen CORS no permitido para el proxy LLM".to_string(),
            ));
        }

        let body = serialize_proxy_body(request);
        if body.len() > self.config.max_body_bytes {
            return Err(LlmError::UnsafeConfiguration(format!(
                "body HTTP excede el maximo permitido: limite={}, observado={}",
                self.config.max_body_bytes,
                body.len()
            )));
        }

        Ok(ProxyHttpRequest {
            method: "POST".to_string(),
            endpoint: self.config.endpoint.clone(),
            origin: origin.to_string(),
            timeout_ticks: self.config.limits.timeout_ticks,
            body,
        })
    }
}

fn serialize_proxy_body(request: &LlmRequest) -> String {
    format!(
        "{{\"prompt\":\"{}\",\"max_tokens\":{},\"prompt_tokens\":{},\"simulated_latency_ticks\":{}}}",
        escape_json(&request.prompt),
        request.max_tokens,
        count_tokens(&request.prompt),
        request.simulated_latency_ticks
    )
}

fn escape_json(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn safe_config() -> LlmProxyConfig {
        let cors = ProxyCorsPolicy::restricted(vec!["http://localhost:5173".to_string()])
            .expect("cors seguro");
        LlmProxyConfig::new("http://localhost:8787/llm", LlmLimits::new(20, 20, 5), cors)
            .expect("configuracion segura")
    }

    #[test]
    fn proxy_rejects_cors_wildcard() {
        let error = ProxyCorsPolicy::restricted(vec!["*".to_string()]).expect_err("debe fallar");
        assert!(error.to_string().contains("CORS"));
    }

    #[test]
    fn proxy_rejects_keys_in_endpoint() {
        let cors = ProxyCorsPolicy::restricted(vec!["http://localhost:5173".to_string()])
            .expect("cors seguro");
        let error = LlmProxyConfig::new(
            "http://localhost:8787/llm?api_key=secreta",
            LlmLimits::new(20, 20, 5),
            cors,
        )
        .expect_err("debe rechazar claves en URL");

        assert!(error.to_string().contains("claves"));
    }

    #[test]
    fn proxy_builds_limited_request() {
        let provider = HttpProxyProvider::new(safe_config());
        let request = LlmRequest::new("plan seguro")
            .with_max_tokens(10)
            .with_simulated_latency_ticks(2);

        let prepared = provider
            .build_request(&request, "http://localhost:5173")
            .expect("solicitud valida");

        assert_eq!(prepared.method, "POST");
        assert_eq!(prepared.timeout_ticks, 5);
        assert!(prepared.body.contains("plan seguro"));
    }

    #[test]
    fn proxy_rejects_oversized_body() {
        let config = safe_config()
            .with_max_body_bytes(10)
            .expect("configuracion valida");
        let provider = HttpProxyProvider::new(config);
        let request = LlmRequest::new("plan seguro")
            .with_max_tokens(10)
            .with_simulated_latency_ticks(2);

        let error = provider
            .build_request(&request, "http://localhost:5173")
            .expect_err("debe rechazar body grande");

        assert!(error.to_string().contains("body HTTP"));
    }
}
