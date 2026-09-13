//! Retry controls call the production adapter, not the blocked outer reducer.

use animus_std::coding_session_host::CodingSessionProviderService;

const FINAL_STREAM: &[u8] =
    include_bytes!("../../../../animus-std-openai/tests/fixtures/openai-responses-final.sse");
const MALFORMED_STREAM: &[u8] = b"not an SSE completion";

#[derive(Clone, Copy)]
enum Refusal {
    Snapshot,
    Provider,
    Terminal,
}

#[test]
fn snapshot_refusal_prevents_a_second_adapter_dispatch() {
    assert_retry_refused(Refusal::Snapshot);
}

#[test]
fn provider_failure_prevents_a_second_adapter_dispatch() {
    assert_retry_refused(Refusal::Provider);
}

#[test]
fn terminal_refusal_prevents_a_second_adapter_dispatch() {
    assert_retry_refused(Refusal::Terminal);
}

#[test]
fn a_complete_continuation_preserves_the_final_reply() {
    let (mut adapter, request) = ready_adapter(FINAL_STREAM);
    let snapshots = adapter.snapshot_store.committed.len();
    let terminals = adapter.snapshot_store.terminal.len();
    assert!(matches!(
        adapter.dispatch(&request),
        Ok(animus_runtime::coding_session_host::CodingSessionProviderReply::Final(_))
    ));
    assert_eq!(
        adapter
            .provider
            .remaining()
            .expect("remaining stream count"),
        1
    );
    assert_eq!(
        adapter.snapshot_store.committed.len(),
        snapshots.checked_add(1).expect("snapshot count")
    );
    assert_eq!(
        adapter.snapshot_store.terminal.len(),
        terminals.checked_add(1).expect("terminal count")
    );
    assert!(!adapter.shared.borrow().is_blocked());
}

fn assert_retry_refused(refusal: Refusal) {
    let bytes = match refusal {
        Refusal::Provider => MALFORMED_STREAM,
        Refusal::Snapshot | Refusal::Terminal => FINAL_STREAM,
    };
    let (mut adapter, request) = ready_adapter(bytes);
    adapter.snapshot_store.refuse_snapshot = matches!(refusal, Refusal::Snapshot);
    adapter.snapshot_store.refuse_terminal = matches!(refusal, Refusal::Terminal);
    let before = adapter.provider.remaining().expect("remaining streams");
    assert!(matches!(
        adapter.dispatch(&request),
        Err(animus_runtime::coding_session_host::CodingSessionServiceIssue::Failed)
    ));
    let remaining = adapter
        .provider
        .remaining()
        .expect("remaining streams after failure");
    if matches!(refusal, Refusal::Snapshot) {
        assert_eq!(remaining, before);
    } else {
        assert_eq!(remaining, before.checked_sub(1).expect("one provider call"));
    }
    adapter.snapshot_store.refuse_snapshot = false;
    adapter.snapshot_store.refuse_terminal = false;
    let snapshots = adapter.snapshot_store.committed.len();
    let terminals = adapter.snapshot_store.terminal.len();
    assert!(matches!(
        adapter.dispatch(&request),
        Err(animus_runtime::coding_session_host::CodingSessionServiceIssue::Stale)
    ));
    assert_eq!(
        adapter
            .provider
            .remaining()
            .expect("remaining streams after retry"),
        remaining
    );
    assert_eq!(adapter.snapshot_store.committed.len(), snapshots);
    assert_eq!(adapter.snapshot_store.terminal.len(), terminals);
    assert!(adapter.shared.borrow().is_blocked());
}

fn ready_adapter(
    bytes: &[u8],
) -> (
    super::super::dispatch::ProviderAdapter<
        crate::FixtureStreamProvider,
        super::persistence::Snapshots,
    >,
    animus_runtime::coding_session_host::CodingSessionProviderRequest,
) {
    let mut session = super::pending_tool_result_session();
    session
        .drive_one()
        .expect("commit and observe the tool result");
    let request = session
        .host
        .state_snapshot()
        .pending_effect()
        .and_then(|effect| match effect {
            animus_runtime::coding_session_host::CodingSessionEffectRequest::Provider(request) => {
                Some(request.clone())
            }
            _ => None,
        })
        .expect("continuation provider request");
    let mut adapter = session.host.into_services().provider;
    adapter.provider = crate::FixtureStreamProvider::new(
        [bytes.to_vec(), FINAL_STREAM.to_vec()],
        adapter.config.max_provider_response_bytes(),
    );
    (adapter, request)
}
