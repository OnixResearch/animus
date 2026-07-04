/// Scheduled run denial reason.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScheduledRunDenial {
    /// Recipe is not trusted.
    UntrustedRecipe,
    /// Capability policy denied the run.
    Capability,
    /// Sandbox policy denied the run.
    Sandbox,
    /// Queue service is missing or denied.
    QueueService,
    /// Cancellation handle is missing.
    CancellationHandle,
    /// Policy snapshot is stale.
    StalePolicy,
}

/// Scheduled run admission input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScheduledRunAdmissionDraft {
    /// Whether recipe trust is admitted.
    pub recipe_trusted: bool,
    /// Whether capability policy allows the run.
    pub capability_allowed: bool,
    /// Whether sandbox policy allows the run.
    pub sandbox_allowed: bool,
    /// Whether queue service admitted the run.
    pub queue_admitted: bool,
    /// Whether a cancellation handle is available.
    pub cancellation_handle: bool,
    /// Whether active policy is current.
    pub policy_current: bool,
}

/// Scheduled run admission status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScheduledRunAdmissionStatus {
    /// Run is admitted for host dispatch.
    Admitted,
    /// Run is denied before dispatch.
    Denied {
        /// Denial reason.
        reason: ScheduledRunDenial,
    },
}

/// Evaluates scheduled run admission before dispatch.
#[must_use]
pub const fn admit_scheduled_run(draft: ScheduledRunAdmissionDraft) -> ScheduledRunAdmissionStatus {
    match scheduled_denial_reason(draft) {
        Some(reason) => ScheduledRunAdmissionStatus::Denied { reason },
        None => ScheduledRunAdmissionStatus::Admitted,
    }
}

const fn scheduled_denial_reason(draft: ScheduledRunAdmissionDraft) -> Option<ScheduledRunDenial> {
    match trust_or_policy_denial(draft) {
        Some(reason) => Some(reason),
        None => runtime_denial(draft),
    }
}

const fn trust_or_policy_denial(draft: ScheduledRunAdmissionDraft) -> Option<ScheduledRunDenial> {
    if !draft.recipe_trusted {
        return Some(ScheduledRunDenial::UntrustedRecipe);
    }
    if !draft.capability_allowed {
        return Some(ScheduledRunDenial::Capability);
    }
    if !draft.sandbox_allowed {
        return Some(ScheduledRunDenial::Sandbox);
    }
    None
}

const fn runtime_denial(draft: ScheduledRunAdmissionDraft) -> Option<ScheduledRunDenial> {
    if !draft.queue_admitted {
        return Some(ScheduledRunDenial::QueueService);
    }
    if !draft.cancellation_handle {
        return Some(ScheduledRunDenial::CancellationHandle);
    }
    if !draft.policy_current {
        return Some(ScheduledRunDenial::StalePolicy);
    }
    None
}
