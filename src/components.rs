use bevy::prelude::*;

// --------- IDENTIDAD ---------
#[derive(Component)]
pub struct LoopAgent;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopState {
    Thinking,
    Acting,
    Observing,
    Spawning,
    Supervising,
    Terminated,
}

// --------- ERA 1: ReAct ---------
#[derive(Component)]
pub struct ThinkTimer(pub Timer);

#[derive(Component)]
pub struct ActTimer(pub Timer);

#[derive(Component)]
pub struct ObserveTimer(pub Timer);

#[derive(Component)]
pub struct WiredTool {
    pub tool_id: Entity,
    pub cooldown: Timer,
}

// --------- ERA 2: Autoprompting ---------
#[derive(Component)]
pub struct TaskDecomposer {
    pub original_task: String,
    pub subtasks: Vec<String>,
    pub decomposition_depth: u32,
}

#[derive(Component)]
pub struct SubLoop {
    pub parent: Entity,
    pub lifetime: Timer,
}

// --------- ERA 3: Ralph Loop ---------
#[derive(Component)]
pub struct RalphDna {
    pub prompt_hash: u64,
    pub sync_offset: f32,
}

// --------- ERA 4: Ralph formalizado ---------
#[derive(Component)]
pub struct FormalCommand {
    pub command: CommandType,
    pub arguments: Vec<String>,
    pub execution_timer: Timer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandType {
    Goal,
    Plan,
    Delegate,
    Verify,
    Terminate,
}

#[derive(Component)]
pub struct GoalTree {
    pub root: String,
    pub completed_nodes: Vec<String>,
    pub is_complete: bool,
}

#[derive(Component)]
pub struct GoalNode {
    pub parent: Entity,
    pub depth: u32,
}

// --------- ERA 5: Orquestacion multiagente ---------
#[derive(Component)]
pub struct Supervisor {
    pub workers: Vec<Entity>,
    pub heartbeat_interval: Timer,
    pub last_heartbeats: Vec<(Entity, f32)>,
}

#[derive(Component)]
pub struct Worker {
    pub supervisor: Option<Entity>,
    pub task_load: f32,
}

#[derive(Component)]
pub struct ConsensusVoter {
    pub term: u32,
    pub voted_for: Option<Entity>,
    pub state: ConsensusState,
    pub election_timeout: Timer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsensusState {
    Follower,
    Candidate,
    Leader,
}

#[derive(Component)]
pub struct ByzantineSuspect {
    pub trust_score: f32,
    pub inconsistency_count: u32,
}

// --------- VISUALIZACIÓN ---------
#[derive(Component)]
pub struct LoopVisual {
    pub base_color: Color,
    pub pulse_speed: f32,
    pub radius: f32,
}

#[derive(Component)]
pub struct ConnectionLine {
    pub from: Entity,
    pub to: Entity,
    pub line_type: ConnectionType,
    pub color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    ToolWire,
    ParentChild,
    DnaSync,
    CommandFlow,
    Heartbeat,
    ConsensusVote,
}

#[derive(Component)]
pub struct FloatingLabel {
    pub text: String,
}
