use loopscape::core::event::CoreEvent;
use loopscape::core::replay::{replay_records, replay_trace_jsonl};
use loopscape::core::scheduler::{SimulationConfig, SimulationState};
use loopscape::core::trace::{
    records_from_events, write_events_jsonl, TraceRecord, TRACE_FORMAT_VERSION,
};

#[test]
fn replay_trace_jsonl_reproduce_eventos_grabados() {
    let mut state = SimulationState::new(SimulationConfig::new(123));
    state.run_ticks(8);

    let path = std::env::temp_dir().join("loopscape-replay-determinista.jsonl");
    write_events_jsonl(&state.events, &path).expect("debe escribir traza JSONL");

    let summary = replay_trace_jsonl(&path).expect("debe reproducir traza JSONL");
    std::fs::remove_file(&path).ok();

    assert_eq!(summary.event_count, state.events.len());
    assert_eq!(summary.first_tick, 1);
    assert!(summary.last_tick >= summary.first_tick);
    assert_eq!(summary.first_event_kind, "TickAdvanced");
}

#[test]
fn replay_records_es_determinista_para_misma_semilla() {
    let mut first = SimulationState::new(SimulationConfig::new(123));
    let mut second = SimulationState::new(SimulationConfig::new(123));
    first.run_ticks(12);
    second.run_ticks(12);

    let first_summary =
        replay_records(&records_from_events(&first.events)).expect("primer replay valido");
    let second_summary =
        replay_records(&records_from_events(&second.events)).expect("segundo replay valido");

    assert_eq!(first_summary, second_summary);
}

#[test]
fn replay_records_rechaza_retroceso_de_tick() {
    let records = vec![
        TraceRecord {
            format: TRACE_FORMAT_VERSION.to_string(),
            sequence: 0,
            tick: 4,
            kind: "TickAdvanced".to_string(),
            event: CoreEvent::TickAdvanced { tick: 4 },
        },
        TraceRecord {
            format: TRACE_FORMAT_VERSION.to_string(),
            sequence: 1,
            tick: 3,
            kind: "TickAdvanced".to_string(),
            event: CoreEvent::TickAdvanced { tick: 3 },
        },
    ];

    let error = replay_records(&records).expect_err("debe rechazar traza no monotona");

    assert!(error.contains("tick retrocede"));
}
