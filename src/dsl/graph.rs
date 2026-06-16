use serde::{Deserialize, Serialize};

use super::ast::{OrchestrationCommand, OrchestrationProgram};
use super::command::CommandKind;
use super::error::DslError;
use super::validator::{validate_program, validate_source};

const GRAPH_VERSION: &str = "loopscape.orchestration.graph.v1";

/// Grafo logico exportable de un programa DSL de orquestacion.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OrchestrationGraph {
    #[serde(rename = "metadatos")]
    pub metadata: GraphMetadata,
    #[serde(rename = "nodos")]
    pub nodes: Vec<GraphNode>,
    #[serde(rename = "aristas")]
    pub edges: Vec<GraphEdge>,
}

/// Metadatos reproducibles del grafo exportado.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GraphMetadata {
    pub version: String,
    pub source: Option<String>,
    pub command_count: usize,
    pub node_count: usize,
    pub edge_count: usize,
}

/// Nodo logico derivado de un comando o paso del DSL.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub kind: String,
    pub label: String,
    pub command: String,
    pub source_line: Option<usize>,
    pub order: usize,
}

/// Arista logica entre nodos del flujo de orquestacion.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GraphEdge {
    pub id: String,
    pub from: String,
    pub to: String,
    pub kind: String,
    pub label: String,
}

/// Construye un grafo logico desde un programa DSL ya parseado.
pub fn graph_from_program(
    program: &OrchestrationProgram,
    source: Option<String>,
) -> Result<OrchestrationGraph, DslError> {
    validate_program(program)?;

    let nodes = program
        .commands
        .iter()
        .enumerate()
        .flat_map(|(command_index, command)| nodes_from_command(command_index, command))
        .collect::<Vec<_>>();

    let mut graph = OrchestrationGraph {
        metadata: GraphMetadata {
            version: GRAPH_VERSION.to_string(),
            source,
            command_count: program.commands.len(),
            node_count: nodes.len(),
            edge_count: 0,
        },
        nodes,
        edges: Vec::new(),
    };
    rebuild_flow_edges(&mut graph);
    Ok(graph)
}

/// Parsea, valida y exporta un script DSL como grafo logico.
pub fn graph_from_source(
    input: &str,
    source: Option<String>,
) -> Result<OrchestrationGraph, DslError> {
    let program = validate_source(input)?;
    graph_from_program(&program, source)
}

/// Serializa un grafo de orquestacion como JSON legible.
pub fn graph_to_json(graph: &OrchestrationGraph) -> Result<String, DslError> {
    serde_json::to_string_pretty(graph).map_err(|error| {
        DslError::invalid_program(format!("no se pudo serializar el grafo JSON: {error}"))
    })
}

/// Parsea, valida y serializa un script DSL como grafo JSON.
pub fn graph_json_from_source(input: &str, source: Option<String>) -> Result<String, DslError> {
    let graph = graph_from_source(input, source)?;
    graph_to_json(&graph)
}

fn rebuild_flow_edges(graph: &mut OrchestrationGraph) {
    graph.edges.clear();

    for index in 1..graph.nodes.len() {
        let edge = flow_edge(
            index - 1,
            &graph.nodes[index - 1].id,
            &graph.nodes[index].id,
        );
        graph.edges.push(edge);
    }

    graph.metadata.node_count = graph.nodes.len();
    graph.metadata.edge_count = graph.edges.len();
}

fn nodes_from_command(command_index: usize, command: &OrchestrationCommand) -> Vec<GraphNode> {
    match command.kind {
        CommandKind::Goal => vec![node_from_command(
            command_index,
            0,
            "goal",
            argument_label(command, 0, "objetivo"),
            command,
        )],
        CommandKind::Plan => command
            .arguments
            .iter()
            .enumerate()
            .map(|(step_index, step)| {
                node_from_command(
                    command_index,
                    step_index,
                    "plan_step",
                    step.clone(),
                    command,
                )
            })
            .collect(),
        CommandKind::Delegate => vec![node_from_command(
            command_index,
            0,
            "delegate",
            format!(
                "{} -> {}",
                argument_label(command, 0, "destino"),
                argument_label(command, 1, "worker")
            ),
            command,
        )],
        CommandKind::Verify => vec![node_from_command(
            command_index,
            0,
            "verify",
            command.arguments.join(" "),
            command,
        )],
        CommandKind::Terminate => vec![node_from_command(
            command_index,
            0,
            "terminate",
            command.arguments.join(" "),
            command,
        )],
    }
}

fn node_from_command(
    command_index: usize,
    local_index: usize,
    kind: &str,
    label: String,
    command: &OrchestrationCommand,
) -> GraphNode {
    GraphNode {
        id: stable_node_id(command_index, local_index, kind),
        kind: kind.to_string(),
        label,
        command: command.kind.keyword().to_string(),
        source_line: command.source_line,
        order: command_index,
    }
}

fn stable_node_id(command_index: usize, local_index: usize, kind: &str) -> String {
    format!("cmd-{command_index:03}-{kind}-{local_index:03}")
}

fn flow_edge(index: usize, from: &str, to: &str) -> GraphEdge {
    GraphEdge {
        id: format!("edge-{index:03}"),
        from: from.to_string(),
        to: to.to_string(),
        kind: "flow".to_string(),
        label: "siguiente".to_string(),
    }
}

fn argument_label(command: &OrchestrationCommand, index: usize, fallback: &str) -> String {
    command
        .arguments
        .get(index)
        .cloned()
        .unwrap_or_else(|| fallback.to_string())
}

#[cfg(test)]
mod tests {
    use super::{graph_from_source, graph_json_from_source};

    fn source() -> &'static str {
        r#"/goal rescatar_victimas
/plan buscar -> clasificar -> asistir
/delegate sector_a worker_1
/verify checklist_final
/terminate when verified"#
    }

    #[test]
    fn graph_exports_stable_nodes_and_edges() {
        let graph = graph_from_source(source(), Some("examples/rescate.loop".to_string()))
            .expect("debe exportar grafo");

        assert_eq!(graph.metadata.command_count, 5);
        assert_eq!(graph.metadata.node_count, 7);
        assert_eq!(graph.metadata.edge_count, 6);
        assert_eq!(graph.nodes[0].id, "cmd-000-goal-000");
        assert_eq!(graph.nodes[1].id, "cmd-001-plan_step-000");
        assert_eq!(graph.edges[0].from, "cmd-000-goal-000");
        assert_eq!(graph.edges[0].to, "cmd-001-plan_step-000");
    }

    #[test]
    fn graph_json_uses_spanish_top_level_keys() {
        let json = graph_json_from_source(source(), None).expect("debe generar JSON");
        let value: serde_json::Value = serde_json::from_str(&json).expect("debe ser JSON valido");

        assert!(value.get("metadatos").is_some());
        assert!(value.get("nodos").is_some());
        assert!(value.get("aristas").is_some());
    }
}
