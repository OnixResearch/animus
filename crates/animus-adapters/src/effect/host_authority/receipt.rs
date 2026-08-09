use alloc::{format, string::String, vec::Vec};

use super::{HostAuthorityDecision, HostAuthorityIssue, HostAuthorityStatus};

const RECEIPT_DOMAIN: &[u8] = b"animus/host-authority-receipt/v1";
const RECEIPT_SCHEMA: &str = "animus.host-authority-receipt.v1";
const BLAKE3_PREFIX: &str = "b3:";
const MAX_NON_CLAIMS: usize = 16;

/// Required bounded claim limits for all generic authority receipts.
pub const HOST_AUTHORITY_NON_CLAIMS: [&str; 8] = [
    "global_trust",
    "key_truth",
    "global_revocation_freshness",
    "replay_durability",
    "sandbox_confinement",
    "effect_correctness",
    "agent_safety",
    "completion_or_release_readiness",
];

/// Receipt input over one already-validated safe decision.
#[derive(Clone, Copy, Debug)]
pub struct HostAuthorityReceiptDraft<'a> {
    /// Exact safe decision.
    pub decision: &'a HostAuthorityDecision,
    /// Safe dispatch class, such as `planned`, `committed`, or `blocked`.
    pub dispatch_class: &'a str,
    /// Required non-claims. The complete fixed set is mandatory.
    pub non_claims: &'a [&'a str],
}

/// Metadata-only authority receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HostAuthorityReceipt {
    /// Receipt schema.
    pub schema: &'static str,
    /// Provider identity.
    pub provider_identity: String,
    /// Policy identity.
    pub policy_identity: String,
    /// Request identity.
    pub request_identity: String,
    /// Binding identity.
    pub binding_identity: String,
    /// Decision identity.
    pub decision_identity: String,
    /// Safe action label.
    pub action: &'static str,
    /// Safe ability label.
    pub ability: String,
    /// Decision status.
    pub status: HostAuthorityStatus,
    /// Safe issue class.
    pub issue: HostAuthorityIssue,
    /// Checked provider fact count.
    pub checked_fact_count: u32,
    /// Redacted principal hash.
    pub principal_hash: String,
    /// Redacted resource hash.
    pub resource_hash: String,
    /// Safe dispatch class.
    pub dispatch_class: String,
    /// Required non-claims.
    pub non_claims: Vec<String>,
    /// Complete receipt identity.
    pub receipt_identity: String,
}

/// Receipt leak class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostAuthorityReceiptLeak {
    /// Bearer, key, signature, proof, nonce, or credential material appeared.
    Credential,
    /// Prompt, tool argument, provider payload, environment, path, or output material appeared.
    PrivateMaterial,
    /// A completion, safety, trust, effect, or release overclaim appeared.
    Overclaim,
}

/// Build one metadata-only authority receipt.
///
/// # Errors
///
/// Returns [`HostAuthorityIssue::UnsafeReceipt`] for missing non-claims or unsafe dispatch metadata.
pub fn build_host_authority_receipt(
    draft: HostAuthorityReceiptDraft<'_>,
) -> Result<HostAuthorityReceipt, HostAuthorityIssue> {
    if draft.non_claims.len() > MAX_NON_CLAIMS
        || draft.non_claims != HOST_AUTHORITY_NON_CLAIMS
        || !safe_dispatch_class(draft.dispatch_class)
    {
        return Err(HostAuthorityIssue::UnsafeReceipt);
    }
    let receipt_identity = receipt_identity(&draft)?;
    Ok(HostAuthorityReceipt {
        schema: RECEIPT_SCHEMA,
        provider_identity: String::from(draft.decision.provider_identity()),
        policy_identity: String::from(draft.decision.policy_identity()),
        request_identity: String::from(draft.decision.request_identity()),
        binding_identity: String::from(draft.decision.binding_identity()),
        decision_identity: String::from(draft.decision.decision_identity()),
        action: draft.decision.action().as_str(),
        ability: String::from(draft.decision.ability()),
        status: draft.decision.status(),
        issue: draft.decision.issue(),
        checked_fact_count: draft.decision.checked_fact_count(),
        principal_hash: String::from(draft.decision.principal_hash()),
        resource_hash: String::from(draft.decision.resource_hash()),
        dispatch_class: String::from(draft.dispatch_class),
        non_claims: draft
            .non_claims
            .iter()
            .map(|value| String::from(*value))
            .collect(),
        receipt_identity,
    })
}

/// Reject rendered receipt text that contains private material or overclaims.
///
/// # Errors
///
/// Returns the first stable leak class in credential, private-material, then overclaim order.
pub fn validate_host_authority_receipt_text(text: &str) -> Result<(), HostAuthorityReceiptLeak> {
    let lowered = text.to_ascii_lowercase();
    if [
        "bearer ",
        "compact_token",
        "private_key",
        "signature",
        "proof_body",
        "replay_nonce",
        "credential",
    ]
    .iter()
    .any(|marker| lowered.contains(marker))
    {
        return Err(HostAuthorityReceiptLeak::Credential);
    }
    if [
        "raw_prompt",
        "tool_arguments",
        "provider_payload",
        "environment_value",
        "private_path",
        "effect_output",
    ]
    .iter()
    .any(|marker| lowered.contains(marker))
    {
        return Err(HostAuthorityReceiptLeak::PrivateMaterial);
    }
    if [
        "globally trusted",
        "sandbox is secure",
        "effect is correct",
        "agent is safe",
        "completion proved",
        "release ready",
    ]
    .iter()
    .any(|marker| lowered.contains(marker))
    {
        return Err(HostAuthorityReceiptLeak::Overclaim);
    }
    Ok(())
}

fn safe_dispatch_class(value: &str) -> bool {
    matches!(value, "planned" | "committed" | "blocked")
}

fn receipt_identity(draft: &HostAuthorityReceiptDraft<'_>) -> Result<String, HostAuthorityIssue> {
    let checked = draft.decision.checked_fact_count().to_be_bytes();
    let mut hasher = blake3::Hasher::new();
    hasher.update(RECEIPT_DOMAIN);
    for field in [
        RECEIPT_SCHEMA.as_bytes(),
        draft.decision.decision_identity().as_bytes(),
        draft.decision.request_identity().as_bytes(),
        draft.decision.binding_identity().as_bytes(),
        draft.decision.provider_identity().as_bytes(),
        draft.decision.policy_identity().as_bytes(),
        draft.decision.action().as_str().as_bytes(),
        draft.decision.ability().as_bytes(),
        draft.decision.principal_hash().as_bytes(),
        draft.decision.resource_hash().as_bytes(),
        checked.as_slice(),
        draft.dispatch_class.as_bytes(),
    ] {
        let length = u64::try_from(field.len()).map_err(|_| HostAuthorityIssue::InvalidBound)?;
        hasher.update(&length.to_be_bytes());
        hasher.update(field);
    }
    for non_claim in draft.non_claims {
        let length =
            u64::try_from(non_claim.len()).map_err(|_| HostAuthorityIssue::InvalidBound)?;
        hasher.update(&length.to_be_bytes());
        hasher.update(non_claim.as_bytes());
    }
    Ok(format!("{BLAKE3_PREFIX}{}", hasher.finalize().to_hex()))
}
