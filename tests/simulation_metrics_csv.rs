use loopscape::core::metrics::{write_metrics_csv, SimulationMetricsCsvRow, METRICS_CSV_HEADER};
use loopscape::core::scheduler::{SimulationConfig, SimulationState};

#[test]
fn metrics_csv_contains_expected_header() {
    let mut state = SimulationState::new(SimulationConfig::new(123));
    state.run_ticks(10);

    let path = std::env::temp_dir().join("loopscape-metrics-test.csv");
    write_metrics_csv(&state, &path).expect("debe escribir metricas CSV");
    let content = std::fs::read_to_string(&path).expect("debe leer metricas CSV");
    std::fs::remove_file(&path).ok();

    assert!(content.starts_with(METRICS_CSV_HEADER));
    assert_eq!(content.lines().count(), 2);
}

#[test]
fn metrics_row_is_deterministic_for_same_seed() {
    let mut first = SimulationState::new(SimulationConfig::new(123));
    let mut second = SimulationState::new(SimulationConfig::new(123));
    first.run_ticks(20);
    second.run_ticks(20);

    let first_row = SimulationMetricsCsvRow::from_state(&first);
    let second_row = SimulationMetricsCsvRow::from_state(&second);

    assert_eq!(first_row, second_row);
    assert_eq!(first_row.ticks, 20);
    assert_eq!(first_row.tokens_used, 0);
    assert_eq!(first_row.failures_detected, 0);
    assert_eq!(first_row.failures_recovered, 0);
}
