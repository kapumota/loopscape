use bevy::prelude::*;

mod app;
mod components;
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

fn simulation_config_from_args(
    args: &[String],
    seed: u64,
    agents: u32,
    tasks: u32,
    dsl_failures: Option<loopscape::dsl::DslFailureScenario>,
) -> loopscape::core::scheduler::SimulationConfig {
    let worker_timeout_ticks = parse_arg_u64(args, "--supervisor-timeout").unwrap_or(3);
    let worker_restart_limit = parse_arg_u32(args, "--worker-restart-limit").unwrap_or(1);

    let mut recoverable_failures = dsl_failures
        .as_ref()
        .map(|scenario| scenario.recoverable_failures.clone())
        .unwrap_or_else(loopscape::core::failure::RecoverableFailurePlan::none);
    let cli_failures = recoverable_failure_plan_from_args(args);
    recoverable_failures.failures.extend(cli_failures.failures);

    loopscape::core::scheduler::SimulationConfig::new(seed)
        .with_size(agents, tasks)
        .with_supervisor(worker_timeout_ticks, worker_restart_limit)
        .with_recoverable_failures(recoverable_failures)
}

fn recoverable_failure_plan_from_args(
    args: &[String],
) -> loopscape::core::failure::RecoverableFailurePlan {
    let values = parse_arg_values(args, "--worker-failure");
    let Some(first) = values.first() else {
        return loopscape::core::failure::RecoverableFailurePlan::none();
    };

    let (worker_id, start_tick, duration_ticks) = parse_worker_failure_tuple(first);
    let mut plan = loopscape::core::failure::RecoverableFailurePlan::worker_hangs(
        worker_id,
        start_tick,
        duration_ticks,
    )
    .unwrap_or_else(|error| exit_with_argument_error("--worker-failure", &error));

    for value in values.iter().skip(1) {
        let failure = parse_worker_failure_spec(value);
        plan = plan.with_failure(failure);
    }

    plan
}

fn parse_worker_failure_spec(value: &str) -> loopscape::core::failure::WorkerFailureSpec {
    let (worker_id, start_tick, duration_ticks) = parse_worker_failure_tuple(value);
    loopscape::core::failure::WorkerFailureSpec::new(worker_id, start_tick, duration_ticks)
        .unwrap_or_else(|error| exit_with_argument_error("--worker-failure", &error))
}

fn parse_worker_failure_tuple(value: &str) -> (u32, u64, u64) {
    let parts = value.split(':').collect::<Vec<_>>();
    if parts.len() != 3 {
        exit_with_argument_error(
            "--worker-failure",
            "usa el formato worker:tick_inicio:duracion",
        );
    }

    let worker_id = parts[0]
        .parse::<u32>()
        .unwrap_or_else(|_| exit_with_argument_error("--worker-failure", "worker invalido"));
    let start_tick = parts[1]
        .parse::<u64>()
        .unwrap_or_else(|_| exit_with_argument_error("--worker-failure", "tick inicial invalido"));
    let duration_ticks = parts[2]
        .parse::<u64>()
        .unwrap_or_else(|_| exit_with_argument_error("--worker-failure", "duracion invalida"));

    (worker_id, start_tick, duration_ticks)
}

fn byzantine_failure_plan_from_args(
    args: &[String],
    dsl_failures: Option<&loopscape::dsl::DslFailureScenario>,
) -> loopscape::core::byzantine::ByzantineFailurePlan {
    let mut plan = dsl_failures
        .map(|scenario| scenario.byzantine_failures.clone())
        .unwrap_or_else(loopscape::core::byzantine::ByzantineFailurePlan::none);
    let values = parse_arg_values(args, "--byzantine-failure");
    let Some(first) = values.first() else {
        return plan;
    };

    if plan.failures.is_empty() {
        let (worker_id, false_value) = parse_byzantine_failure_tuple(first);
        plan =
            loopscape::core::byzantine::ByzantineFailurePlan::worker_lies(worker_id, false_value)
                .unwrap_or_else(|error| exit_with_argument_error("--byzantine-failure", &error));
    } else {
        let failure = parse_byzantine_failure_spec(first);
        plan = plan.with_failure(failure);
    }

    for value in values.iter().skip(1) {
        let failure = parse_byzantine_failure_spec(value);
        plan = plan.with_failure(failure);
    }

    plan
}

fn parse_byzantine_failure_spec(value: &str) -> loopscape::core::byzantine::ByzantineFailureSpec {
    let (worker_id, false_value) = parse_byzantine_failure_tuple(value);
    loopscape::core::byzantine::ByzantineFailureSpec::new(worker_id, false_value)
        .unwrap_or_else(|error| exit_with_argument_error("--byzantine-failure", &error))
}

fn parse_byzantine_failure_tuple(value: &str) -> (u32, String) {
    let Some((worker_id, false_value)) = value.split_once(':') else {
        exit_with_argument_error("--byzantine-failure", "usa el formato worker:valor_falso");
    };

    let worker_id = worker_id
        .parse::<u32>()
        .unwrap_or_else(|_| exit_with_argument_error("--byzantine-failure", "worker invalido"));

    (worker_id, false_value.to_string())
}

fn print_supervisor_summary(state: &loopscape::core::scheduler::SimulationState) {
    let rows = crate::app::supervisor::supervisor_rows(&state.supervisor);
    let labels = crate::app::supervisor::supervisor_event_labels(&state.supervisor.events);
    let _primer_worker = state.supervisor.worker(0);
    let ultimo_tick = state
        .supervisor
        .events
        .last()
        .map(loopscape::core::supervisor::SupervisorEvent::tick)
        .unwrap_or(state.tick);

    println!("Supervisor workers: {}", rows.len());
    println!("Supervisor ultimo tick: {ultimo_tick}");

    for row in rows.iter().take(3) {
        println!(
            "Worker {} estado={} heartbeat={} reinicios={}",
            row.worker_id, row.estado, row.ultimo_heartbeat, row.reinicios
        );
    }

    for label in labels.iter().rev().take(3).rev() {
        println!("Evento supervisor: {label}");
    }
}

fn print_byzantine_vote_summary(
    args: &[String],
    agents: u32,
    dsl_failures: Option<&loopscape::dsl::DslFailureScenario>,
) -> bool {
    let cli_vote_value = parse_arg_value(args, "--byzantine-vote");
    let dsl_vote_value = dsl_failures.and_then(|scenario| scenario.byzantine_vote_value.clone());
    let vote_value = cli_vote_value.or(dsl_vote_value);
    let plan = byzantine_failure_plan_from_args(args, dsl_failures);

    if vote_value.is_none() && plan.failures.is_empty() {
        return false;
    }

    if agents == 0 {
        exit_with_argument_error(
            "--agents",
            "la votacion bizantina requiere al menos un worker",
        );
    }

    let honest_value = vote_value.unwrap_or_else(|| "valor_correcto".to_string());
    let worker_ids = (0..agents).collect::<Vec<_>>();
    let responses =
        loopscape::core::byzantine::build_worker_responses(&worker_ids, honest_value, &plan);
    let voter = voter_from_args(args, worker_ids.len());
    let outcome = voter.decide(&responses);
    let accepted = outcome.decision.is_accepted();

    println!("Votacion bizantina simplificada");
    println!("Workers votantes: {}", worker_ids.len());
    println!("Respuestas falsas: {}", outcome.false_responses);
    println!("Resultado aceptado: {accepted}");

    match outcome.decision {
        loopscape::core::byzantine::VoteDecision::Accepted { value, votes } => {
            println!("Decision: aceptada valor={value} votos={votes}");
        }
        loopscape::core::byzantine::VoteDecision::Rejected { reason } => {
            println!("Decision: rechazada razon={reason}");
        }
    }

    for count in outcome.counts {
        println!("Conteo voto: {} -> {}", count.value, count.votes);
    }

    true
}

fn voter_from_args(
    args: &[String],
    total_workers: usize,
) -> loopscape::core::byzantine::SimpleMajorityVoter {
    match (
        parse_arg_usize(args, "--vote-minimum-responses"),
        parse_arg_usize(args, "--vote-required-votes"),
    ) {
        (Some(minimum_responses), Some(required_votes)) => {
            let config =
                loopscape::core::byzantine::VotingConfig::new(minimum_responses, required_votes)
                    .unwrap_or_else(|error| {
                        exit_with_argument_error("--vote-required-votes", &error)
                    });
            loopscape::core::byzantine::SimpleMajorityVoter::new(config)
        }
        (None, None) => loopscape::core::byzantine::SimpleMajorityVoter::majority(total_workers)
            .unwrap_or_else(|error| exit_with_argument_error("--agents", &error)),
        _ => exit_with_argument_error(
            "--vote-minimum-responses",
            "define tambien --vote-required-votes",
        ),
    }
}

fn run_byzantine_vote_from_args() -> bool {
    let args = std::env::args().collect::<Vec<_>>();
    if parse_arg_value(&args, "--script").is_some() {
        return false;
    }

    let has_vote = parse_arg_value(&args, "--byzantine-vote").is_some();
    let has_fault = !parse_arg_values(&args, "--byzantine-failure").is_empty();
    if !has_vote && !has_fault {
        return false;
    }

    let agents = parse_arg_u32(&args, "--agents").unwrap_or(5);
    print_byzantine_vote_summary(&args, agents, None);
    true
}

fn parse_arg_usize(args: &[String], name: &str) -> Option<usize> {
    parse_arg_value(args, name).and_then(|value| value.parse::<usize>().ok())
}

fn parse_arg_values(args: &[String], name: &str) -> Vec<String> {
    let inline_prefix = format!("{name}=");
    let mut values = Vec::new();

    for (index, arg) in args.iter().enumerate() {
        if let Some(value) = arg.strip_prefix(&inline_prefix) {
            values.extend(split_compact_values(value));
        } else if arg == name {
            if let Some(value) = args.get(index + 1) {
                values.extend(split_compact_values(value));
            }
        }
    }

    values
}

fn split_compact_values(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(str::to_string)
        .collect()
}

fn exit_with_argument_error(argument: &str, message: &str) -> ! {
    eprintln!("Argumento invalido {argument}: {message}");
    std::process::exit(2);
}

fn run_metrics_comparison_from_args() -> bool {
    let args = std::env::args().collect::<Vec<_>>();
    let Some((baseline_path, candidate_path)) = parse_arg_pair(&args, "--compare-metrics") else {
        return false;
    };

    if baseline_path.trim().is_empty() || candidate_path.trim().is_empty() {
        eprintln!("El argumento --compare-metrics requiere dos rutas CSV no vacias");
        std::process::exit(2);
    }

    let comparison =
        match loopscape::core::compare::compare_metrics_files(&baseline_path, &candidate_path) {
            Ok(comparison) => comparison,
            Err(error) => {
                eprintln!("No se pudo comparar metricas CSV: {error}");
                std::process::exit(2);
            }
        };

    if let Some(output_path) = parse_arg_value(&args, "--compare-output") {
        if output_path.trim().is_empty() {
            eprintln!("El argumento --compare-output no puede estar vacio");
            std::process::exit(2);
        }

        if let Err(error) =
            loopscape::core::compare::write_metrics_comparison_csv(&comparison, &output_path)
        {
            eprintln!("No se pudo escribir comparacion CSV {output_path}: {error}");
            std::process::exit(2);
        }

        println!("Comparacion CSV exportada: {output_path}");
    }

    println!("Loopscape comparacion de corridas");
    println!("Base: {baseline_path}");
    println!("Candidata: {candidate_path}");
    println!("Delta ticks: {}", comparison.delta.ticks);
    println!(
        "Delta tareas completadas: {}",
        comparison.delta.completed_tasks
    );
    println!("Delta loops activos: {}", comparison.delta.active_loops);
    println!("Delta tokens usados: {}", comparison.delta.tokens_used);
    println!(
        "Delta fallos detectados: {}",
        comparison.delta.failures_detected
    );
    println!(
        "Delta fallos recuperados: {}",
        comparison.delta.failures_recovered
    );
    println!(
        "Delta latencia promedio: {:.3}",
        comparison.delta.average_latency
    );
    println!("Comparacion completada correctamente");
    true
}

fn run_replay_from_args() -> bool {
    let args = std::env::args().collect::<Vec<_>>();
    let Some(trace_path) = parse_arg_value(&args, "--replay") else {
        return false;
    };

    if trace_path.trim().is_empty() {
        eprintln!("El argumento --replay no puede estar vacio");
        std::process::exit(2);
    }

    let summary = match loopscape::core::replay::replay_trace_jsonl(&trace_path) {
        Ok(summary) => summary,
        Err(error) => {
            eprintln!("No se pudo reproducir la traza JSONL {trace_path}: {error}");
            std::process::exit(2);
        }
    };

    println!("Loopscape replay determinista");
    println!("Traza: {trace_path}");
    println!("Formato: {}", summary.format);
    println!("Eventos reproducidos: {}", summary.event_count);
    println!("Tick inicial: {}", summary.first_tick);
    println!("Tick final: {}", summary.last_tick);
    println!("Primer evento: {}", summary.first_event_kind);
    println!("Ultimo evento: {}", summary.last_event_kind);
    println!("Replay completado correctamente");
    true
}

fn run_import_graph_from_args() -> bool {
    let args = std::env::args().collect::<Vec<_>>();
    let Some(graph_path) = parse_arg_value(&args, "--graph") else {
        return false;
    };

    if graph_path.trim().is_empty() {
        eprintln!("El argumento --graph no puede estar vacio");
        std::process::exit(2);
    }

    let source = match std::fs::read_to_string(&graph_path) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("No se pudo leer el grafo JSON {graph_path}: {error}");
            std::process::exit(2);
        }
    };

    let graph = match loopscape::dsl::graph_from_json(&source) {
        Ok(graph) => graph,
        Err(error) => {
            eprintln!("El grafo JSON no es valido: {error}");
            std::process::exit(2);
        }
    };

    let seed = parse_arg_u64(&args, "--seed").unwrap_or(123);
    let ticks = parse_arg_u32(&args, "--ticks").unwrap_or(50);
    let config = simulation_config_from_args(&args, seed, 5, 12, None);
    let mut state = loopscape::core::scheduler::SimulationState::new(config);
    state.run_ticks(ticks);
    export_metrics_from_args(&args, &state);
    record_core_events_from_args(&args, &state.events);
    print_supervisor_summary(&state);
    print_byzantine_vote_summary(&args, 5, None);

    println!("Loopscape grafo de orquestacion");
    println!("Grafo: {graph_path}");
    println!("Formato: {}", graph.metadata.version);
    println!(
        "Origen: {}",
        graph
            .metadata
            .source
            .as_deref()
            .unwrap_or("no especificado")
    );
    println!("Semilla: {seed}");
    println!("Ticks ejecutados: {ticks}");
    println!("Comandos importados: {}", graph.metadata.command_count);
    println!("Nodos importados: {}", graph.nodes.len());
    println!("Aristas importadas: {}", graph.edges.len());
    println!("Eventos del nucleo generados: {}", state.events.len());
    println!("Tareas completas: {}", state.metrics.completed_tasks);
    println!("Tareas pendientes: {}", state.metrics.pending_tasks);

    if let Some(first_node) = graph.nodes.first() {
        println!("Nodo inicial: {}", describe_graph_node(first_node));
    }

    if let Some(last_node) = graph.nodes.last() {
        println!("Nodo final: {}", describe_graph_node(last_node));
    }

    println!("Grafo DSL importado correctamente");
    true
}

fn export_metrics_from_args(args: &[String], state: &loopscape::core::scheduler::SimulationState) {
    let Some(metrics_path) = parse_arg_value(args, "--metrics") else {
        return;
    };

    if metrics_path.trim().is_empty() {
        eprintln!("El argumento --metrics no puede estar vacio");
        std::process::exit(2);
    }

    if let Err(error) = loopscape::core::metrics::write_metrics_csv(state, &metrics_path) {
        eprintln!("No se pudo exportar metricas CSV {metrics_path}: {error}");
        std::process::exit(2);
    }

    println!("Metricas CSV exportadas: {metrics_path}");
}

fn record_core_events_from_args(args: &[String], events: &[loopscape::core::event::CoreEvent]) {
    let Some(record_path) = parse_arg_value(args, "--record") else {
        return;
    };

    if record_path.trim().is_empty() {
        eprintln!("El argumento --record no puede estar vacio");
        std::process::exit(2);
    }

    if let Err(error) = loopscape::core::trace::write_events_jsonl(events, &record_path) {
        eprintln!("No se pudo registrar eventos JSONL {record_path}: {error}");
        std::process::exit(2);
    }

    println!("Eventos JSONL registrados: {record_path}");
}

fn describe_graph_node(node: &loopscape::dsl::GraphNode) -> String {
    format!("{} tipo={} etiqueta={}", node.id, node.kind, node.label)
}

fn run_export_graph_from_args() -> bool {
    let args = std::env::args().collect::<Vec<_>>();
    let Some(script_path) = parse_arg_value(&args, "--script") else {
        return false;
    };
    let Some(output_path) = parse_arg_value(&args, "--export-graph") else {
        return false;
    };

    if script_path.trim().is_empty() {
        eprintln!("El argumento --script no puede estar vacio");
        std::process::exit(2);
    }

    if output_path.trim().is_empty() {
        eprintln!("El argumento --export-graph no puede estar vacio");
        std::process::exit(2);
    }

    let source = match std::fs::read_to_string(&script_path) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("No se pudo leer el script DSL {script_path}: {error}");
            std::process::exit(2);
        }
    };

    let graph = match loopscape::dsl::graph_from_source(&source, Some(script_path.clone())) {
        Ok(graph) => graph,
        Err(error) => {
            eprintln!("No se pudo construir el grafo DSL: {error}");
            std::process::exit(2);
        }
    };

    let json = match loopscape::dsl::graph_to_json(&graph) {
        Ok(json) => json,
        Err(error) => {
            eprintln!("No se pudo serializar el grafo DSL: {error}");
            std::process::exit(2);
        }
    };

    let output = std::path::Path::new(&output_path);
    if let Some(parent) = output.parent() {
        if !parent.as_os_str().is_empty() {
            if let Err(error) = std::fs::create_dir_all(parent) {
                eprintln!("No se pudo crear el directorio de salida {parent:?}: {error}");
                std::process::exit(2);
            }
        }
    }

    if let Err(error) = std::fs::write(output, json) {
        eprintln!("No se pudo escribir el grafo DSL {output_path}: {error}");
        std::process::exit(2);
    }

    println!("Loopscape DSL de orquestacion");
    println!("Script: {script_path}");
    println!("Grafo exportado: {output_path}");
    println!("Nodos: {}", graph.nodes.len());
    println!("Aristas: {}", graph.edges.len());
    println!("Grafo DSL exportado correctamente");
    true
}

fn run_dsl_script_from_args() -> bool {
    let args = std::env::args().collect::<Vec<_>>();
    if wants_visual_dsl_script(&args) {
        return false;
    }
    let Some(script_path) = parse_arg_value(&args, "--script") else {
        return false;
    };

    if script_path.trim().is_empty() {
        eprintln!("El argumento --script no puede estar vacio");
        std::process::exit(2);
    }

    let seed = parse_arg_u64(&args, "--seed").unwrap_or(123);
    let ticks = parse_arg_u32(&args, "--ticks").unwrap_or(50);
    let source = match std::fs::read_to_string(&script_path) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("No se pudo leer el script DSL {script_path}: {error}");
            std::process::exit(2);
        }
    };

    let program = match loopscape::dsl::validate_source(&source) {
        Ok(program) => program,
        Err(error) => {
            eprintln!("El script DSL no es valido: {error}");
            std::process::exit(2);
        }
    };
    let dsl_events = match loopscape::dsl::interpret_program(&program) {
        Ok(events) => events,
        Err(error) => {
            eprintln!("El script DSL no es valido: {error}");
            std::process::exit(2);
        }
    };
    let dsl_failures = match loopscape::dsl::failure_scenario_from_program(&program) {
        Ok(scenario) => scenario,
        Err(error) => {
            eprintln!("No se pudo extraer fallos del DSL: {error}");
            std::process::exit(2);
        }
    };

    let config = simulation_config_from_args(&args, seed, 5, 12, Some(dsl_failures.clone()));
    let mut state = loopscape::core::scheduler::SimulationState::new(config);
    state.run_ticks(ticks);
    export_metrics_from_args(&args, &state);
    record_core_events_from_args(&args, &state.events);
    print_supervisor_summary(&state);
    print_byzantine_vote_summary(&args, 5, Some(&dsl_failures));

    println!("Loopscape DSL de orquestacion");
    println!("Script: {script_path}");
    println!("Semilla: {seed}");
    println!("Ticks ejecutados: {ticks}");
    println!("Eventos DSL generados: {}", dsl_events.len());
    println!("Eventos del nucleo generados: {}", state.events.len());
    println!("Tareas completas: {}", state.metrics.completed_tasks);
    println!("Tareas pendientes: {}", state.metrics.pending_tasks);

    for (index, event) in dsl_events.iter().enumerate() {
        println!("Evento DSL {}: {}", index + 1, describe_dsl_event(event));
    }

    println!("Script DSL completado correctamente");
    true
}

fn describe_dsl_event(event: &loopscape::core::event::CoreEvent) -> String {
    match event {
        loopscape::core::event::CoreEvent::GoalCreated { goal, .. } => {
            format!("GoalCreated objetivo={goal}")
        }
        loopscape::core::event::CoreEvent::PlanStepCreated { index, step, .. } => {
            format!("PlanStepCreated indice={index} paso={step}")
        }
        loopscape::core::event::CoreEvent::DelegationRequested { target, worker, .. } => {
            format!("DelegationRequested destino={target} worker={worker}")
        }
        loopscape::core::event::CoreEvent::VerificationRequested { checklist, .. } => {
            format!("VerificationRequested checklist={checklist}")
        }
        loopscape::core::event::CoreEvent::TerminationPolicySet { policy, .. } => {
            format!("TerminationPolicySet politica={policy}")
        }
        _ => "CoreEvent no generado por DSL".to_string(),
    }
}

fn load_visual_dsl_program_from_args() -> Option<LoadedDslProgram> {
    let args = std::env::args().collect::<Vec<_>>();
    if !wants_visual_dsl_script(&args) {
        return None;
    }

    let script_path = parse_arg_value(&args, "--script")?;
    let source = match std::fs::read_to_string(&script_path) {
        Ok(source) => source,
        Err(error) => {
            return Some(LoadedDslProgram::with_error(
                script_path,
                format!("no se pudo leer el script DSL: {error}"),
            ));
        }
    };

    let program = match loopscape::dsl::validate_source(&source) {
        Ok(program) => program,
        Err(error) => return Some(LoadedDslProgram::with_error(script_path, error.to_string())),
    };

    Some(LoadedDslProgram::from_script_lines(
        script_path,
        program.to_script_lines(),
    ))
}

fn wants_visual_dsl_script(args: &[String]) -> bool {
    parse_arg_value(args, "--script").is_some()
        && (has_flag(args, "--visual") || has_flag(args, "--viewer"))
}

fn requires_visual_runtime(args: &[String]) -> bool {
    if has_flag(args, "--smoke") || has_flag(args, "--headless") {
        return false;
    }

    if parse_arg_value(args, "--script").is_some() && !wants_visual_dsl_script(args) {
        return false;
    }

    true
}

fn has_graphical_environment() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        true
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        std::env::var_os("DISPLAY").is_some()
            || std::env::var_os("WAYLAND_DISPLAY").is_some()
            || std::env::var_os("WAYLAND_SOCKET").is_some()
    }
}

fn exit_if_visual_environment_missing() -> bool {
    let args = std::env::args().collect::<Vec<_>>();

    if !requires_visual_runtime(&args) || has_graphical_environment() {
        return false;
    }

    eprintln!("No se detecto entorno grafico para iniciar Loopscape en modo visual.");
    eprintln!(
        "Ejecuta sin --visual para modo remoto o abre una sesion con DISPLAY, WAYLAND_DISPLAY o WAYLAND_SOCKET."
    );
    std::process::exit(2);
}

fn run_core_headless_from_args() -> bool {
    let args = std::env::args().collect::<Vec<_>>();
    if wants_visual_dsl_script(&args) {
        return false;
    }
    let smoke_requested = has_flag(&args, "--smoke") || has_flag(&args, "--headless");
    let ticks_requested = args
        .iter()
        .any(|arg| arg == "--ticks" || arg.starts_with("--ticks="));

    if !smoke_requested && !ticks_requested {
        return false;
    }

    let seed = parse_arg_u64(&args, "--seed").unwrap_or(123);
    let ticks = parse_arg_u32(&args, "--ticks").unwrap_or(10);
    let agents = parse_arg_u32(&args, "--agents").unwrap_or(5);
    let tasks = parse_arg_u32(&args, "--tasks").unwrap_or(12);
    let mode = if smoke_requested {
        "prueba de humo nativa"
    } else {
        "ejecucion nativa sin ventana"
    };

    let config = simulation_config_from_args(&args, seed, agents, tasks, None);
    let mut state = loopscape::core::scheduler::SimulationState::new(config);
    state.run_ticks(ticks);
    export_metrics_from_args(&args, &state);
    record_core_events_from_args(&args, &state.events);
    print_supervisor_summary(&state);
    print_byzantine_vote_summary(&args, agents, None);

    println!("Loopscape core determinista");
    println!("Modo: {}", mode);
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
    println!("Prueba de humo completada correctamente");

    true
}

fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|arg| arg == name)
}

fn parse_arg_u64(args: &[String], name: &str) -> Option<u64> {
    parse_arg_value(args, name).and_then(|value| value.parse::<u64>().ok())
}

fn parse_arg_u32(args: &[String], name: &str) -> Option<u32> {
    parse_arg_value(args, name).and_then(|value| value.parse::<u32>().ok())
}

fn parse_arg_pair(args: &[String], flag: &str) -> Option<(String, String)> {
    let index = args.iter().position(|arg| arg == flag)?;
    let first = args.get(index + 1)?.clone();
    let second = args.get(index + 2)?.clone();
    Some((first, second))
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
    if run_metrics_comparison_from_args() {
        return;
    }

    if run_replay_from_args() {
        return;
    }

    if run_import_graph_from_args() {
        return;
    }

    if run_export_graph_from_args() {
        return;
    }

    if run_byzantine_vote_from_args() {
        return;
    }

    if run_dsl_script_from_args() {
        return;
    }

    if run_core_headless_from_args() {
        return;
    }

    if exit_if_visual_environment_missing() {
        return;
    }

    let loaded_dsl_program = load_visual_dsl_program_from_args();

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
        .init_resource::<LoadedDslProgram>()
        .init_resource::<HTTPClient>()
        .init_resource::<llm_integration::ReActContext>();

    if let Some(dsl_program) = loaded_dsl_program {
        app.insert_resource(dsl_program);
    }

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

    let tick = crate::app::bevy_adapter::estimate_tick_from_seconds(metrics.era_timer);
    let core_metrics =
        crate::app::bevy_adapter::core_metrics_from_visual_states(tick, loop_states.iter());

    metrics.active_loops = core_metrics.active_loops;
    metrics.throughput = core_metrics.throughput;
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
