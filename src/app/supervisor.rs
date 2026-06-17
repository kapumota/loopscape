use crate::core::supervisor::{SupervisorEvent, SupervisorState, WorkerStatus};

/// Fila estable para presentar workers supervisados sin acoplarse a Bevy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SupervisorPanelRow {
    pub worker_id: u32,
    pub nombre: String,
    pub estado: String,
    pub ultimo_heartbeat: u64,
    pub reinicios: u32,
}

pub fn supervisor_rows(state: &SupervisorState) -> Vec<SupervisorPanelRow> {
    state
        .workers
        .iter()
        .map(|worker| SupervisorPanelRow {
            worker_id: worker.id,
            nombre: worker.name.clone(),
            estado: status_label(&worker.status).to_string(),
            ultimo_heartbeat: worker.last_heartbeat_tick,
            reinicios: worker.restart_count,
        })
        .collect()
}

pub fn supervisor_event_labels(events: &[SupervisorEvent]) -> Vec<String> {
    events.iter().map(supervisor_event_label).collect()
}

fn status_label(status: &WorkerStatus) -> &'static str {
    match status {
        WorkerStatus::Running => "activo",
        WorkerStatus::TimedOut => "timeout",
        WorkerStatus::Restarting => "reiniciando",
        WorkerStatus::Stopped => "detenido",
    }
}

fn supervisor_event_label(event: &SupervisorEvent) -> String {
    match event {
        SupervisorEvent::HeartbeatReceived(heartbeat) => {
            format!(
                "tick {} worker {} heartbeat",
                heartbeat.tick, heartbeat.worker_id
            )
        }
        SupervisorEvent::WorkerTimedOut(timeout) => {
            format!("tick {} worker {} timeout", timeout.tick, timeout.worker_id)
        }
        SupervisorEvent::WorkerRestarted {
            tick,
            worker_id,
            restart_count,
        } => format!("tick {tick} worker {worker_id} reinicio {restart_count}"),
        SupervisorEvent::WorkerRestartLimitReached {
            tick,
            worker_id,
            restart_count,
        } => format!("tick {tick} worker {worker_id} limite {restart_count}"),
    }
}

#[cfg(test)]
mod tests {
    use super::{supervisor_event_labels, supervisor_rows};
    use crate::core::supervisor::{RestartPolicy, SupervisorState, WorkerState};

    #[test]
    fn supervisor_rows_use_spanish_status_labels() {
        let mut supervisor = SupervisorState::new(RestartPolicy::never());
        supervisor
            .register_worker(WorkerState::new(1, "worker_a", 2).expect("worker valido"))
            .expect("registro valido");

        let rows = supervisor_rows(&supervisor);

        assert_eq!(rows[0].estado, "activo");
    }

    #[test]
    fn supervisor_event_labels_are_stable() {
        let mut supervisor = SupervisorState::new(RestartPolicy::never());
        supervisor
            .register_worker(WorkerState::new(1, "worker_a", 2).expect("worker valido"))
            .expect("registro valido");
        supervisor.heartbeat(1, 1).expect("heartbeat valido");

        let labels = supervisor_event_labels(&supervisor.events);

        assert_eq!(labels[0], "tick 1 worker 1 heartbeat");
    }
}
