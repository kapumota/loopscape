use serde::{Deserialize, Serialize};

/// Estado lógico de un loop dentro del núcleo puro.
///
/// Este estado no conoce colores, gizmos ni entidades de Bevy.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CoreLoopState {
    Idle,
    Thinking,
    Acting,
    Observing,
    Terminated,
}
