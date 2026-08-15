//! Positive and negative fixtures for the agent watch-graph family.
//!
//! Negative cases: cycle-closing enables, unknown agents, duplicate edges,
//! stale generations, missing edges, malformed retry progress, unknown status
//! inputs, and receipt material leaks.

use alloc::vec;
use alloc::vec::Vec;

use super::graph::{AgentLabel, WatchAction, WatchDenial, WatchGraph};
use super::receipt::{
    WatchReceipt, build_watch_receipt, validate_watch_evidence_text, watch_receipt_line,
};
use super::status::{
    AgentStatusInput, RetryErrorClass, SanitizedRetryProgress, WatchStatus, project_watcher_status,
};
use super::validate::{
    validate_known_agents, validate_status_generation, validate_watch_graph, watch_graph_is_acyclic,
};

fn agent(value: &str) -> AgentLabel {
    AgentLabel::parse(value).expect("valid agent label")
}

fn known_agents() -> Vec<AgentLabel> {
    vec![agent("alpha"), agent("beta"), agent("gamma")]
}

fn enable(graph: &WatchGraph, watcher: &str, target: &str, generation: u64) -> WatchGraph {
    let known = known_agents();
    match super::apply_watch_action(
        graph,
        &WatchAction::Enable {
            watcher: agent(watcher),
            target: agent(target),
            generation,
        },
        &known,
    ) {
        super::WatchResult::Applied(next) => *next,
        super::WatchResult::Rejected(denial) => panic!("unexpected rejection: {denial:?}"),
    }
}

#[test]
fn positive_enables_build_an_acyclic_watch_graph() {
    // r[verify agent_harness_runtime.agent_watch.verification]
    let empty = WatchGraph::default();
    let one = enable(&empty, "alpha", "beta", 1);
    let two = enable(&one, "beta", "gamma", 2);
    assert_eq!(two.edges().len(), 2);
    assert!(watch_graph_is_acyclic(&two));
    assert!(validate_watch_graph(&two, &known_agents()).is_ok());
    assert_eq!(two.identity().len(), 67); // b3: + 64 hex
}

#[test]
fn positive_transition_rejection_leaves_state_unchanged() {
    // r[verify agent_harness_runtime.agent_watch.acyclic]
    let empty = WatchGraph::default();
    let one = enable(&empty, "alpha", "beta", 1);
    let closed = super::apply_watch_action(
        &one,
        &WatchAction::Enable {
            watcher: agent("beta"),
            target: agent("alpha"),
            generation: 2,
        },
        &known_agents(),
    );
    assert!(
        matches!(
            closed,
            super::WatchResult::Rejected(WatchDenial::CycleWouldForm)
        ),
        "cycle-closing enable must be rejected"
    );
    // The reject must not change the graph.
    let unchanged = match &closed {
        super::WatchResult::Rejected(_) => &one,
        super::WatchResult::Applied(_) => panic!("expected rejection"),
    };
    assert!(matches!(
        super::apply_watch_action(
            unchanged,
            &WatchAction::Enable {
                watcher: agent("beta"),
                target: agent("alpha"),
                generation: 2,
            },
            &known_agents(),
        ),
        super::WatchResult::Rejected(WatchDenial::CycleWouldForm)
    ));
}

#[test]
fn positive_disable_removes_only_the_named_edge() {
    // r[verify agent_harness_runtime.agent_watch.verification]
    let empty = WatchGraph::default();
    let one = enable(&empty, "alpha", "beta", 1);
    let result = super::apply_watch_action(
        &one,
        &WatchAction::Disable {
            watcher: agent("alpha"),
            target: agent("beta"),
            generation: 2,
        },
        &known_agents(),
    );
    let next = match result {
        super::WatchResult::Applied(next) => *next,
        super::WatchResult::Rejected(denial) => panic!("unexpected rejection: {denial:?}"),
    };
    assert!(next.is_empty());
}

#[test]
fn positive_watcher_status_projection_is_watcher_only() {
    // r[verify agent_harness_runtime.agent_watch.status]
    let empty = WatchGraph::default();
    let graph = enable(&empty, "alpha", "beta", 1);
    let progress = SanitizedRetryProgress {
        attempt: 2,
        remaining: 3,
        last_error_class: Some(RetryErrorClass::RateLimited),
    };
    let inputs = vec![
        AgentStatusInput {
            agent_id: agent("alpha"),
            status: WatchStatus::Working,
            retry: None,
            generation: 1,
        },
        AgentStatusInput {
            agent_id: agent("beta"),
            status: WatchStatus::Blocked,
            retry: Some(progress),
            generation: 1,
        },
    ];
    let views = project_watcher_status(&graph, &inputs).expect("projection");
    assert_eq!(views.len(), 1);
    assert_eq!(views[0].agent_id.as_str(), "beta");
    assert_eq!(views[0].status, WatchStatus::Blocked);
    assert!(views[0].retry.is_some());
}

#[test]
fn positive_receipt_is_metadata_only() {
    // r[verify agent_harness_runtime.agent_watch.receipts]
    let empty = WatchGraph::default();
    let graph = enable(&empty, "alpha", "beta", 1);
    let receipt: WatchReceipt = build_watch_receipt(&graph.identity(), 1, 1, None, 3);
    let line = watch_receipt_line(&receipt);
    assert_eq!(receipt.edge_count, 1);
    assert_eq!(receipt.view_count, 1);
    assert!(line.starts_with('b'));
    assert!(validate_watch_evidence_text(&line).is_ok());
    assert!(
        validate_watch_evidence_text("reasoning text and /home/ leaks raw prompt").is_err(),
        "evidence text with raw material must be rejected"
    );
}

#[test]
fn negative_unknown_agents_are_rejected() {
    // r[verify agent_harness_runtime.agent_watch.fail_closed]
    let empty = WatchGraph::default();
    let result = super::apply_watch_action(
        &empty,
        &WatchAction::Enable {
            watcher: agent("alpha"),
            target: agent("ghost"),
            generation: 1,
        },
        &known_agents(),
    );
    assert_eq!(
        result,
        super::WatchResult::Rejected(WatchDenial::UnknownAgent)
    );
}

#[test]
fn negative_duplicate_and_stale_edges_are_rejected() {
    // r[verify agent_harness_runtime.agent_watch.fail_closed]
    let empty = WatchGraph::default();
    let one = enable(&empty, "alpha", "beta", 2);
    let duplicate = super::apply_watch_action(
        &one,
        &WatchAction::Enable {
            watcher: agent("alpha"),
            target: agent("beta"),
            generation: 2,
        },
        &known_agents(),
    );
    assert_eq!(
        duplicate,
        super::WatchResult::Rejected(WatchDenial::DuplicateEdge)
    );
    let stale = super::apply_watch_action(
        &one,
        &WatchAction::Enable {
            watcher: agent("alpha"),
            target: agent("beta"),
            generation: 1,
        },
        &known_agents(),
    );
    assert_eq!(
        stale,
        super::WatchResult::Rejected(WatchDenial::StaleGeneration)
    );
}

#[test]
fn negative_missing_disable_edge_is_rejected() {
    // r[verify agent_harness_runtime.agent_watch.fail_closed]
    let empty = WatchGraph::default();
    let result = super::apply_watch_action(
        &empty,
        &WatchAction::Disable {
            watcher: agent("alpha"),
            target: agent("beta"),
            generation: 1,
        },
        &known_agents(),
    );
    assert_eq!(
        result,
        super::WatchResult::Rejected(WatchDenial::MissingEdge)
    );
}

#[test]
fn negative_malformed_status_and_generation_regressions_fail_closed() {
    // r[verify agent_harness_runtime.agent_watch.fail_closed]
    let empty = WatchGraph::default();
    let graph = enable(&empty, "alpha", "beta", 1);
    // Retry record with attempt=0 is malformed.
    let bad_progress = SanitizedRetryProgress {
        attempt: 0,
        remaining: 1,
        last_error_class: None,
    };
    let malformed_input = AgentStatusInput {
        agent_id: agent("beta"),
        status: WatchStatus::Working,
        retry: Some(bad_progress),
        generation: 1,
    };
    assert!(project_watcher_status(&graph, &[malformed_input]).is_err());
    // A watcher target without a status input fails closed.
    assert!(project_watcher_status(&graph, &[]).is_err());
    // A generation regression is rejected.
    assert_eq!(
        validate_status_generation(2, 1),
        Err(super::validate::WatchValidationIssue::NonMonotonicGeneration)
    );
    // Empty known-agent set is rejected.
    assert_eq!(
        validate_known_agents(&[]),
        Err(super::validate::WatchValidationIssue::EmptyKnownAgentSet)
    );
}
