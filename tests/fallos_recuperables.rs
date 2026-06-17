use loopscape::core::event::CoreEvent;
use loopscape::core::failure::RecoverableFailurePlan;
use loopscape::core::metrics::SimulationMetricsCsvRow;
use loopscape::core::scheduler::{SimulationConfig, SimulationState};

#[test]
fn simulation_detects_and_recovers_hung_worker() {
    let plan = RecoverableFailurePlan::worker_hangs(0, 1, 3).expect("plan valido");
    let config = SimulationConfig::new(101)
        .with_size(1, 1)
        .with_supervisor(1, 1)
        .with_recoverable_failures(plan);
    let mut state = SimulationState::new(config);

    state.run_ticks(4);

    assert!(state
        .events
        .iter()
        .any(|event| matches!(event, CoreEvent::WorkerTimedOut { .. })));
    assert!(state
        .events
        .iter()
        .any(|event| matches!(event, CoreEvent::WorkerRestarted { .. })));
    assert_eq!(state.supervisor.metrics().failures_detected, 1);
    assert_eq!(state.supervisor.metrics().failures_recovered, 1);
}

#[test]
fn simulation_without_failure_plan_keeps_failure_metrics_at_zero() {
    let config = SimulationConfig::new(202)
        .with_size(2, 3)
        .with_supervisor(2, 1);
    let mut state = SimulationState::new(config);

    state.run_ticks(5);

    assert_eq!(state.supervisor.metrics().failures_detected, 0);
    assert_eq!(state.supervisor.metrics().failures_recovered, 0);
}

#[test]
fn simulation_exports_recovered_failures_to_metrics_csv() {
    let plan = RecoverableFailurePlan::worker_hangs(0, 1, 3).expect("plan valido");
    let config = SimulationConfig::new(303)
        .with_size(1, 1)
        .with_supervisor(1, 1)
        .with_recoverable_failures(plan);
    let mut state = SimulationState::new(config);

    state.run_ticks(4);
    let row = SimulationMetricsCsvRow::from_state(&state);

    assert_eq!(row.failures_detected, 1);
    assert_eq!(row.failures_recovered, 1);
}
