use serde::{Deserialize, Serialize};

use super::error::DslError;

/// Tipo de comando soportado por el DSL de orquestacion.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum CommandKind {
    Goal,
    Plan,
    Delegate,
    Verify,
    Terminate,
    WorkerFailure,
    ByzantineFailure,
    ByzantineVote,
}

impl CommandKind {
    /// Devuelve la palabra reservada textual del comando.
    pub fn keyword(self) -> &'static str {
        match self {
            Self::Goal => "/goal",
            Self::Plan => "/plan",
            Self::Delegate => "/delegate",
            Self::Verify => "/verify",
            Self::Terminate => "/terminate",
            Self::WorkerFailure => "/worker-failure",
            Self::ByzantineFailure => "/byzantine-failure",
            Self::ByzantineVote => "/byzantine-vote",
        }
    }

    /// Construye un tipo de comando a partir de una palabra reservada.
    pub fn from_keyword(value: &str) -> Result<Self, DslError> {
        match value {
            "/goal" => Ok(Self::Goal),
            "/plan" => Ok(Self::Plan),
            "/delegate" => Ok(Self::Delegate),
            "/verify" => Ok(Self::Verify),
            "/terminate" => Ok(Self::Terminate),
            "/worker-failure" => Ok(Self::WorkerFailure),
            "/byzantine-failure" => Ok(Self::ByzantineFailure),
            "/byzantine-vote" => Ok(Self::ByzantineVote),
            other => Err(DslError::UnknownCommand {
                keyword: other.to_string(),
            }),
        }
    }
}

impl std::fmt::Display for CommandKind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.keyword())
    }
}

#[cfg(test)]
mod tests {
    use super::CommandKind;

    #[test]
    fn command_kind_returns_keyword() {
        assert_eq!(CommandKind::Goal.keyword(), "/goal");
        assert_eq!(CommandKind::Plan.keyword(), "/plan");
        assert_eq!(CommandKind::Delegate.keyword(), "/delegate");
        assert_eq!(CommandKind::Verify.keyword(), "/verify");
        assert_eq!(CommandKind::Terminate.keyword(), "/terminate");
        assert_eq!(CommandKind::WorkerFailure.keyword(), "/worker-failure");
        assert_eq!(
            CommandKind::ByzantineFailure.keyword(),
            "/byzantine-failure"
        );
        assert_eq!(CommandKind::ByzantineVote.keyword(), "/byzantine-vote");
    }

    #[test]
    fn command_kind_parses_known_keyword() {
        assert_eq!(CommandKind::from_keyword("/goal"), Ok(CommandKind::Goal));
        assert_eq!(CommandKind::from_keyword("/plan"), Ok(CommandKind::Plan));
        assert_eq!(
            CommandKind::from_keyword("/delegate"),
            Ok(CommandKind::Delegate)
        );
        assert_eq!(
            CommandKind::from_keyword("/verify"),
            Ok(CommandKind::Verify)
        );
        assert_eq!(
            CommandKind::from_keyword("/terminate"),
            Ok(CommandKind::Terminate)
        );
    }

    #[test]
    fn command_kind_rejects_unknown_keyword() {
        let error =
            CommandKind::from_keyword("/otro").expect_err("debe rechazar comando desconocido");

        assert_eq!(error.to_string(), "comando DSL desconocido: /otro");
    }
}
