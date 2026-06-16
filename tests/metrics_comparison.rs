use loopscape::core::compare::{
    compare_metrics_files, compare_metrics_rows, write_metrics_comparison_csv,
    METRICS_COMPARISON_CSV_HEADER,
};
use loopscape::core::metrics::{write_metrics_csv, SimulationMetricsCsvRow};
use loopscape::core::scheduler::{SimulationConfig, SimulationState};

#[test]
fn compare_metrics_files_detects_equal_runs() {
    let mut first = SimulationState::new(SimulationConfig::new(123));
    let mut second = SimulationState::new(SimulationConfig::new(123));
    first.run_ticks(20);
    second.run_ticks(20);

    let first_path = std::env::temp_dir().join("loopscape-compare-first.csv");
    let second_path = std::env::temp_dir().join("loopscape-compare-second.csv");
    write_metrics_csv(&first, &first_path).expect("debe escribir primera corrida");
    write_metrics_csv(&second, &second_path).expect("debe escribir segunda corrida");

    let comparison =
        compare_metrics_files(&first_path, &second_path).expect("debe comparar metricas");
    std::fs::remove_file(&first_path).ok();
    std::fs::remove_file(&second_path).ok();

    assert_eq!(comparison.delta.ticks, 0);
    assert_eq!(comparison.delta.completed_tasks, 0);
    assert_eq!(comparison.delta.active_loops, 0);
}

#[test]
fn compare_metrics_rows_reports_expected_differences() {
    let baseline = SimulationMetricsCsvRow {
        ticks: 10,
        completed_tasks: 1,
        active_loops: 1,
        tokens_used: 0,
        failures_detected: 0,
        failures_recovered: 0,
        average_latency: 2.0,
    };
    let candidate = SimulationMetricsCsvRow {
        ticks: 15,
        completed_tasks: 3,
        active_loops: 2,
        tokens_used: 0,
        failures_detected: 1,
        failures_recovered: 1,
        average_latency: 3.25,
    };

    let comparison = compare_metrics_rows(baseline, candidate);

    assert_eq!(comparison.delta.ticks, 5);
    assert_eq!(comparison.delta.completed_tasks, 2);
    assert_eq!(comparison.delta.active_loops, 1);
    assert_eq!(comparison.delta.failures_detected, 1);
    assert_eq!(comparison.delta.failures_recovered, 1);
    assert_eq!(comparison.delta.average_latency, 1.25);
}

#[test]
fn comparison_csv_report_is_written() {
    let row = SimulationMetricsCsvRow {
        ticks: 10,
        completed_tasks: 2,
        active_loops: 1,
        tokens_used: 0,
        failures_detected: 0,
        failures_recovered: 0,
        average_latency: 2.0,
    };
    let comparison = compare_metrics_rows(row.clone(), row);
    let path = std::env::temp_dir().join("loopscape-comparison-report.csv");

    write_metrics_comparison_csv(&comparison, &path).expect("debe escribir comparacion CSV");
    let content = std::fs::read_to_string(&path).expect("debe leer comparacion CSV");
    std::fs::remove_file(&path).ok();

    assert!(content.starts_with(METRICS_COMPARISON_CSV_HEADER));
    assert!(content.contains("tareas_completadas"));
    assert!(content.contains("latencia_promedio"));
}
