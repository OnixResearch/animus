//! Raw ACP request admission into private nominal roles.

// r[impl agent_harness_runtime.acp_nominal_identities.wire_boundary]
use crate::identities::{
    AcpCorrelationId, AcpIdentityIssue, AcpSessionId, ResourceIdentity, SessionReference,
    WorkspaceIdentity,
};
use crate::model::{
    AcpDecodedRequest, AcpFailureClass, AcpFeature, AcpPromptPart, AcpProtocolError,
};

const FIELD_CORRELATION: &str = "correlation";
const FIELD_SESSION: &str = "session";
const FIELD_WORKSPACE: &str = "workspace";
const FIELD_RESOURCE: &str = "resource";

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum AcpAdmittedPromptPart {
    Text(String),
    ResourceLink {
        safe_name: String,
        resource_identity: ResourceIdentity,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum AcpAdmittedRequest {
    Initialize {
        correlation: AcpCorrelationId,
        protocol_version: String,
        required_features: Vec<AcpFeature>,
    },
    NewSession {
        correlation: AcpCorrelationId,
        workspace: WorkspaceIdentity,
        mcp_server_count: u32,
    },
    LoadSession {
        correlation: AcpCorrelationId,
        session_reference: SessionReference,
        workspace: WorkspaceIdentity,
        mcp_server_count: u32,
    },
    Prompt {
        correlation: AcpCorrelationId,
        session: AcpSessionId,
        parts: Vec<AcpAdmittedPromptPart>,
    },
    Cancel {
        session: AcpSessionId,
    },
    Unsupported {
        correlation: Option<AcpCorrelationId>,
        method: String,
    },
}

pub(crate) fn admit_request(
    raw: &AcpDecodedRequest,
) -> Result<AcpAdmittedRequest, AcpProtocolError> {
    match raw {
        AcpDecodedRequest::Initialize {
            correlation_id,
            protocol_version,
            required_features,
        } => Ok(AcpAdmittedRequest::Initialize {
            correlation: external(correlation_id, FIELD_CORRELATION)?,
            protocol_version: protocol_version.clone(),
            required_features: required_features.clone(),
        }),
        AcpDecodedRequest::NewSession {
            correlation_id,
            workspace_identity,
            mcp_server_count,
        } => Ok(AcpAdmittedRequest::NewSession {
            correlation: external(correlation_id, FIELD_CORRELATION)?,
            workspace: blake3(workspace_identity, FIELD_WORKSPACE)?,
            mcp_server_count: *mcp_server_count,
        }),
        AcpDecodedRequest::LoadSession {
            correlation_id,
            session_reference,
            workspace_identity,
            mcp_server_count,
        } => Ok(AcpAdmittedRequest::LoadSession {
            correlation: external(correlation_id, FIELD_CORRELATION)?,
            session_reference: external(session_reference, FIELD_SESSION)?,
            workspace: blake3(workspace_identity, FIELD_WORKSPACE)?,
            mcp_server_count: *mcp_server_count,
        }),
        AcpDecodedRequest::Prompt {
            correlation_id,
            session_id,
            parts,
        } => Ok(AcpAdmittedRequest::Prompt {
            correlation: external(correlation_id, FIELD_CORRELATION)?,
            session: external(session_id, FIELD_SESSION)?,
            parts: admit_parts(parts)?,
        }),
        AcpDecodedRequest::Cancel { session_id } => Ok(AcpAdmittedRequest::Cancel {
            session: external(session_id, FIELD_SESSION)?,
        }),
        AcpDecodedRequest::Unsupported {
            correlation_id,
            method,
        } => Ok(AcpAdmittedRequest::Unsupported {
            correlation: correlation_id
                .as_deref()
                .map(|value| external(value, FIELD_CORRELATION))
                .transpose()?,
            method: method.clone(),
        }),
    }
}

fn admit_parts(parts: &[AcpPromptPart]) -> Result<Vec<AcpAdmittedPromptPart>, AcpProtocolError> {
    parts
        .iter()
        .map(|part| match part {
            AcpPromptPart::Text(text) => Ok(AcpAdmittedPromptPart::Text(text.clone())),
            AcpPromptPart::ResourceLink {
                safe_name,
                resource_identity,
            } => Ok(AcpAdmittedPromptPart::ResourceLink {
                safe_name: safe_name.clone(),
                resource_identity: blake3(resource_identity, FIELD_RESOURCE)?,
            }),
        })
        .collect()
}

fn external<T>(value: &str, field: &'static str) -> Result<T, AcpProtocolError>
where
    T: ExternalIdentity,
{
    T::parse(value).map_err(|issue| identity_error(issue, field))
}

fn blake3<T>(value: &str, field: &'static str) -> Result<T, AcpProtocolError>
where
    T: Blake3Identity,
{
    T::parse(value).map_err(|issue| identity_error(issue, field))
}

trait ExternalIdentity: Sized {
    fn parse(value: &str) -> Result<Self, AcpIdentityIssue>;
}

trait Blake3Identity: Sized {
    fn parse(value: &str) -> Result<Self, AcpIdentityIssue>;
}

impl ExternalIdentity for AcpCorrelationId {
    fn parse(value: &str) -> Result<Self, AcpIdentityIssue> {
        Self::parse(value)
    }
}
impl ExternalIdentity for AcpSessionId {
    fn parse(value: &str) -> Result<Self, AcpIdentityIssue> {
        Self::parse(value)
    }
}
impl ExternalIdentity for SessionReference {
    fn parse(value: &str) -> Result<Self, AcpIdentityIssue> {
        Self::parse(value)
    }
}
impl Blake3Identity for WorkspaceIdentity {
    fn parse(value: &str) -> Result<Self, AcpIdentityIssue> {
        Self::parse(value)
    }
}
impl Blake3Identity for ResourceIdentity {
    fn parse(value: &str) -> Result<Self, AcpIdentityIssue> {
        Self::parse(value)
    }
}

const fn identity_error(_issue: AcpIdentityIssue, field: &'static str) -> AcpProtocolError {
    AcpProtocolError {
        failure: AcpFailureClass::MalformedFrame,
        field,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HASH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    #[test]
    fn raw_request_admits_typed_roles() {
        // r[verify agent_harness_runtime.acp_nominal_identities.wire_boundary.valid]
        let admitted = admit_request(&AcpDecodedRequest::NewSession {
            correlation_id: "7".to_string(),
            workspace_identity: HASH.to_string(),
            mcp_server_count: 0,
        })
        .expect("typed request");
        assert!(matches!(admitted, AcpAdmittedRequest::NewSession { .. }));
    }

    #[test]
    fn malformed_identity_fails_before_state_use() {
        // r[verify agent_harness_runtime.acp_nominal_identities.wire_boundary.invalid]
        let error = admit_request(&AcpDecodedRequest::Prompt {
            correlation_id: "bad correlation".to_string(),
            session_id: "session.one".to_string(),
            parts: vec![AcpPromptPart::Text("hello".to_string())],
        })
        .expect_err("malformed identity");
        assert_eq!(error.failure, AcpFailureClass::MalformedFrame);
        assert_eq!(error.field, FIELD_CORRELATION);
    }
}
