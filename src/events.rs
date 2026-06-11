use bevy::prelude::*;
use crate::components::*;

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
