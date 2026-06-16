use loopscape::dsl::{
    graph_contract, graph_from_json, graph_from_source, graph_roundtrip_from_json,
    graph_roundtrip_from_source, graph_to_json, validate_graph_contract, GRAPH_FORMAT_VERSION,
};

fn source() -> &'static str {
    r#"/goal rescatar_victimas
/plan buscar -> clasificar -> asistir
/delegate sector_a worker_1
/verify checklist_final
/terminate when verified"#
}

#[test]
fn graph_contract_exposes_stable_shape() {
    let contract = graph_contract();

    assert_eq!(contract.version, GRAPH_FORMAT_VERSION);
    assert_eq!(contract.top_level_keys, &["metadatos", "nodos", "aristas"]);
    assert!(contract.node_fields.contains(&"id"));
    assert!(contract.edge_fields.contains(&"from"));
}

#[test]
fn roundtrip_preserves_canonical_json() {
    let first = graph_roundtrip_from_source(source(), Some("examples/rescate.loop".to_string()))
        .expect("debe producir roundtrip desde DSL");
    let second = graph_roundtrip_from_json(&first).expect("debe reimportar el grafo exportado");

    assert_eq!(first, second);
}

#[test]
fn exported_graph_keeps_stable_ids() {
    let graph = graph_from_source(source(), Some("examples/rescate.loop".to_string()))
        .expect("debe exportar grafo");

    let node_ids = graph
        .nodes
        .iter()
        .map(|node| node.id.as_str())
        .collect::<Vec<_>>();
    let edge_ids = graph
        .edges
        .iter()
        .map(|edge| edge.id.as_str())
        .collect::<Vec<_>>();

    assert_eq!(
        node_ids,
        vec![
            "cmd-000-goal-000",
            "cmd-001-plan_step-000",
            "cmd-001-plan_step-001",
            "cmd-001-plan_step-002",
            "cmd-002-delegate-000",
            "cmd-003-verify-000",
            "cmd-004-terminate-000",
        ]
    );
    assert_eq!(
        edge_ids,
        vec!["edge-000", "edge-001", "edge-002", "edge-003", "edge-004", "edge-005"]
    );
}

#[test]
fn contract_rejects_non_canonical_edge_id() {
    let mut graph = graph_from_source(source(), None).expect("debe exportar grafo");
    graph.edges[0].id = "edge-rota".to_string();
    let json = graph_to_json(&graph).expect("debe serializar grafo alterado");

    let error = graph_from_json(&json).expect_err("debe rechazar id fuera de contrato");

    assert!(error.to_string().contains("id de arista fuera de contrato"));
}

#[test]
fn contract_rejects_empty_node_label() {
    let mut graph = graph_from_source(source(), None).expect("debe exportar grafo");
    graph.nodes[0].label.clear();

    let error = validate_graph_contract(&graph).expect_err("debe rechazar etiqueta vacia");

    assert!(error.to_string().contains("no tiene etiqueta"));
}
