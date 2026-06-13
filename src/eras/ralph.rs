use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn setup_ralph_era(
    mut commands: Commands,
    mut global_prompt: ResMut<GlobalPrompt>,
    mut metrics: ResMut<Metrics>,
) {
    metrics.active_loops = 0;
    metrics.era_timer = 0.0;
    global_prompt.text = "Shared Ralph DNA: process all tasks with minimal steps".to_string();
    global_prompt.hash = 123456789;
    global_prompt.last_modified = 0.0;

    // Crea 20 loops Ralph identicos
    for i in 0..20 {
        let angle = (i as f32) * std::f32::consts::TAU / 20.0;
        let radius = 200.0;
        let x = angle.cos() * radius;
        let y = angle.sin() * radius;

        let entity = commands
            .spawn((
                LoopAgent,
                LoopState::Thinking,
                RalphDna {
                    prompt_hash: global_prompt.hash,
                    sync_offset: i as f32 * 0.5,
                },
                ThinkTimer(Timer::from_seconds(1.5, TimerMode::Repeating)),
                ActTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
                ObserveTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
                Transform::from_xyz(x, y, 0.0),
                LoopVisual {
                    base_color: Color::srgb(0.3, 0.7, 0.9),
                    pulse_speed: 2.0,
                    radius: 16.0,
                },
            ))
            .id();

        // Linea al centro del monolito
        commands.spawn((ConnectionLine {
            from: entity,
            to: Entity::from_raw(0), // se actualizará en otro sistema o se ignora
            line_type: ConnectionType::DnaSync,
            color: Color::srgb(0.3, 0.7, 0.9).with_alpha(0.3),
        },));
    }
}

pub fn shared_dna_propagation(
    mut ralphs: Query<(&mut RalphDna, &mut LoopVisual)>,
    global_prompt: Res<GlobalPrompt>,
) {
    if global_prompt.is_changed() {
        for (mut dna, mut visual) in ralphs.iter_mut() {
            dna.prompt_hash = global_prompt.hash;
            visual.pulse_speed = 20.0;
            visual.base_color = Color::srgb(1.0, 1.0, 1.0);
        }
    }
}

pub fn swarm_sync_visuals(
    mut ralphs: Query<(&RalphDna, &mut Transform, &mut LoopVisual)>,
    time: Res<Time>,
) {
    let t = time.elapsed_secs();
    for (dna, mut transform, mut visual) in ralphs.iter_mut() {
        let sync = (t * 2.0 + dna.sync_offset).sin();
        transform.scale = Vec3::splat(1.0 + sync * 0.2);
        if visual.pulse_speed > 2.0 {
            visual.pulse_speed -= 0.5 * time.delta_secs();
        }
        if visual.pulse_speed <= 2.0 && visual.base_color == Color::srgb(1.0, 1.0, 1.0) {
            visual.base_color = Color::srgb(0.3, 0.7, 0.9);
        }
    }
}

pub fn mutate_dna_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut global_prompt: ResMut<GlobalPrompt>,
) {
    if keyboard.just_pressed(KeyCode::KeyM) {
        global_prompt.hash = global_prompt.hash.wrapping_add(1);
        global_prompt.text.push_str(" [mutado]");
    }
}
