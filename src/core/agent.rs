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

#[cfg(test)]
mod tests {
    use super::{AgentId, CoreAgent};
    use crate::core::loop_state::CoreLoopState;
    use crate::core::task::TaskId;

    #[test]
    fn new_agent_starts_available() {
        let agent = CoreAgent::new(AgentId(7), "worker");

        assert_eq!(agent.id, AgentId(7));
        assert_eq!(agent.role, "worker");
        assert_eq!(agent.state, CoreLoopState::Idle);
        assert_eq!(agent.energy, 100);
        assert_eq!(agent.completed_tasks, 0);
        assert_eq!(agent.current_task, None);
        assert!(agent.is_available());
    }

    #[test]
    fn assigned_agent_is_not_available() {
        let mut agent = CoreAgent::new(AgentId(1), "worker");
        agent.current_task = Some(TaskId(10));

        assert!(!agent.is_available());
    }

    #[test]
    fn non_idle_agent_is_not_available() {
        let mut agent = CoreAgent::new(AgentId(2), "worker");
        agent.state = CoreLoopState::Thinking;

        assert!(!agent.is_available());
    }
}
