use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn setup_productized_era(mut commands: Commands, mut metrics: ResMut<Metrics>) {
    metrics.active_loops = 0;
    metrics.era_timer = 0.0;

    // Crea 5 loops con comandos formales
    let commands_list = [
        (
            CommandType::Goal,
            vec!["rescatar".to_string(), "victimas".to_string()],
        ),
        (
            CommandType::Plan,
            vec!["fase1".to_string(), "fase2".to_string()],
        ),
        (
            CommandType::Delegate,
            vec!["sector_a".to_string(), "sector_b".to_string()],
        ),
        (CommandType::Verify, vec!["finalizacion".to_string()]),
        (CommandType::Terminate, vec![]),
    ];

    for (i, (cmd_type, args)) in commands_list.iter().enumerate() {
        let x = (i as f32 - 2.0) * 150.0;
        let color = match cmd_type {
            CommandType::Goal => Color::srgb(1.0, 0.2, 0.2),
            CommandType::Plan => Color::srgb(1.0, 0.5, 0.2),
            CommandType::Delegate => Color::srgb(0.2, 0.5, 1.0),
            CommandType::Verify => Color::srgb(0.2, 1.0, 0.5),
            CommandType::Terminate => Color::srgb(0.5, 0.5, 0.5),
        };

        commands.spawn((
            LoopAgent,
            LoopState::Thinking,
            FormalCommand {
                command: cmd_type.clone(),
                arguments: args.clone(),
                execution_timer: Timer::from_seconds(4.0, TimerMode::Once),
            },
            GoalTree {
                root: format!("{:?}", cmd_type),
                completed_nodes: vec![],
                is_complete: false,
            },
            Transform::from_xyz(x, 0.0, 0.0),
            LoopVisual {
                base_color: color,
                pulse_speed: 3.0,
                radius: 22.0,
            },
            ThinkTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
            ActTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
            ObserveTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
        ));
    }
}

pub fn command_execution_system(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut FormalCommand,
        &mut GoalTree,
        &mut LoopState,
        &Transform,
    )>,
    time: Res<Time>,
) {
    for (entity, mut cmd, mut tree, mut state, transform) in query.iter_mut() {
        cmd.execution_timer.tick(time.delta());

        match cmd.command {
            CommandType::Goal => {
                if cmd.execution_timer.just_finished() && tree.completed_nodes.is_empty() {
                    tree.completed_nodes.push("objetivo_definido".to_string());
                    // Crea nodos visuales del arbol
                    spawn_goal_node(&mut commands, entity, transform.translation, "objetivo", 0);
                }
            }
            CommandType::Plan => {
                if cmd.execution_timer.just_finished() && tree.completed_nodes.len() == 1 {
                    tree.completed_nodes.push("planificado".to_string());
                    spawn_goal_node(
                        &mut commands,
                        entity,
                        transform.translation + Vec3::new(0.0, 30.0, 0.0),
                        "plan",
                        1,
                    );
                }
            }
            CommandType::Delegate => {
                if cmd.execution_timer.just_finished() && tree.completed_nodes.len() == 2 {
                    tree.completed_nodes.push("delegado".to_string());
                }
            }
            CommandType::Verify => {
                if cmd.execution_timer.just_finished() && tree.completed_nodes.len() == 3 {
                    tree.completed_nodes.push("verificado".to_string());
                    tree.is_complete = true;
                }
            }
            CommandType::Terminate => {
                if tree.is_complete && cmd.execution_timer.just_finished() {
                    *state = LoopState::Terminated;
                }
            }
        }
    }
}

fn spawn_goal_node(commands: &mut Commands, _parent: Entity, pos: Vec3, label: &str, depth: u32) {
    let color = match depth {
        0 => Color::srgb(1.0, 0.2, 0.2),
        1 => Color::srgb(1.0, 0.5, 0.2),
        _ => Color::srgb(1.0, 0.8, 0.2),
    };

    commands.spawn((
        Text2d(label.to_string()),
        TextColor(color),
        TextFont::default().with_font_size(14.0),
        Transform::from_translation(pos + Vec3::new(0.0, 25.0 + (depth as f32 * 20.0), 0.0)),
        GoalNode {
            parent: _parent,
            depth,
        },
    ));
}

pub fn auto_termination_cleanup(
    mut commands: Commands,
    query: Query<(Entity, &LoopState), With<LoopAgent>>,
    mut metrics: ResMut<Metrics>,
) {
    let terminated_count = query
        .iter()
        .filter(|(_, s)| **s == LoopState::Terminated)
        .count();
    if terminated_count > 0 {
        metrics.active_loops = query.iter().count() - terminated_count;
        for (entity, state) in query.iter() {
            if *state == LoopState::Terminated {
                commands.entity(entity).despawn();
            }
        }
    }
}
