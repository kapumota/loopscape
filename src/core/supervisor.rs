use serde::{Deserialize, Serialize};

/// Identificador estable de un worker supervisado.
pub type WorkerId = u32;

/// Estado operativo de un worker dentro del supervisor.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum WorkerStatus {
    Running,
    TimedOut,
    Restarting,
    Stopped,
}

/// Politica determinista para reiniciar workers ante timeout.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum RestartPolicy {
    Never,
    OnTimeout { max_restarts: u32 },
}

impl RestartPolicy {
    pub fn never() -> Self {
        Self::Never
    }

    pub fn on_timeout(max_restarts: u32) -> Self {
        Self::OnTimeout { max_restarts }
    }

    pub fn can_restart(&self, restart_count: u32) -> bool {
        match self {
            Self::Never => false,
            Self::OnTimeout { max_restarts } => restart_count < *max_restarts,
        }
    }
}

/// Heartbeat determinista emitido por un worker.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HeartbeatEvent {
    pub tick: u64,
    pub worker_id: WorkerId,
}

/// Timeout determinista detectado por el supervisor.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkerTimeout {
    pub tick: u64,
    pub worker_id: WorkerId,
    pub last_heartbeat_tick: u64,
    pub timeout_ticks: u64,
    pub elapsed_ticks: u64,
}

/// Eventos internos del supervisor multiagente.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SupervisorEvent {
    HeartbeatReceived(HeartbeatEvent),
    WorkerTimedOut(WorkerTimeout),
    WorkerRestarted {
        tick: u64,
        worker_id: WorkerId,
        restart_count: u32,
    },
    WorkerRestartLimitReached {
        tick: u64,
        worker_id: WorkerId,
        restart_count: u32,
    },
}

impl SupervisorEvent {
    pub fn tick(&self) -> u64 {
        match self {
            Self::HeartbeatReceived(event) => event.tick,
            Self::WorkerTimedOut(event) => event.tick,
            Self::WorkerRestarted { tick, .. } | Self::WorkerRestartLimitReached { tick, .. } => {
                *tick
            }
        }
    }
}

/// Estado determinista de un worker supervisado.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkerState {
    pub id: WorkerId,
    pub name: String,
    pub status: WorkerStatus,
    pub last_heartbeat_tick: u64,
    pub timeout_ticks: u64,
    pub restart_count: u32,
}

impl WorkerState {
    pub fn new(id: WorkerId, name: impl Into<String>, timeout_ticks: u64) -> Result<Self, String> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err("el worker debe tener nombre".to_string());
        }
        if timeout_ticks == 0 {
            return Err("timeout_ticks debe ser mayor que cero".to_string());
        }

        Ok(Self {
            id,
            name,
            status: WorkerStatus::Running,
            last_heartbeat_tick: 0,
            timeout_ticks,
            restart_count: 0,
        })
    }

    pub fn is_active(&self) -> bool {
        matches!(
            self.status,
            WorkerStatus::Running | WorkerStatus::Restarting
        )
    }
}

/// Resumen de resiliencia producido por el supervisor.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SupervisorMetrics {
    pub workers_total: usize,
    pub workers_active: usize,
    pub workers_timed_out: usize,
    pub failures_detected: u64,
    pub failures_recovered: u64,
}

/// Supervisor multiagente determinista.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SupervisorState {
    pub tick: u64,
    pub workers: Vec<WorkerState>,
    pub restart_policy: RestartPolicy,
    pub events: Vec<SupervisorEvent>,
}

impl SupervisorState {
    pub fn new(restart_policy: RestartPolicy) -> Self {
        Self {
            tick: 0,
            workers: Vec::new(),
            restart_policy,
            events: Vec::new(),
        }
    }

    pub fn register_worker(&mut self, worker: WorkerState) -> Result<(), String> {
        if self.workers.iter().any(|item| item.id == worker.id) {
            return Err(format!("worker duplicado: {}", worker.id));
        }
        self.workers.push(worker);
        Ok(())
    }

    pub fn heartbeat(&mut self, tick: u64, worker_id: WorkerId) -> Result<(), String> {
        self.tick = self.tick.max(tick);
        let worker = self.worker_mut(worker_id)?;
        worker.last_heartbeat_tick = tick;
        worker.status = WorkerStatus::Running;

        self.events
            .push(SupervisorEvent::HeartbeatReceived(HeartbeatEvent {
                tick,
                worker_id,
            }));
        Ok(())
    }

    pub fn advance_to_tick(&mut self, tick: u64) -> Result<Vec<SupervisorEvent>, String> {
        if tick < self.tick {
            return Err("el supervisor no puede retroceder ticks".to_string());
        }

        self.tick = tick;
        let start = self.events.len();
        let worker_ids = self
            .workers
            .iter()
            .map(|worker| worker.id)
            .collect::<Vec<_>>();

        for worker_id in worker_ids {
            self.check_worker_timeout(worker_id)?;
        }

        Ok(self.events[start..].to_vec())
    }

    pub fn metrics(&self) -> SupervisorMetrics {
        let workers_active = self
            .workers
            .iter()
            .filter(|worker| worker.is_active())
            .count();
        let workers_timed_out = self
            .workers
            .iter()
            .filter(|worker| worker.status == WorkerStatus::TimedOut)
            .count();
        let failures_detected = self
            .events
            .iter()
            .filter(|event| matches!(event, SupervisorEvent::WorkerTimedOut(_)))
            .count() as u64;
        let failures_recovered = self
            .events
            .iter()
            .filter(|event| matches!(event, SupervisorEvent::WorkerRestarted { .. }))
            .count() as u64;

        SupervisorMetrics {
            workers_total: self.workers.len(),
            workers_active,
            workers_timed_out,
            failures_detected,
            failures_recovered,
        }
    }

    pub fn worker(&self, worker_id: WorkerId) -> Option<&WorkerState> {
        self.workers.iter().find(|worker| worker.id == worker_id)
    }

    fn worker_mut(&mut self, worker_id: WorkerId) -> Result<&mut WorkerState, String> {
        self.workers
            .iter_mut()
            .find(|worker| worker.id == worker_id)
            .ok_or_else(|| format!("worker no encontrado: {worker_id}"))
    }

    fn check_worker_timeout(&mut self, worker_id: WorkerId) -> Result<(), String> {
        let tick = self.tick;
        let timeout_event = {
            let worker = self.worker_mut(worker_id)?;
            if worker.status != WorkerStatus::Running {
                return Ok(());
            }

            let elapsed_ticks = tick.saturating_sub(worker.last_heartbeat_tick);
            if elapsed_ticks <= worker.timeout_ticks {
                return Ok(());
            }

            worker.status = WorkerStatus::TimedOut;
            WorkerTimeout {
                tick,
                worker_id,
                last_heartbeat_tick: worker.last_heartbeat_tick,
                timeout_ticks: worker.timeout_ticks,
                elapsed_ticks,
            }
        };

        self.events
            .push(SupervisorEvent::WorkerTimedOut(timeout_event));
        self.apply_restart_policy(worker_id)?;
        Ok(())
    }

    fn apply_restart_policy(&mut self, worker_id: WorkerId) -> Result<(), String> {
        let tick = self.tick;
        let restart_event = {
            let policy = self.restart_policy.clone();
            let worker = self.worker_mut(worker_id)?;
            if policy.can_restart(worker.restart_count) {
                worker.status = WorkerStatus::Restarting;
                worker.restart_count += 1;
                worker.last_heartbeat_tick = tick;
                worker.status = WorkerStatus::Running;
                SupervisorEvent::WorkerRestarted {
                    tick,
                    worker_id,
                    restart_count: worker.restart_count,
                }
            } else {
                SupervisorEvent::WorkerRestartLimitReached {
                    tick,
                    worker_id,
                    restart_count: worker.restart_count,
                }
            }
        };

        self.events.push(restart_event);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{RestartPolicy, SupervisorEvent, SupervisorState, WorkerState, WorkerStatus};

    #[test]
    fn heartbeat_updates_worker_tick_and_records_event() {
        let mut supervisor = SupervisorState::new(RestartPolicy::never());
        supervisor
            .register_worker(WorkerState::new(1, "worker_a", 3).expect("worker valido"))
            .expect("registro valido");

        supervisor.heartbeat(2, 1).expect("heartbeat valido");

        let worker = supervisor.worker(1).expect("worker registrado");
        assert_eq!(worker.last_heartbeat_tick, 2);
        assert!(matches!(
            supervisor.events.last(),
            Some(SupervisorEvent::HeartbeatReceived(_))
        ));
    }

    #[test]
    fn supervisor_detects_timeout_deterministically() {
        let mut supervisor = SupervisorState::new(RestartPolicy::never());
        supervisor
            .register_worker(WorkerState::new(1, "worker_a", 2).expect("worker valido"))
            .expect("registro valido");

        let events = supervisor.advance_to_tick(3).expect("avance valido");

        assert!(events
            .iter()
            .any(|event| matches!(event, SupervisorEvent::WorkerTimedOut(_))));
        assert_eq!(
            supervisor.worker(1).expect("worker registrado").status,
            WorkerStatus::TimedOut
        );
    }

    #[test]
    fn restart_policy_recovers_timed_out_worker() {
        let mut supervisor = SupervisorState::new(RestartPolicy::on_timeout(1));
        supervisor
            .register_worker(WorkerState::new(1, "worker_a", 2).expect("worker valido"))
            .expect("registro valido");

        supervisor.advance_to_tick(3).expect("avance valido");

        let worker = supervisor.worker(1).expect("worker registrado");
        assert_eq!(worker.status, WorkerStatus::Running);
        assert_eq!(worker.restart_count, 1);
        assert_eq!(supervisor.metrics().failures_detected, 1);
        assert_eq!(supervisor.metrics().failures_recovered, 1);
    }

    #[test]
    fn restart_limit_prevents_unbounded_restarts() {
        let mut supervisor = SupervisorState::new(RestartPolicy::on_timeout(1));
        supervisor
            .register_worker(WorkerState::new(1, "worker_a", 2).expect("worker valido"))
            .expect("registro valido");

        supervisor.advance_to_tick(3).expect("primer timeout");
        supervisor.advance_to_tick(6).expect("segundo timeout");

        assert!(supervisor
            .events
            .iter()
            .any(|event| matches!(event, SupervisorEvent::WorkerRestartLimitReached { .. })));
        assert_eq!(
            supervisor.worker(1).expect("worker registrado").status,
            WorkerStatus::TimedOut
        );
    }
}
