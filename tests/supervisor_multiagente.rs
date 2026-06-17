use loopscape::core::supervisor::{
    RestartPolicy, SupervisorEvent, SupervisorState, WorkerState, WorkerStatus,
};

#[test]
fn supervisor_detects_and_recovers_failures() {
    let mut supervisor = SupervisorState::new(RestartPolicy::on_timeout(1));
    supervisor
        .register_worker(WorkerState::new(1, "worker_busqueda", 2).expect("worker valido"))
        .expect("registro valido");

    supervisor.advance_to_tick(3).expect("avance valido");

    assert!(supervisor
        .events
        .iter()
        .any(|event| matches!(event, SupervisorEvent::WorkerTimedOut(_))));
    assert!(supervisor
        .events
        .iter()
        .any(|event| matches!(event, SupervisorEvent::WorkerRestarted { .. })));
    assert_eq!(supervisor.metrics().failures_detected, 1);
    assert_eq!(supervisor.metrics().failures_recovered, 1);
}

#[test]
fn supervisor_rejects_duplicate_workers() {
    let mut supervisor = SupervisorState::new(RestartPolicy::never());
    let worker = WorkerState::new(1, "worker_a", 2).expect("worker valido");
    supervisor
        .register_worker(worker.clone())
        .expect("primer registro valido");

    let error = supervisor
        .register_worker(worker)
        .expect_err("debe rechazar duplicado");

    assert!(error.contains("duplicado"));
}

#[test]
fn supervisor_stops_after_restart_limit() {
    let mut supervisor = SupervisorState::new(RestartPolicy::on_timeout(1));
    supervisor
        .register_worker(WorkerState::new(1, "worker_a", 2).expect("worker valido"))
        .expect("registro valido");

    supervisor.advance_to_tick(3).expect("primer timeout");
    supervisor.advance_to_tick(6).expect("segundo timeout");

    let worker = supervisor.worker(1).expect("worker registrado");
    assert_eq!(worker.status, WorkerStatus::TimedOut);
    assert_eq!(supervisor.metrics().failures_detected, 2);
    assert_eq!(supervisor.metrics().failures_recovered, 1);
}
