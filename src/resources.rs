use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GlobalPrompt {
    pub text: String,
    pub hash: u64,
    pub last_modified: f32,
}

#[derive(Resource, Default)]
pub struct TaskQueue {
    pub tasks: Vec<String>,
    pub incoming_rate: f32,
}

#[derive(Resource, Default)]
pub struct Metrics {
    pub active_loops: usize,
    pub throughput: f32,
    pub consensus_term: u32,
    pub era_timer: f32,
}

#[derive(Resource, Default)]
pub struct XRayMode {
    pub enabled: bool,
}

#[derive(Resource)]
pub struct EraConfig {
    pub current_era_index: usize,
    pub era_names: Vec<String>,
}

impl Default for EraConfig {
    fn default() -> Self {
        Self {
            current_era_index: 0,
            era_names: vec![
                "Menu".to_string(),
                "ReAct (2022)".to_string(),
                "Autoprompting (2023)".to_string(),
                "Ralph Loop (2025)".to_string(),
                "Ralph formalizado (2026)".to_string(),
                "Orquestacion multiagente (futuro)".to_string(),
            ],
        }
    }
}
