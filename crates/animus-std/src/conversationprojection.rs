//! Generated conversation policy loading and deterministic cache fixture.

use animus_runtime::conversation_projection::{
    ConversationCacheEvidence, ConversationCacheStatus, ConversationProjectionCache,
    ConversationProjectionPlan, ConversationProjectionPolicy, ConversationProjectionPolicyInput,
};
use serde::Deserialize;

/// Safe projection policy configuration error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConversationProjectionConfigError {
    Read(std::io::ErrorKind),
    Json,
    Validation(animus_runtime::conversation_projection::ConversationProjectionIssue),
}

/// Parses and revalidates generated policy JSON.
///
/// # Errors
/// Returns bounded JSON or policy validation categories.
pub fn parse_conversation_projection_policy_json(
    input: &str,
) -> Result<ConversationProjectionPolicy, ConversationProjectionConfigError> {
    let value: GeneratedPolicy =
        serde_json::from_str(input).map_err(|_| ConversationProjectionConfigError::Json)?;
    animus_runtime::conversation_projection::validate_conversation_projection_policy(
        &value.into_input(),
    )
    .map_err(|error| ConversationProjectionConfigError::Validation(error.issue()))
}

/// Loads policy from an explicit path.
///
/// # Errors
/// Returns bounded read, JSON, or policy validation categories.
pub fn load_conversation_projection_policy(
    path: impl AsRef<std::path::Path>,
) -> Result<ConversationProjectionPolicy, ConversationProjectionConfigError> {
    let input = std::fs::read_to_string(path)
        .map_err(|error| ConversationProjectionConfigError::Read(error.kind()))?;
    parse_conversation_projection_policy_json(&input)
}

/// Deterministic disposable projection cache fixture.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ConversationMemoryCache {
    plan_hash: Option<String>,
    cursor_hash: Option<String>,
    action_hashes: Vec<String>,
    fail_next: bool,
}
impl ConversationMemoryCache {
    /// Makes the next cache call return partial evidence without mutation.
    pub fn fail_next(&mut self) {
        self.fail_next = true;
    }
    /// Returns the current cursor identity.
    #[must_use]
    pub fn cursor_hash(&self) -> Option<&str> {
        self.cursor_hash.as_deref()
    }
}
impl ConversationProjectionCache for ConversationMemoryCache {
    fn apply(&mut self, plan: &ConversationProjectionPlan) -> ConversationCacheEvidence {
        if self.fail_next {
            self.fail_next = false;
            return ConversationCacheEvidence {
                plan_hash: plan.plan_hash.clone(),
                action_hashes: Vec::new(),
                cursor_hash: plan.cursor.cursor_hash.clone(),
                status: ConversationCacheStatus::Partial,
            };
        }
        if self.plan_hash.as_deref() == Some(plan.plan_hash.as_str()) {
            return ConversationCacheEvidence {
                plan_hash: plan.plan_hash.clone(),
                action_hashes: self.action_hashes.clone(),
                cursor_hash: plan.cursor.cursor_hash.clone(),
                status: ConversationCacheStatus::AlreadyApplied,
            };
        }
        self.plan_hash = Some(plan.plan_hash.clone());
        self.cursor_hash = Some(plan.cursor.cursor_hash.clone());
        self.action_hashes = plan
            .actions
            .iter()
            .map(|action| action.action_hash.clone())
            .collect();
        ConversationCacheEvidence {
            plan_hash: plan.plan_hash.clone(),
            action_hashes: self.action_hashes.clone(),
            cursor_hash: plan.cursor.cursor_hash.clone(),
            status: ConversationCacheStatus::Applied,
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedPolicy {
    schema: String,
    policy_id: String,
    max_entries: u32,
    max_parts_per_item: u32,
    reasoning_posture: String,
    tool_posture: String,
    cursor_posture: String,
    cache_posture: String,
    receipt_fields: Vec<String>,
    non_claims: Vec<String>,
    policy_hash: String,
}
impl GeneratedPolicy {
    fn into_input(self) -> ConversationProjectionPolicyInput {
        ConversationProjectionPolicyInput {
            schema: self.schema,
            policy_id: self.policy_id,
            max_entries: self.max_entries,
            max_parts_per_item: self.max_parts_per_item,
            reasoning_posture: self.reasoning_posture,
            tool_posture: self.tool_posture,
            cursor_posture: self.cursor_posture,
            cache_posture: self.cache_posture,
            receipt_fields: self.receipt_fields,
            non_claims: self.non_claims,
            policy_hash: self.policy_hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use animus_runtime::conversation_projection::{
        ConversationProjectionCache, ConversationProjectionPlanKind,
    };
    #[test]
    fn generated_policy_parses() {
        assert!(
            parse_conversation_projection_policy_json(include_str!(
                "../../../config/generated/conversation-projection-policy.json"
            ))
            .is_ok()
        );
    }
    #[test]
    fn malformed_policy_fails() {
        assert_eq!(
            parse_conversation_projection_policy_json("{").expect_err("json"),
            ConversationProjectionConfigError::Json
        );
    }
    #[test]
    fn cache_is_idempotent_and_failure_does_not_mutate() {
        let mut cache = ConversationMemoryCache::default();
        let plan = fixture_plan();
        cache.fail_next();
        let failed = cache.apply(&plan);
        assert_eq!(failed.status, ConversationCacheStatus::Partial);
        assert!(cache.cursor_hash().is_none());
        let applied = cache.apply(&plan);
        let repeated = cache.apply(&plan);
        assert_eq!(applied.status, ConversationCacheStatus::Applied);
        assert_eq!(repeated.status, ConversationCacheStatus::AlreadyApplied);
    }
    fn fixture_plan() -> ConversationProjectionPlan {
        ConversationProjectionPlan {
            kind: ConversationProjectionPlanKind::Noop,
            actions: Vec::new(),
            cursor: animus_runtime::conversation_projection::ConversationProjectionCursor {
                stream_hash: hash("stream"),
                policy_hash: hash("policy"),
                session_id: "session.one".to_string(),
                branch_id: "branch.one".to_string(),
                leaf_hash: hash("leaf"),
                generation: 1,
                last_entry_hash: hash("entry"),
                last_sequence: 1,
                active_turn_hash: None,
                cursor_hash: hash("cursor"),
            },
            plan_hash: hash("plan"),
        }
    }
    fn hash(value: &str) -> String {
        blake3::hash(value.as_bytes()).to_hex().to_string()
    }
}
