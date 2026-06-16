#![cfg(feature = "llm-proxy")]

use loopscape::llm::{HttpProxyProvider, LlmLimits, LlmProxyConfig, LlmRequest, ProxyCorsPolicy};

#[test]
fn optional_proxy_respects_cors_and_limits() {
    let cors = ProxyCorsPolicy::restricted(vec!["http://localhost:5173".to_string()])
        .expect("cors restringido");
    let config = LlmProxyConfig::new("http://localhost:8787/llm", LlmLimits::new(16, 12, 4), cors)
        .expect("configuracion valida");
    let provider = HttpProxyProvider::new(config);
    let request = LlmRequest::new("delegar tarea")
        .with_max_tokens(8)
        .with_simulated_latency_ticks(2);

    let prepared = provider
        .build_request(&request, "http://localhost:5173")
        .expect("solicitud HTTP preparada");

    assert_eq!(prepared.method, "POST");
    assert_eq!(prepared.timeout_ticks, 4);
    assert!(prepared.body.contains("delegar tarea"));
}

#[test]
fn optional_proxy_rejects_untrusted_origin() {
    let cors = ProxyCorsPolicy::restricted(vec!["http://localhost:5173".to_string()])
        .expect("cors restringido");
    let config = LlmProxyConfig::new("http://localhost:8787/llm", LlmLimits::new(16, 12, 4), cors)
        .expect("configuracion valida");
    let provider = HttpProxyProvider::new(config);
    let request = LlmRequest::new("delegar tarea")
        .with_max_tokens(8)
        .with_simulated_latency_ticks(2);

    let error = provider
        .build_request(&request, "http://sitio-no-confiable")
        .expect_err("origen no permitido");

    assert!(error.to_string().contains("CORS"));
}
