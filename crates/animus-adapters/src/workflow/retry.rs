const CHECK_LABEL_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("recipe_retry_check_label");
const EFFECT_KIND_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("recipe_retry_effect_kind");
const CAPABILITY_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("recipe_retry_capability");
const SANDBOX_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("recipe_retry_sandbox");
const TIMEOUT_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("recipe_retry_timeout");
const CLEANUP_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("recipe_retry_cleanup");
const RECEIPT_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("recipe_retry_receipt");

const EFFECT_VALIDATION_CHECK: &str = "validation_check";
const EFFECT_CLEANUP_REQUEST: &str = "cleanup_request";
const EFFECT_SHELL: &str = "shell";
const MIN_RETRY_ATTEMPTS: u32 = 1;

type Text = alloc::string::String;
type Collection<T> = alloc::vec::Vec<T>;

/// Untrusted retry policy metadata.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecipeRetryPolicyDraft<'a> {
    /// Maximum bounded retry attempts.
    pub max_attempts: u32,
    /// Host-mediated checks and cleanup requests.
    pub checks: &'a [RecipeRetryCheckDraft<'a>],
}

/// Untrusted retry check metadata.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecipeRetryCheckDraft<'a> {
    /// Stable check label.
    pub label: &'a str,
    /// Host effect kind label.
    pub effect_kind: &'a str,
    /// Capability label required to request the effect.
    pub capability: &'a str,
    /// Sandbox label required for the effect.
    pub sandbox: &'a str,
    /// Timeout class label required before the host may run the effect.
    pub timeout_class: &'a str,
    /// Optional cleanup request label.
    pub cleanup_request: Option<&'a str>,
    /// Safe receipt label for the effect request.
    pub receipt_label: &'a str,
}

/// Supported host-mediated retry effect kinds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecipeHostEffectKind {
    /// Validation check requested from the host.
    ValidationCheck,
    /// Cleanup request represented as data for the host.
    CleanupRequest,
}

impl RecipeHostEffectKind {
    /// Parses a stable retry effect kind label.
    ///
    /// # Errors
    ///
    /// Returns [`super::RecipeWorkflowError`] when the label is empty, unsupported,
    /// or explicitly asks reusable core to run a shell effect.
    pub fn parse(value: &str) -> Result<Self, super::RecipeWorkflowError> {
        let normalized = super::required_text(EFFECT_KIND_FIELD, value)?;
        if normalized.as_str() == EFFECT_SHELL {
            return Err(super::RecipeWorkflowError::UnsafeRetryEffect { effect: normalized });
        }
        Self::from_normalized(normalized.as_str()).ok_or(
            super::RecipeWorkflowError::UnsupportedWorkflowValue {
                field: EFFECT_KIND_FIELD.as_str(),
                value: normalized,
            },
        )
    }

    fn from_normalized(value: &str) -> Option<Self> {
        match value {
            EFFECT_VALIDATION_CHECK => Some(Self::ValidationCheck),
            EFFECT_CLEANUP_REQUEST => Some(Self::CleanupRequest),
            _ => None,
        }
    }

    /// Returns the stable retry effect kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ValidationCheck => EFFECT_VALIDATION_CHECK,
            Self::CleanupRequest => EFFECT_CLEANUP_REQUEST,
        }
    }
}

/// Validated retry policy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecipeRetryPolicy {
    max_attempts: u32,
    checks: Collection<RecipeRetryCheck>,
}

impl RecipeRetryPolicy {
    /// Validates retry metadata as host-effect requests only.
    ///
    /// # Errors
    ///
    /// Returns [`super::RecipeWorkflowError`] when attempts are unbounded, checks
    /// are missing, labels are malformed, shell effects are requested, or timeout
    /// and receipt labels are absent.
    pub fn validate(
        draft: &RecipeRetryPolicyDraft<'_>,
    ) -> Result<Self, super::RecipeWorkflowError> {
        if draft.max_attempts < MIN_RETRY_ATTEMPTS {
            return Err(super::RecipeWorkflowError::ZeroRetryAttempts);
        }
        if draft.checks.is_empty() {
            return Err(super::RecipeWorkflowError::EmptyCollection {
                field: CHECK_LABEL_FIELD.as_str(),
            });
        }
        let checks = collect_checks(draft.checks)?;
        Ok(Self {
            max_attempts: draft.max_attempts,
            checks,
        })
    }

    /// Returns the maximum retry attempts.
    #[must_use]
    pub const fn max_attempts(&self) -> u32 {
        self.max_attempts
    }

    /// Returns validated host-mediated checks.
    #[must_use]
    pub fn checks(&self) -> &[RecipeRetryCheck] {
        self.checks.as_slice()
    }
}

/// Validated retry check metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecipeRetryCheck {
    label: Text,
    effect_kind: RecipeHostEffectKind,
    capability: Text,
    sandbox: Text,
    timeout_class: Text,
    cleanup_request: Option<Text>,
    receipt_label: Text,
}

impl RecipeRetryCheck {
    fn validate(draft: &RecipeRetryCheckDraft<'_>) -> Result<Self, super::RecipeWorkflowError> {
        let label = super::required_text(CHECK_LABEL_FIELD, draft.label)?;
        let effect_kind = RecipeHostEffectKind::parse(draft.effect_kind)?;
        let capability = super::required_text(CAPABILITY_FIELD, draft.capability)?;
        let sandbox = super::required_text(SANDBOX_FIELD, draft.sandbox)?;
        let timeout_class = super::required_text(TIMEOUT_FIELD, draft.timeout_class)?;
        let cleanup_request = super::optional_text(CLEANUP_FIELD, draft.cleanup_request)?;
        let receipt_label = super::required_text(RECEIPT_FIELD, draft.receipt_label)?;

        Ok(Self {
            label,
            effect_kind,
            capability,
            sandbox,
            timeout_class,
            cleanup_request,
            receipt_label,
        })
    }

    /// Returns the retry check label.
    #[must_use]
    pub fn label(&self) -> &str {
        self.label.as_str()
    }

    /// Returns the host effect kind.
    #[must_use]
    pub const fn effect_kind(&self) -> RecipeHostEffectKind {
        self.effect_kind
    }

    /// Returns the required capability label.
    #[must_use]
    pub fn capability(&self) -> &str {
        self.capability.as_str()
    }

    /// Returns the required sandbox label.
    #[must_use]
    pub fn sandbox(&self) -> &str {
        self.sandbox.as_str()
    }

    /// Returns the timeout class label.
    #[must_use]
    pub fn timeout_class(&self) -> &str {
        self.timeout_class.as_str()
    }

    /// Returns the cleanup request label, when declared.
    #[must_use]
    pub fn cleanup_request(&self) -> Option<&str> {
        self.cleanup_request.as_deref()
    }

    /// Returns the safe receipt label.
    #[must_use]
    pub fn receipt_label(&self) -> &str {
        self.receipt_label.as_str()
    }
}

/// Data required to classify retry requests against active host denials.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecipeRetryPlanDraft<'a> {
    /// Validated retry policy.
    pub policy: &'a RecipeRetryPolicy,
    /// Capability labels currently denied by host policy.
    pub denied_capabilities: &'a [&'a str],
    /// Sandbox labels currently denied by host policy.
    pub denied_sandboxes: &'a [&'a str],
}

/// Retry plan status for one host-effect request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecipeRetryRequestStatus {
    /// Host may evaluate the request; reusable core has not executed it.
    PendingHostEffect,
    /// Host policy denied the request before execution.
    Denied {
        /// Denial reason class.
        reason: RecipeRetryDenialReason,
    },
}

/// Retry request denial reason.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecipeRetryDenialReason {
    /// Active capability policy denied the request.
    Capability,
    /// Active sandbox policy denied the request.
    Sandbox,
}

/// Planned retry host-effect request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecipeRetryRequest {
    label: Text,
    status: RecipeRetryRequestStatus,
}

impl RecipeRetryRequest {
    /// Returns the retry check label.
    #[must_use]
    pub fn label(&self) -> &str {
        self.label.as_str()
    }

    /// Returns the classified host-effect status.
    #[must_use]
    pub const fn status(&self) -> RecipeRetryRequestStatus {
        self.status
    }
}

/// Pure retry plan; it records requests but never reports validation success.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecipeRetryPlan {
    requests: Collection<RecipeRetryRequest>,
}

impl RecipeRetryPlan {
    /// Returns planned host-effect requests.
    #[must_use]
    pub fn requests(&self) -> &[RecipeRetryRequest] {
        self.requests.as_slice()
    }
}

/// Builds a retry plan from validated metadata and active host denials.
#[must_use]
pub fn plan_recipe_retry(draft: RecipeRetryPlanDraft<'_>) -> RecipeRetryPlan {
    let mut requests: Collection<RecipeRetryRequest> =
        Collection::with_capacity(draft.policy.checks().len());
    for check in draft.policy.checks() {
        let status =
            classify_retry_request(check, draft.denied_capabilities, draft.denied_sandboxes);
        requests.push(RecipeRetryRequest {
            label: Text::from(check.label()),
            status,
        });
    }
    RecipeRetryPlan { requests }
}

fn collect_checks(
    drafts: &[RecipeRetryCheckDraft<'_>],
) -> Result<Collection<RecipeRetryCheck>, super::RecipeWorkflowError> {
    let mut checks: Collection<RecipeRetryCheck> = Collection::with_capacity(drafts.len());
    for draft in drafts {
        let check = RecipeRetryCheck::validate(draft)?;
        if checks
            .iter()
            .any(|existing| existing.label() == check.label())
        {
            return Err(super::RecipeWorkflowError::DuplicateName {
                field: CHECK_LABEL_FIELD.as_str(),
                value: Text::from(check.label()),
            });
        }
        checks.push(check);
    }
    Ok(checks)
}

fn classify_retry_request(
    check: &RecipeRetryCheck,
    denied_capabilities: &[&str],
    denied_sandboxes: &[&str],
) -> RecipeRetryRequestStatus {
    if denied_capabilities
        .iter()
        .any(|denied| *denied == check.capability())
    {
        return RecipeRetryRequestStatus::Denied {
            reason: RecipeRetryDenialReason::Capability,
        };
    }
    if denied_sandboxes
        .iter()
        .any(|denied| *denied == check.sandbox())
    {
        return RecipeRetryRequestStatus::Denied {
            reason: RecipeRetryDenialReason::Sandbox,
        };
    }
    RecipeRetryRequestStatus::PendingHostEffect
}
