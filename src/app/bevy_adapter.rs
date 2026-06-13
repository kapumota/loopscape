#![allow(dead_code)]

use crate::core::loop_state::CoreLoopState;

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
