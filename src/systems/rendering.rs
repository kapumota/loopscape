use crate::components::*;
use crate::resources::XRayMode;
use bevy::prelude::*;

pub fn loop_rendering(
    mut gizmos: Gizmos,
    loops: Query<(&Transform, &LoopVisual, &LoopState), With<LoopAgent>>,
    time: Res<Time>,
    xray: Res<XRayMode>,
) {
    for (transform, visual, state) in loops.iter() {
        let pulse = (time.elapsed_secs() * visual.pulse_speed).sin() * 0.3 + 1.0;
        let radius = visual.radius * pulse;

        let color = match state {
            LoopState::Thinking => Color::srgb(0.2, 0.4, 1.0),
            LoopState::Acting => Color::srgb(0.2, 1.0, 0.3),
            LoopState::Observing => Color::srgb(1.0, 0.8, 0.2),
            LoopState::Spawning => Color::srgb(0.8, 0.3, 1.0),
            LoopState::Supervising => Color::srgb(1.0, 0.2, 0.8),
            LoopState::Terminated => Color::srgb(0.3, 0.3, 0.3),
        };

        gizmos.circle_2d(transform.translation.truncate(), radius, color);
        if xray.enabled {
            gizmos.circle_2d(
                transform.translation.truncate(),
                radius + 8.0,
                Color::srgb(1.0, 1.0, 1.0).with_alpha(0.35),
            );
        }
    }
}

pub fn connection_line_rendering(
    mut gizmos: Gizmos,
    lines: Query<&ConnectionLine>,
    transforms: Query<&Transform>,
    time: Res<Time>,
) {
    for line in lines.iter() {
        if let (Ok(from_transform), Ok(to_transform)) =
            (transforms.get(line.from), transforms.get(line.to))
        {
            let pulse = (time.elapsed_secs() * 3.0).sin() * 0.3 + 0.7;
            let color = line.color.with_alpha(pulse);
            gizmos.line_2d(
                from_transform.translation.truncate(),
                to_transform.translation.truncate(),
                color,
            );
        }
    }
}

pub fn ralph_monolith_rendering(
    mut gizmos: Gizmos,
    era: Res<State<crate::GameEra>>,
    time: Res<Time>,
) {
    if era.get() == &crate::GameEra::RalphLoop {
        let pulse = (time.elapsed_secs() * 2.0).sin() * 5.0 + 30.0;
        gizmos.circle_2d(
            Vec2::ZERO,
            pulse,
            Color::srgb(0.3, 0.7, 0.9).with_alpha(0.3),
        );
        gizmos.circle_2d(Vec2::ZERO, 20.0, Color::srgb(0.3, 0.7, 0.9));
    }
}
