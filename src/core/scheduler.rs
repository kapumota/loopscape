use serde::{Deserialize, Serialize};

use super::agent::{AgentId, CoreAgent};
use super::event::CoreEvent;
use super::loop_state::CoreLoopState;
use super::metrics::CoreMetrics;
use super::rng::DeterministicRng;
use super::task::{CoreTask, TaskId, TaskStatus};

/// Configuración inicial de una simulación determinista.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub seed: u64,
    pub initial_agents: u32,
    pub initial_tasks: u32,
}

impl SimulationConfig {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            initial_agents: 5,
            initial_tasks: 12,
        }
    }

    pub fn with_size(mut self, initial_agents: u32, initial_tasks: u32) -> Self {
        self.initial_agents = initial_agents;
        self.initial_tasks = initial_tasks;
        self
    }
}

/// Estado completo y serializable del núcleo.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimulationState {
    pub tick: u64,
    pub seed: u64,
    pub agents: Vec<CoreAgent>,
    pub tasks: Vec<CoreTask>,
    pub events: Vec<CoreEvent>,
    pub metrics: CoreMetrics,
    rng: DeterministicRng,
}

impl SimulationState {
    pub fn new(config: SimulationConfig) -> Self {
        let agents = (0..config.initial_agents)
            .map(|index| CoreAgent::new(AgentId(u64::from(index)), format!("agente_{index}")))
            .collect::<Vec<_>>();

        let tasks = (0..config.initial_tasks)
            .map(|index| CoreTask::new(TaskId(u64::from(index)), format!("tarea_{index}")))
            .collect::<Vec<_>>();

        let mut state = Self {
            tick: 0,
            seed: config.seed,
            agents,
            tasks,
            events: Vec::new(),
            metrics: CoreMetrics {
                tick: 0,
                active_loops: 0,
                total_tasks: 0,
                completed_tasks: 0,
                pending_tasks: 0,
                assigned_tasks: 0,
                throughput: 0.0,
            },
            rng: DeterministicRng::new(config.seed),
        };

        state.refresh_metrics();
        state
    }

    pub fn run_ticks(&mut self, ticks: u32) {
        for _ in 0..ticks {
            self.tick();
        }
    }

    pub fn tick(&mut self) {
        self.tick += 1;
        self.events
            .push(CoreEvent::TickAdvanced { tick: self.tick });

        self.assign_pending_tasks();
        self.advance_agents();
        self.refresh_metrics();
    }

    fn assign_pending_tasks(&mut self) {
        let pending_indices = self
            .tasks
            .iter()
            .enumerate()
            .filter_map(|(index, task)| {
                if task.status == TaskStatus::Pending {
                    Some(index)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for task_index in pending_indices {
            let Some(agent_index) = self.agents.iter().position(CoreAgent::is_available) else {
                return;
            };

            let duration = 1 + self.rng.next_range(5);
            let task_id = self.tasks[task_index].id;
            let agent_id = self.agents[agent_index].id;

            self.tasks[task_index].status = TaskStatus::Assigned;
            self.tasks[task_index].assigned_to = Some(agent_id);
            self.tasks[task_index].remaining_ticks = duration;

            self.change_agent_state(agent_index, CoreLoopState::Thinking);
            self.agents[agent_index].current_task = Some(task_id);

            self.events.push(CoreEvent::TaskAssigned {
                tick: self.tick,
                task: task_id,
                agent: agent_id,
                duration,
            });
        }
    }

    fn advance_agents(&mut self) {
        for agent_index in 0..self.agents.len() {
            match self.agents[agent_index].state {
                CoreLoopState::Idle | CoreLoopState::Terminated => {}
                CoreLoopState::Thinking => {
                    self.change_agent_state(agent_index, CoreLoopState::Acting);
                }
                CoreLoopState::Acting => {
                    self.advance_agent_task(agent_index);
                }
                CoreLoopState::Observing => {
                    self.change_agent_state(agent_index, CoreLoopState::Idle);
                }
            }
        }
    }

    fn advance_agent_task(&mut self, agent_index: usize) {
        let Some(task_id) = self.agents[agent_index].current_task else {
            self.change_agent_state(agent_index, CoreLoopState::Idle);
            return;
        };

        let Some(task_index) = self.tasks.iter().position(|task| task.id == task_id) else {
            self.agents[agent_index].current_task = None;
            self.change_agent_state(agent_index, CoreLoopState::Idle);
            return;
        };

        if self.tasks[task_index].remaining_ticks > 0 {
            self.tasks[task_index].remaining_ticks -= 1;
        }

        if self.tasks[task_index].remaining_ticks == 0 {
            let agent_id = self.agents[agent_index].id;

            self.tasks[task_index].status = TaskStatus::Completed;
            self.agents[agent_index].completed_tasks += 1;
            self.agents[agent_index].current_task = None;

            self.events.push(CoreEvent::TaskCompleted {
                tick: self.tick,
                task: task_id,
                agent: agent_id,
            });

            self.change_agent_state(agent_index, CoreLoopState::Observing);
        }
    }

    fn change_agent_state(&mut self, agent_index: usize, next: CoreLoopState) {
        let current = self.agents[agent_index].state;

        if current == next {
            return;
        }

        let agent_id = self.agents[agent_index].id;
        self.agents[agent_index].state = next;

        self.events.push(CoreEvent::AgentStateChanged {
            tick: self.tick,
            agent: agent_id,
            from: current,
            to: next,
        });
    }

    fn refresh_metrics(&mut self) {
        self.metrics = CoreMetrics::from_state(self);
    }
}

#[cfg(test)]
mod tests {
    use super::{SimulationConfig, SimulationState};
    use crate::core::event::CoreEvent;
    use crate::core::task::TaskStatus;

    #[test]
    fn scheduler_initializes_configured_state() {
        let config = SimulationConfig::new(55).with_size(2, 3);
        let state = SimulationState::new(config);

        assert_eq!(state.tick, 0);
        assert_eq!(state.seed, 55);
        assert_eq!(state.agents.len(), 2);
        assert_eq!(state.tasks.len(), 3);
        assert_eq!(state.metrics.active_loops, 2);
        assert_eq!(state.metrics.total_tasks, 3);
    }

    #[test]
    fn scheduler_assigns_tasks_on_first_tick() {
        let config = SimulationConfig::new(7).with_size(1, 1);
        let mut state = SimulationState::new(config);

        state.tick();

        assert_eq!(state.tick, 1);
        assert_eq!(state.tasks[0].status, TaskStatus::Assigned);
        assert!(state.tasks[0].assigned_to.is_some());
        assert!(state
            .events
            .iter()
            .any(|event| matches!(event, CoreEvent::TaskAssigned { .. })));
    }

    #[test]
    fn scheduler_completes_tasks_deterministically() {
        let config = SimulationConfig::new(123).with_size(2, 4);
        let mut first = SimulationState::new(config.clone());
        let mut second = SimulationState::new(config);

        first.run_ticks(20);
        second.run_ticks(20);

        assert_eq!(first, second);
        assert!(first.metrics.completed_tasks > 0);
    }
}
