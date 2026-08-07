//! Metadata-only ACP host receipt projection.

#![allow(missing_docs)]

use crate::core::domain_hash;
use crate::identities::{
    AcpCorrelationId, AuthorityProfileIdentity, ReceiptIdentity, SessionIdentity,
};
use crate::model::{
    ACP_HASH_HEX_BYTES, ACP_NON_CLAIMS, ACP_PROTOCOL_COHORT, ACP_PROTOCOL_LICENSE,
    ACP_PROTOCOL_VERSION, ACP_RECEIPT_HASH_DOMAIN, ACP_RECEIPT_SCHEMA, ACP_REDACTION_MARKER,
    AcpCleanupStatus, AcpConnectionProfile, AcpFailureClass, AcpProtocolError, AcpReceipt,
    AcpReceiptDraft, AcpSensitiveDraft, AcpTransportStatus,
};

const FIELD_RECEIPT: &str = "receipt";
const FIELD_SENSITIVE: &str = "sensitive";
const RECEIPT_BASE_FIELD_CAPACITY: usize = 16;

/// # Errors
/// Returns a typed error when sensitive material is supplied or receipt facts are inconsistent.
// r[impl agent_harness_runtime.acp_stdio_host.receipts]
pub fn project_acp_receipt(
    profile: &AcpConnectionProfile,
    draft: &AcpReceiptDraft,
    sensitive: AcpSensitiveDraft<'_>,
) -> Result<AcpReceipt, AcpProtocolError> {
    reject_sensitive(sensitive)?;
    validate_hash(draft.connection_identity.as_str())?;
    validate_optional_hash(draft.session_identity.as_ref().map(SessionIdentity::as_str))?;
    validate_optional_hash(
        draft
            .correlation_identity
            .as_ref()
            .map(AcpCorrelationId::as_str),
    )?;
    validate_optional_hash(
        draft
            .authority_profile_identity
            .as_ref()
            .map(AuthorityProfileIdentity::as_str),
    )?;
    if draft.status == AcpTransportStatus::CleanEof && draft.failure.is_some() {
        return Err(error(AcpFailureClass::SensitiveEvidence, FIELD_RECEIPT));
    }
    let feature_labels = profile
        .features
        .iter()
        .map(|feature| String::from(feature.as_str()))
        .collect::<Vec<_>>();
    let fields = receipt_hash_fields(draft, feature_labels.as_slice());
    let field_refs = fields.iter().map(String::as_str).collect::<Vec<_>>();
    let receipt_identity = ReceiptIdentity::from_domain_hash(domain_hash(
        ACP_RECEIPT_HASH_DOMAIN,
        field_refs.as_slice(),
    ));
    Ok(AcpReceipt {
        schema: ACP_RECEIPT_SCHEMA,
        cohort: ACP_PROTOCOL_COHORT,
        license: ACP_PROTOCOL_LICENSE,
        protocol_version: ACP_PROTOCOL_VERSION,
        feature_labels,
        operation: String::from(draft.operation.as_str()),
        status: String::from(transport_status_label(draft.status)),
        failure: draft.failure.map(|failure| String::from(failure.as_str())),
        connection_identity: draft.connection_identity.as_str().to_string(),
        session_identity: draft
            .session_identity
            .as_ref()
            .map(|identity| identity.as_str().to_string()),
        correlation_identity: draft
            .correlation_identity
            .as_ref()
            .map(|identity| identity.as_str().to_string()),
        authority_profile_identity: draft
            .authority_profile_identity
            .as_ref()
            .map(|identity| identity.as_str().to_string()),
        frame_count: draft.frame_count,
        update_count: draft.update_count,
        update_bytes: draft.update_bytes,
        tool_count: draft.tool_count,
        permission_count: draft.permission_count,
        cleanup_status: String::from(cleanup_status_label(draft.cleanup)),
        redaction_marker: ACP_REDACTION_MARKER,
        non_claims: ACP_NON_CLAIMS
            .iter()
            .map(|claim| String::from(*claim))
            .collect(),
        receipt_identity: receipt_identity.as_str().to_string(),
    })
}

fn receipt_hash_fields(draft: &AcpReceiptDraft, features: &[String]) -> Vec<String> {
    let mut fields = Vec::with_capacity(features.len().saturating_add(RECEIPT_BASE_FIELD_CAPACITY));
    fields.push(String::from(ACP_RECEIPT_SCHEMA));
    fields.push(String::from(ACP_PROTOCOL_COHORT));
    fields.push(String::from(ACP_PROTOCOL_VERSION));
    fields.extend(features.iter().cloned());
    fields.push(String::from(draft.operation.as_str()));
    fields.push(String::from(transport_status_label(draft.status)));
    fields.push(draft.failure.map_or_else(
        || String::from("none"),
        |failure| String::from(failure.as_str()),
    ));
    fields.push(draft.connection_identity.as_str().to_string());
    fields.push(optional_identity_label(
        draft.session_identity.as_ref().map(SessionIdentity::as_str),
    ));
    fields.push(optional_identity_label(
        draft
            .correlation_identity
            .as_ref()
            .map(AcpCorrelationId::as_str),
    ));
    fields.push(optional_identity_label(
        draft
            .authority_profile_identity
            .as_ref()
            .map(AuthorityProfileIdentity::as_str),
    ));
    fields.push(draft.frame_count.to_string());
    fields.push(draft.update_count.to_string());
    fields.push(draft.update_bytes.to_string());
    fields.push(draft.tool_count.to_string());
    fields.push(draft.permission_count.to_string());
    fields.push(String::from(cleanup_status_label(draft.cleanup)));
    fields.push(String::from(ACP_REDACTION_MARKER));
    fields
}

fn reject_sensitive(sensitive: AcpSensitiveDraft<'_>) -> Result<(), AcpProtocolError> {
    let fields = [
        sensitive.raw_frame,
        sensitive.prompt,
        sensitive.tool_arguments,
        sensitive.tool_output,
        sensitive.provider_payload,
        sensitive.credential,
        sensitive.environment_value,
        sensitive.physical_path,
        sensitive.hidden_reasoning,
        sensitive.raw_diagnostic,
    ];
    if fields.iter().any(Option::is_some) {
        Err(error(AcpFailureClass::SensitiveEvidence, FIELD_SENSITIVE))
    } else {
        Ok(())
    }
}

fn validate_optional_hash(value: Option<&str>) -> Result<(), AcpProtocolError> {
    if let Some(identity) = value {
        validate_hash(identity)?;
    }
    Ok(())
}

fn validate_hash(value: &str) -> Result<(), AcpProtocolError> {
    if value.len() != ACP_HASH_HEX_BYTES || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        Err(error(AcpFailureClass::SensitiveEvidence, FIELD_RECEIPT))
    } else {
        Ok(())
    }
}

fn optional_identity_label(value: Option<&str>) -> String {
    value.map_or_else(|| String::from("none"), String::from)
}

const fn transport_status_label(status: AcpTransportStatus) -> &'static str {
    match status {
        AcpTransportStatus::CleanEof => "clean_eof",
        AcpTransportStatus::Failed => "failed",
    }
}

const fn cleanup_status_label(status: AcpCleanupStatus) -> &'static str {
    match status {
        AcpCleanupStatus::Completed => "completed",
        AcpCleanupStatus::Failed => "failed",
        AcpCleanupStatus::NotRequired => "not_required",
    }
}

const fn error(failure: AcpFailureClass, field: &'static str) -> AcpProtocolError {
    AcpProtocolError { failure, field }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identities::{
        AcpConnectionIdentity, AcpCorrelationId, AuthorityProfileIdentity, SessionIdentity,
    };
    use crate::model::{AcpOperationKind, AcpTransportProfile};

    const HASH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const RAW_PROMPT: &str = "secret prompt";

    #[test]
    fn receipt_is_deterministic_and_metadata_only() {
        let draft = receipt_draft();
        let first = project_acp_receipt(
            &AcpConnectionProfile::reviewed(),
            &draft,
            AcpSensitiveDraft::default(),
        )
        .expect("first receipt");
        let second = project_acp_receipt(
            &AcpConnectionProfile::reviewed(),
            &draft,
            AcpSensitiveDraft::default(),
        )
        .expect("second receipt");

        assert_eq!(first, second);
        assert_eq!(first.receipt_identity.len(), ACP_HASH_HEX_BYTES);
        assert_eq!(first.redaction_marker, ACP_REDACTION_MARKER);
        assert_eq!(
            AcpConnectionProfile::reviewed().transport,
            AcpTransportProfile::reviewed()
        );
    }

    #[test]
    // r[verify agent_harness_runtime.acp_stdio_host.receipts]
    fn receipt_rejects_raw_prompt_and_inconsistent_success() {
        let sensitive = project_acp_receipt(
            &AcpConnectionProfile::reviewed(),
            &receipt_draft(),
            AcpSensitiveDraft {
                prompt: Some(RAW_PROMPT),
                ..AcpSensitiveDraft::default()
            },
        )
        .expect_err("raw prompt rejected");
        assert_eq!(sensitive.failure, AcpFailureClass::SensitiveEvidence);

        let inconsistent = project_acp_receipt(
            &AcpConnectionProfile::reviewed(),
            &AcpReceiptDraft {
                failure: Some(AcpFailureClass::BrokenPipe),
                ..receipt_draft()
            },
            AcpSensitiveDraft::default(),
        )
        .expect_err("success with failure rejected");
        assert_eq!(inconsistent.failure, AcpFailureClass::SensitiveEvidence);
    }

    fn receipt_draft() -> AcpReceiptDraft {
        AcpReceiptDraft {
            connection_identity: AcpConnectionIdentity::parse(HASH).expect("connection"),
            session_identity: Some(SessionIdentity::parse(HASH).expect("session")),
            correlation_identity: Some(AcpCorrelationId::parse(HASH).expect("correlation")),
            authority_profile_identity: Some(
                AuthorityProfileIdentity::parse(HASH).expect("authority"),
            ),
            operation: AcpOperationKind::Prompt,
            status: AcpTransportStatus::CleanEof,
            failure: None,
            cleanup: AcpCleanupStatus::Completed,
            frame_count: 1,
            update_count: 1,
            update_bytes: 1,
            tool_count: 0,
            permission_count: 0,
        }
    }
}
