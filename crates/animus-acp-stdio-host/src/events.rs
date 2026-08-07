//! Pure bounded projection from shared runtime events to ACP updates.

#![allow(missing_docs)]

use crate::identities::{AcpCorrelationId, AcpSessionId};
use crate::model::{
    AcpFailureClass, AcpProtocolError, AcpTransportProfile, AcpUpdate, AcpUpdateKind,
};

const FIELD_EVENT: &str = "event";
const FIELD_UPDATE_BOUND: &str = "update_bound";
const FIRST_SEQUENCE: u32 = 1;

/// # Errors
/// Returns a typed projection error when event content or configured bounds are unsafe.
// r[impl agent_harness_runtime.acp_stdio_host.events]
pub fn project_runtime_events(
    session_id: &str,
    events: &[animus_runtime::RuntimeEvent],
    profile: AcpTransportProfile,
) -> Result<Vec<AcpUpdate>, AcpProtocolError> {
    let mut updates = Vec::with_capacity(events.len());
    let mut projected_bytes = 0_usize;
    let mut active_tool_correlation = None;
    for event in events {
        let sequence = u32::try_from(updates.len())
            .map_err(|_error| projection_error(AcpFailureClass::Backpressure, FIELD_UPDATE_BOUND))?
            .saturating_add(FIRST_SEQUENCE);
        let update = project_event(
            session_id,
            sequence,
            event,
            active_tool_correlation.as_deref(),
        )?;
        projected_bytes = projected_bytes.saturating_add(update.estimated_bytes());
        if updates.len() >= profile.updates_per_operation
            || projected_bytes > profile.update_bytes_per_operation
        {
            return Err(projection_error(
                AcpFailureClass::Backpressure,
                FIELD_UPDATE_BOUND,
            ));
        }
        match update.kind {
            AcpUpdateKind::ToolStarted => {
                active_tool_correlation.clone_from(&update.correlation_identity);
            }
            AcpUpdateKind::ToolCompleted | AcpUpdateKind::ToolDenied => {
                active_tool_correlation = None;
            }
            _other => {}
        }
        updates.push(update);
    }
    debug_assert!(updates.len() <= profile.updates_per_operation);
    Ok(updates)
}

fn project_event(
    session_id: &str,
    sequence: u32,
    event: &animus_runtime::RuntimeEvent,
    active_tool_correlation: Option<&str>,
) -> Result<AcpUpdate, AcpProtocolError> {
    validate_safe_session_id(session_id)?;
    let update = project_model_or_tool_event(session_id, sequence, event, active_tool_correlation)
        .or_else(|| project_session_event(session_id, sequence, event))
        .unwrap_or_else(|| {
            update(
                sequence,
                session_id,
                UpdateFields::new(AcpUpdateKind::Diagnostic, event.kind().as_str()),
            )
        });
    validate_update(&update)?;
    Ok(update)
}

fn project_model_or_tool_event(
    session_id: &str,
    sequence: u32,
    event: &animus_runtime::RuntimeEvent,
    active_tool_correlation: Option<&str>,
) -> Option<AcpUpdate> {
    let fields = match event {
        animus_runtime::RuntimeEvent::ModelStreamFrame { frame }
            if frame.is_default_final_visible() =>
        {
            UpdateFields::new(AcpUpdateKind::AgentMessage, "visible_content")
                .safe_text(frame.body().map(String::from))
                .correlation(Some(frame.call_id().as_str()))
        }
        animus_runtime::RuntimeEvent::ModelResponded { response }
        | animus_runtime::RuntimeEvent::FinalResponse { response } => {
            UpdateFields::new(AcpUpdateKind::AgentMessage, "visible_content")
                .safe_text(Some(String::from(response.message())))
        }
        animus_runtime::RuntimeEvent::ToolRequested { request } => {
            UpdateFields::new(AcpUpdateKind::ToolStarted, "pending")
                .safe_text(Some(String::from(request.name())))
                .correlation(Some(request.correlation_id()))
        }
        animus_runtime::RuntimeEvent::ToolResponded { .. } => {
            UpdateFields::new(AcpUpdateKind::ToolCompleted, "completed_output_omitted")
                .correlation(active_tool_correlation)
        }
        animus_runtime::RuntimeEvent::ToolDenied { request, .. } => {
            UpdateFields::new(AcpUpdateKind::ToolDenied, "denied")
                .safe_text(Some(String::from(request.name())))
                .correlation(Some(request.correlation_id()))
        }
        animus_runtime::RuntimeEvent::ModelStreamFrame { .. } => UpdateFields::new(
            AcpUpdateKind::Redacted,
            "hidden_or_non_content_stream_frame_omitted",
        ),
        animus_runtime::RuntimeEvent::ModelRequested { .. } => {
            UpdateFields::new(AcpUpdateKind::Redacted, "provider_request_omitted")
        }
        _other => return None,
    };
    Some(update(sequence, session_id, fields))
}

fn project_session_event(
    session_id: &str,
    sequence: u32,
    event: &animus_runtime::RuntimeEvent,
) -> Option<AcpUpdate> {
    let fields = match event {
        animus_runtime::RuntimeEvent::SessionStateChanged {
            run_id,
            status,
            queue_count,
            ..
        } => UpdateFields::new(AcpUpdateKind::SessionStatus, status)
            .safe_text(Some(format!("queue_count={queue_count}")))
            .correlation(run_id.as_deref()),
        animus_runtime::RuntimeEvent::SessionCommitObserved {
            run_id,
            message_kind,
            head_hash,
            ..
        } => UpdateFields::new(AcpUpdateKind::DurableBoundary, run_id)
            .safe_text(Some(message_kind.clone()))
            .correlation(Some(head_hash.as_str())),
        animus_runtime::RuntimeEvent::SessionRecoveryObserved {
            status, head_hash, ..
        } => UpdateFields::new(AcpUpdateKind::DurableBoundary, status)
            .correlation(Some(head_hash.as_str())),
        animus_runtime::RuntimeEvent::SessionAuthorityProfileObserved {
            status,
            profile_identity,
            ..
        } => UpdateFields::new(AcpUpdateKind::AuthorityStatus, status)
            .correlation(Some(profile_identity.as_str())),
        animus_runtime::RuntimeEvent::Cancelled { .. } => {
            UpdateFields::new(AcpUpdateKind::Cancellation, "observed")
        }
        animus_runtime::RuntimeEvent::Shutdown { status } => {
            UpdateFields::new(AcpUpdateKind::Terminal, status.as_str())
        }
        _other => return None,
    };
    Some(update(sequence, session_id, fields))
}

struct UpdateFields<'a> {
    kind: AcpUpdateKind,
    safe_text: Option<String>,
    correlation_identity: Option<&'a str>,
    status: &'a str,
}

impl<'a> UpdateFields<'a> {
    const fn new(kind: AcpUpdateKind, status: &'a str) -> Self {
        Self {
            kind,
            safe_text: None,
            correlation_identity: None,
            status,
        }
    }

    fn safe_text(mut self, safe_text: Option<String>) -> Self {
        self.safe_text = safe_text;
        self
    }

    const fn correlation(mut self, correlation_identity: Option<&'a str>) -> Self {
        self.correlation_identity = correlation_identity;
        self
    }
}

fn update(sequence: u32, session_id: &str, fields: UpdateFields<'_>) -> AcpUpdate {
    AcpUpdate {
        sequence,
        kind: fields.kind,
        session_id: AcpSessionId::from_admitted(String::from(session_id)),
        safe_text: fields.safe_text,
        correlation_identity: fields
            .correlation_identity
            .map(|identity| AcpCorrelationId::from_admitted(String::from(identity))),
        status: String::from(fields.status),
    }
}

fn validate_update(update: &AcpUpdate) -> Result<(), AcpProtocolError> {
    let tool_correlation_missing = matches!(
        update.kind,
        AcpUpdateKind::ToolStarted | AcpUpdateKind::ToolCompleted | AcpUpdateKind::ToolDenied
    ) && update.correlation_identity.is_none();
    if update.sequence == 0
        || tool_correlation_missing
        || AcpSessionId::parse(update.session_id.as_str()).is_err()
        || update
            .correlation_identity
            .as_ref()
            .is_some_and(|identity| AcpCorrelationId::parse(identity.as_str()).is_err())
        || update.status.is_empty()
        || update.status.contains('\0')
        || update
            .safe_text
            .as_ref()
            .is_some_and(|text| text.contains('\0'))
        || update
            .correlation_identity
            .as_ref()
            .is_some_and(|identity| identity.contains('\0'))
    {
        return Err(projection_error(
            AcpFailureClass::ProjectionRejected,
            FIELD_EVENT,
        ));
    }
    Ok(())
}

fn validate_safe_session_id(session_id: &str) -> Result<(), AcpProtocolError> {
    if session_id.is_empty()
        || session_id.len() > crate::model::ACP_MAX_SAFE_LABEL_BYTES
        || session_id.contains('\0')
    {
        Err(projection_error(
            AcpFailureClass::ProjectionRejected,
            FIELD_EVENT,
        ))
    } else {
        Ok(())
    }
}

const fn projection_error(failure: AcpFailureClass, field: &'static str) -> AcpProtocolError {
    AcpProtocolError { failure, field }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SESSION_ID: &str = "session.fixture";
    const RAW_PROMPT: &str = "secret prompt material";
    const VISIBLE_RESPONSE: &str = "visible assistant response";
    const CROSS_FRONTEND_EVENT_COUNT: usize = 3;
    const CROSS_FRONTEND_QUEUE_COUNT: u32 = 2;
    const TOOL_NAME: &str = "fixture_tool";
    const TOOL_SCHEMA: &str = "schema.acp-fixture-tool.v1";
    const TOOL_ARGUMENT_NAME: &str = "query";
    const TOOL_ARGUMENT_VALUE: &str = "bounded";
    const TOOL_CALL_ID: &str = "tool.fixture";
    const TOOL_MUTATION_TARGET: &str = "resource.fixture";
    const TOOL_CAPABILITY: &str = "tool.read";
    const TOOL_OUTPUT: &str = "private tool output";
    const TOOL_OUTPUT_BUDGET: u32 = 128;

    #[test]
    fn visible_assistant_content_is_projected_in_order() {
        let events = vec![
            animus_runtime::RuntimeEvent::SessionStateChanged {
                session_id: String::from(SESSION_ID),
                run_id: Some(String::from("run.fixture")),
                status: String::from("active"),
                queue_count: 0,
            },
            animus_runtime::RuntimeEvent::FinalResponse {
                response: animus_adapters::ModelResponse::new(VISIBLE_RESPONSE)
                    .expect("model response"),
            },
        ];
        let updates = project_runtime_events(
            SESSION_ID,
            events.as_slice(),
            AcpTransportProfile::reviewed(),
        )
        .expect("projection");

        assert_eq!(updates.len(), events.len());
        assert_eq!(updates[0].sequence, FIRST_SEQUENCE);
        assert_eq!(updates[1].safe_text.as_deref(), Some(VISIBLE_RESPONSE));
    }

    #[test]
    fn tool_lifecycle_preserves_request_correlation_and_omits_output() {
        let events = vec![
            animus_runtime::RuntimeEvent::ToolRequested {
                request: tool_request(),
            },
            animus_runtime::RuntimeEvent::ToolResponded {
                response: tool_response(),
            },
        ];
        let updates = project_runtime_events(
            SESSION_ID,
            events.as_slice(),
            AcpTransportProfile::reviewed(),
        )
        .expect("tool projection");
        let rendered = format!("{updates:?}");

        assert_eq!(updates[0].kind, AcpUpdateKind::ToolStarted);
        assert_eq!(updates[1].kind, AcpUpdateKind::ToolCompleted);
        assert!(
            updates
                .iter()
                .all(|update| update.correlation_identity.as_deref() == Some(TOOL_CALL_ID))
        );
        assert!(!rendered.contains(TOOL_OUTPUT));
    }

    #[test]
    fn provider_prompt_and_tool_payload_material_are_not_projected() {
        let events = vec![animus_runtime::RuntimeEvent::ModelRequested {
            request: animus_adapters::ModelRequest::new(RAW_PROMPT).expect("model request"),
        }];
        let updates = project_runtime_events(
            SESSION_ID,
            events.as_slice(),
            AcpTransportProfile::reviewed(),
        )
        .expect("redacted projection");
        let rendered = format!("{updates:?}");

        assert!(!rendered.contains(RAW_PROMPT));
        assert_eq!(updates[0].kind, AcpUpdateKind::Redacted);
    }

    #[test]
    // r[verify agent_harness_runtime.acp_stdio_host.events]
    // r[verify agent_harness_runtime.acp_stdio_host.verification]
    fn headless_tui_and_acp_consume_one_event_order_without_transcript_ownership() {
        let events = vec![
            animus_runtime::RuntimeEvent::SessionStateChanged {
                session_id: String::from(SESSION_ID),
                run_id: Some(String::from("run.fixture")),
                status: String::from("active"),
                queue_count: CROSS_FRONTEND_QUEUE_COUNT,
            },
            animus_runtime::RuntimeEvent::Cancelled {
                reason: String::from("correlated cancellation observed"),
            },
            animus_runtime::RuntimeEvent::Shutdown {
                status: animus_runtime::RuntimeTerminalKind::FinalResponse,
            },
        ];
        let headless = animus_runtime::map_runtime_events_to_headless(events.as_slice())
            .expect("headless projection");
        let acp = project_runtime_events(
            SESSION_ID,
            events.as_slice(),
            AcpTransportProfile::reviewed(),
        )
        .expect("ACP projection");
        let mut tui = animus_tui::CodingSessionRenderModel::new();
        for event in &events {
            tui.apply_runtime_event(event);
        }

        assert_eq!(headless.len(), CROSS_FRONTEND_EVENT_COUNT);
        assert_eq!(acp.len(), CROSS_FRONTEND_EVENT_COUNT);
        assert!(
            acp.iter()
                .enumerate()
                .all(|(index, update)| update.sequence
                    == u32::try_from(index + 1).expect("sequence"))
        );
        assert_eq!(tui.state().queue_count, CROSS_FRONTEND_QUEUE_COUNT);
        assert!(tui.state().cancellation_observed);
        assert_eq!(
            tui.state().terminal_status.as_deref(),
            Some("final_response")
        );
        assert_eq!(events.len(), CROSS_FRONTEND_EVENT_COUNT);
    }

    #[test]
    fn response_without_preceding_tool_request_fails_closed() {
        let event = animus_runtime::RuntimeEvent::ToolResponded {
            response: tool_response(),
        };
        let error = project_runtime_events(SESSION_ID, &[event], AcpTransportProfile::reviewed())
            .expect_err("uncorrelated tool response denied");

        assert_eq!(error.failure, AcpFailureClass::ProjectionRejected);
    }

    #[test]
    fn update_count_and_byte_bounds_fail_closed() {
        let event = animus_runtime::RuntimeEvent::Stopped;
        let profile = AcpTransportProfile {
            updates_per_operation: 1,
            update_bytes_per_operation: 1,
            ..AcpTransportProfile::reviewed()
        };
        let error = project_runtime_events(SESSION_ID, &[event], profile)
            .expect_err("small byte bound denied");

        assert_eq!(error.failure, AcpFailureClass::Backpressure);
    }

    fn tool_request() -> animus_adapters::ToolRequest {
        let arguments = animus_adapters::tool::PreparedArguments::prepare(
            &animus_adapters::tool::ArgumentPreparationRequest {
                current_schema_identity: TOOL_SCHEMA,
                allowed_field_names: &[TOOL_ARGUMENT_NAME],
                compatibility_rules: &[],
                fields: &[animus_adapters::tool::ArgumentFieldDraft {
                    name: Some(TOOL_ARGUMENT_NAME),
                    value: Some(TOOL_ARGUMENT_VALUE),
                }],
            },
        )
        .expect("tool arguments");
        animus_adapters::ToolRequest::from_draft(&animus_adapters::tool::RequestDraft {
            name: Some(TOOL_NAME),
            arguments: Some(&arguments),
            correlation_id: Some(TOOL_CALL_ID),
            output_policy: animus_adapters::tool::OutputPolicy {
                visible_char_budget: TOOL_OUTPUT_BUDGET,
            },
            mutation_targets: &[TOOL_MUTATION_TARGET],
            required_capability_labels: &[TOOL_CAPABILITY],
            approval_posture: animus_adapters::tool::ApprovalPosture::PreApproved.as_str(),
        })
        .expect("tool request")
    }

    fn tool_response() -> animus_adapters::ToolResponse {
        let bounded = animus_adapters::tool::BoundedOutput::new(
            &animus_adapters::tool::OutputDraft {
                body: TOOL_OUTPUT,
                full_output_reference: None,
                hash_requirement: animus_adapters::tool::HashRequirement::Required,
            },
            animus_adapters::tool::OutputPolicy {
                visible_char_budget: TOOL_OUTPUT_BUDGET,
            },
        )
        .expect("bounded tool output");
        animus_adapters::ToolResponse::from_bounded(
            bounded,
            animus_adapters::tool::OutputRedactionClass::Safe,
        )
    }
}
