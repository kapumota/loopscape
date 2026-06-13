use serde::{Deserialize, Serialize};

use super::scheduler::SimulationState;
use super::task::TaskStatus;

/// Entrada normalizada para calcular métricas del núcleo.
///
/// La capa visual puede construir esta entrada desde Bevy, pero la fórmula de
/// métricas se mantiene centralizada en el core.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CoreMetricsInput {
    pub tick: u64,
    pub active_loops: usize,
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub pending_tasks: usize,
    pub assigned_tasks: usize,
}

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

        Self::from_input(CoreMetricsInput {
            tick: state.tick,
            active_loops: state.agents.len(),
            total_tasks: state.tasks.len(),
            completed_tasks,
            pending_tasks,
            assigned_tasks,
        })
    }

    pub fn from_input(input: CoreMetricsInput) -> Self {
        let throughput = if input.tick == 0 {
            0.0
        } else {
            input.completed_tasks as f32 / input.tick as f32
        };

        Self {
            tick: input.tick,
            active_loops: input.active_loops,
            total_tasks: input.total_tasks,
            completed_tasks: input.completed_tasks,
            pending_tasks: input.pending_tasks,
            assigned_tasks: input.assigned_tasks,
            throughput,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CoreMetrics, CoreMetricsInput};

    #[test]
    fn metrics_from_input_are_deterministic() {
        let input = CoreMetricsInput {
            tick: 10,
            active_loops: 4,
            total_tasks: 8,
            completed_tasks: 3,
            pending_tasks: 2,
            assigned_tasks: 3,
        };

        let first = CoreMetrics::from_input(input.clone());
        let second = CoreMetrics::from_input(input);

        assert_eq!(first, second);
        assert_eq!(first.throughput, 0.3);
    }
}
