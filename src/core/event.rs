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
    GoalCreated {
        tick: u64,
        goal: String,
    },
    PlanStepCreated {
        tick: u64,
        index: u32,
        step: String,
    },
    DelegationRequested {
        tick: u64,
        target: String,
        worker: String,
    },
    VerificationRequested {
        tick: u64,
        checklist: String,
    },
    TerminationPolicySet {
        tick: u64,
        policy: String,
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

    #[test]
    fn core_event_can_represent_dsl_interpreter_outputs() {
        let events = vec![
            CoreEvent::GoalCreated {
                tick: 0,
                goal: "rescatar_victimas".to_string(),
            },
            CoreEvent::PlanStepCreated {
                tick: 0,
                index: 0,
                step: "buscar".to_string(),
            },
            CoreEvent::DelegationRequested {
                tick: 0,
                target: "sector_a".to_string(),
                worker: "worker_1".to_string(),
            },
            CoreEvent::VerificationRequested {
                tick: 0,
                checklist: "checklist_final".to_string(),
            },
            CoreEvent::TerminationPolicySet {
                tick: 0,
                policy: "when_verified".to_string(),
            },
        ];

        assert_eq!(events.len(), 5);
    }
}
