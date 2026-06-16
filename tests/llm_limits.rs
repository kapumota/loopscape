use loopscape::llm::{LlmLimits, LlmProvider, LlmRequest, MockProvider, ReplayProvider};

#[test]
fn mock_provider_enforces_prompt_token_limit() {
    let limits = LlmLimits::new(2, 20, 10);
    let request = LlmRequest::new("uno dos tres");
    let mut provider = MockProvider::new().with_limits(limits);

    let error = provider
        .complete(&request)
        .expect_err("debe rechazar prompt sobre limite");

    assert!(error.to_string().contains("prompt"));
}

#[test]
fn mock_provider_enforces_timeout_limit() {
    let limits = LlmLimits::new(20, 20, 2);
    let request = LlmRequest::new("delegar busqueda")
        .with_max_tokens(20)
        .with_simulated_latency_ticks(5);
    let mut provider = MockProvider::new().with_limits(limits);

    let error = provider
        .complete(&request)
        .expect_err("debe rechazar timeout simulado");

    assert!(error.to_string().contains("timeout"));
}

#[test]
fn replay_provider_enforces_response_token_limit() {
    let limits = LlmLimits::new(20, 2, 10);
    let request = LlmRequest::new("verificar plan").with_max_tokens(2);
    let mut provider =
        ReplayProvider::from_responses(vec!["uno dos tres".to_string()]).with_limits(limits);

    let error = provider
        .complete(&request)
        .expect_err("debe rechazar respuesta sobre limite");

    assert!(error.to_string().contains("respuesta"));
}

#[test]
fn provider_returns_latency_and_token_usage_when_within_limits() {
    let limits = LlmLimits::new(20, 20, 10);
    let request = LlmRequest::new("delegar busqueda")
        .with_max_tokens(20)
        .with_simulated_latency_ticks(4);
    let mut provider = MockProvider::with_prefix("simulado").with_limits(limits);

    let response = provider.complete(&request).expect("respuesta valida");

    assert_eq!(response.simulated_latency_ticks, 4);
    assert!(response.tokens_used > 0);
    assert!(response.deterministic);
}
