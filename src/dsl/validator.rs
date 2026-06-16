use super::ast::OrchestrationProgram;
use super::command::CommandKind;
use super::error::DslError;
use super::parser::parse;

/// Validador semantico del DSL de orquestacion.
pub struct SemanticValidator;

impl SemanticValidator {
    /// Valida las reglas semanticas minimas del programa.
    pub fn validate(program: &OrchestrationProgram) -> Result<(), DslError> {
        validate_goal_count(program)?;
        validate_plan_steps(program)?;
        validate_delegate_workers(program)?;
        validate_verified_termination_order(program)?;
        Ok(())
    }
}

/// Valida un programa ya parseado.
pub fn validate_program(program: &OrchestrationProgram) -> Result<(), DslError> {
    SemanticValidator::validate(program)
}

/// Parsea y valida un script DSL completo.
pub fn validate_source(input: &str) -> Result<OrchestrationProgram, DslError> {
    let program = parse(input)?;
    validate_program(&program)?;
    Ok(program)
}

fn validate_goal_count(program: &OrchestrationProgram) -> Result<(), DslError> {
    let goal_count = program.command_count_by_kind(CommandKind::Goal);

    match goal_count {
        1 => Ok(()),
        0 => Err(DslError::invalid_program(
            "el programa DSL necesita exactamente un /goal",
        )),
        _ => Err(DslError::invalid_program(
            "el programa DSL no puede tener mas de un /goal",
        )),
    }
}

fn validate_plan_steps(program: &OrchestrationProgram) -> Result<(), DslError> {
    for command in &program.commands {
        if command.kind == CommandKind::Plan && command.arguments.is_empty() {
            return Err(DslError::invalid_program(
                "el comando /plan necesita al menos un paso",
            ));
        }
    }

    Ok(())
}

fn validate_delegate_workers(program: &OrchestrationProgram) -> Result<(), DslError> {
    for command in &program.commands {
        if command.kind != CommandKind::Delegate {
            continue;
        }

        if command.arguments.len() < 2 {
            return Err(DslError::invalid_program(
                "el comando /delegate necesita objetivo y worker",
            ));
        }

        if command.arguments[1].trim().is_empty() {
            return Err(DslError::invalid_program(
                "el comando /delegate no puede referenciar un worker vacio",
            ));
        }
    }

    Ok(())
}

fn validate_verified_termination_order(program: &OrchestrationProgram) -> Result<(), DslError> {
    let mut verify_seen = false;

    for command in &program.commands {
        match command.kind {
            CommandKind::Verify => verify_seen = true,
            CommandKind::Terminate
                if is_verified_termination(&command.arguments) && !verify_seen =>
            {
                return Err(DslError::invalid_program(
                    "el comando /verify debe aparecer antes de /terminate when verified",
                ));
            }
            _ => {}
        }
    }

    Ok(())
}

fn is_verified_termination(arguments: &[String]) -> bool {
    if arguments.len() == 1 {
        return arguments[0] == "when_verified" || arguments[0] == "verified";
    }

    arguments.len() == 2 && arguments[0] == "when" && arguments[1] == "verified"
}

#[cfg(test)]
mod tests {
    use super::{validate_program, validate_source};
    use crate::dsl::{CommandKind, OrchestrationCommand, OrchestrationProgram};

    fn valid_program() -> OrchestrationProgram {
        validate_source(
            r#"/goal rescatar_victimas
/plan buscar -> clasificar -> asistir
/delegate sector_a worker_1
/verify checklist_final
/terminate when verified"#,
        )
        .expect("debe validar flujo minimo")
    }

    #[test]
    fn validator_accepts_minimal_flow() {
        let program = valid_program();

        assert_eq!(program.commands.len(), 5);
        assert_eq!(program.command_count_by_kind(CommandKind::Goal), 1);
    }

    #[test]
    fn validator_requires_exactly_one_goal() {
        let error = validate_source(
            r#"/plan buscar -> clasificar
/verify checklist_final
/terminate when verified"#,
        )
        .expect_err("debe rechazar programa sin goal");

        assert_eq!(
            error.to_string(),
            "el programa DSL necesita exactamente un /goal"
        );
    }

    #[test]
    fn validator_rejects_multiple_goals() {
        let error = validate_source(
            r#"/goal uno
/goal dos
/plan buscar
/verify checklist_final
/terminate when verified"#,
        )
        .expect_err("debe rechazar multiples goals");

        assert_eq!(
            error.to_string(),
            "el programa DSL no puede tener mas de un /goal"
        );
    }

    #[test]
    fn validator_rejects_empty_plan_in_manual_ast() {
        let program = OrchestrationProgram::new(vec![
            OrchestrationCommand::goal("rescatar_victimas").expect("debe crear goal"),
            OrchestrationCommand {
                kind: CommandKind::Plan,
                arguments: Vec::new(),
                source_line: Some(2),
            },
        ]);

        let error = validate_program(&program).expect_err("debe rechazar plan vacio");

        assert_eq!(
            error.to_string(),
            "el comando /plan necesita al menos un paso"
        );
    }

    #[test]
    fn validator_rejects_delegate_without_worker() {
        let program = OrchestrationProgram::new(vec![
            OrchestrationCommand::goal("rescatar_victimas").expect("debe crear goal"),
            OrchestrationCommand {
                kind: CommandKind::Delegate,
                arguments: vec!["sector_a".to_string()],
                source_line: Some(2),
            },
        ]);

        let error = validate_program(&program).expect_err("debe rechazar worker ausente");

        assert_eq!(
            error.to_string(),
            "el comando /delegate necesita objetivo y worker"
        );
    }

    #[test]
    fn validator_rejects_empty_delegate_worker() {
        let program = OrchestrationProgram::new(vec![
            OrchestrationCommand::goal("rescatar_victimas").expect("debe crear goal"),
            OrchestrationCommand {
                kind: CommandKind::Delegate,
                arguments: vec!["sector_a".to_string(), "".to_string()],
                source_line: Some(2),
            },
        ]);

        let error = validate_program(&program).expect_err("debe rechazar worker vacio");

        assert_eq!(
            error.to_string(),
            "el comando /delegate no puede referenciar un worker vacio"
        );
    }

    #[test]
    fn validator_requires_verify_before_verified_termination() {
        let error = validate_source(
            r#"/goal rescatar_victimas
/plan buscar -> clasificar
/terminate when verified
/verify checklist_final"#,
        )
        .expect_err("debe rechazar terminacion antes de verify");

        assert_eq!(
            error.to_string(),
            "el comando /verify debe aparecer antes de /terminate when verified"
        );
    }

    #[test]
    fn validator_keeps_unknown_command_error_clear() {
        let error = validate_source("/otro valor").expect_err("debe rechazar comando desconocido");

        assert_eq!(error.to_string(), "comando DSL desconocido: /otro");
    }
}
