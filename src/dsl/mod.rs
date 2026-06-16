//! DSL de orquestacion de Loopscape.
//!
//! Este modulo agrupa el AST, lexer, parser, validador semantico e interprete.
//! El DSL se mantiene separado de la visualizacion para conservar pruebas rapidas.

pub mod ast;
pub mod command;
pub mod error;
pub mod graph;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod token;
pub mod validator;

pub use ast::{OrchestrationCommand, OrchestrationProgram};
pub use command::CommandKind;
pub use error::DslError;
pub use graph::{
    graph_from_program, graph_from_source, graph_json_from_source, graph_to_json, GraphEdge,
    GraphMetadata, GraphNode, OrchestrationGraph,
};
pub use interpreter::{interpret_program, interpret_source, DslInterpreter, InterpreterConfig};
pub use lexer::{lex, Lexer};
pub use parser::{parse, Parser};
pub use token::{Token, TokenKind, TokenSpan};
pub use validator::{validate_program, validate_source, SemanticValidator};

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
