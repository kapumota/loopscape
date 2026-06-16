use loopscape::llm::{assert_sandboxed, LlmProvider, LlmRequest, MockProvider, ReplayProvider};

#[test]
fn mock_provider_declares_safe_capabilities() {
    let provider = MockProvider::new();
    let capabilities = provider.capabilities();

    assert_eq!(capabilities.name, "mock");
    assert!(!capabilities.network_enabled);
    assert!(!capabilities.secrets_required);
    assert!(capabilities.replayable);
    assert!(assert_sandboxed(&capabilities).is_ok());
}

#[test]
fn mock_provider_returns_repeatable_response() {
    let request = LlmRequest::new("delegar busqueda");
    let mut provider = MockProvider::with_prefix("simulado");

    let first = provider.complete(&request).expect("respuesta mock valida");
    let second = provider.complete(&request).expect("respuesta mock valida");

    assert_eq!(first, second);
    assert_eq!(first.provider, "mock");
    assert!(first.deterministic);
}

#[test]
fn replay_provider_uses_predefined_responses() {
    let request = LlmRequest::new("verificar plan");
    let mut provider = ReplayProvider::from_responses(vec![
        "respuesta uno".to_string(),
        "respuesta dos".to_string(),
    ]);

    let first = provider
        .complete(&request)
        .expect("respuesta replay valida");
    let second = provider
        .complete(&request)
        .expect("respuesta replay valida");

    assert_eq!(first.text, "respuesta uno");
    assert_eq!(second.text, "respuesta dos");
    assert_eq!(provider.remaining(), 0);
}

#[test]
fn replay_provider_fails_when_responses_are_exhausted() {
    let request = LlmRequest::new("verificar plan");
    let mut provider = ReplayProvider::from_responses(Vec::new());

    let error = provider
        .complete(&request)
        .expect_err("debe fallar sin respuestas");
    assert!(error.to_string().contains("no tiene respuesta"));
}
