use bevy::prelude::*;
use crate::components::*;
use crate::events::*;
use crate::resources::*;

pub fn setup_orchestration_era(
    mut commands: Commands,
    mut metrics: ResMut<Metrics>,
) {
    metrics.active_loops = 0;
    metrics.era_timer = 0.0;

    // Crea 1 supervisor con 5 trabajadores.
    let supervisor_entity = commands.spawn((
        LoopAgent,
        LoopState::Supervising,
        Supervisor {
            workers: vec![],
            heartbeat_interval: Timer::from_seconds(2.0, TimerMode::Repeating),
            last_heartbeats: vec![],
        },
        ConsensusVoter {
            term: 1,
            voted_for: None,
            state: ConsensusState::Leader,
            election_timeout: Timer::from_seconds(5.0, TimerMode::Once),
        },
        Transform::from_xyz(0.0, 100.0, 0.0),
        LoopVisual {
            base_color: Color::srgb(1.0, 0.2, 0.8),
            pulse_speed: 4.0,
            radius: 30.0,
        },
    )).id();

    let mut worker_entities = vec![];
    for i in 0..5 {
        let angle = (i as f32) * std::f32::consts::TAU / 5.0;
        let radius = 150.0;
        let x = angle.cos() * radius;
        let y = angle.sin() * radius - 50.0;

        let worker_entity = commands.spawn((
            LoopAgent,
            LoopState::Thinking,
            Worker {
                supervisor: Some(supervisor_entity),
                task_load: (i as f32) * 0.2,
            },
            ConsensusVoter {
                term: 1,
                voted_for: Some(supervisor_entity),
                state: ConsensusState::Follower,
                election_timeout: Timer::from_seconds(
                    3.0 + (i as f32 * 0.5),
                    TimerMode::Once,
                ),
            },
            ByzantineSuspect {
                trust_score: 1.0,
                inconsistency_count: 0,
            },
            Transform::from_xyz(x, y, 0.0),
            LoopVisual {
                base_color: Color::srgb(0.2, 0.8, 0.4),
                pulse_speed: 2.5,
                radius: 18.0,
            },
            ThinkTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
            ActTimer(Timer::from_seconds(0.8, TimerMode::Repeating)),
            ObserveTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
        )).id();

        worker_entities.push(worker_entity);

        // Crea la linea visual de latido entre supervisor y trabajador.
        commands.spawn((
            ConnectionLine {
                from: supervisor_entity,
                to: worker_entity,
                line_type: ConnectionType::Heartbeat,
                color: Color::srgb(0.0, 1.0, 0.5).with_alpha(0.5),
            },
        ));
    }

    // Actualiza la lista de trabajadores del supervisor.
    commands.entity(supervisor_entity).insert(Supervisor {
        workers: worker_entities.clone(),
        heartbeat_interval: Timer::from_seconds(2.0, TimerMode::Repeating),
        last_heartbeats: worker_entities.iter().map(|e| (*e, 0.0)).collect(),
    });
}

pub fn heartbeat_system(
    mut supervisors: Query<(Entity, &mut Supervisor, &Transform)>,
    mut workers: Query<(&Worker, &mut LoopVisual, &Transform), Without<Supervisor>>,
    time: Res<Time>,
    mut gizmos: Gizmos,
) {
    for (_supervisor_entity, mut supervisor, supervisor_transform) in supervisors.iter_mut() {
        supervisor.heartbeat_interval.tick(time.delta());

        if supervisor.heartbeat_interval.just_finished() {
            let now = time.elapsed_secs();
            for worker_entity in supervisor.workers.iter() {
                if let Ok((_worker, mut visual, worker_transform)) = workers.get_mut(*worker_entity) {
                    gizmos.line_2d(
                        supervisor_transform.translation.truncate(),
                        worker_transform.translation.truncate(),
                        Color::srgb(0.0, 1.0, 0.5),
                    );
                    visual.base_color = Color::srgb(0.0, 1.0, 0.5);

                    // Actualiza la marca temporal del ultimo latido.
                    if let Some(entry) = supervisor.last_heartbeats.iter_mut()
                        .find(|(entity, _)| *entity == *worker_entity) {
                        entry.1 = now;
                    }
                }
            }
        }
    }
}

pub fn consensus_voting(
    mut voters: Query<(Entity, &mut ConsensusVoter, &mut LoopVisual)>,
    time: Res<Time>,
) {
    for (entity, mut voter, mut visual) in voters.iter_mut() {
        voter.election_timeout.tick(time.delta());

        match voter.state {
            ConsensusState::Follower => {
                if voter.election_timeout.finished() {
                    voter.state = ConsensusState::Candidate;
                    voter.term += 1;
                    voter.voted_for = Some(entity);
                    visual.base_color = Color::srgb(1.0, 1.0, 0.0);
                    voter.election_timeout = Timer::from_seconds(2.0, TimerMode::Once);
                } else {
                    visual.base_color = Color::srgb(0.5, 0.5, 0.5);
                }
            }
            ConsensusState::Candidate => {
                // Simulacion: si pasan 2 segundos sin lider, el candidato se vuelve lider.
                if voter.election_timeout.finished() {
                    voter.state = ConsensusState::Leader;
                    visual.base_color = Color::srgb(1.0, 0.2, 0.8);
                }
            }
            ConsensusState::Leader => {
                visual.base_color = Color::srgb(1.0, 0.2, 0.8);
            }
        }
    }
}

pub fn byzantine_detection(
    mut suspects: Query<&mut ByzantineSuspect>,
    keyboard: Res<ButtonInput<KeyCode>>,
    _events: EventWriter<ByzantineFaultEvent>,
) {
    if keyboard.just_pressed(KeyCode::KeyB) {
        // Inyecta un fallo en el primer trabajador disponible.
        if let Some(mut suspect) = suspects.iter_mut().next() {
            suspect.trust_score -= 0.3;
            suspect.inconsistency_count += 1;
        }
    }
}

pub fn byzantine_visuals(
    mut suspects: Query<(&ByzantineSuspect, &mut LoopVisual)>,
) {
    for (suspect, mut visual) in suspects.iter_mut() {
        if suspect.trust_score < 0.5 {
            visual.base_color = Color::srgb(1.0, 0.0, 0.0);
            visual.pulse_speed = 10.0;
        }
    }
}
