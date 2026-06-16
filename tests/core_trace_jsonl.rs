use loopscape::core::scheduler::{SimulationConfig, SimulationState};
use loopscape::core::trace::{read_events_jsonl, records_from_events, write_events_jsonl};

#[test]
fn trace_jsonl_roundtrip_preserves_core_events() {
    let mut state = SimulationState::new(SimulationConfig::new(123));
    state.run_ticks(5);

    let path = std::env::temp_dir().join("loopscape-core-trace-roundtrip.jsonl");
    write_events_jsonl(&state.events, &path).expect("debe escribir eventos JSONL");
    let records = read_events_jsonl(&path).expect("debe leer eventos JSONL");
    std::fs::remove_file(&path).ok();

    assert_eq!(records.len(), state.events.len());
    assert_eq!(records[0].sequence, 0);
    assert_eq!(records[0].tick, 1);
    assert_eq!(records[0].kind, "TickAdvanced");
}

#[test]
fn trace_records_are_deterministic_for_same_seed() {
    let mut first = SimulationState::new(SimulationConfig::new(123));
    let mut second = SimulationState::new(SimulationConfig::new(123));
    first.run_ticks(8);
    second.run_ticks(8);

    let first_records = records_from_events(&first.events);
    let second_records = records_from_events(&second.events);

    assert_eq!(first_records, second_records);
}
