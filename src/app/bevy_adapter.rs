#![allow(dead_code)]

use crate::components::LoopState as VisualLoopState;
use crate::core::loop_state::CoreLoopState;
use crate::core::metrics::{CoreMetrics, CoreMetricsInput};

/// Traduce estados del núcleo a nombres visibles para la capa Bevy.
pub fn core_state_label(state: CoreLoopState) -> &'static str {
    match state {
        CoreLoopState::Idle => "inactivo",
        CoreLoopState::Thinking => "pensando",
        CoreLoopState::Acting => "actuando",
        CoreLoopState::Observing => "observando",
        CoreLoopState::Terminated => "terminado",
    }
}

/// Traduce el estado visual de Bevy al estado lógico del núcleo.
pub fn core_state_from_visual(state: &VisualLoopState) -> CoreLoopState {
    match state {
        VisualLoopState::Spawning => CoreLoopState::Idle,
        VisualLoopState::Thinking => CoreLoopState::Thinking,
        VisualLoopState::Acting => CoreLoopState::Acting,
        VisualLoopState::Observing => CoreLoopState::Observing,
        VisualLoopState::Supervising => CoreLoopState::Acting,
        VisualLoopState::Terminated => CoreLoopState::Terminated,
    }
}

/// Convierte segundos de la capa visual en ticks lógicos aproximados.
///
/// Esta función solo adapta escala temporal. La lógica de métricas permanece
/// dentro de `CoreMetrics`.
pub fn estimate_tick_from_seconds(elapsed_seconds: f32) -> u64 {
    let safe_seconds = elapsed_seconds.max(0.0);
    (safe_seconds * 60.0).round() as u64
}

/// Deriva métricas del core desde estados visuales sin duplicar fórmulas.
pub fn core_metrics_from_visual_states<'a, I>(tick: u64, states: I) -> CoreMetrics
where
    I: IntoIterator<Item = &'a VisualLoopState>,
{
    let core_states = states
        .into_iter()
        .map(core_state_from_visual)
        .collect::<Vec<_>>();

    let active_loops = core_states
        .iter()
        .filter(|state| **state != CoreLoopState::Terminated)
        .count();

    let completed_tasks = core_states
        .iter()
        .filter(|state| **state == CoreLoopState::Terminated)
        .count();

    let pending_tasks = core_states
        .iter()
        .filter(|state| **state == CoreLoopState::Idle)
        .count();

    let total_tasks = core_states.len();
    let assigned_tasks = total_tasks
        .saturating_sub(pending_tasks)
        .saturating_sub(completed_tasks);

    CoreMetrics::from_input(CoreMetricsInput {
        tick,
        active_loops,
        total_tasks,
        completed_tasks,
        pending_tasks,
        assigned_tasks,
    })
}
