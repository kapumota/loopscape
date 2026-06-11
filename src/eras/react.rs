use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;

pub fn setup_react_era(
    mut commands: Commands,
    mut metrics: ResMut<Metrics>,
) {
    metrics.active_loops = 0;
    metrics.era_timer = 0.0;

    // Crea 5 loops ReAct conectados a herramientas estaticas
    for i in 0..5 {
        let x = (i as f32 - 2.0) * 120.0;
        let y = 100.0;

        let loop_entity = commands.spawn((
            LoopAgent,
            LoopState::Thinking,
            ThinkTimer(Timer::from_seconds(2.0, TimerMode::Repeating)),
            ActTimer(Timer::from_seconds(1.5, TimerMode::Repeating)),
            ObserveTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
            Transform::from_xyz(x, y, 0.0),
            LoopVisual {
                base_color: Color::srgb(0.2, 0.4, 1.0),
                pulse_speed: 2.0,
                radius: 20.0,
            },
        )).id();

        // Crea herramienta estatica
        let tool_entity = commands.spawn((
            Transform::from_xyz(x, y - 80.0, 0.0),
            LoopVisual {
                base_color: Color::srgb(0.5, 0.5, 0.5),
                pulse_speed: 0.5,
                radius: 12.0,
            },
        )).id();

        commands.entity(loop_entity).insert(WiredTool {
            tool_id: tool_entity,
            cooldown: Timer::from_seconds(3.0, TimerMode::Repeating),
        });

        // Linea de conexion
        commands.spawn((
            ConnectionLine {
                from: loop_entity,
                to: tool_entity,
                line_type: ConnectionType::ToolWire,
                color: Color::srgb(0.3, 0.3, 0.3),
            },
        ));
    }
}

pub fn react_cycle_system(
    mut query: Query<(
        &mut LoopState,
        &mut ThinkTimer,
        &mut ActTimer,
        &mut ObserveTimer,
        &mut LoopVisual,
    )>,
    time: Res<Time>,
) {
    for (mut state, mut think, mut act, mut observe, mut visual) in query.iter_mut() {
        match *state {
            LoopState::Thinking => {
                think.0.tick(time.delta());
                visual.base_color = Color::srgb(0.2, 0.4, 1.0);
                if think.0.just_finished() {
                    *state = LoopState::Acting;
                }
            }
            LoopState::Acting => {
                act.0.tick(time.delta());
                visual.base_color = Color::srgb(0.2, 1.0, 0.3);
                if act.0.just_finished() {
                    *state = LoopState::Observing;
                }
            }
            LoopState::Observing => {
                observe.0.tick(time.delta());
                visual.base_color = Color::srgb(1.0, 0.8, 0.2);
                if observe.0.just_finished() {
                    *state = LoopState::Thinking;
                }
            }
            _ => {}
        }
    }
}

pub fn tool_cooldown_system(
    mut query: Query<&mut WiredTool>,
    time: Res<Time>,
) {
    for mut tool in query.iter_mut() {
        tool.cooldown.tick(time.delta());
    }
}
