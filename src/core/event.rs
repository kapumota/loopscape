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

#[cfg(test)]
mod tests {
    use super::CoreEvent;
    use crate::core::agent::AgentId;
    use crate::core::task::TaskId;

    #[test]
    fn core_event_serializes_task_assignment() {
        let event = CoreEvent::TaskAssigned {
            tick: 4,
            task: TaskId(2),
            agent: AgentId(1),
            duration: 3,
        };

        let encoded = serde_json::to_string(&event).expect("debe serializar evento del core");
        let decoded: CoreEvent =
            serde_json::from_str(&encoded).expect("debe reconstruir evento del core");

        assert_eq!(event, decoded);
    }
}
