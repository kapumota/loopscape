use serde::{Deserialize, Serialize};

use super::loop_state::CoreLoopState;
use super::task::TaskId;

/// Identificador estable de agente en el núcleo.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AgentId(pub u64);

/// Agente lógico sin dependencias de Bevy.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CoreAgent {
    pub id: AgentId,
    pub role: String,
    pub state: CoreLoopState,
    pub energy: u32,
    pub completed_tasks: u32,
    pub current_task: Option<TaskId>,
}

impl CoreAgent {
    pub fn new(id: AgentId, role: impl Into<String>) -> Self {
        Self {
            id,
            role: role.into(),
            state: CoreLoopState::Idle,
            energy: 100,
            completed_tasks: 0,
            current_task: None,
        }
    }

    pub fn is_available(&self) -> bool {
        self.state == CoreLoopState::Idle && self.current_task.is_none()
    }
}
