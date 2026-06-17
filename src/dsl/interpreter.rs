use crate::core::event::CoreEvent;

use super::ast::{OrchestrationCommand, OrchestrationProgram};
use super::command::CommandKind;
use super::error::DslError;
use super::validator::{validate_program, validate_source};

/// Configuracion minima del interprete del DSL.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub struct InterpreterConfig {
    pub tick: u64,
}

/// Interprete que traduce programas DSL validados a eventos del nucleo.
pub struct DslInterpreter {
    config: InterpreterConfig,
}

impl DslInterpreter {
    /// Crea un interprete con configuracion por defecto.
    pub fn new() -> Self {
        Self {
            config: InterpreterConfig::default(),
        }
    }

    /// Crea un interprete con configuracion explicita.
    pub fn with_config(config: InterpreterConfig) -> Self {
        Self { config }
    }

    /// Convierte un programa DSL en eventos internos del nucleo.
    pub fn interpret(&self, program: &OrchestrationProgram) -> Result<Vec<CoreEvent>, DslError> {
        validate_program(program)?;

        let mut events = Vec::new();

        for command in &program.commands {
            match command.kind {
                CommandKind::Goal => events.push(CoreEvent::GoalCreated {
                    tick: self.config.tick,
                    goal: required_argument(command, 0, "/goal")?,
                }),
                CommandKind::Plan => {
                    for (index, step) in command.arguments.iter().enumerate() {
                        events.push(CoreEvent::PlanStepCreated {
                            tick: self.config.tick,
                            index: index as u32,
                            step: step.clone(),
                        });
                    }
                }
                CommandKind::Delegate => events.push(CoreEvent::DelegationRequested {
                    tick: self.config.tick,
                    target: required_argument(command, 0, "/delegate")?,
                    worker: required_argument(command, 1, "/delegate")?,
                }),
                CommandKind::Verify => events.push(CoreEvent::VerificationRequested {
                    tick: self.config.tick,
                    checklist: required_joined_arguments(command, "/verify")?,
                }),
                CommandKind::Terminate => events.push(CoreEvent::TerminationPolicySet {
                    tick: self.config.tick,
                    policy: normalize_policy(&command.arguments)?,
                }),
                CommandKind::WorkerFailure
                | CommandKind::ByzantineFailure
                | CommandKind::ByzantineVote => {}
            }
        }

        Ok(events)
    }

    /// Parsea, valida e interpreta un script DSL completo.
    pub fn interpret_source(&self, input: &str) -> Result<Vec<CoreEvent>, DslError> {
        let program = validate_source(input)?;
        self.interpret(&program)
    }
}

impl Default for DslInterpreter {
    fn default() -> Self {
        Self::new()
    }
}

/// Convierte un programa validado a eventos del nucleo.
pub fn interpret_program(program: &OrchestrationProgram) -> Result<Vec<CoreEvent>, DslError> {
    DslInterpreter::new().interpret(program)
}

/// Parsea, valida e interpreta un script DSL completo.
pub fn interpret_source(input: &str) -> Result<Vec<CoreEvent>, DslError> {
    DslInterpreter::new().interpret_source(input)
}

fn required_argument(
    command: &OrchestrationCommand,
    index: usize,
    command_name: &str,
) -> Result<String, DslError> {
    command.arguments.get(index).cloned().ok_or_else(|| {
        DslError::invalid_program(format!(
            "el comando {command_name} no tiene suficientes argumentos"
        ))
    })
}

fn required_joined_arguments(
    command: &OrchestrationCommand,
    command_name: &str,
) -> Result<String, DslError> {
    if command.arguments.is_empty() {
        return Err(DslError::invalid_program(format!(
            "el comando {command_name} necesita argumentos"
        )));
    }

    Ok(command.arguments.join(" "))
}

fn normalize_policy(arguments: &[String]) -> Result<String, DslError> {
    if arguments.is_empty() {
        return Err(DslError::invalid_program(
            "el comando /terminate necesita argumentos",
        ));
    }

    if arguments.len() == 2 && arguments[0] == "when" && arguments[1] == "verified" {
        return Ok("when_verified".to_string());
    }

    Ok(arguments.join("_"))
}

#[cfg(test)]
mod tests {
    use super::{interpret_program, interpret_source, DslInterpreter, InterpreterConfig};
    use crate::core::event::CoreEvent;
    use crate::dsl::{parse, validate_source};

    fn source() -> &'static str {
        r#"/goal rescatar_victimas
/plan buscar -> clasificar -> asistir
/delegate sector_a worker_1
/verify checklist_final
/terminate when verified"#
    }

    #[test]
    fn interpreter_converts_minimal_flow_to_core_events() {
        let events = interpret_source(source()).expect("debe interpretar flujo minimo");

        assert_eq!(events.len(), 7);
        assert!(matches!(events[0], CoreEvent::GoalCreated { .. }));
        assert!(matches!(events[1], CoreEvent::PlanStepCreated { .. }));
        assert!(matches!(events[2], CoreEvent::PlanStepCreated { .. }));
        assert!(matches!(events[3], CoreEvent::PlanStepCreated { .. }));
        assert!(matches!(events[4], CoreEvent::DelegationRequested { .. }));
        assert!(matches!(events[5], CoreEvent::VerificationRequested { .. }));
        assert!(matches!(events[6], CoreEvent::TerminationPolicySet { .. }));
    }

    #[test]
    fn interpreter_preserves_plan_step_order() {
        let events = interpret_source(source()).expect("debe interpretar pasos");
        let steps = events
            .iter()
            .filter_map(|event| match event {
                CoreEvent::PlanStepCreated { index, step, .. } => Some((*index, step.clone())),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert_eq!(
            steps,
            vec![
                (0, "buscar".to_string()),
                (1, "clasificar".to_string()),
                (2, "asistir".to_string()),
            ]
        );
    }

    #[test]
    fn interpreter_normalizes_verified_termination_policy() {
        let events = interpret_source(source()).expect("debe interpretar terminacion");
        let policy = events.iter().find_map(|event| match event {
            CoreEvent::TerminationPolicySet { policy, .. } => Some(policy.clone()),
            _ => None,
        });

        assert_eq!(policy, Some("when_verified".to_string()));
    }

    #[test]
    fn interpreter_uses_configured_tick() {
        let program = validate_source(source()).expect("debe validar programa");
        let interpreter = DslInterpreter::with_config(InterpreterConfig { tick: 42 });
        let events = interpreter
            .interpret(&program)
            .expect("debe interpretar con tick fijo");

        assert!(events.iter().all(|event| match event {
            CoreEvent::GoalCreated { tick, .. }
            | CoreEvent::PlanStepCreated { tick, .. }
            | CoreEvent::DelegationRequested { tick, .. }
            | CoreEvent::VerificationRequested { tick, .. }
            | CoreEvent::TerminationPolicySet { tick, .. } => *tick == 42,
            _ => true,
        }));
    }

    #[test]
    fn interpreter_rejects_invalid_program_before_events() {
        let program =
            parse("/plan buscar -> clasificar").expect("debe parsear programa incompleto");
        let error = interpret_program(&program).expect_err("debe rechazar programa invalido");

        assert_eq!(
            error.to_string(),
            "el programa DSL necesita exactamente un /goal"
        );
    }
}
