#![allow(dead_code)]

// Eventos reservados para transiciones, fallos y orquestacion avanzada.
// En Fase 1 se validan compilacion y ejecucion; su uso completo queda para
// las fases de simulacion, DSL y multiagente.
use bevy::prelude::*;

#[derive(Event)]
pub struct EraTransitionEvent {
    pub next_era_index: usize,
}

#[derive(Event)]
pub struct SpawnSubLoopEvent {
    pub parent: Entity,
    pub position: Vec3,
    pub task: String,
    pub depth: u32,
}

#[derive(Event)]
pub struct HeartbeatEvent {
    pub from: Entity,
    pub to: Entity,
}

#[derive(Event)]
pub struct ByzantineFaultEvent {
    pub target: Entity,
}
