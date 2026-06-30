const MODEL_COUNT_LIMIT: u32 = u32::MAX;

/// Pure model input for the authorization issue decision boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct AuthorizationModelInput {
    /// Whether the reviewed policy contract permits the request shape.
    pub(crate) contract_permits: bool,
    /// Number of verified grants matching the request.
    pub(crate) matching_grants: u32,
    /// Number of local policy caveats that deny the request.
    pub(crate) local_denies: u32,
    /// Required sandbox state after descriptor validation.
    pub(crate) sandbox: AuthorizationModelSandbox,
}

/// Pure model sandbox state for authorization issue selection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AuthorizationModelSandbox {
    /// No sandbox constraint applies.
    NotRequired,
    /// Required sandbox is enforceable.
    Enforceable,
    /// Required sandbox descriptor is malformed.
    Malformed,
    /// Required sandbox cannot be enforced by the selected adapter.
    Unavailable,
}

impl AuthorizationModelInput {
    /// Builds model input from the production authorization context without I/O.
    #[must_use]
    pub(crate) fn from_context(context: &super::AuthorizationContext<'_>) -> Self {
        Self {
            contract_permits: context.contract.permits(context.request),
            matching_grants: model_count(
                context
                    .grants
                    .iter()
                    .filter(|grant| grant.authorizes(context.request))
                    .count(),
            ),
            local_denies: model_count(
                context
                    .caveats
                    .iter()
                    .filter(|caveat| matches!(caveat.decision(), super::LocalCaveatDecision::Deny))
                    .count(),
            ),
            sandbox: AuthorizationModelSandbox::from_constraint(context.sandbox),
        }
    }
}

impl AuthorizationModelSandbox {
    fn from_constraint(sandbox: &super::SandboxConstraint) -> Self {
        match sandbox {
            super::SandboxConstraint::NotRequired => Self::NotRequired,
            super::SandboxConstraint::Required {
                state: super::SandboxState::Enforceable,
                ..
            } => Self::Enforceable,
            super::SandboxConstraint::Required {
                state: super::SandboxState::Malformed,
                ..
            } => Self::Malformed,
            super::SandboxConstraint::Required {
                state: super::SandboxState::Unavailable,
                ..
            } => Self::Unavailable,
        }
    }
}

/// Selects the expected authorization issue for pure model inputs.
#[must_use]
pub(crate) const fn model_authorization_issue(
    input: AuthorizationModelInput,
) -> super::AuthorizationIssue {
    if !input.contract_permits {
        return super::AuthorizationIssue::ContractMismatch;
    }
    if input.local_denies > 0 {
        return super::AuthorizationIssue::LocalPolicyDenied;
    }
    if input.matching_grants == 0 {
        return super::AuthorizationIssue::MissingUcanGrant;
    }
    match input.sandbox {
        AuthorizationModelSandbox::NotRequired | AuthorizationModelSandbox::Enforceable => {
            super::AuthorizationIssue::None
        }
        AuthorizationModelSandbox::Malformed => super::AuthorizationIssue::SandboxMalformed,
        AuthorizationModelSandbox::Unavailable => super::AuthorizationIssue::SandboxUnavailable,
    }
}

fn model_count(count: usize) -> u32 {
    u32::try_from(count).unwrap_or(MODEL_COUNT_LIMIT)
}
