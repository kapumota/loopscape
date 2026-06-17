use loopscape::core::byzantine::{
    build_worker_responses, ByzantineFailurePlan, SimpleMajorityVoter, VoteDecision, WorkerResponse,
};

#[test]
fn byzantine_majority_accepts_honest_value() {
    let plan = ByzantineFailurePlan::worker_lies(1, "resultado_falso").expect("plan valido");
    let responses = build_worker_responses(&[0, 1, 2], "resultado_correcto", &plan);
    let voter = SimpleMajorityVoter::majority(3).expect("votador valido");

    let outcome = voter.decide(&responses);

    assert_eq!(outcome.false_responses, 1);
    assert_eq!(
        outcome.decision,
        VoteDecision::Accepted {
            value: "resultado_correcto".to_string(),
            votes: 2,
        }
    );
}

#[test]
fn byzantine_majority_rejects_split_vote() {
    let responses = vec![
        WorkerResponse::honest(0, "a"),
        WorkerResponse::false_response(1, "b"),
        WorkerResponse::honest(2, "c"),
    ];
    let voter = SimpleMajorityVoter::majority(3).expect("votador valido");

    let outcome = voter.decide(&responses);

    assert!(matches!(outcome.decision, VoteDecision::Rejected { .. }));
}

#[test]
fn byzantine_plan_can_hold_multiple_false_workers() {
    let plan = ByzantineFailurePlan::worker_lies(1, "falso_a")
        .expect("plan valido")
        .with_failure(
            loopscape::core::byzantine::ByzantineFailureSpec::new(3, "falso_b")
                .expect("fallo valido"),
        );
    let responses = build_worker_responses(&[0, 1, 2, 3, 4], "verdadero", &plan);
    let voter = SimpleMajorityVoter::majority(5).expect("votador valido");

    let outcome = voter.decide(&responses);

    assert_eq!(outcome.false_responses, 2);
    assert_eq!(
        outcome.decision,
        VoteDecision::Accepted {
            value: "verdadero".to_string(),
            votes: 3,
        }
    );
}
