use serde::{Deserialize, Serialize};

use super::agent::AgentId;

/// Identificador estable de tarea.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TaskId(pub u64);

/// Estado lógico de una tarea.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Assigned,
    Completed,
}

/// Tarea simulada por ticks discretos.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CoreTask {
    pub id: TaskId,
    pub description: String,
    pub status: TaskStatus,
    pub assigned_to: Option<AgentId>,
    pub remaining_ticks: u32,
}

impl CoreTask {
    pub fn new(id: TaskId, description: impl Into<String>) -> Self {
        Self {
            id,
            description: description.into(),
            status: TaskStatus::Pending,
            assigned_to: None,
            remaining_ticks: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CoreTask, TaskId, TaskStatus};

    #[test]
    fn new_task_starts_pending() {
        let task = CoreTask::new(TaskId(3), "clasificar datos");

        assert_eq!(task.id, TaskId(3));
        assert_eq!(task.description, "clasificar datos");
        assert_eq!(task.status, TaskStatus::Pending);
        assert_eq!(task.assigned_to, None);
        assert_eq!(task.remaining_ticks, 0);
    }
}
