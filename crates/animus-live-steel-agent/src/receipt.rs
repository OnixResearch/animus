/// Required bounded claim exclusions for every live Steel agent run.
pub const LIVE_SHELL_NON_CLAIMS: &[&str] = &[
    "provider_correctness",
    "tool_correctness",
    "task_success",
    "sandboxing",
    "daemon_readiness",
    "terminal_readiness",
    "production_readiness",
    "release_eligibility",
];

const RECEIPT_SCHEMA: &str = "animus.live-steel-agent-run-receipt.v1";
const RECEIPT_DOMAIN: &str = "animus.live-steel-agent-run-receipt.v1";
const REDACTION_MARKER: &str =
    "prompt-provider-tool-steel-host-child-credential-and-path-content-omitted";

/// Aggregate facts used to build one metadata-only run receipt.
pub(crate) struct LiveSteelAgentReceiptDraft<'a> {
    pub config_identity: &'a str,
    pub session_identity: &'a str,
    pub committed_message_count: u32,
    pub provider_dispatch_count: u32,
    pub tool_dispatch_count: u32,
    pub host_observation_count: u64,
    pub event_count: u32,
    pub kernel_head_identity: &'a str,
}

/// Metadata-only run receipt. It contains no prompt, provider body, tool
/// arguments, Steel source, host result, child content, credential, or path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveSteelAgentRunReceipt {
    status: &'static str,
    config_identity: String,
    session_identity: String,
    committed_message_count: u32,
    provider_dispatch_count: u32,
    tool_dispatch_count: u32,
    host_observation_count: u64,
    event_count: u32,
    kernel_head_identity: String,
    redaction_marker: &'static str,
    non_claims: Vec<&'static str>,
    receipt_identity: String,
}

impl LiveSteelAgentRunReceipt {
    pub(crate) fn completed(draft: &LiveSteelAgentReceiptDraft<'_>) -> Self {
        let counts = format!(
            "{}:{}:{}:{}:{}",
            draft.committed_message_count,
            draft.provider_dispatch_count,
            draft.tool_dispatch_count,
            draft.host_observation_count,
            draft.event_count,
        );
        let non_claim_text = LIVE_SHELL_NON_CLAIMS.join(",");
        let receipt_identity = hash_parts(
            RECEIPT_DOMAIN,
            &[
                "completed",
                draft.config_identity,
                draft.session_identity,
                counts.as_str(),
                draft.kernel_head_identity,
                REDACTION_MARKER,
                non_claim_text.as_str(),
            ],
        );
        Self {
            status: "completed",
            config_identity: String::from(draft.config_identity),
            session_identity: String::from(draft.session_identity),
            committed_message_count: draft.committed_message_count,
            provider_dispatch_count: draft.provider_dispatch_count,
            tool_dispatch_count: draft.tool_dispatch_count,
            host_observation_count: draft.host_observation_count,
            event_count: draft.event_count,
            kernel_head_identity: String::from(draft.kernel_head_identity),
            redaction_marker: REDACTION_MARKER,
            non_claims: LIVE_SHELL_NON_CLAIMS.to_vec(),
            receipt_identity,
        }
    }

    /// Returns the receipt schema.
    #[must_use]
    #[allow(clippy::unused_self, reason = "schema is exposed uniformly with receipt fields")]
    pub const fn schema(&self) -> &'static str {
        RECEIPT_SCHEMA
    }

    /// Returns the terminal status.
    #[must_use]
    pub const fn status(&self) -> &'static str {
        self.status
    }

    /// Returns the validated configuration identity.
    #[must_use]
    pub fn config_identity(&self) -> &str {
        self.config_identity.as_str()
    }

    /// Returns the session identity.
    #[must_use]
    pub fn session_identity(&self) -> &str {
        self.session_identity.as_str()
    }

    /// Returns the committed message count.
    #[must_use]
    pub const fn committed_message_count(&self) -> u32 {
        self.committed_message_count
    }

    /// Returns the provider dispatch count.
    #[must_use]
    pub const fn provider_dispatch_count(&self) -> u32 {
        self.provider_dispatch_count
    }

    /// Returns the tool dispatch count.
    #[must_use]
    pub const fn tool_dispatch_count(&self) -> u32 {
        self.tool_dispatch_count
    }

    /// Returns the completed host-effect observation count.
    #[must_use]
    pub const fn host_observation_count(&self) -> u64 {
        self.host_observation_count
    }

    /// Returns the ordered runtime event count.
    #[must_use]
    pub const fn event_count(&self) -> u32 {
        self.event_count
    }

    /// Returns the committed private kernel chain head.
    #[must_use]
    pub fn kernel_head_identity(&self) -> &str {
        self.kernel_head_identity.as_str()
    }

    /// Returns the fixed omission marker.
    #[must_use]
    pub const fn redaction_marker(&self) -> &'static str {
        self.redaction_marker
    }

    /// Returns required non-claims.
    #[must_use]
    pub fn non_claims(&self) -> &[&'static str] {
        self.non_claims.as_slice()
    }

    /// Returns the domain-separated BLAKE3 receipt identity.
    #[must_use]
    pub fn receipt_identity(&self) -> &str {
        self.receipt_identity.as_str()
    }

    /// Renders one stable metadata-only evidence line.
    #[must_use]
    pub fn render(&self) -> String {
        format!(
            "LIVE_STEEL_AGENT schema={} status={} committed_messages={} provider_dispatches={} tool_dispatches={} host_observations={} events={} config_identity={} session_identity={} kernel_head_identity={} receipt_identity={} redaction={} non_claims={}",
            self.schema(),
            self.status,
            self.committed_message_count,
            self.provider_dispatch_count,
            self.tool_dispatch_count,
            self.host_observation_count,
            self.event_count,
            self.config_identity,
            self.session_identity,
            self.kernel_head_identity,
            self.receipt_identity,
            self.redaction_marker,
            self.non_claims.join(","),
        )
    }
}

fn hash_parts(domain: &str, parts: &[&str]) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(domain.as_bytes());
    for part in parts {
        let length = u64::try_from(part.len()).unwrap_or(u64::MAX);
        hasher.update(&length.to_le_bytes());
        hasher.update(part.as_bytes());
    }
    hasher.finalize().to_hex().to_string()
}
