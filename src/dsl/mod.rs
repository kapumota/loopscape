//! DSL de orquestacion de Loopscape.
//!
//! Esta fase define solo el AST y el modelo de comandos. El lexer, parser,
//! validador semantico e interprete se agregan en microfases posteriores.

pub mod ast;
pub mod command;
pub mod error;

pub use ast::{OrchestrationCommand, OrchestrationProgram};
pub use command::CommandKind;
pub use error::DslError;

#[cfg(test)]
mod tests {
    use super::{CommandKind, OrchestrationCommand, OrchestrationProgram};

    #[test]
    fn dsl_program_can_represent_minimal_flow() {
        let program = OrchestrationProgram::new(vec![
            OrchestrationCommand::goal("rescatar_victimas").expect("debe crear goal"),
            OrchestrationCommand::plan(["buscar", "clasificar", "asistir"])
                .expect("debe crear plan"),
            OrchestrationCommand::delegate("sector_a", "worker_1").expect("debe crear delegate"),
            OrchestrationCommand::verify("checklist_final").expect("debe crear verify"),
            OrchestrationCommand::terminate("when_verified").expect("debe crear terminate"),
        ]);

        assert_eq!(program.commands.len(), 5);
        assert_eq!(program.command_count_by_kind(CommandKind::Goal), 1);
        assert!(program.has_termination());
    }
}
