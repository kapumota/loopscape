use crate::resources::*;
use crate::GameEra;
use bevy::prelude::*;

pub fn setup_ui(mut commands: Commands) {
    commands.spawn(
        Text::new("LOOPSCAPE - Presiona 1-5 para cambiar de era, M para mutar ADN, B para fallo bizantino, X para Rayos X"),
    )
    .insert(TextFont::default().with_font_size(16.0))
    .insert(TextColor(Color::srgb(0.9, 0.9, 0.9)))
    .insert(Node {
        position_type: PositionType::Absolute,
        top: Val::Px(10.0),
        left: Val::Px(10.0),
        ..default()
    });

    commands
        .spawn(Text::new("Era: Menu"))
        .insert(TextFont::default().with_font_size(20.0))
        .insert(TextColor(Color::srgb(1.0, 0.8, 0.2)))
        .insert(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(40.0),
            left: Val::Px(10.0),
            ..default()
        })
        .insert(EraLabel);

    commands
        .spawn(Text::new("Loops: 0 | Rendimiento: 0.0"))
        .insert(TextFont::default().with_font_size(14.0))
        .insert(TextColor(Color::srgb(0.7, 0.7, 0.7)))
        .insert(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(70.0),
            left: Val::Px(10.0),
            ..default()
        })
        .insert(MetricsLabel);
}

#[derive(Component)]
pub struct EraLabel;

#[derive(Component)]
pub struct MetricsLabel;

#[allow(clippy::type_complexity)]
pub fn update_ui(
    era: Res<State<GameEra>>,
    metrics: Res<Metrics>,
    xray: Res<XRayMode>,
    era_config: Res<EraConfig>,
    mut labels: ParamSet<(
        Query<&mut Text, With<EraLabel>>,
        Query<&mut Text, With<MetricsLabel>>,
    )>,
) {
    let era_name = match era.get() {
        GameEra::Menu => "Menu".to_string(),
        GameEra::ReAct => era_config.era_names[1].clone(),
        GameEra::SelfPrompting => era_config.era_names[2].clone(),
        GameEra::RalphLoop => era_config.era_names[3].clone(),
        GameEra::ProductizedRalph => era_config.era_names[4].clone(),
        GameEra::MultiAgentOrchestration => era_config.era_names[5].clone(),
    };

    for mut text in labels.p0().iter_mut() {
        text.0 = format!("Era: {}", era_name);
    }

    for mut text in labels.p1().iter_mut() {
        text.0 = format!(
            "Loops: {} | Rendimiento: {:.1} | Termino: {} | Rayos X: {}",
            metrics.active_loops,
            metrics.throughput,
            metrics.consensus_term,
            if xray.enabled { "ON" } else { "OFF" }
        );
    }
}
