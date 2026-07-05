const UNSAFE_DATASPACE_MARKERS: &[&str] = &[
    "/home/",
    "/Users/",
    "raw_prompt:",
    "provider_payload:",
    "credential:",
    "file_body:",
    "tool_input:",
    "tool_output:",
    "model_output:",
    "unbounded_log:",
    "begin private key",
    "api_key=",
    "aws_secret_access_key",
    "sk-",
];
const UNSAFE_PROJECTION_REASON: &str = "projection contains unsafe raw source material";
const RAW_SOURCE_REDACTION_REASON: &str =
    "sensitive-source projections require explicit redaction markers";
const TRANSPORT_UNAVAILABLE_REASON: &str = "syndicate transport unavailable";

/// Optional live-dataspace transport used by the host-owned adapter shell.
pub trait SyndicateTransport {
    /// Publishes one already validated dataspace assertion.
    ///
    /// # Errors
    ///
    /// Returns [`SyndicateAdapterError`] when the host transport cannot publish safely.
    fn publish_assertion(
        &mut self,
        assertion: &animus_adapters::Assertion,
    ) -> Result<(), SyndicateAdapterError>;

    /// Queries matching assertions for one safe subscription.
    ///
    /// # Errors
    ///
    /// Returns [`SyndicateAdapterError`] when the host transport cannot query safely.
    fn query_assertions(
        &mut self,
        subscription: &animus_adapters::Subscription,
    ) -> Result<Vec<animus_adapters::Assertion>, SyndicateAdapterError>;

    /// Handles one safe host orchestration event.
    ///
    /// # Errors
    ///
    /// Returns [`SyndicateAdapterError`] when the host transport cannot process the event safely.
    fn handle_orchestration_event(
        &mut self,
        event: &animus_adapters::OrchestrationEvent,
    ) -> Result<(), SyndicateAdapterError>;
}

/// Feature-gated host-owned dataspace adapter seam.
pub struct SyndicateHostDataspaceAdapter<T> {
    transport: T,
}

impl<T> SyndicateHostDataspaceAdapter<T> {
    /// Builds an adapter shell over caller-owned transport/configuration.
    #[must_use]
    pub const fn new(transport: T) -> Self {
        Self { transport }
    }

    /// Returns the wrapped host transport.
    #[must_use]
    pub const fn transport(&self) -> &T {
        &self.transport
    }

    /// Returns mutable access to the wrapped host transport.
    #[must_use]
    pub fn transport_mut(&mut self) -> &mut T {
        &mut self.transport
    }
}

impl<T> animus_adapters::AdapterService for SyndicateHostDataspaceAdapter<T>
where
    T: SyndicateTransport,
{
    type Error = SyndicateAdapterError;

    fn publish(&mut self, assertion: &animus_adapters::Assertion) -> Result<(), Self::Error> {
        self.transport.publish_assertion(assertion)
    }

    fn subscribe(
        &mut self,
        subscription: &animus_adapters::Subscription,
    ) -> Result<Vec<animus_adapters::Assertion>, Self::Error> {
        self.transport.query_assertions(subscription)
    }

    fn handle_event(
        &mut self,
        event: &animus_adapters::OrchestrationEvent,
    ) -> Result<(), Self::Error> {
        self.transport.handle_orchestration_event(event)
    }
}

/// Observation family projected to a bounded redacted dataspace assertion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SyndicateObservationKind {
    /// Runtime or reusable loop observation.
    Runtime,
    /// Headless command/event observation.
    Headless,
    /// Tool lifecycle observation.
    ToolLifecycle,
    /// Background work progress observation.
    BackgroundWork,
    /// Session or provenance summary observation.
    SessionProvenance,
    /// Top-level agent state observation.
    Agent,
    /// Subagent state observation.
    Subagent,
}

impl SyndicateObservationKind {
    /// Returns the matching dataspace assertion-kind label.
    #[must_use]
    pub const fn assertion_kind(self) -> &'static str {
        match self {
            Self::Runtime | Self::Headless => "runtime_event",
            Self::ToolLifecycle => "tool_state",
            Self::BackgroundWork => "background_work",
            Self::SessionProvenance => "provenance_summary",
            Self::Agent => "agent_lifecycle",
            Self::Subagent => "subagent_lifecycle",
        }
    }
}

/// Borrowed projection inputs before safe dataspace assertion validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyndicateObservationDraft<'a> {
    /// Stable assertion identity.
    pub stable_id: &'a str,
    /// Observation family.
    pub observation_kind: SyndicateObservationKind,
    /// Safe status class label.
    pub status: &'a str,
    /// Scope boundary for the assertion.
    pub scope: animus_adapters::Scope,
    /// Redaction marker labels proving raw source material was omitted.
    pub redactions: &'a [&'a str],
    /// Optional BLAKE3 safe content hash for redacted source material.
    pub content_hash: Option<&'a str>,
    /// Bounded safe item count.
    pub item_count: u32,
    /// Optional deterministic lease expiry tick.
    pub lease_expires_at: Option<u64>,
    /// Whether the source observation carried prompt/provider/file/tool/model/log/credential material.
    pub contains_sensitive_source: bool,
}

/// Typed error raised by the optional host dataspace seam.
#[derive(Debug)]
pub enum SyndicateAdapterError {
    /// Projection attempted to carry unsafe raw material.
    UnsafeProjection {
        /// Safe reason label.
        reason: &'static str,
    },
    /// Adapter DTO validation failed.
    AdapterContract {
        /// Source adapter contract error.
        source: animus_adapters::AdapterContractError,
    },
    /// Subscription validation failed.
    Subscription {
        /// Source subscription issue.
        source: animus_adapters::SubscriptionIssue,
    },
    /// The explicit live transport was not configured or unavailable.
    LiveBackendUnavailable,
    /// Caller attempted orchestration outside the expected scope.
    ScopeMismatch,
    /// Transport returned a safe bounded failure reason.
    Transport {
        /// Safe bounded reason.
        reason: &'static str,
    },
}

impl core::fmt::Display for SyndicateAdapterError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnsafeProjection { reason } => {
                write!(formatter, "unsafe dataspace projection: {reason}")
            }
            Self::AdapterContract { source } => {
                write!(formatter, "adapter contract failed: {source}")
            }
            Self::Subscription { source } => write!(
                formatter,
                "dataspace subscription failed: {}",
                source.as_str()
            ),
            Self::LiveBackendUnavailable => formatter.write_str(TRANSPORT_UNAVAILABLE_REASON),
            Self::ScopeMismatch => formatter.write_str("dataspace orchestration scope mismatch"),
            Self::Transport { reason } => write!(formatter, "dataspace transport failed: {reason}"),
        }
    }
}

impl std::error::Error for SyndicateAdapterError {}

impl From<animus_adapters::AdapterContractError> for SyndicateAdapterError {
    fn from(source: animus_adapters::AdapterContractError) -> Self {
        Self::AdapterContract { source }
    }
}

impl From<animus_adapters::SubscriptionIssue> for SyndicateAdapterError {
    fn from(source: animus_adapters::SubscriptionIssue) -> Self {
        Self::Subscription { source }
    }
}

/// Projects one host/runtime observation into a bounded dataspace assertion.
///
/// # Errors
///
/// Returns [`SyndicateAdapterError`] when fields are unsafe or adapter DTO validation fails.
pub fn project_observation(
    draft: SyndicateObservationDraft<'_>,
) -> Result<animus_adapters::Assertion, SyndicateAdapterError> {
    validate_safe_text(draft.stable_id)?;
    validate_safe_text(draft.status)?;
    if let Some(hash) = draft.content_hash {
        validate_safe_text(hash)?;
    }
    for redaction in draft.redactions {
        validate_safe_text(redaction)?;
    }
    if draft.contains_sensitive_source && draft.redactions.is_empty() {
        return Err(SyndicateAdapterError::UnsafeProjection {
            reason: RAW_SOURCE_REDACTION_REASON,
        });
    }

    Ok(animus_adapters::Assertion::new(
        animus_adapters::AssertionDraft {
            stable_id: draft.stable_id,
            kind: draft.observation_kind.assertion_kind(),
            status: draft.status,
            scope: draft.scope,
            redactions: draft.redactions,
            content_hash: draft.content_hash,
            item_count: draft.item_count,
            lease_expires_at: draft.lease_expires_at,
        },
    )?)
}

/// Validates that an orchestration event is scoped to the expected owner.
///
/// # Errors
///
/// Returns [`SyndicateAdapterError::ScopeMismatch`] when the event crosses owner scope.
pub fn admit_scoped_orchestration(
    expected_scope: &animus_adapters::Scope,
    event: &animus_adapters::OrchestrationEvent,
) -> Result<(), SyndicateAdapterError> {
    if expected_scope.same_actor_scope(event.scope()) {
        return Ok(());
    }
    Err(SyndicateAdapterError::ScopeMismatch)
}

/// Builds a safe subscription DTO for one scope and assertion-kind set.
///
/// # Errors
///
/// Returns [`SyndicateAdapterError`] when the query is unsafe or crosses scope.
pub fn build_safe_subscription(
    scope: animus_adapters::Scope,
    kinds: &[&str],
) -> Result<animus_adapters::Subscription, SyndicateAdapterError> {
    for kind in kinds {
        validate_safe_text(kind)?;
    }
    Ok(animus_adapters::Subscription::new(
        animus_adapters::SubscriptionDraft {
            scope,
            kinds,
            include_raw_payloads: false,
            cross_workspace: false,
        },
    )?)
}

fn validate_safe_text(value: &str) -> Result<(), SyndicateAdapterError> {
    let lower = value.to_ascii_lowercase();
    for marker in UNSAFE_DATASPACE_MARKERS {
        if lower.contains(marker) {
            return Err(SyndicateAdapterError::UnsafeProjection {
                reason: UNSAFE_PROJECTION_REASON,
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use animus_adapters::AdapterService;

    const TENANT: &str = "tenant:local";
    const WORKSPACE: &str = "workspace:main";
    const OTHER_WORKSPACE: &str = "workspace:other";
    const SESSION: &str = "session:main";
    const BRANCH: &str = "branch:main";
    const FACET: &str = "facet:loop";
    const ACTOR_KIND: &str = "agent";
    const ACTOR_ID: &str = "agent:main";
    const ASSERTION_ID: &str = "assertion:runtime";
    const STATUS_ACTIVE: &str = "active";
    const STATUS_CANCELLED: &str = "cancelled";
    const REDACTION_PROMPT: &str = "prompt_omitted";
    const REDACTION_PROVIDER: &str = "provider_payload_omitted";
    const REDACTION_CREDENTIAL: &str = "credential_omitted";
    const REDACTION_LOGS: &str = "logs_bounded";
    const SAFE_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";
    const RAW_PROMPT_ID: &str = "raw_prompt: private";
    const EVENT_ID: &str = "event:cancel";
    const EVENT_CANCELLED: &str = "cancelled";
    const ITEM_COUNT: u32 = 1;
    const NO_ITEMS: usize = 0;
    const ONE_ITEM: usize = 1;

    #[derive(Default)]
    struct MemoryTransport {
        assertions: Vec<animus_adapters::Assertion>,
        unavailable: bool,
    }

    impl SyndicateTransport for MemoryTransport {
        fn publish_assertion(
            &mut self,
            assertion: &animus_adapters::Assertion,
        ) -> Result<(), SyndicateAdapterError> {
            if self.unavailable {
                return Err(SyndicateAdapterError::LiveBackendUnavailable);
            }
            self.assertions.push(assertion.clone());
            Ok(())
        }

        fn query_assertions(
            &mut self,
            subscription: &animus_adapters::Subscription,
        ) -> Result<Vec<animus_adapters::Assertion>, SyndicateAdapterError> {
            if self.unavailable {
                return Err(SyndicateAdapterError::LiveBackendUnavailable);
            }
            Ok(self
                .assertions
                .iter()
                .filter(|assertion| {
                    assertion
                        .scope()
                        .same_tenant_workspace(subscription.scope())
                })
                .filter(|assertion| subscription.allows(assertion.kind()))
                .cloned()
                .collect())
        }

        fn handle_orchestration_event(
            &mut self,
            event: &animus_adapters::OrchestrationEvent,
        ) -> Result<(), SyndicateAdapterError> {
            if self.unavailable {
                return Err(SyndicateAdapterError::LiveBackendUnavailable);
            }
            if event.kind() == animus_adapters::OrchestrationEventKind::Cancelled {
                for assertion in &mut self.assertions {
                    if assertion.scope().same_actor_scope(event.scope()) {
                        let replacement =
                            animus_adapters::Assertion::new(animus_adapters::AssertionDraft {
                                stable_id: assertion.stable_id(),
                                kind: "runtime_event",
                                status: STATUS_CANCELLED,
                                scope: assertion.scope().clone(),
                                redactions: &[
                                    REDACTION_PROMPT,
                                    REDACTION_PROVIDER,
                                    REDACTION_CREDENTIAL,
                                    REDACTION_LOGS,
                                ],
                                content_hash: assertion
                                    .content_hash()
                                    .map(animus_adapters::SafeHash::as_str),
                                item_count: assertion.item_count(),
                                lease_expires_at: assertion.lease_expires_at(),
                            })?;
                        *assertion = replacement;
                    }
                }
            }
            Ok(())
        }
    }

    fn scope(workspace: &str) -> animus_adapters::Scope {
        animus_adapters::Scope::new(animus_adapters::ScopeDraft {
            tenant_id: TENANT,
            workspace_id: workspace,
            session_id: SESSION,
            branch_id: BRANCH,
            facet_id: FACET,
            actor_kind: ACTOR_KIND,
            actor_id: ACTOR_ID,
        })
        .expect("scope should validate")
    }

    fn observation(workspace: &str) -> SyndicateObservationDraft<'_> {
        SyndicateObservationDraft {
            stable_id: ASSERTION_ID,
            observation_kind: SyndicateObservationKind::Runtime,
            status: STATUS_ACTIVE,
            scope: scope(workspace),
            redactions: &[
                REDACTION_PROMPT,
                REDACTION_PROVIDER,
                REDACTION_CREDENTIAL,
                REDACTION_LOGS,
            ],
            content_hash: Some(SAFE_HASH),
            item_count: ITEM_COUNT,
            lease_expires_at: None,
            contains_sensitive_source: true,
        }
    }

    fn cancellation_event(scope: animus_adapters::Scope) -> animus_adapters::OrchestrationEvent {
        animus_adapters::OrchestrationEvent::new(animus_adapters::OrchestrationEventDraft {
            stable_id: EVENT_ID,
            kind: EVENT_CANCELLED,
            scope,
            target_owner: None,
        })
        .expect("event should validate")
    }

    #[test]
    fn projection_covers_all_observation_families() {
        // r[verify agent_harness_runtime.syndicate_host_adapter.projection.redacted]
        let families = [
            SyndicateObservationKind::Runtime,
            SyndicateObservationKind::Headless,
            SyndicateObservationKind::ToolLifecycle,
            SyndicateObservationKind::BackgroundWork,
            SyndicateObservationKind::SessionProvenance,
            SyndicateObservationKind::Agent,
            SyndicateObservationKind::Subagent,
        ];

        for family in families {
            let assertion = project_observation(SyndicateObservationDraft {
                observation_kind: family,
                ..observation(WORKSPACE)
            })
            .expect("projection should validate");
            assert_eq!(
                assertion
                    .content_hash()
                    .map(animus_adapters::SafeHash::as_str),
                Some(SAFE_HASH)
            );
            assert!(
                assertion
                    .redactions()
                    .contains(&animus_adapters::AssertionRedaction::ProviderPayloadOmitted)
            );
        }
    }

    #[test]
    fn projection_rejects_raw_sensitive_material_and_missing_redactions() {
        // r[verify agent_harness_runtime.syndicate_host_adapter.projection.redacted]
        let raw = project_observation(SyndicateObservationDraft {
            stable_id: RAW_PROMPT_ID,
            ..observation(WORKSPACE)
        })
        .expect_err("raw prompt marker should fail");
        let missing_redactions = project_observation(SyndicateObservationDraft {
            redactions: &[],
            ..observation(WORKSPACE)
        })
        .expect_err("sensitive source requires redactions");

        assert!(matches!(
            raw,
            SyndicateAdapterError::UnsafeProjection { .. }
        ));
        assert!(matches!(
            missing_redactions,
            SyndicateAdapterError::UnsafeProjection {
                reason: RAW_SOURCE_REDACTION_REASON,
            }
        ));
    }

    #[test]
    fn adapter_publishes_queries_and_handles_scoped_cancellation() {
        // r[verify agent_harness_runtime.syndicate_host_adapter.orchestration]
        let assertion =
            project_observation(observation(WORKSPACE)).expect("projection should pass");
        let mut adapter = SyndicateHostDataspaceAdapter::new(MemoryTransport::default());
        adapter.publish(&assertion).expect("publish should pass");
        let query = build_safe_subscription(scope(WORKSPACE), &["runtime_event"])
            .expect("subscription should pass");

        let matches = adapter.subscribe(&query).expect("query should pass");
        assert_eq!(matches.len(), ONE_ITEM);

        let event = cancellation_event(scope(WORKSPACE));
        admit_scoped_orchestration(assertion.scope(), &event).expect("event scope should match");
        adapter.handle_event(&event).expect("event should pass");
        let matches = adapter.subscribe(&query).expect("query should pass");
        assert_eq!(
            matches[0].status(),
            animus_adapters::AssertionStatus::Cancelled
        );
    }

    #[test]
    fn adapter_rejects_cross_scope_and_unavailable_backend() {
        // r[verify agent_harness_runtime.syndicate_host_adapter.failure]
        let assertion =
            project_observation(observation(WORKSPACE)).expect("projection should pass");
        let cross_event = cancellation_event(scope(OTHER_WORKSPACE));
        let scope_error = admit_scoped_orchestration(assertion.scope(), &cross_event)
            .expect_err("cross workspace should fail");
        let mut unavailable = SyndicateHostDataspaceAdapter::new(MemoryTransport {
            assertions: Vec::new(),
            unavailable: true,
        });
        let unavailable_error = unavailable
            .publish(&assertion)
            .expect_err("unavailable backend should fail");

        assert!(matches!(scope_error, SyndicateAdapterError::ScopeMismatch));
        assert!(matches!(
            unavailable_error,
            SyndicateAdapterError::LiveBackendUnavailable
        ));
        assert_eq!(unavailable.transport().assertions.len(), NO_ITEMS);
    }
}
