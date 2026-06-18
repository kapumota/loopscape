use std::fs;

use loopscape::dsl::{
    failure_scenario_from_source, graph_from_source, interpret_source, validate_source,
};

const SCENARIOS: &[&str] = &["react_basic", "dsl_delegation", "multiagent_failure"];

fn scenario_source(name: &str) -> String {
    fs::read_to_string(format!("scenarios/{name}.loop")).expect("debe leer escenario comparable")
}

#[test]
fn comparable_scenarios_exist_and_validate() {
    for name in SCENARIOS {
        let source = scenario_source(name);
        let program = validate_source(&source).expect("debe validar escenario comparable");
        let events = interpret_source(&source).expect("debe interpretar escenario comparable");
        let graph = graph_from_source(&source, Some(format!("scenarios/{name}.loop")))
            .expect("debe exportar grafo comparable");

        assert!(graph.metadata.command_count >= 5);
        assert!(graph.metadata.node_count >= graph.metadata.command_count);
        assert!(graph.metadata.edge_count + 1 >= graph.metadata.node_count);
        assert!(!events.is_empty());
        assert_eq!(program.commands.len(), graph.metadata.command_count);
    }
}

#[test]
fn basic_and_delegation_scenarios_have_no_failure_model() {
    for name in ["react_basic", "dsl_delegation"] {
        let source = scenario_source(name);
        let scenario = failure_scenario_from_source(&source).expect("debe extraer escenario");

        assert!(scenario.recoverable_failures.failures.is_empty());
        assert!(scenario.byzantine_failures.failures.is_empty());
        assert_eq!(scenario.byzantine_vote_value, None);
    }
}

#[test]
fn multiagent_failure_scenario_exposes_recoverable_and_byzantine_failure() {
    let source = scenario_source("multiagent_failure");
    let scenario = failure_scenario_from_source(&source).expect("debe extraer fallos");

    assert!(scenario.recoverable_failures.is_worker_hung(1, 4));
    assert_eq!(
        scenario.byzantine_failures.false_value_for(2),
        Some("falso")
    );
    assert_eq!(scenario.byzantine_vote_value, Some("verdadero".to_string()));
}

#[test]
fn multiagent_failure_scenario_has_failure_nodes_in_graph() {
    let source = scenario_source("multiagent_failure");
    let graph = graph_from_source(
        &source,
        Some("scenarios/multiagent_failure.loop".to_string()),
    )
    .expect("debe exportar grafo");
    let kinds = graph
        .nodes
        .iter()
        .map(|node| node.kind.as_str())
        .collect::<Vec<_>>();

    assert!(kinds.contains(&"worker_failure"));
    assert!(kinds.contains(&"byzantine_failure"));
    assert!(kinds.contains(&"byzantine_vote"));
}
