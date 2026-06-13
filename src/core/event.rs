use serde::{Deserialize, Serialize};

use super::agent::AgentId;
use super::loop_state::CoreLoopState;
use super::task::TaskId;

/// Evento interno del núcleo determinista.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CoreEvent {
    TickAdvanced {
        tick: u64,
    },
    TaskAssigned {
        tick: u64,
        task: TaskId,
        agent: AgentId,
        duration: u32,
    },
    AgentStateChanged {
        tick: u64,
        agent: AgentId,
        from: CoreLoopState,
        to: CoreLoopState,
    },
    TaskCompleted {
        tick: u64,
        task: TaskId,
        agent: AgentId,
    },
}
