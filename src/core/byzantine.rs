use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::supervisor::WorkerId;

/// Tipo de respuesta emitida por un worker durante una votacion.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum WorkerResponseKind {
    Honest,
    False,
}

/// Respuesta individual de un worker.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkerResponse {
    pub worker_id: WorkerId,
    pub value: String,
    pub kind: WorkerResponseKind,
}

impl WorkerResponse {
    pub fn honest(worker_id: WorkerId, value: impl Into<String>) -> Self {
        Self {
            worker_id,
            value: value.into(),
            kind: WorkerResponseKind::Honest,
        }
    }

    pub fn false_response(worker_id: WorkerId, value: impl Into<String>) -> Self {
        Self {
            worker_id,
            value: value.into(),
            kind: WorkerResponseKind::False,
        }
    }

    pub fn is_false(&self) -> bool {
        self.kind == WorkerResponseKind::False
    }
}

/// Especifica que un worker respondera con un valor falso.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ByzantineFailureSpec {
    pub worker_id: WorkerId,
    pub false_value: String,
}

impl ByzantineFailureSpec {
    pub fn new(worker_id: WorkerId, false_value: impl Into<String>) -> Result<Self, String> {
        let false_value = false_value.into();
        if false_value.trim().is_empty() {
            return Err("false_value no puede estar vacio".to_string());
        }

        Ok(Self {
            worker_id,
            false_value,
        })
    }
}

/// Plan determinista de fallos bizantinos simplificados.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ByzantineFailurePlan {
    pub failures: Vec<ByzantineFailureSpec>,
}

impl ByzantineFailurePlan {
    pub fn none() -> Self {
        Self {
            failures: Vec::new(),
        }
    }

    pub fn worker_lies(
        worker_id: WorkerId,
        false_value: impl Into<String>,
    ) -> Result<Self, String> {
        Ok(Self {
            failures: vec![ByzantineFailureSpec::new(worker_id, false_value)?],
        })
    }

    pub fn with_failure(mut self, failure: ByzantineFailureSpec) -> Self {
        self.failures.push(failure);
        self
    }

    pub fn false_value_for(&self, worker_id: WorkerId) -> Option<&str> {
        self.failures
            .iter()
            .find(|failure| failure.worker_id == worker_id)
            .map(|failure| failure.false_value.as_str())
    }
}

/// Conteo estable de votos por valor.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct VoteCount {
    pub value: String,
    pub votes: usize,
}

/// Decision final producida por la votacion simple.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum VoteDecision {
    Accepted { value: String, votes: usize },
    Rejected { reason: String },
}

impl VoteDecision {
    pub fn is_accepted(&self) -> bool {
        matches!(self, VoteDecision::Accepted { .. })
    }
}

/// Resultado completo de una votacion.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct VotingOutcome {
    pub decision: VoteDecision,
    pub counts: Vec<VoteCount>,
    pub false_responses: usize,
}

/// Configuracion de una votacion por mayoria simple.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct VotingConfig {
    pub minimum_responses: usize,
    pub required_votes: usize,
}

impl VotingConfig {
    pub fn new(minimum_responses: usize, required_votes: usize) -> Result<Self, String> {
        if minimum_responses == 0 {
            return Err("minimum_responses debe ser mayor que cero".to_string());
        }
        if required_votes == 0 {
            return Err("required_votes debe ser mayor que cero".to_string());
        }
        if required_votes > minimum_responses {
            return Err("required_votes no puede exceder minimum_responses".to_string());
        }

        Ok(Self {
            minimum_responses,
            required_votes,
        })
    }

    pub fn majority(total_workers: usize) -> Result<Self, String> {
        if total_workers == 0 {
            return Err("total_workers debe ser mayor que cero".to_string());
        }

        Self::new(total_workers, (total_workers / 2) + 1)
    }
}

/// Votador determinista de mayoria simple.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SimpleMajorityVoter {
    pub config: VotingConfig,
}

impl SimpleMajorityVoter {
    pub fn new(config: VotingConfig) -> Self {
        Self { config }
    }

    pub fn majority(total_workers: usize) -> Result<Self, String> {
        Ok(Self::new(VotingConfig::majority(total_workers)?))
    }

    pub fn decide(&self, responses: &[WorkerResponse]) -> VotingOutcome {
        let counts = count_votes(responses);
        let false_responses = responses
            .iter()
            .filter(|response| response.is_false())
            .count();

        if responses.len() < self.config.minimum_responses {
            return VotingOutcome {
                decision: VoteDecision::Rejected {
                    reason: "respuestas insuficientes para votar".to_string(),
                },
                counts,
                false_responses,
            };
        }

        let Some(max_votes) = counts.iter().map(|count| count.votes).max() else {
            return VotingOutcome {
                decision: VoteDecision::Rejected {
                    reason: "sin respuestas para votar".to_string(),
                },
                counts,
                false_responses,
            };
        };

        let winners = counts
            .iter()
            .filter(|count| count.votes == max_votes)
            .collect::<Vec<_>>();

        if winners.len() != 1 {
            return VotingOutcome {
                decision: VoteDecision::Rejected {
                    reason: "votacion empatada".to_string(),
                },
                counts,
                false_responses,
            };
        }

        let winner = winners[0];
        if winner.votes < self.config.required_votes {
            return VotingOutcome {
                decision: VoteDecision::Rejected {
                    reason: "sin mayoria suficiente".to_string(),
                },
                counts,
                false_responses,
            };
        }

        VotingOutcome {
            decision: VoteDecision::Accepted {
                value: winner.value.clone(),
                votes: winner.votes,
            },
            counts,
            false_responses,
        }
    }
}

pub fn build_worker_responses(
    worker_ids: &[WorkerId],
    honest_value: impl Into<String>,
    plan: &ByzantineFailurePlan,
) -> Vec<WorkerResponse> {
    let honest_value = honest_value.into();

    worker_ids
        .iter()
        .map(|worker_id| {
            if let Some(false_value) = plan.false_value_for(*worker_id) {
                WorkerResponse::false_response(*worker_id, false_value.to_string())
            } else {
                WorkerResponse::honest(*worker_id, honest_value.clone())
            }
        })
        .collect()
}

fn count_votes(responses: &[WorkerResponse]) -> Vec<VoteCount> {
    let mut counts = BTreeMap::<String, usize>::new();

    for response in responses {
        *counts.entry(response.value.clone()).or_insert(0) += 1;
    }

    counts
        .into_iter()
        .map(|(value, votes)| VoteCount { value, votes })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{
        build_worker_responses, ByzantineFailurePlan, SimpleMajorityVoter, VoteDecision,
        WorkerResponse,
    };

    #[test]
    fn majority_accepts_honest_value_with_one_false_response() {
        let plan = ByzantineFailurePlan::worker_lies(2, "rechazar").expect("plan valido");
        let responses = build_worker_responses(&[0, 1, 2], "aprobar", &plan);
        let voter = SimpleMajorityVoter::majority(3).expect("votador valido");

        let outcome = voter.decide(&responses);

        assert_eq!(outcome.false_responses, 1);
        assert_eq!(
            outcome.decision,
            VoteDecision::Accepted {
                value: "aprobar".to_string(),
                votes: 2,
            }
        );
    }

    #[test]
    fn voting_rejects_tie() {
        let responses = vec![
            WorkerResponse::honest(0, "aprobar"),
            WorkerResponse::honest(1, "rechazar"),
        ];
        let voter =
            SimpleMajorityVoter::new(super::VotingConfig::new(2, 2).expect("configuracion valida"));

        let outcome = voter.decide(&responses);

        assert!(matches!(outcome.decision, VoteDecision::Rejected { .. }));
    }

    #[test]
    fn voting_rejects_insufficient_responses() {
        let responses = vec![WorkerResponse::honest(0, "aprobar")];
        let voter = SimpleMajorityVoter::majority(3).expect("votador valido");

        let outcome = voter.decide(&responses);

        assert!(matches!(outcome.decision, VoteDecision::Rejected { .. }));
    }
}
