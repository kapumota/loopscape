#![allow(dead_code)]

// Recursos globales reservados para UI, metricas y modos visuales.
// Algunos campos se activaran al cerrar las fases de editor y replay.
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DslCommandStatus {
    Pending,
    Active,
    Completed,
    Error,
}

impl DslCommandStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Pending => "pendiente",
            Self::Active => "activo",
            Self::Completed => "completado",
            Self::Error => "error",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DslViewerCommand {
    pub line: usize,
    pub text: String,
    pub status: DslCommandStatus,
}

impl DslViewerCommand {
    pub fn new(line: usize, text: impl Into<String>) -> Self {
        Self {
            line,
            text: text.into(),
            status: DslCommandStatus::Pending,
        }
    }
}

#[derive(Resource, Clone, Debug, Default)]
pub struct LoadedDslProgram {
    pub script_path: Option<String>,
    pub commands: Vec<DslViewerCommand>,
    pub current_index: usize,
    pub error: Option<String>,
}

impl LoadedDslProgram {
    pub fn from_script_lines(script_path: impl Into<String>, script_lines: Vec<String>) -> Self {
        let commands = script_lines
            .into_iter()
            .enumerate()
            .map(|(index, line)| DslViewerCommand::new(index + 1, line))
            .collect::<Vec<_>>();

        let mut program = Self {
            script_path: Some(script_path.into()),
            commands,
            current_index: 0,
            error: None,
        };
        program.refresh_statuses();
        program
    }

    pub fn with_error(script_path: impl Into<String>, error: impl Into<String>) -> Self {
        Self {
            script_path: Some(script_path.into()),
            commands: vec![DslViewerCommand {
                line: 1,
                text: "script DSL invalido".to_string(),
                status: DslCommandStatus::Error,
            }],
            current_index: 0,
            error: Some(error.into()),
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.script_path.is_some() || self.error.is_some()
    }

    pub fn advance_to_tick(&mut self, tick: usize) {
        if self.commands.is_empty() || self.error.is_some() {
            return;
        }

        self.current_index = tick.min(self.commands.len().saturating_sub(1));
        self.refresh_statuses();
    }

    pub fn to_panel_text(&self) -> String {
        if !self.is_loaded() {
            return "DSL: sin script visual cargado\nUsa --script ruta.loop --visual para abrir el visor".to_string();
        }

        let mut lines = Vec::new();
        lines.push("Visor DSL".to_string());

        if let Some(path) = &self.script_path {
            lines.push(format!("Script: {path}"));
        }

        if let Some(error) = &self.error {
            lines.push("Estado: error".to_string());
            lines.push(error.clone());
        }

        for (index, command) in self.commands.iter().enumerate() {
            let marker = if index == self.current_index && self.error.is_none() {
                ">>"
            } else {
                "  "
            };
            lines.push(format!(
                "{marker} {:02}. [{}] {}",
                command.line,
                command.status.label(),
                command.text
            ));
        }

        lines.join("\n")
    }

    fn refresh_statuses(&mut self) {
        for (index, command) in self.commands.iter_mut().enumerate() {
            command.status = if self.error.is_some() {
                DslCommandStatus::Error
            } else if index < self.current_index {
                DslCommandStatus::Completed
            } else if index == self.current_index {
                DslCommandStatus::Active
            } else {
                DslCommandStatus::Pending
            };
        }
    }
}
