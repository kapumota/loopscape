use serde::{Deserialize, Serialize};

use super::scheduler::SimulationState;
use super::task::TaskStatus;

/// Métricas derivadas del estado del núcleo.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CoreMetrics {
    pub tick: u64,
    pub active_loops: usize,
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub pending_tasks: usize,
    pub assigned_tasks: usize,
    pub throughput: f32,
}

impl CoreMetrics {
    pub fn from_state(state: &SimulationState) -> Self {
        let completed_tasks = state
            .tasks
            .iter()
            .filter(|task| task.status == TaskStatus::Completed)
            .count();

        let pending_tasks = state
            .tasks
            .iter()
            .filter(|task| task.status == TaskStatus::Pending)
            .count();

        let assigned_tasks = state
            .tasks
            .iter()
            .filter(|task| task.status == TaskStatus::Assigned)
            .count();

        let throughput = if state.tick == 0 {
            0.0
        } else {
            completed_tasks as f32 / state.tick as f32
        };

        Self {
            tick: state.tick,
            active_loops: state.agents.len(),
            total_tasks: state.tasks.len(),
            completed_tasks,
            pending_tasks,
            assigned_tasks,
            throughput,
        }
    }
}
