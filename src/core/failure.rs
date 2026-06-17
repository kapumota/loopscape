use serde::{Deserialize, Serialize};

use super::supervisor::WorkerId;

/// Fallo recuperable y determinista aplicado a un worker.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkerFailureSpec {
    pub worker_id: WorkerId,
    pub start_tick: u64,
    pub duration_ticks: u64,
}

impl WorkerFailureSpec {
    pub fn new(worker_id: WorkerId, start_tick: u64, duration_ticks: u64) -> Result<Self, String> {
        if duration_ticks == 0 {
            return Err("duration_ticks debe ser mayor que cero".to_string());
        }

        Ok(Self {
            worker_id,
            start_tick,
            duration_ticks,
        })
    }

    pub fn contains_tick(&self, tick: u64) -> bool {
        let end_tick = self.start_tick.saturating_add(self.duration_ticks);
        tick >= self.start_tick && tick < end_tick
    }
}

/// Plan determinista de fallos recuperables para una corrida.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct RecoverableFailurePlan {
    pub failures: Vec<WorkerFailureSpec>,
}

impl RecoverableFailurePlan {
    pub fn none() -> Self {
        Self {
            failures: Vec::new(),
        }
    }

    pub fn worker_hangs(
        worker_id: WorkerId,
        start_tick: u64,
        duration_ticks: u64,
    ) -> Result<Self, String> {
        Ok(Self {
            failures: vec![WorkerFailureSpec::new(
                worker_id,
                start_tick,
                duration_ticks,
            )?],
        })
    }

    pub fn with_failure(mut self, failure: WorkerFailureSpec) -> Self {
        self.failures.push(failure);
        self
    }

    pub fn is_worker_hung(&self, worker_id: WorkerId, tick: u64) -> bool {
        self.failures
            .iter()
            .any(|failure| failure.worker_id == worker_id && failure.contains_tick(tick))
    }
}

#[cfg(test)]
mod tests {
    use super::{RecoverableFailurePlan, WorkerFailureSpec};

    #[test]
    fn worker_failure_spec_rejects_zero_duration() {
        let error = WorkerFailureSpec::new(1, 3, 0).expect_err("debe fallar");

        assert!(error.contains("duration_ticks"));
    }

    #[test]
    fn recoverable_failure_plan_marks_hung_worker_by_tick() {
        let plan = RecoverableFailurePlan::worker_hangs(2, 4, 3).expect("plan valido");

        assert!(!plan.is_worker_hung(2, 3));
        assert!(plan.is_worker_hung(2, 4));
        assert!(plan.is_worker_hung(2, 6));
        assert!(!plan.is_worker_hung(2, 7));
        assert!(!plan.is_worker_hung(3, 4));
    }
}
