//! Pure terminal client reduction tests.

use animus_live_terminal::{
    ApprovalDecision, ChildAction, CleanupObservation, ClientCommand, ClientSessionEvent,
    ClientSnapshot, ControlIntent, SessionAction, TerminalClientState, TerminalCoreError,
    TerminalExit, TerminalInput, TerminalIteration, TerminalLimits, TerminalReceiptFacts,
    command_from_control, reduce_iteration,
};

const ROOT: &str = "root-terminal";
const WORKER: &str = "worker-terminal";
const HASH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const CONFIG_HASH: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const MAX_EVENTS: u32 = 4;
const MAX_PROMPT_BYTES: u32 = 32;
const MAX_WIDTH: u16 = 160;
const MAX_HEIGHT: u16 = 80;
const FULL_CLEANUP_ACTIONS: u32 = 5;
const ALTERNATE_CLEANUP_ACTIONS: u32 = 4;
const MAX_LABEL_BYTES: usize = 256;

#[test]
fn snapshot_input_runtime_events_commands_and_receipt_are_bounded() {
    let mut state = state();
    let plan = reduce_iteration(
        &mut state,
        &TerminalIteration {
            input: Some(TerminalInput::Character('h')),
            session_events: &[ClientSessionEvent::Runtime(
                animus_runtime::RuntimeEvent::SessionStateChanged {
                    session_id: String::from("session-terminal"),
                    run_id: Some(String::from("run-terminal")),
                    status: String::from("streaming"),
                    queue_count: 1,
                },
            )],
            cancelled: false,
            signalled: false,
        },
        limits(),
    )
    .expect("first iteration");
    assert!(plan.command.is_none());
    assert!(plan.render);
    assert_eq!(state.view().coding_session().state().queue_count, 1);
    assert_eq!(state.view().prompt().content(), "h");

    reduce_iteration(
        &mut state,
        &TerminalIteration {
            input: Some(TerminalInput::Character('i')),
            session_events: &[],
            cancelled: false,
            signalled: false,
        },
        limits(),
    )
    .expect("second iteration");
    let submitted = reduce_iteration(
        &mut state,
        &TerminalIteration {
            input: Some(TerminalInput::Submit),
            session_events: &[],
            cancelled: false,
            signalled: false,
        },
        limits(),
    )
    .expect("submit");
    assert!(matches!(
        submitted.command,
        Some(ClientCommand::Prompt { ref text, .. }) if text == "hi"
    ));

    state.record_rendered().expect("render count");
    let receipt = state
        .receipt(TerminalReceiptFacts {
            config_identity: CONFIG_HASH,
            connection: "direct",
            cleanup: CleanupObservation {
                completed_transitions: FULL_CLEANUP_ACTIONS,
                attempted_actions: FULL_CLEANUP_ACTIONS,
                completed_actions: FULL_CLEANUP_ACTIONS,
                failed_actions: 0,
            },
            exit: TerminalExit::Normal,
        })
        .expect("metadata receipt");
    let alternate_cleanup = state
        .receipt(TerminalReceiptFacts {
            config_identity: CONFIG_HASH,
            connection: "direct",
            cleanup: CleanupObservation {
                completed_transitions: ALTERNATE_CLEANUP_ACTIONS,
                attempted_actions: ALTERNATE_CLEANUP_ACTIONS,
                completed_actions: ALTERNATE_CLEANUP_ACTIONS,
                failed_actions: 0,
            },
            exit: TerminalExit::Normal,
        })
        .expect("alternate complete cleanup");
    assert_ne!(receipt.receipt_identity, alternate_cleanup.receipt_identity);
    let rendered = receipt.render();
    assert!(rendered.contains("connection=direct"));
    assert!(rendered.contains("surface=not-production-readiness"));
    assert!(!rendered.contains("hi"));
    assert!(!rendered.contains(ROOT));
    assert!(!rendered.contains(WORKER));
}

#[test]
fn controls_emit_all_typed_descriptors_without_effect_execution() {
    let mut state = state();
    let commands = [
        command_from_control(
            &mut state,
            ControlIntent::QueueEdit {
                item_id: String::from("queue-one"),
                expected_generation: 1,
                expected_queue_identity: String::from(HASH),
                replacement: String::from("private replacement"),
            },
            MAX_PROMPT_BYTES,
        )
        .expect("queue edit"),
        command_from_control(
            &mut state,
            ControlIntent::QueueRemove {
                item_id: String::from("queue-one"),
                expected_generation: 1,
                expected_queue_identity: String::from(HASH),
            },
            MAX_PROMPT_BYTES,
        )
        .expect("queue remove"),
        command_from_control(
            &mut state,
            ControlIntent::Approval {
                correlation_id: String::from(HASH),
                decision: ApprovalDecision::Approve,
            },
            MAX_PROMPT_BYTES,
        )
        .expect("approval"),
        command_from_control(&mut state, ControlIntent::Cancel, MAX_PROMPT_BYTES).expect("cancel"),
        command_from_control(
            &mut state,
            ControlIntent::Child {
                child_id: String::from("child-one"),
                action: ChildAction::Inspect,
            },
            MAX_PROMPT_BYTES,
        )
        .expect("child"),
        command_from_control(
            &mut state,
            ControlIntent::Session(SessionAction::Stop),
            MAX_PROMPT_BYTES,
        )
        .expect("session"),
    ];
    assert_eq!(
        commands.map(|command| command.class()),
        [
            "queue_edit",
            "queue_remove",
            "approval",
            "cancel",
            "child",
            "session",
        ]
    );
}

#[test]
fn queue_conflict_requests_fresh_snapshot_only_through_client_command() {
    let mut state = state();
    let plan = reduce_iteration(
        &mut state,
        &TerminalIteration {
            input: None,
            session_events: &[ClientSessionEvent::QueueConflict {
                generation: 2,
                queue_identity: String::from(HASH),
            }],
            cancelled: false,
            signalled: false,
        },
        limits(),
    )
    .expect("conflict reduction");
    assert!(matches!(
        plan.command,
        Some(ClientCommand::Session {
            action: SessionAction::RefreshSnapshot,
            ..
        })
    ));
    assert!(state.view().queue_conflict().is_none());
}

#[test]
fn event_flood_stale_sequence_oversized_prompt_and_cleanup_overclaim_fail_closed() {
    let mut flooded = state();
    let events = vec![
        ClientSessionEvent::Failed {
            class: String::from("bounded_failure"),
        };
        usize::try_from(MAX_EVENTS).expect("event bound") + 1
    ];
    assert_eq!(
        reduce_iteration(
            &mut flooded,
            &TerminalIteration {
                input: None,
                session_events: events.as_slice(),
                cancelled: false,
                signalled: false,
            },
            limits(),
        ),
        Err(TerminalCoreError::EventFlood)
    );

    let mut stale = state();
    assert_eq!(
        reduce_iteration(
            &mut stale,
            &TerminalIteration {
                input: None,
                session_events: &[ClientSessionEvent::Observation {
                    sequence: 0,
                    class: String::from("streaming"),
                    observation_identity: String::from(HASH),
                }],
                cancelled: false,
                signalled: false,
            },
            limits(),
        ),
        Err(TerminalCoreError::Sequence)
    );

    let mut prompt = state();
    for _character in 0..MAX_PROMPT_BYTES {
        reduce_iteration(
            &mut prompt,
            &TerminalIteration {
                input: Some(TerminalInput::Character('x')),
                session_events: &[],
                cancelled: false,
                signalled: false,
            },
            limits(),
        )
        .expect("within prompt bound");
    }
    assert_eq!(
        reduce_iteration(
            &mut prompt,
            &TerminalIteration {
                input: Some(TerminalInput::Character('x')),
                session_events: &[],
                cancelled: false,
                signalled: false,
            },
            limits(),
        ),
        Err(TerminalCoreError::Bound)
    );

    assert_eq!(
        prompt.receipt(TerminalReceiptFacts {
            config_identity: CONFIG_HASH,
            connection: "fallback",
            cleanup: CleanupObservation {
                completed_transitions: FULL_CLEANUP_ACTIONS,
                attempted_actions: FULL_CLEANUP_ACTIONS,
                completed_actions: FULL_CLEANUP_ACTIONS,
                failed_actions: 0,
            },
            exit: TerminalExit::Failed,
        }),
        Err(TerminalCoreError::Identity)
    );
    assert_eq!(
        prompt.receipt(TerminalReceiptFacts {
            config_identity: CONFIG_HASH,
            connection: "direct",
            cleanup: CleanupObservation {
                completed_transitions: FULL_CLEANUP_ACTIONS,
                attempted_actions: FULL_CLEANUP_ACTIONS,
                completed_actions: ALTERNATE_CLEANUP_ACTIONS,
                failed_actions: 1,
            },
            exit: TerminalExit::Failed,
        }),
        Err(TerminalCoreError::Cleanup)
    );
}

#[test]
fn overlong_event_labels_fail_before_view_projection() {
    let mut overlong = state();
    assert_eq!(
        reduce_iteration(
            &mut overlong,
            &TerminalIteration {
                input: None,
                session_events: &[ClientSessionEvent::Observation {
                    sequence: 1,
                    class: "a".repeat(MAX_LABEL_BYTES + 1),
                    observation_identity: String::from(HASH),
                }],
                cancelled: false,
                signalled: false,
            },
            limits(),
        ),
        Err(TerminalCoreError::Identity)
    );
}

fn state() -> TerminalClientState {
    TerminalClientState::from_snapshot(
        &ClientSnapshot {
            root_id: String::from(ROOT),
            worker_id: String::from(WORKER),
            generation: 1,
            event_sequence: 0,
            restore_status: animus_local_daemon::RestoreStatus::Restored,
            events: Vec::new(),
        },
        (80, 24),
        limits(),
    )
    .expect("snapshot")
}

const fn limits() -> TerminalLimits {
    TerminalLimits {
        event_batch: MAX_EVENTS,
        prompt_bytes: MAX_PROMPT_BYTES,
        width: MAX_WIDTH,
        height: MAX_HEIGHT,
    }
}
