//! Pure coding-session event adapter for TUI hosts.

/// TUI-owned wrapper around the shared coding-session frontend reducer.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CodingSessionRenderModel {
    state: animus_runtime::coding_session_host::CodingSessionFrontendState,
}

impl CodingSessionRenderModel {
    /// Creates an idle coding-session render model.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Applies one provider/UI-neutral runtime event without transcript mutation authority.
    pub fn apply_runtime_event(&mut self, event: &animus_runtime::RuntimeEvent) {
        self.state = animus_runtime::coding_session_host::reduce_coding_session_frontend_event(
            &self.state,
            event,
        );
    }

    /// Returns the shared metadata-only frontend state.
    #[must_use]
    pub const fn state(&self) -> &animus_runtime::coding_session_host::CodingSessionFrontendState {
        &self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SESSION_ID: &str = "session.frontend";
    const RUN_ID: &str = "run.frontend";
    const ENTRY_ID: &str = "entry.frontend";
    const TOOL_NAME: &str = "frontend_tool";
    const TOOL_SCHEMA: &str = "schema.frontend-tool.v1";
    const TOOL_ARGUMENT: &str = "query";
    const TOOL_ARGUMENT_VALUE: &str = "bounded";
    const TOOL_CALL_ID: &str = "call.frontend";
    const TOOL_TARGET: &str = "resource.frontend";
    const TOOL_CAPABILITY: &str = "tool.read";
    const CANCELLATION_REASON: &str = "correlated cancellation observed";
    const HEAD_HASH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const QUEUE_IDENTITY: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    const PRIVATE_QUEUE_TEXT: &str = "private queued prompt";
    const FIRST_CURSOR: u64 = 1;
    const FIRST_COMMITTED_MESSAGE_COUNT: u32 = 1;
    const FIRST_QUEUE_COUNT: u32 = 1;
    const TOOL_VISIBLE_CHARS: u32 = 128;
    const EXPECTED_EVENT_COUNT: usize = 5;

    #[test]
    fn headless_and_tui_consume_identical_session_event_order() {
        // r[verify agent_harness_runtime.coding_session_host.events]
        // r[verify agent_harness_runtime.coding_session_host.conformance]
        let events = vec![
            animus_runtime::RuntimeEvent::SessionStateChanged {
                session_id: String::from(SESSION_ID),
                run_id: Some(String::from(RUN_ID)),
                status: String::from("steering"),
                queue_count: FIRST_QUEUE_COUNT,
            },
            animus_runtime::RuntimeEvent::SessionCommitObserved {
                session_id: String::from(SESSION_ID),
                run_id: String::from(RUN_ID),
                entry_id: String::from(ENTRY_ID),
                message_kind: String::from("user"),
                head_hash: String::from(HEAD_HASH),
                durable_cursor: FIRST_CURSOR,
            },
            animus_runtime::RuntimeEvent::ToolRequested {
                request: tool_request(),
            },
            animus_runtime::RuntimeEvent::Cancelled {
                reason: String::from(CANCELLATION_REASON),
            },
            animus_runtime::RuntimeEvent::Shutdown {
                status: animus_runtime::RuntimeTerminalKind::FinalResponse,
            },
        ];
        let headless = animus_runtime::map_runtime_events_to_headless(events.as_slice())
            .expect("headless projection");
        let mut tui = CodingSessionRenderModel::new();
        for event in &events {
            tui.apply_runtime_event(event);
        }

        assert_eq!(headless.len(), EXPECTED_EVENT_COUNT);
        assert_eq!(
            headless[0].kind(),
            animus_runtime::RuntimeEventKind::SessionStateChanged
        );
        assert_eq!(tui.state().queue_count, FIRST_QUEUE_COUNT);
        assert_eq!(
            tui.state().committed_message_count,
            FIRST_COMMITTED_MESSAGE_COUNT
        );
        assert_eq!(
            tui.state().active_tool_correlation.as_deref(),
            Some(TOOL_CALL_ID)
        );
        assert!(tui.state().cancellation_observed);
        assert_eq!(
            tui.state().terminal_status.as_deref(),
            Some("final_response")
        );
    }

    fn tool_request() -> animus_adapters::ToolRequest {
        let arguments = animus_adapters::tool::PreparedArguments::prepare(
            &animus_adapters::tool::ArgumentPreparationRequest {
                current_schema_identity: TOOL_SCHEMA,
                allowed_field_names: &[TOOL_ARGUMENT],
                compatibility_rules: &[],
                fields: &[animus_adapters::tool::ArgumentFieldDraft {
                    name: Some(TOOL_ARGUMENT),
                    value: Some(TOOL_ARGUMENT_VALUE),
                }],
            },
        )
        .expect("frontend tool arguments");
        assert_eq!(arguments.schema_identity(), TOOL_SCHEMA);
        assert_eq!(arguments.fields().len(), 1);
        animus_adapters::ToolRequest::from_draft(&animus_adapters::tool::RequestDraft {
            name: Some(TOOL_NAME),
            arguments: Some(&arguments),
            correlation_id: Some(TOOL_CALL_ID),
            output_policy: animus_adapters::tool::OutputPolicy {
                visible_char_budget: TOOL_VISIBLE_CHARS,
            },
            mutation_targets: &[TOOL_TARGET],
            required_capability_labels: &[TOOL_CAPABILITY],
            approval_posture: animus_adapters::tool::ApprovalPosture::PreApproved.as_str(),
        })
        .expect("frontend tool request")
    }

    #[test]
    fn queue_mutation_frontend_mapping_uses_metadata_only() {
        // r[verify agent_harness_runtime.coding_session_queue_cas.verification]
        let mut model = CodingSessionRenderModel::new();
        model.apply_runtime_event(
            &animus_runtime::RuntimeEvent::SessionQueueMutationObserved {
                mutation: animus_runtime::coding_session_host::CodingSessionQueueMutationEvent {
                    operation: animus_runtime::coding_session_host::CodingSessionQueueMutationOperation::Remove,
                    status: "conflict",
                    item_id: String::from("queue.frontend"),
                    entry_id: String::from(ENTRY_ID),
                    position: Some(0),
                    generation: animus_runtime::coding_session_host::CodingSessionQueueGeneration::new(1),
                    queue_identity: String::from(QUEUE_IDENTITY),
                    conflict: Some(
                        animus_runtime::coding_session_host::CodingSessionQueueConflictKind::StaleGeneration,
                    ),
                    redaction_marker: "private_queue_text_omitted",
                },
            },
        );
        assert_eq!(model.state().queue_generation.value(), 1);
        assert_eq!(
            model.state().queue_identity.as_deref(),
            Some(QUEUE_IDENTITY)
        );
        assert_eq!(
            model.state().queue_mutation_status.as_deref(),
            Some("conflict")
        );
        assert_eq!(
            model.state().queue_conflict,
            Some(
                animus_runtime::coding_session_host::CodingSessionQueueConflictKind::StaleGeneration
            )
        );
        assert!(!format!("{:?}", model.state()).contains(PRIVATE_QUEUE_TEXT));
    }

    #[test]
    fn transient_progress_does_not_create_committed_transcript_boundaries() {
        let mut model = CodingSessionRenderModel::new();
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::SessionStateChanged {
            session_id: String::from(SESSION_ID),
            run_id: Some(String::from(RUN_ID)),
            status: String::from("provider_dispatched"),
            queue_count: 0,
        });
        assert_eq!(model.state().committed_message_count, 0);
        assert_eq!(model.state().queue_count, 0);
    }
}
