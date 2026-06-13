use bevy::prelude::*;

mod app;
mod components;
mod core;
mod eras;
mod events;
mod llm_integration;
mod networking;
mod resources;
mod systems;

use components::{ConnectionLine, ConsensusVoter, GoalNode, LoopAgent, LoopState, LoopVisual};
use events::*;
use networking::*;
use resources::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameEra {
    #[default]
    Menu,
    ReAct,
    SelfPrompting,
    RalphLoop,
    ProductizedRalph,
    MultiAgentOrchestration,
}

fn run_core_headless_from_args() -> bool {
    let args = std::env::args().collect::<Vec<_>>();

    if !args
        .iter()
        .any(|arg| arg == "--ticks" || arg.starts_with("--ticks="))
    {
        return false;
    }

    let seed = parse_arg_u64(&args, "--seed").unwrap_or(123);
    let ticks = parse_arg_u32(&args, "--ticks").unwrap_or(100);
    let agents = parse_arg_u32(&args, "--agents").unwrap_or(5);
    let tasks = parse_arg_u32(&args, "--tasks").unwrap_or(12);

    let config = crate::core::scheduler::SimulationConfig::new(seed).with_size(agents, tasks);
    let mut state = crate::core::scheduler::SimulationState::new(config);
    state.run_ticks(ticks);

    println!("Loopscape core determinista");
    println!("Semilla: {}", seed);
    println!("Ticks ejecutados: {}", ticks);
    println!("Agentes iniciales: {}", agents);
    println!("Tareas iniciales: {}", tasks);
    println!("Agentes activos: {}", state.metrics.active_loops);
    println!("Tareas totales: {}", state.metrics.total_tasks);
    println!("Tareas completas: {}", state.metrics.completed_tasks);
    println!("Tareas pendientes: {}", state.metrics.pending_tasks);
    println!("Tareas asignadas: {}", state.metrics.assigned_tasks);
    println!("Rendimiento: {:.3}", state.metrics.throughput);
    println!("Eventos generados: {}", state.events.len());

    true
}

fn parse_arg_u64(args: &[String], name: &str) -> Option<u64> {
    parse_arg_value(args, name).and_then(|value| value.parse::<u64>().ok())
}

fn parse_arg_u32(args: &[String], name: &str) -> Option<u32> {
    parse_arg_value(args, name).and_then(|value| value.parse::<u32>().ok())
}

fn parse_arg_value(args: &[String], name: &str) -> Option<String> {
    let inline_prefix = format!("{name}=");

    for arg in args {
        if let Some(value) = arg.strip_prefix(&inline_prefix) {
            return Some(value.to_string());
        }
    }

    args.windows(2).find_map(|pair| {
        if pair[0] == name {
            Some(pair[1].clone())
        } else {
            None
        }
    })
}

fn main() {
    if run_core_headless_from_args() {
        return;
    }

    let mut app = App::new();

    // Plugins base
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Loopscape - Sandbox visual de loops agenticos".to_string(),
            resolution: (1280.0, 720.0).into(),
            ..default()
        }),
        ..default()
    }));

    // FPS overlay deshabilitado en Fase 1 para compilar con Bevy 0.15 sin features extra.

    // Recursos
    app.init_resource::<GlobalPrompt>()
        .init_resource::<TaskQueue>()
        .init_resource::<Metrics>()
        .init_resource::<XRayMode>()
        .init_resource::<EraConfig>()
        .init_resource::<HTTPClient>()
        .init_resource::<llm_integration::ReActContext>();

    // Eventos
    app.add_event::<EraTransitionEvent>()
        .add_event::<SpawnSubLoopEvent>()
        .add_event::<HeartbeatEvent>()
        .add_event::<ByzantineFaultEvent>()
        .add_event::<llm_integration::LLMRequestEvent>()
        .add_event::<llm_integration::LLMResponseEvent>()
        .add_event::<networking::HTTPRequestEvent>()
        .add_event::<networking::HTTPResponseEvent>();

    // Runtime nativo
    #[cfg(not(target_arch = "wasm32"))]
    app.init_resource::<networking::native::NativeRuntime>();

    // Estados
    app.init_state::<GameEra>();

    // Inicializacion
    app.add_systems(
        Startup,
        (systems::camera::setup_camera, systems::ui::setup_ui),
    );

    // Entrada por era
    app.add_systems(OnEnter(GameEra::ReAct), eras::react::setup_react_era)
        .add_systems(
            OnEnter(GameEra::SelfPrompting),
            eras::self_prompting::setup_self_prompt_era,
        )
        .add_systems(OnEnter(GameEra::RalphLoop), eras::ralph::setup_ralph_era)
        .add_systems(
            OnEnter(GameEra::ProductizedRalph),
            eras::productized::setup_productized_era,
        )
        .add_systems(
            OnEnter(GameEra::MultiAgentOrchestration),
            eras::orchestration::setup_orchestration_era,
        );

    // Limpieza al salir de cada era para evitar acumulacion de entidades visuales.
    app.add_systems(OnExit(GameEra::ReAct), cleanup_era_entities)
        .add_systems(OnExit(GameEra::SelfPrompting), cleanup_era_entities)
        .add_systems(OnExit(GameEra::RalphLoop), cleanup_era_entities)
        .add_systems(OnExit(GameEra::ProductizedRalph), cleanup_era_entities)
        .add_systems(
            OnExit(GameEra::MultiAgentOrchestration),
            cleanup_era_entities,
        );

    // Actualizacion especifica por era
    app.add_systems(
        Update,
        (
            eras::react::react_cycle_system,
            eras::react::tool_cooldown_system,
        )
            .run_if(in_state(GameEra::ReAct)),
    )
    .add_systems(
        Update,
        (
            eras::self_prompting::autonomous_decomposition,
            eras::self_prompting::spawn_sub_loops,
            eras::self_prompting::sub_loop_lifetime,
        )
            .run_if(in_state(GameEra::SelfPrompting)),
    )
    .add_systems(
        Update,
        (
            eras::ralph::shared_dna_propagation,
            eras::ralph::swarm_sync_visuals,
            eras::ralph::mutate_dna_system,
        )
            .run_if(in_state(GameEra::RalphLoop)),
    )
    .add_systems(
        Update,
        (
            eras::productized::command_execution_system,
            eras::productized::auto_termination_cleanup,
        )
            .run_if(in_state(GameEra::ProductizedRalph)),
    )
    .add_systems(
        Update,
        (
            eras::orchestration::heartbeat_system,
            eras::orchestration::consensus_voting,
            eras::orchestration::byzantine_detection,
            eras::orchestration::byzantine_visuals,
        )
            .run_if(in_state(GameEra::MultiAgentOrchestration)),
    );

    // Sistemas LLM (corren en todas las eras donde hay LLMBrain)
    app.add_systems(
        Update,
        (
            llm_integration::detect_llm_needs,
            llm_integration::visualize_react_trace,
        ),
    );

    // Red
    #[cfg(not(target_arch = "wasm32"))]
    app.add_systems(Update, networking::handle_http_requests);

    #[cfg(target_arch = "wasm32")]
    app.add_systems(Update, networking::handle_http_requests_wasm_system);

    // Sistemas globales
    app.add_systems(
        Update,
        (
            systems::camera::camera_controls,
            systems::ui::update_ui,
            update_metrics_system,
            era_transition_input,
            xray_toggle,
            llm_panel_toggle,
        ),
    );

    // Renderizado
    app.add_systems(
        PostUpdate,
        (
            systems::rendering::loop_rendering,
            systems::rendering::connection_line_rendering,
            systems::rendering::ralph_monolith_rendering,
        ),
    );

    app.run();
}

fn era_transition_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_era: Res<State<GameEra>>,
    mut next_era: ResMut<NextState<GameEra>>,
) {
    let target = if keyboard.just_pressed(KeyCode::Digit1) {
        Some(GameEra::ReAct)
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        Some(GameEra::SelfPrompting)
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        Some(GameEra::RalphLoop)
    } else if keyboard.just_pressed(KeyCode::Digit4) {
        Some(GameEra::ProductizedRalph)
    } else if keyboard.just_pressed(KeyCode::Digit5) {
        Some(GameEra::MultiAgentOrchestration)
    } else {
        None
    };

    if let Some(new_era) = target {
        if current_era.get() != &new_era {
            next_era.set(new_era);
        }
    }
}

fn cleanup_era_entities(
    mut commands: Commands,
    visual_entities: Query<Entity, With<LoopVisual>>,
    connection_entities: Query<Entity, With<ConnectionLine>>,
    goal_nodes: Query<Entity, With<GoalNode>>,
) {
    for entity in visual_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in connection_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in goal_nodes.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn update_metrics_system(
    time: Res<Time>,
    loop_states: Query<&LoopState, With<LoopAgent>>,
    voters: Query<&ConsensusVoter>,
    mut metrics: ResMut<Metrics>,
) {
    metrics.era_timer += time.delta_secs();

    metrics.active_loops = loop_states
        .iter()
        .filter(|state| **state != LoopState::Terminated)
        .count();

    metrics.throughput = if metrics.era_timer > 0.0 {
        metrics.active_loops as f32 / metrics.era_timer
    } else {
        0.0
    };

    metrics.consensus_term = voters.iter().map(|voter| voter.term).max().unwrap_or(0);
}

fn xray_toggle(keyboard: Res<ButtonInput<KeyCode>>, mut xray: ResMut<XRayMode>) {
    if keyboard.just_pressed(KeyCode::KeyX) {
        xray.enabled = !xray.enabled;
    }
}

fn llm_panel_toggle(keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::KeyL) {
        // En WASM, llamamos al bridge JS
        #[cfg(target_arch = "wasm32")]
        {
            // Aqui iria la llamada a JS mediante web-sys
        }
    }
}
