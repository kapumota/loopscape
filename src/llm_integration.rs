use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// --------- TIPOS DE LLM ---------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LLMRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32,
    pub max_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LLMResponse {
    pub choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Choice {
    pub message: Message,
    pub finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ReActStep {
    pub thought: String,
    pub action: String,
    pub action_input: String,
    pub observation: String,
}

// --------- COMPONENTES PARA LLM ---------
#[derive(Component)]
pub struct LLMBrain {
    pub api_url: String,
    pub api_key: String,
    pub model: String,
    pub is_processing: bool,
    pub pending_prompt: Option<String>,
}

#[derive(Component, Debug, Clone)]
pub struct ReActTrace {
    pub steps: Vec<ReActStep>,
    pub current_step: u32,
    pub task: String,
    pub is_complete: bool,
}

#[derive(Component)]
pub struct LLMResponsePending;

#[derive(Event)]
pub struct LLMRequestEvent {
    pub entity: Entity,
    pub prompt: String,
    pub context: ReActContext,
}

#[derive(Event)]
pub struct LLMResponseEvent {
    pub entity: Entity,
    pub response: String,
    pub react_step: Option<ReActStep>,
}

#[derive(Debug, Clone)]
pub struct ReActContext {
    pub previous_thoughts: Vec<String>,
    pub available_tools: Vec<String>,
    pub task_description: String,
}

impl Default for ReActContext {
    fn default() -> Self {
        Self {
            previous_thoughts: vec![],
            available_tools: vec![
                "buscar".to_string(),
                "calcular".to_string(),
                "leer_archivo".to_string(),
            ],
            task_description: "Procesar la tarea entrante".to_string(),
        }
    }
}

// --------- SISTEMAS LLM ---------

/// Sistema que detecta loops que necesitan procesamiento LLM.
pub fn detect_llm_needs(
    mut commands: Commands,
    query: Query<(Entity, &crate::components::LoopState, &LLMBrain), Without<LLMResponsePending>>,
) {
    for (entity, state, brain) in query.iter() {
        if *state == crate::components::LoopState::Thinking && !brain.is_processing {
            if brain.pending_prompt.is_some() {
                commands.entity(entity).insert(LLMResponsePending);
                // En una implementacion real, aqui se lanzaria la peticion HTTP.
                // Por ahora se mantiene como simulacion local.
            }
        }
    }
}

/// Parsea una respuesta LLM en formato ReAct.
pub fn parse_react_response(response: &str) -> Option<ReActStep> {
    parse_react_response_with_labels(response, "Thought:", "Action:", "Entrada de accion:", "Observation:")
        .or_else(|| {
            parse_react_response_with_labels(
                response,
                "Pensamiento:",
                "Accion:",
                "Entrada de accion:",
                "Observacion:",
            )
        })
}

fn parse_react_response_with_labels(
    response: &str,
    thought_label: &str,
    action_label: &str,
    action_input_label: &str,
    observation_label: &str,
) -> Option<ReActStep> {
    let thought = extract_between(response, thought_label, action_label)?;
    let action = extract_between(response, action_label, action_input_label)?;
    let action_input = extract_between(response, action_input_label, observation_label)?;
    let observation = extract_after(response, observation_label)?;

    Some(ReActStep {
        thought: thought.trim().to_string(),
        action: action.trim().to_string(),
        action_input: action_input.trim().to_string(),
        observation: observation.trim().to_string(),
    })
}

fn extract_between(text: &str, start: &str, end: &str) -> Option<String> {
    let start_idx = text.find(start)? + start.len();
    let end_idx = text[start_idx..].find(end)?;
    Some(text[start_idx..start_idx + end_idx].trim().to_string())
}

fn extract_after(text: &str, start: &str) -> Option<String> {
    let start_idx = text.find(start)? + start.len();
    Some(text[start_idx..].trim().to_string())
}

/// Genera el prompt ReAct para el LLM.
pub fn generate_react_prompt(task: &str, context: &ReActContext) -> String {
    let tools_list = context.available_tools.join(", ");
    let previous = if context.previous_thoughts.is_empty() {
        "Sin pensamientos previos.".to_string()
    } else {
        context.previous_thoughts.join("\n")
    };

    format!(
        r#"Eres un agente dentro de un loop ReAct. Resuelve la tarea paso a paso.

Tarea: {}
Herramientas disponibles: {}
Pensamientos previos: {}

Responde exactamente con este formato:
Pensamiento: [tu razonamiento]
Accion: [nombre de herramienta o finalizar]
Entrada de accion: [entrada para la herramienta]
Observacion: [resultado esperado]"#,
        task, tools_list, previous
    )
}

/// Visualiza la traza ReAct como una burbuja sobre el agente.
pub fn visualize_react_trace(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &ReActTrace), With<crate::components::LoopAgent>>,
) {
    for (transform, trace) in query.iter() {
        if trace.steps.last().is_some() {
            // Dibuja una burbuja de pensamiento sobre el agente.
            let pos = transform.translation + Vec3::new(0.0, 30.0, 0.0);
            gizmos.circle_2d(
                pos.truncate(),
                25.0,
                Color::srgb(0.9, 0.9, 1.0).with_alpha(0.3),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_react_response_supports_legacy_labels() {
        let response = "Thought: revisar la tarea\nAccion: buscar\nEntrada de accion: loops\nObservation: datos encontrados";
        let step = parse_react_response(response).expect("debe parsear formato ReAct heredado");

        assert_eq!(step.thought, "revisar la tarea");
        assert_eq!(step.action, "buscar");
        assert_eq!(step.action_input, "loops");
        assert_eq!(step.observation, "datos encontrados");
    }

    #[test]
    fn parse_react_response_supports_spanish_labels() {
        let response = "Pensamiento: revisar la tarea\nAccion: buscar\nEntrada de accion: loops\nObservacion: datos encontrados";
        let step = parse_react_response(response).expect("debe parsear formato ReAct en espanol");

        assert_eq!(step.thought, "revisar la tarea");
        assert_eq!(step.action, "buscar");
        assert_eq!(step.action_input, "loops");
        assert_eq!(step.observation, "datos encontrados");
    }

    #[test]
    fn parse_react_response_rejects_incomplete_text() {
        let response = "Pensamiento: falta accion";
        assert!(parse_react_response(response).is_none());
    }

    #[test]
    fn generate_react_prompt_uses_spanish_visible_text() {
        let context = ReActContext::default();
        let prompt = generate_react_prompt("diagnosticar loop colgado", &context);

        assert!(prompt.contains("Tarea: diagnosticar loop colgado"));
        assert!(prompt.contains("Herramientas disponibles"));
        assert!(prompt.contains("Pensamiento:"));
    }
}
