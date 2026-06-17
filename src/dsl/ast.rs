use serde::{Deserialize, Serialize};

use super::command::CommandKind;
use super::error::DslError;

/// Comando normalizado del DSL de orquestacion.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OrchestrationCommand {
    pub kind: CommandKind,
    pub arguments: Vec<String>,
    pub source_line: Option<usize>,
}

impl OrchestrationCommand {
    /// Crea un comando generico con argumentos normalizados.
    pub fn new<I, S>(kind: CommandKind, arguments: I) -> Result<Self, DslError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let arguments = arguments
            .into_iter()
            .map(Into::into)
            .filter(|value: &String| !value.trim().is_empty())
            .collect::<Vec<_>>();

        if arguments.is_empty() {
            return Err(DslError::EmptyArguments {
                command: kind.keyword().to_string(),
            });
        }

        Ok(Self {
            kind,
            arguments,
            source_line: None,
        })
    }

    /// Asocia una linea de origen sin cambiar el contenido del comando.
    pub fn with_source_line(mut self, source_line: usize) -> Self {
        self.source_line = Some(source_line);
        self
    }

    /// Construye un comando `/goal`.
    pub fn goal(name: impl Into<String>) -> Result<Self, DslError> {
        Self::new(CommandKind::Goal, [name.into()])
    }

    /// Construye un comando `/plan`.
    pub fn plan<I, S>(steps: I) -> Result<Self, DslError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self::new(CommandKind::Plan, steps)
    }

    /// Construye un comando `/delegate`.
    pub fn delegate(
        target: impl Into<String>,
        worker: impl Into<String>,
    ) -> Result<Self, DslError> {
        Self::new(CommandKind::Delegate, [target.into(), worker.into()])
    }

    /// Construye un comando `/verify`.
    pub fn verify(check: impl Into<String>) -> Result<Self, DslError> {
        Self::new(CommandKind::Verify, [check.into()])
    }

    /// Construye un comando `/terminate`.
    pub fn terminate(condition: impl Into<String>) -> Result<Self, DslError> {
        Self::new(CommandKind::Terminate, [condition.into()])
    }

    /// Construye un comando `/worker-failure`.
    pub fn worker_failure(
        worker_id: impl Into<String>,
        start_tick: impl Into<String>,
        duration_ticks: impl Into<String>,
    ) -> Result<Self, DslError> {
        Self::new(
            CommandKind::WorkerFailure,
            [worker_id.into(), start_tick.into(), duration_ticks.into()],
        )
    }

    /// Construye un comando `/byzantine-failure`.
    pub fn byzantine_failure(
        worker_id: impl Into<String>,
        false_value: impl Into<String>,
    ) -> Result<Self, DslError> {
        Self::new(
            CommandKind::ByzantineFailure,
            [worker_id.into(), false_value.into()],
        )
    }

    /// Construye un comando `/byzantine-vote`.
    pub fn byzantine_vote(value: impl Into<String>) -> Result<Self, DslError> {
        Self::new(CommandKind::ByzantineVote, [value.into()])
    }

    /// Devuelve una representacion textual util para depuracion y pruebas.
    pub fn to_script_line(&self) -> String {
        match self.kind {
            CommandKind::Plan => format!("{} {}", self.kind.keyword(), self.arguments.join(" -> ")),
            _ => format!("{} {}", self.kind.keyword(), self.arguments.join(" ")),
        }
    }
}

/// Programa de orquestacion compuesto por comandos tipados.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct OrchestrationProgram {
    pub commands: Vec<OrchestrationCommand>,
}

impl OrchestrationProgram {
    /// Crea un programa desde una lista de comandos.
    pub fn new(commands: Vec<OrchestrationCommand>) -> Self {
        Self { commands }
    }

    /// Crea un programa vacio para construccion incremental.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Agrega un comando al final del programa.
    pub fn push(&mut self, command: OrchestrationCommand) {
        self.commands.push(command);
    }

    /// Devuelve true si el programa no contiene comandos.
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    /// Cuenta cuantos comandos de un tipo existen en el programa.
    pub fn command_count_by_kind(&self, kind: CommandKind) -> usize {
        self.commands
            .iter()
            .filter(|command| command.kind == kind)
            .count()
    }

    /// Devuelve true si el programa contiene al menos una politica de terminacion.
    pub fn has_termination(&self) -> bool {
        self.command_count_by_kind(CommandKind::Terminate) > 0
    }

    /// Exporta el programa como lineas de script para depuracion.
    pub fn to_script_lines(&self) -> Vec<String> {
        self.commands
            .iter()
            .map(OrchestrationCommand::to_script_line)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{OrchestrationCommand, OrchestrationProgram};
    use crate::dsl::command::CommandKind;

    #[test]
    fn command_constructors_create_minimal_commands() {
        let goal = OrchestrationCommand::goal("rescatar_victimas").expect("debe crear goal");
        let plan = OrchestrationCommand::plan(["buscar", "clasificar", "asistir"])
            .expect("debe crear plan");
        let delegate =
            OrchestrationCommand::delegate("sector_a", "worker_1").expect("debe crear delegate");
        let verify = OrchestrationCommand::verify("checklist_final").expect("debe crear verify");
        let terminate =
            OrchestrationCommand::terminate("when_verified").expect("debe crear terminate");

        assert_eq!(goal.kind, CommandKind::Goal);
        assert_eq!(plan.kind, CommandKind::Plan);
        assert_eq!(delegate.kind, CommandKind::Delegate);
        assert_eq!(verify.kind, CommandKind::Verify);
        assert_eq!(terminate.kind, CommandKind::Terminate);
        let worker_failure = CommandKind::WorkerFailure;
        let byzantine_failure = CommandKind::ByzantineFailure;
        let byzantine_vote = CommandKind::ByzantineVote;

        assert_eq!(worker_failure, CommandKind::WorkerFailure);
        assert_eq!(byzantine_failure, CommandKind::ByzantineFailure);
        assert_eq!(byzantine_vote, CommandKind::ByzantineVote);
    }

    #[test]
    fn empty_arguments_are_rejected() {
        let error = OrchestrationCommand::new(CommandKind::Goal, [""])
            .expect_err("debe rechazar argumentos vacios");

        assert_eq!(error.to_string(), "el comando /goal necesita argumentos");
    }

    #[test]
    fn program_counts_commands_by_kind() {
        let program = OrchestrationProgram::new(vec![
            OrchestrationCommand::goal("rescatar_victimas").expect("debe crear goal"),
            OrchestrationCommand::plan(["buscar", "clasificar"]).expect("debe crear plan"),
            OrchestrationCommand::verify("checklist_final").expect("debe crear verify"),
            OrchestrationCommand::terminate("when_verified").expect("debe crear terminate"),
        ]);

        assert_eq!(program.command_count_by_kind(CommandKind::Goal), 1);
        assert_eq!(program.command_count_by_kind(CommandKind::Plan), 1);
        assert!(program.has_termination());
    }

    #[test]
    fn program_exports_script_lines() {
        let program = OrchestrationProgram::new(vec![
            OrchestrationCommand::goal("rescatar_victimas").expect("debe crear goal"),
            OrchestrationCommand::plan(["buscar", "clasificar"]).expect("debe crear plan"),
        ]);

        assert_eq!(
            program.to_script_lines(),
            vec![
                "/goal rescatar_victimas".to_string(),
                "/plan buscar -> clasificar".to_string(),
            ]
        );
    }

    #[test]
    fn command_can_store_source_line() {
        let command = OrchestrationCommand::verify("checklist_final")
            .expect("debe crear verify")
            .with_source_line(4);

        assert_eq!(command.source_line, Some(4));
    }
}
