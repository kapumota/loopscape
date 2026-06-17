use crate::core::byzantine::{ByzantineFailurePlan, ByzantineFailureSpec};
use crate::core::failure::{RecoverableFailurePlan, WorkerFailureSpec};

use super::ast::OrchestrationProgram;
use super::command::CommandKind;
use super::error::DslError;
use super::validator::{validate_program, validate_source};

/// Escenario de fallos extraido desde el DSL.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DslFailureScenario {
    pub recoverable_failures: RecoverableFailurePlan,
    pub byzantine_failures: ByzantineFailurePlan,
    pub byzantine_vote_value: Option<String>,
}

impl DslFailureScenario {
    pub fn empty() -> Self {
        Self {
            recoverable_failures: RecoverableFailurePlan::none(),
            byzantine_failures: ByzantineFailurePlan::none(),
            byzantine_vote_value: None,
        }
    }
}

/// Extrae fallos declarados en un programa DSL validado.
pub fn failure_scenario_from_program(
    program: &OrchestrationProgram,
) -> Result<DslFailureScenario, DslError> {
    validate_program(program)?;

    let mut scenario = DslFailureScenario::empty();

    for command in &program.commands {
        match command.kind {
            CommandKind::WorkerFailure => {
                let failure = worker_failure_from_arguments(&command.arguments)?;
                scenario.recoverable_failures = scenario.recoverable_failures.with_failure(failure);
            }
            CommandKind::ByzantineFailure => {
                let failure = byzantine_failure_from_arguments(&command.arguments)?;
                scenario.byzantine_failures = scenario.byzantine_failures.with_failure(failure);
            }
            CommandKind::ByzantineVote => {
                scenario.byzantine_vote_value = Some(command.arguments.join(" "));
            }
            _ => {}
        }
    }

    Ok(scenario)
}

/// Parsea, valida y extrae un escenario de fallos desde texto DSL.
pub fn failure_scenario_from_source(input: &str) -> Result<DslFailureScenario, DslError> {
    let program = validate_source(input)?;
    failure_scenario_from_program(&program)
}

fn worker_failure_from_arguments(arguments: &[String]) -> Result<WorkerFailureSpec, DslError> {
    let worker_id = required_argument(arguments, 0, "/worker-failure")?
        .parse::<u32>()
        .map_err(|_| DslError::invalid_program("worker invalido en /worker-failure"))?;
    let start_tick = required_argument(arguments, 1, "/worker-failure")?
        .parse::<u64>()
        .map_err(|_| DslError::invalid_program("tick inicial invalido en /worker-failure"))?;
    let duration_ticks = required_argument(arguments, 2, "/worker-failure")?
        .parse::<u64>()
        .map_err(|_| DslError::invalid_program("duracion invalida en /worker-failure"))?;

    WorkerFailureSpec::new(worker_id, start_tick, duration_ticks).map_err(DslError::invalid_program)
}

fn byzantine_failure_from_arguments(
    arguments: &[String],
) -> Result<ByzantineFailureSpec, DslError> {
    let worker_id = required_argument(arguments, 0, "/byzantine-failure")?
        .parse::<u32>()
        .map_err(|_| DslError::invalid_program("worker invalido en /byzantine-failure"))?;
    let false_value = arguments.get(1..).unwrap_or_default().join(" ");

    ByzantineFailureSpec::new(worker_id, false_value).map_err(DslError::invalid_program)
}

fn required_argument<'a>(
    arguments: &'a [String],
    index: usize,
    command: &str,
) -> Result<&'a str, DslError> {
    arguments
        .get(index)
        .map(String::as_str)
        .ok_or_else(|| DslError::invalid_program(format!("faltan argumentos en {command}")))
}

#[cfg(test)]
mod tests {
    use super::{failure_scenario_from_source, DslFailureScenario};

    #[test]
    fn empty_scenario_has_no_failures() {
        let scenario = DslFailureScenario::empty();

        assert!(scenario.recoverable_failures.failures.is_empty());
        assert!(scenario.byzantine_failures.failures.is_empty());
        assert_eq!(scenario.byzantine_vote_value, None);
    }

    #[test]
    fn scenario_extracts_recoverable_and_byzantine_failures() {
        let scenario = failure_scenario_from_source(
            r#"/goal rescatar_victimas
/plan buscar -> clasificar
/worker-failure 1 4 3
/byzantine-failure 2 falso
/byzantine-vote verdadero
/verify checklist_final
/terminate when verified"#,
        )
        .expect("escenario valido");

        assert!(scenario.recoverable_failures.is_worker_hung(1, 4));
        assert_eq!(
            scenario.byzantine_failures.false_value_for(2),
            Some("falso")
        );
        assert_eq!(scenario.byzantine_vote_value, Some("verdadero".to_string()));
    }
}
