//! Data-only scheduled recipe workflow DTOs and validation helpers.

const ID_FIELD: crate::validation::FieldName = crate::validation::FieldName::new("schedule_id");
const RECIPE_FIELD: crate::validation::FieldName = crate::validation::FieldName::new("recipe_id");
const HASH_FIELD: crate::validation::FieldName = crate::validation::FieldName::new("schedule_hash");
const PARAMETER_HASH_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("parameter_hash");
const CADENCE_FIELD: crate::validation::FieldName = crate::validation::FieldName::new("cadence");
const CALENDAR_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("calendar_policy");
const EXECUTION_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("execution_mode");
const CONCURRENCY_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("concurrency_policy");
const WORKSPACE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("workspace_scope");
const OUTCOME_FIELD: crate::validation::FieldName = crate::validation::FieldName::new("outcome");
const REDACTION_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("redaction");
const ACTION_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("control_action");

const CADENCE_RUN_NOW: &str = "run_now";
const CADENCE_HOURLY: &str = "hourly";
const CADENCE_DAILY: &str = "daily";
const EXECUTION_BACKGROUND: &str = "background";
const EXECUTION_FOREGROUND: &str = "foreground";
const CONCURRENCY_SERIALIZE: &str = "serialize";
const CONCURRENCY_ISOLATE: &str = "isolate";
const CONCURRENCY_DENY: &str = "deny";
const CONTROL_PAUSE: &str = "pause";
const CONTROL_RESUME: &str = "resume";
const CONTROL_EDIT: &str = "edit";
const CONTROL_RUN_NOW: &str = "run_now";
const CONTROL_CANCEL: &str = "cancel";
const BLAKE3_HEX_CHARS: usize = 64;
const ASCII_HEX_RADIX: u32 = 16;
const MIN_RETRY_LIMIT: u32 = 0;
const MIN_RETENTION_COUNT: u32 = 1;
const MIN_INTERVAL_TICKS: u64 = 1;
const TICK_INCREMENT: u64 = 1;

type Text = alloc::string::String;
type Collection<T> = alloc::vec::Vec<T>;

/// Effects that scheduled recipe validation never performs in reusable core.
pub const PROHIBITED_SCHEDULE_EFFECTS: &[&str] = &[
    "timer start",
    "daemon startup",
    "cron edit",
    "process launch",
    "filesystem mutation",
    "environment read",
];

/// Scheduled recipe validation error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ScheduleError {
    /// Generic adapter contract validation failed.
    Contract(crate::AdapterContractError),
    /// A schedule field contained an unsupported value.
    UnsupportedValue {
        /// Boundary field name.
        field: &'static str,
        /// Unsupported normalized value.
        value: Text,
    },
    /// A required collection was empty.
    EmptyCollection {
        /// Boundary field name.
        field: &'static str,
    },
    /// A collection contained duplicate values.
    DuplicateValue {
        /// Boundary field name.
        field: &'static str,
        /// Duplicated value.
        value: Text,
    },
    /// A BLAKE3 hash was malformed.
    InvalidHash {
        /// Boundary field name.
        field: &'static str,
    },
    /// Recipe trust was missing or pending.
    UntrustedRecipe,
    /// Retry or retention bounds were invalid.
    InvalidBounds,
    /// Next-run planning received an invalid interval.
    InvalidPlanningInput,
    /// Run admission denied execution.
    RunDenied {
        /// Denial reason.
        reason: ScheduledRunDenial,
    },
    /// Control request lacked host admission or confirmation.
    ControlDenied,
    /// Rendered record leaked raw material.
    RecordLeak,
}

impl From<crate::AdapterContractError> for ScheduleError {
    fn from(error: crate::AdapterContractError) -> Self {
        Self::Contract(error)
    }
}

/// Scheduled workflow cadence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScheduleCadence {
    /// One immediate run.
    RunNow,
    /// Hourly recurring run.
    Hourly,
    /// Daily recurring run.
    Daily,
}

impl ScheduleCadence {
    /// Parses a stable cadence label.
    ///
    /// # Errors
    ///
    /// Returns [`ScheduleError`] when the label is empty or unsupported.
    pub fn parse(value: &str) -> Result<Self, ScheduleError> {
        parse_value(CADENCE_FIELD, value, Self::from_normalized)
    }

    fn from_normalized(value: &str) -> Option<Self> {
        match value {
            CADENCE_RUN_NOW => Some(Self::RunNow),
            CADENCE_HOURLY => Some(Self::Hourly),
            CADENCE_DAILY => Some(Self::Daily),
            _ => None,
        }
    }
}

/// Scheduled workflow execution mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScheduleExecutionMode {
    /// Host may run through a background queue.
    Background,
    /// Host may run in an explicit foreground flow.
    Foreground,
}

impl ScheduleExecutionMode {
    /// Parses a stable execution mode label.
    ///
    /// # Errors
    ///
    /// Returns [`ScheduleError`] when the label is empty or unsupported.
    pub fn parse(value: &str) -> Result<Self, ScheduleError> {
        parse_value(EXECUTION_FIELD, value, Self::from_normalized)
    }

    fn from_normalized(value: &str) -> Option<Self> {
        match value {
            EXECUTION_BACKGROUND => Some(Self::Background),
            EXECUTION_FOREGROUND => Some(Self::Foreground),
            _ => None,
        }
    }
}

/// Concurrency policy for overlapping scheduled runs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScheduleConcurrencyPolicy {
    /// Serialize overlapping writes.
    Serialize,
    /// Run in isolated workspace when overlap occurs.
    Isolate,
    /// Deny overlapping writes.
    Deny,
}

impl ScheduleConcurrencyPolicy {
    /// Parses a stable concurrency policy label.
    ///
    /// # Errors
    ///
    /// Returns [`ScheduleError`] when the label is empty or unsupported.
    pub fn parse(value: &str) -> Result<Self, ScheduleError> {
        parse_value(CONCURRENCY_FIELD, value, Self::from_normalized)
    }

    fn from_normalized(value: &str) -> Option<Self> {
        match value {
            CONCURRENCY_SERIALIZE => Some(Self::Serialize),
            CONCURRENCY_ISOLATE => Some(Self::Isolate),
            CONCURRENCY_DENY => Some(Self::Deny),
            _ => None,
        }
    }
}

/// Untrusted scheduled workflow descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScheduledWorkflowDraft<'a> {
    /// Schedule identity.
    pub schedule_id: &'a str,
    /// Recipe identity.
    pub recipe_id: &'a str,
    /// Recipe version hash.
    pub recipe_version_hash: &'a str,
    /// Parameter value hashes.
    pub parameter_hashes: &'a [&'a str],
    /// Cadence label.
    pub cadence: &'a str,
    /// Timezone or calendar policy label.
    pub calendar_policy: &'a str,
    /// Execution mode label.
    pub execution_mode: &'a str,
    /// Concurrency policy label.
    pub concurrency_policy: &'a str,
    /// Workspace scope label.
    pub workspace_scope: &'a str,
    /// Retry limit.
    pub retry_limit: u32,
    /// Retention count.
    pub retention_count: u32,
    /// Provenance hash.
    pub provenance_hash: &'a str,
    /// Recipe trust receipt.
    pub recipe_trust: Option<&'a crate::RecipeTrustReceipt>,
}

/// Validated scheduled workflow descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScheduledWorkflow {
    schedule_id: Text,
    recipe_id: Text,
    cadence: ScheduleCadence,
    execution_mode: ScheduleExecutionMode,
    concurrency_policy: ScheduleConcurrencyPolicy,
    workspace_scope: Text,
    retry_limit: u32,
    retention_count: u32,
}

impl ScheduledWorkflow {
    /// Validates a scheduled workflow descriptor without starting a timer or queue task.
    ///
    /// # Errors
    ///
    /// Returns [`ScheduleError`] when fields are malformed, hashes are invalid,
    /// recipe trust is missing, cadence is unsupported, or bounds are invalid.
    pub fn validate(draft: &ScheduledWorkflowDraft<'_>) -> Result<Self, ScheduleError> {
        let receipt = draft.recipe_trust.ok_or(ScheduleError::UntrustedRecipe)?;
        if receipt.status() != crate::RecipeTrustStatus::Admitted {
            return Err(ScheduleError::UntrustedRecipe);
        }
        let schedule_id = required_text(ID_FIELD, draft.schedule_id)?;
        let recipe_id = required_text(RECIPE_FIELD, draft.recipe_id)?;
        validate_hash(HASH_FIELD.as_str(), draft.recipe_version_hash)?;
        validate_hashes(PARAMETER_HASH_FIELD, draft.parameter_hashes, true)?;
        let cadence = ScheduleCadence::parse(draft.cadence)?;
        required_text(CALENDAR_FIELD, draft.calendar_policy)?;
        let execution_mode = ScheduleExecutionMode::parse(draft.execution_mode)?;
        let concurrency_policy = ScheduleConcurrencyPolicy::parse(draft.concurrency_policy)?;
        let workspace_scope = required_text(WORKSPACE_FIELD, draft.workspace_scope)?;
        if draft.retry_limit < MIN_RETRY_LIMIT || draft.retention_count < MIN_RETENTION_COUNT {
            return Err(ScheduleError::InvalidBounds);
        }
        validate_hash(HASH_FIELD.as_str(), draft.provenance_hash)?;
        Ok(Self {
            schedule_id,
            recipe_id,
            cadence,
            execution_mode,
            concurrency_policy,
            workspace_scope,
            retry_limit: draft.retry_limit,
            retention_count: draft.retention_count,
        })
    }

    /// Returns schedule identity.
    #[must_use]
    pub fn schedule_id(&self) -> &str {
        self.schedule_id.as_str()
    }

    /// Returns recipe identity.
    #[must_use]
    pub fn recipe_id(&self) -> &str {
        self.recipe_id.as_str()
    }

    /// Returns cadence.
    #[must_use]
    pub const fn cadence(&self) -> ScheduleCadence {
        self.cadence
    }

    /// Returns execution mode.
    #[must_use]
    pub const fn execution_mode(&self) -> ScheduleExecutionMode {
        self.execution_mode
    }

    /// Returns concurrency policy.
    #[must_use]
    pub const fn concurrency_policy(&self) -> ScheduleConcurrencyPolicy {
        self.concurrency_policy
    }

    /// Returns workspace scope label.
    #[must_use]
    pub fn workspace_scope(&self) -> &str {
        self.workspace_scope.as_str()
    }

    /// Returns retry limit.
    #[must_use]
    pub const fn retry_limit(&self) -> u32 {
        self.retry_limit
    }

    /// Returns retention count.
    #[must_use]
    pub const fn retention_count(&self) -> u32 {
        self.retention_count
    }
}

/// Explicit next-run planning input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NextRunInput<'a> {
    /// Validated schedule.
    pub schedule: &'a ScheduledWorkflow,
    /// Explicit current logical tick.
    pub current_tick: u64,
    /// Explicit recurrence interval in ticks.
    pub interval_ticks: u64,
}

/// Deterministic next-run plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NextRunPlan {
    next_tick: u64,
}

impl NextRunPlan {
    /// Returns the planned next logical tick.
    #[must_use]
    pub const fn next_tick(self) -> u64 {
        self.next_tick
    }
}

/// Plans the next run over explicit logical clock inputs only.
///
/// # Errors
///
/// Returns [`ScheduleError`] when interval inputs are invalid.
pub fn plan_next_run(input: NextRunInput<'_>) -> Result<NextRunPlan, ScheduleError> {
    if input.interval_ticks < MIN_INTERVAL_TICKS {
        return Err(ScheduleError::InvalidPlanningInput);
    }
    let next_tick = match input.schedule.cadence() {
        ScheduleCadence::RunNow => input.current_tick,
        ScheduleCadence::Hourly | ScheduleCadence::Daily => {
            input.current_tick + input.interval_ticks
        }
    };
    Ok(NextRunPlan { next_tick })
}

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
    if !draft.recipe_trusted {
        return ScheduledRunAdmissionStatus::Denied {
            reason: ScheduledRunDenial::UntrustedRecipe,
        };
    }
    if !draft.capability_allowed {
        return ScheduledRunAdmissionStatus::Denied {
            reason: ScheduledRunDenial::Capability,
        };
    }
    if !draft.sandbox_allowed {
        return ScheduledRunAdmissionStatus::Denied {
            reason: ScheduledRunDenial::Sandbox,
        };
    }
    if !draft.queue_admitted {
        return ScheduledRunAdmissionStatus::Denied {
            reason: ScheduledRunDenial::QueueService,
        };
    }
    if !draft.cancellation_handle {
        return ScheduledRunAdmissionStatus::Denied {
            reason: ScheduledRunDenial::CancellationHandle,
        };
    }
    if !draft.policy_current {
        return ScheduledRunAdmissionStatus::Denied {
            reason: ScheduledRunDenial::StalePolicy,
        };
    }
    ScheduledRunAdmissionStatus::Admitted
}

/// Concurrency decision for overlapping runs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScheduledConcurrencyDecision {
    /// Start immediately.
    Start,
    /// Serialize behind active run.
    Serialize,
    /// Start in isolated workspace.
    Isolate,
    /// Deny due to overlap.
    Deny,
}

/// Concurrency evaluation input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScheduledConcurrencyInput {
    /// Concurrency policy.
    pub policy: ScheduleConcurrencyPolicy,
    /// Whether another run is active.
    pub active_run: bool,
    /// Whether active run may write.
    pub active_run_writes: bool,
    /// Whether new run may write.
    pub new_run_writes: bool,
}

/// Evaluates overlapping scheduled-run concurrency.
#[must_use]
pub const fn evaluate_scheduled_concurrency(
    input: ScheduledConcurrencyInput,
) -> ScheduledConcurrencyDecision {
    if !input.active_run || !input.active_run_writes || !input.new_run_writes {
        return ScheduledConcurrencyDecision::Start;
    }
    match input.policy {
        ScheduleConcurrencyPolicy::Serialize => ScheduledConcurrencyDecision::Serialize,
        ScheduleConcurrencyPolicy::Isolate => ScheduledConcurrencyDecision::Isolate,
        ScheduleConcurrencyPolicy::Deny => ScheduledConcurrencyDecision::Deny,
    }
}

/// Untrusted scheduled run record.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScheduledRunRecordDraft<'a> {
    /// Schedule identity hash.
    pub schedule_hash: &'a str,
    /// Recipe identity hash.
    pub recipe_hash: &'a str,
    /// Parameter hashes.
    pub parameter_hashes: &'a [&'a str],
    /// Queue or job identity hash.
    pub queue_job_hash: &'a str,
    /// Session ledger hash.
    pub ledger_hash: &'a str,
    /// Retry count used.
    pub retry_count: u32,
    /// Outcome label.
    pub outcome: &'a str,
    /// Safe evidence hashes.
    pub evidence_hashes: &'a [&'a str],
    /// Redaction markers.
    pub redactions: &'a [&'a str],
}

/// Safe scheduled run record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScheduledRunRecord {
    outcome: Text,
    retry_count: u32,
    parameter_count: u32,
    evidence_count: u32,
    redactions: Collection<Text>,
}

impl ScheduledRunRecord {
    /// Builds a safe scheduled run record.
    ///
    /// # Errors
    ///
    /// Returns [`ScheduleError`] when hashes, outcome, or redactions are malformed.
    pub fn new(draft: &ScheduledRunRecordDraft<'_>) -> Result<Self, ScheduleError> {
        validate_hash(HASH_FIELD.as_str(), draft.schedule_hash)?;
        validate_hash(RECIPE_FIELD.as_str(), draft.recipe_hash)?;
        validate_hashes(PARAMETER_HASH_FIELD, draft.parameter_hashes, true)?;
        validate_hash(HASH_FIELD.as_str(), draft.queue_job_hash)?;
        validate_hash(HASH_FIELD.as_str(), draft.ledger_hash)?;
        validate_hashes(HASH_FIELD, draft.evidence_hashes, true)?;
        let outcome = required_text(OUTCOME_FIELD, draft.outcome)?;
        let redactions = text_collection(REDACTION_FIELD, draft.redactions, true)?;
        Ok(Self {
            outcome,
            retry_count: draft.retry_count,
            parameter_count: draft.parameter_hashes.len() as u32,
            evidence_count: draft.evidence_hashes.len() as u32,
            redactions,
        })
    }

    /// Returns outcome label.
    #[must_use]
    pub fn outcome(&self) -> &str {
        self.outcome.as_str()
    }

    /// Returns retry count.
    #[must_use]
    pub const fn retry_count(&self) -> u32 {
        self.retry_count
    }

    /// Returns parameter hash count.
    #[must_use]
    pub const fn parameter_count(&self) -> u32 {
        self.parameter_count
    }

    /// Returns evidence hash count.
    #[must_use]
    pub const fn evidence_count(&self) -> u32 {
        self.evidence_count
    }

    /// Returns redaction markers.
    #[must_use]
    pub fn redactions(&self) -> &[Text] {
        self.redactions.as_slice()
    }
}

/// Validates rendered scheduled-run record text for raw material.
///
/// # Errors
///
/// Returns [`ScheduleError::RecordLeak`] when raw prompt, secret, path, or tool output appears.
pub fn validate_scheduled_record_text(text: &str) -> Result<(), ScheduleError> {
    let lowered = text.to_ascii_lowercase();
    if lowered.contains("prompt:")
        || lowered.contains("secret")
        || lowered.contains("/home/")
        || lowered.contains("tool output")
    {
        return Err(ScheduleError::RecordLeak);
    }
    Ok(())
}

/// Schedule control action.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScheduleControlAction {
    /// Pause future runs.
    Pause,
    /// Resume future runs.
    Resume,
    /// Edit schedule metadata.
    Edit,
    /// Request immediate run.
    RunNow,
    /// Cancel queued or active run.
    Cancel,
}

impl ScheduleControlAction {
    /// Parses a stable schedule control action label.
    ///
    /// # Errors
    ///
    /// Returns [`ScheduleError`] when the label is empty or unsupported.
    pub fn parse(value: &str) -> Result<Self, ScheduleError> {
        parse_value(ACTION_FIELD, value, Self::from_normalized)
    }

    fn from_normalized(value: &str) -> Option<Self> {
        match value {
            CONTROL_PAUSE => Some(Self::Pause),
            CONTROL_RESUME => Some(Self::Resume),
            CONTROL_EDIT => Some(Self::Edit),
            CONTROL_RUN_NOW => Some(Self::RunNow),
            CONTROL_CANCEL => Some(Self::Cancel),
            _ => None,
        }
    }
}

/// Untrusted schedule control request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScheduleControlDraft<'a> {
    /// Schedule identity.
    pub schedule_id: &'a str,
    /// Action label.
    pub action: &'a str,
    /// Whether host policy admitted the control.
    pub host_admitted: bool,
    /// Whether confirmation was supplied.
    pub confirmed: bool,
}

/// Validated host-mediated schedule control request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScheduleControlRequest {
    schedule_id: Text,
    action: ScheduleControlAction,
}

impl ScheduleControlRequest {
    /// Validates a host-mediated schedule control request.
    ///
    /// # Errors
    ///
    /// Returns [`ScheduleError`] when admission or confirmation is missing.
    pub fn validate(draft: &ScheduleControlDraft<'_>) -> Result<Self, ScheduleError> {
        if !draft.host_admitted || !draft.confirmed {
            return Err(ScheduleError::ControlDenied);
        }
        let schedule_id = required_text(ID_FIELD, draft.schedule_id)?;
        let action = ScheduleControlAction::parse(draft.action)?;
        Ok(Self {
            schedule_id,
            action,
        })
    }

    /// Returns schedule identity.
    #[must_use]
    pub fn schedule_id(&self) -> &str {
        self.schedule_id.as_str()
    }

    /// Returns control action.
    #[must_use]
    pub const fn action(&self) -> ScheduleControlAction {
        self.action
    }
}

fn parse_value<T>(
    field_name: crate::validation::FieldName,
    value: &str,
    parser: impl FnOnce(&str) -> Option<T>,
) -> Result<T, ScheduleError> {
    let normalized = crate::validation::normalized_adapter_field(field_name, value)?;
    parser(normalized.as_str()).ok_or(ScheduleError::UnsupportedValue {
        field: field_name.as_str(),
        value: normalized,
    })
}

fn required_text(
    field_name: crate::validation::FieldName,
    value: &str,
) -> Result<Text, ScheduleError> {
    crate::validation::normalized_adapter_field(field_name, value).map_err(ScheduleError::from)
}

fn text_collection(
    field_name: crate::validation::FieldName,
    values: &[&str],
    required: bool,
) -> Result<Collection<Text>, ScheduleError> {
    if required && values.is_empty() {
        return Err(ScheduleError::EmptyCollection {
            field: field_name.as_str(),
        });
    }
    let mut normalized: Collection<Text> = Collection::with_capacity(values.len());
    for value in values {
        let text = required_text(field_name, value)?;
        if normalized.iter().any(|existing| existing == &text) {
            return Err(ScheduleError::DuplicateValue {
                field: field_name.as_str(),
                value: text,
            });
        }
        normalized.push(text);
    }
    Ok(normalized)
}

fn validate_hashes(
    field_name: crate::validation::FieldName,
    values: &[&str],
    required: bool,
) -> Result<(), ScheduleError> {
    if required && values.is_empty() {
        return Err(ScheduleError::EmptyCollection {
            field: field_name.as_str(),
        });
    }
    let mut seen: Collection<Text> = Collection::with_capacity(values.len());
    for value in values {
        let text = required_text(field_name, value)?;
        validate_hash(field_name.as_str(), text.as_str())?;
        if seen.iter().any(|existing| existing == &text) {
            return Err(ScheduleError::DuplicateValue {
                field: field_name.as_str(),
                value: text,
            });
        }
        seen.push(text);
    }
    Ok(())
}

fn validate_hash(field: &'static str, value: &str) -> Result<(), ScheduleError> {
    let hash_len = value.chars().count();
    if hash_len != BLAKE3_HEX_CHARS {
        return Err(ScheduleError::InvalidHash { field });
    }
    if value
        .chars()
        .any(|character| character.to_digit(ASCII_HEX_RADIX).is_none() || character.is_uppercase())
    {
        return Err(ScheduleError::InvalidHash { field });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCHEDULE_ID: &str = "daily-review";
    const RECIPE_ID: &str = "goose.recipe.workflow";
    const HASH_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const HASH_B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    const HASH_C: &str = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";
    const CALENDAR: &str = "utc";
    const WORKSPACE: &str = "readonly-workspace";
    const RETRY_LIMIT: u32 = 2;
    const RETENTION_COUNT: u32 = 7;
    const CURRENT_TICK: u64 = 10;
    const INTERVAL_TICKS: u64 = 5;
    const NEXT_TICK: u64 = 15;
    const ZERO_INTERVAL: u64 = 0;
    const FIRST_COUNT: u32 = 1;

    fn admitted_recipe() -> crate::RecipeTrustReceipt {
        crate::RecipeTrustReceipt::new(&crate::RecipeProvenanceDraft {
            source_kind: "project",
            source_label: "config/recipe-workflows.ncl",
            content_hash: HASH_A,
            admitted: true,
        })
        .expect("trust receipt should parse")
    }

    fn pending_recipe() -> crate::RecipeTrustReceipt {
        crate::RecipeTrustReceipt::new(&crate::RecipeProvenanceDraft {
            source_kind: "project",
            source_label: "config/recipe-workflows.ncl",
            content_hash: HASH_A,
            admitted: false,
        })
        .expect("trust receipt should parse")
    }

    fn schedule_draft<'a>(receipt: &'a crate::RecipeTrustReceipt) -> ScheduledWorkflowDraft<'a> {
        ScheduledWorkflowDraft {
            schedule_id: SCHEDULE_ID,
            recipe_id: RECIPE_ID,
            recipe_version_hash: HASH_A,
            parameter_hashes: &[HASH_B],
            cadence: CADENCE_DAILY,
            calendar_policy: CALENDAR,
            execution_mode: EXECUTION_BACKGROUND,
            concurrency_policy: CONCURRENCY_SERIALIZE,
            workspace_scope: WORKSPACE,
            retry_limit: RETRY_LIMIT,
            retention_count: RETENTION_COUNT,
            provenance_hash: HASH_C,
            recipe_trust: Some(receipt),
        }
    }

    #[test]
    fn schedule_descriptor_accepts_read_only_recurring_and_run_now() {
        let receipt = admitted_recipe();
        let schedule = ScheduledWorkflow::validate(&schedule_draft(&receipt))
            .expect("schedule should validate");
        let run_now = ScheduledWorkflow::validate(&ScheduledWorkflowDraft {
            cadence: CADENCE_RUN_NOW,
            execution_mode: EXECUTION_FOREGROUND,
            ..schedule_draft(&receipt)
        })
        .expect("run-now schedule should validate");

        assert_eq!(schedule.schedule_id(), SCHEDULE_ID);
        assert_eq!(schedule.cadence(), ScheduleCadence::Daily);
        assert_eq!(run_now.cadence(), ScheduleCadence::RunNow);
    }

    #[test]
    fn schedule_descriptor_rejects_invalid_cadence_missing_recipe_and_untrusted_recipe() {
        let admitted = admitted_recipe();
        let pending = pending_recipe();

        assert!(matches!(
            ScheduledWorkflow::validate(&ScheduledWorkflowDraft {
                cadence: "sometimes",
                ..schedule_draft(&admitted)
            }),
            Err(ScheduleError::UnsupportedValue { .. })
        ));
        assert!(matches!(
            ScheduledWorkflow::validate(&ScheduledWorkflowDraft {
                recipe_id: "",
                ..schedule_draft(&admitted)
            }),
            Err(ScheduleError::Contract(
                crate::AdapterContractError::EmptyField { .. }
            ))
        ));
        assert_eq!(
            ScheduledWorkflow::validate(&schedule_draft(&pending)),
            Err(ScheduleError::UntrustedRecipe)
        );
    }

    #[test]
    fn next_run_planning_uses_explicit_ticks_only() {
        let receipt = admitted_recipe();
        let schedule = ScheduledWorkflow::validate(&schedule_draft(&receipt))
            .expect("schedule should validate");
        let plan = plan_next_run(NextRunInput {
            schedule: &schedule,
            current_tick: CURRENT_TICK,
            interval_ticks: INTERVAL_TICKS,
        })
        .expect("plan should compute");

        assert_eq!(plan.next_tick(), NEXT_TICK);
        assert_eq!(
            plan_next_run(NextRunInput {
                schedule: &schedule,
                current_tick: CURRENT_TICK,
                interval_ticks: ZERO_INTERVAL,
            }),
            Err(ScheduleError::InvalidPlanningInput)
        );
        assert!(alloc::format!("{:?}", PROHIBITED_SCHEDULE_EFFECTS).contains("cron edit"));
    }

    #[test]
    fn run_admission_denies_missing_queue_untrusted_policy_and_cancellation() {
        assert_eq!(
            admit_scheduled_run(ScheduledRunAdmissionDraft {
                recipe_trusted: true,
                capability_allowed: true,
                sandbox_allowed: true,
                queue_admitted: true,
                cancellation_handle: true,
                policy_current: true,
            }),
            ScheduledRunAdmissionStatus::Admitted
        );
        assert_eq!(
            admit_scheduled_run(ScheduledRunAdmissionDraft {
                queue_admitted: false,
                recipe_trusted: true,
                capability_allowed: true,
                sandbox_allowed: true,
                cancellation_handle: true,
                policy_current: true,
            }),
            ScheduledRunAdmissionStatus::Denied {
                reason: ScheduledRunDenial::QueueService,
            }
        );
        assert_eq!(
            admit_scheduled_run(ScheduledRunAdmissionDraft {
                cancellation_handle: false,
                recipe_trusted: true,
                capability_allowed: true,
                sandbox_allowed: true,
                queue_admitted: true,
                policy_current: true,
            }),
            ScheduledRunAdmissionStatus::Denied {
                reason: ScheduledRunDenial::CancellationHandle,
            }
        );
    }

    #[test]
    fn concurrency_conflicts_are_serialized_isolated_or_denied() {
        let overlap = ScheduledConcurrencyInput {
            policy: ScheduleConcurrencyPolicy::Serialize,
            active_run: true,
            active_run_writes: true,
            new_run_writes: true,
        };

        assert_eq!(
            evaluate_scheduled_concurrency(overlap),
            ScheduledConcurrencyDecision::Serialize
        );
        assert_eq!(
            evaluate_scheduled_concurrency(ScheduledConcurrencyInput {
                policy: ScheduleConcurrencyPolicy::Isolate,
                ..overlap
            }),
            ScheduledConcurrencyDecision::Isolate
        );
        assert_eq!(
            evaluate_scheduled_concurrency(ScheduledConcurrencyInput {
                policy: ScheduleConcurrencyPolicy::Deny,
                ..overlap
            }),
            ScheduledConcurrencyDecision::Deny
        );
    }

    #[test]
    fn run_records_are_redacted_hash_and_count_records() {
        let record = ScheduledRunRecord::new(&ScheduledRunRecordDraft {
            schedule_hash: HASH_A,
            recipe_hash: HASH_B,
            parameter_hashes: &[HASH_C],
            queue_job_hash: HASH_A,
            ledger_hash: HASH_B,
            retry_count: RETRY_LIMIT,
            outcome: "success",
            evidence_hashes: &[HASH_C],
            redactions: &["prompt_hashed", "secret_parameter_omitted"],
        })
        .expect("record should parse");

        assert_eq!(record.parameter_count(), FIRST_COUNT);
        assert_eq!(record.evidence_count(), FIRST_COUNT);
        assert_eq!(
            validate_scheduled_record_text("hash counts redactions"),
            Ok(())
        );
        assert_eq!(
            validate_scheduled_record_text("prompt: hi secret /home/user tool output"),
            Err(ScheduleError::RecordLeak)
        );
    }

    #[test]
    fn controls_require_host_admission_and_confirmation() {
        let request = ScheduleControlRequest::validate(&ScheduleControlDraft {
            schedule_id: SCHEDULE_ID,
            action: CONTROL_RUN_NOW,
            host_admitted: true,
            confirmed: true,
        })
        .expect("control should validate");

        assert_eq!(request.action(), ScheduleControlAction::RunNow);
        assert_eq!(
            ScheduleControlRequest::validate(&ScheduleControlDraft {
                schedule_id: SCHEDULE_ID,
                action: CONTROL_EDIT,
                host_admitted: false,
                confirmed: true,
            }),
            Err(ScheduleError::ControlDenied)
        );
        assert_eq!(
            ScheduleControlRequest::validate(&ScheduleControlDraft {
                schedule_id: SCHEDULE_ID,
                action: CONTROL_CANCEL,
                host_admitted: true,
                confirmed: false,
            }),
            Err(ScheduleError::ControlDenied)
        );
    }
}
