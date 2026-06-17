//! Núcleo determinista de simulación de Loopscape.
//!
//! Este módulo no depende de Bevy. Su objetivo es permitir simulaciones
//! reproducibles por semilla, con ticks discretos, eventos tipados y métricas
//! derivadas del estado interno.

pub mod agent;
pub mod byzantine;
pub mod compare;
pub mod event;
pub mod failure;
pub mod loop_state;
pub mod metrics;
pub mod replay;
pub mod rng;
pub mod scheduler;
pub mod task;
pub mod trace;

#[cfg(test)]
mod tests {
    use super::metrics::CoreMetrics;
    use super::scheduler::{SimulationConfig, SimulationState};
    use super::task::TaskStatus;

    #[test]
    fn core_deterministic_same_seed_same_state() {
        let config = SimulationConfig::new(123);
        let mut first = SimulationState::new(config.clone());
        let mut second = SimulationState::new(config);

        first.run_ticks(40);
        second.run_ticks(40);

        assert_eq!(first, second);
    }

    #[test]
    fn core_deterministic_different_seed_changes_trace() {
        let mut first = SimulationState::new(SimulationConfig::new(123));
        let mut second = SimulationState::new(SimulationConfig::new(999));

        first.run_ticks(20);
        second.run_ticks(20);

        assert_ne!(first.events, second.events);
    }

    #[test]
    fn core_state_is_serializable() {
        let mut state = SimulationState::new(SimulationConfig::new(123));
        state.run_ticks(10);

        let encoded = serde_json::to_string(&state).expect("debe serializar estado del core");
        let decoded: SimulationState =
            serde_json::from_str(&encoded).expect("debe reconstruir estado del core");

        assert_eq!(state, decoded);
    }

    #[test]
    fn core_metrics_are_derived_from_state() {
        let mut state = SimulationState::new(SimulationConfig::new(321));
        state.run_ticks(12);

        let metrics = CoreMetrics::from_state(&state);
        let completed = state
            .tasks
            .iter()
            .filter(|task| task.status == TaskStatus::Completed)
            .count();

        assert_eq!(metrics.completed_tasks, completed);
        assert_eq!(metrics.active_loops, state.agents.len());
        assert_eq!(metrics.total_tasks, state.tasks.len());
    }
}
pub mod supervisor;
