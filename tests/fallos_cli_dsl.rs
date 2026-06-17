use loopscape::core::byzantine::{build_worker_responses, SimpleMajorityVoter, VoteDecision};
use loopscape::dsl::{
    failure_scenario_from_source, graph_from_source, interpret_source, validate_source,
};

fn source() -> &'static str {
    r#"/goal rescatar_victimas
/plan buscar -> clasificar
/worker-failure 1 4 3
/byzantine-failure 2 falso
/byzantine-vote verdadero
/verify checklist_final
/terminate when verified"#
}

#[test]
fn dsl_accepts_recoverable_and_byzantine_failures() {
    let program = validate_source(source()).expect("debe validar DSL con fallos");
    let scenario =
        loopscape::dsl::failure_scenario_from_program(&program).expect("debe extraer escenario");

    assert!(scenario.recoverable_failures.is_worker_hung(1, 5));
    assert_eq!(
        scenario.byzantine_failures.false_value_for(2),
        Some("falso")
    );
    assert_eq!(scenario.byzantine_vote_value, Some("verdadero".to_string()));
}

#[test]
fn dsl_failure_commands_do_not_pollute_core_events() {
    let events = interpret_source(source()).expect("debe interpretar DSL con fallos");

    assert!(events
        .iter()
        .any(|event| matches!(event, loopscape::core::event::CoreEvent::GoalCreated { .. })));
    assert!(!events.iter().any(|event| matches!(
        event,
        loopscape::core::event::CoreEvent::WorkerTimedOut { .. }
    )));
}

#[test]
fn dsl_failure_commands_are_visible_in_graph() {
    let graph = graph_from_source(source(), None).expect("debe exportar grafo");
    let kinds = graph
        .nodes
        .iter()
        .map(|node| node.kind.as_str())
        .collect::<Vec<_>>();

    assert!(kinds.contains(&"worker_failure"));
    assert!(kinds.contains(&"byzantine_failure"));
    assert!(kinds.contains(&"byzantine_vote"));
}

#[test]
fn dsl_byzantine_scenario_can_be_voted() {
    let scenario = failure_scenario_from_source(source()).expect("escenario valido");
    let responses = build_worker_responses(
        &[0, 1, 2],
        scenario.byzantine_vote_value.expect("valor esperado"),
        &scenario.byzantine_failures,
    );
    let voter = SimpleMajorityVoter::majority(3).expect("votador valido");

    let outcome = voter.decide(&responses);

    assert_eq!(
        outcome.decision,
        VoteDecision::Accepted {
            value: "verdadero".to_string(),
            votes: 2,
        }
    );
}
