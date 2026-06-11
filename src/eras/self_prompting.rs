use bevy::prelude::*;
use crate::components::*;
use crate::events::*;
use crate::resources::*;

pub fn setup_self_prompt_era(
    mut commands: Commands,
    mut metrics: ResMut<Metrics>,
) {
    metrics.active_loops = 0;
    metrics.era_timer = 0.0;

    // Crea 3 loops padre que descomponen tareas
    for i in 0..3 {
        let x = (i as f32 - 1.0) * 200.0;
        commands.spawn((
            LoopAgent,
            LoopState::Thinking,
            TaskDecomposer {
                original_task: format!("Complex task {}", i),
                subtasks: vec![],
                decomposition_depth: 0,
            },
            ThinkTimer(Timer::from_seconds(3.0, TimerMode::Repeating)),
            ActTimer(Timer::from_seconds(2.0, TimerMode::Repeating)),
            ObserveTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
            Transform::from_xyz(x, 0.0, 0.0),
            LoopVisual {
                base_color: Color::srgb(0.8, 0.3, 1.0),
                pulse_speed: 3.0,
                radius: 28.0,
            },
        ));
    }
}

pub fn autonomous_decomposition(
    mut commands: Commands,
    mut parents: Query<(Entity, &mut TaskDecomposer, &Transform), Without<SubLoop>>,
    mut events: EventWriter<SpawnSubLoopEvent>,
) {
    for (parent_entity, mut decomposer, transform) in parents.iter_mut() {
        if decomposer.subtasks.is_empty() && decomposer.decomposition_depth < 2 {
            decomposer.subtasks = vec![
                "analizar".to_string(),
                "recuperar".to_string(),
                "sintetizar".to_string(),
            ];

            for (j, subtask) in decomposer.subtasks.iter().enumerate() {
                let angle = (j as f32) * std::f32::consts::TAU / 3.0;
                let offset = Vec3::new(angle.cos() * 60.0, angle.sin() * 60.0, 0.0);
                events.send(SpawnSubLoopEvent {
                    parent: parent_entity,
                    position: transform.translation + offset,
                    task: subtask.clone(),
                    depth: decomposer.decomposition_depth + 1,
                });
            }
        }
    }
}

pub fn spawn_sub_loops(
    mut commands: Commands,
    mut events: EventReader<SpawnSubLoopEvent>,
    mut metrics: ResMut<Metrics>,
) {
    for ev in events.read() {
        let sub_entity = commands.spawn((
            LoopAgent,
            LoopState::Thinking,
            SubLoop {
                parent: ev.parent,
                lifetime: Timer::from_seconds(6.0, TimerMode::Once),
            },
            TaskDecomposer {
                original_task: ev.task.clone(),
                subtasks: vec![],
                decomposition_depth: ev.depth,
            },
            Transform::from_translation(ev.position),
            LoopVisual {
                base_color: Color::srgb(0.6, 0.2, 0.9),
                pulse_speed: 8.0,
                radius: 14.0,
            },
            ThinkTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
            ActTimer(Timer::from_seconds(0.8, TimerMode::Repeating)),
            ObserveTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
        )).id();

        // Linea padre-hijo
        commands.spawn((
            ConnectionLine {
                from: ev.parent,
                to: sub_entity,
                line_type: ConnectionType::ParentChild,
                color: Color::srgb(0.6, 0.2, 0.9),
            },
        ));

        metrics.active_loops += 1;
    }
}

pub fn sub_loop_lifetime(
    mut commands: Commands,
    mut sub_loops: Query<(Entity, &mut SubLoop, &mut LoopVisual)>,
    time: Res<Time>,
) {
    for (entity, mut sub_loop, mut visual) in sub_loops.iter_mut() {
        sub_loop.lifetime.tick(time.delta());
        let progress = sub_loop.lifetime.elapsed_secs() / sub_loop.lifetime.duration().as_secs_f32();
        visual.base_color.set_alpha(1.0 - progress);
        if sub_loop.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}
