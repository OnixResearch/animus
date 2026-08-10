//! Packaged deterministic authoritative-completion host smoke.

mod fixtures;
mod workflow;

use std::path::Path;

use workflow::{CompletionReport, LoadedCompletionInput, run_completion_workflow};

const ARG_ROLE_POLICY: &str = "--role-policy";
const ARG_MODEL_POLICY: &str = "--model-policy";
const ARG_CHECK_REGISTRY: &str = "--check-registry";
const ARG_SUBJECT: &str = "--subject";
const ARG_HEAD_STATE: &str = "--head-state";
const HEAD_CLEAN: &str = "clean";
const HEAD_DIRTY: &str = "dirty";
const ARG_JUDGE_VETO: &str = "--judge-veto";
const ARG_ROLE_SELF_ATTESTATION: &str = "--role-self-attestation";
const ARG_MISSING_RESULT: &str = "--missing-result";
const ARG_STALE_SUBJECT: &str = "--stale-subject";
const ARG_MISSING_CAIRN: &str = "--missing-cairn";
const RECORD_HEADER: &str = "ANIMUS_AUTHORITATIVE_COMPLETION_SMOKE";
const RECORD_NON_CLAIM: &str = "NON_CLAIM";
const SMOKE_SCHEMA: &str = "animus.authoritative-completion-host-smoke.v1";
const STATUS_COMPLETED: &str = "completed";
const STATUS_FAILED: &str = "failed";
const ERROR_MISSING_ARGUMENT: &str = "missing-argument";
const ERROR_MISSING_ARGUMENT_VALUE: &str = "missing-argument-value";
const ERROR_UNKNOWN_ARGUMENT: &str = "unknown-argument";
const ERROR_CONFLICTING_FIXTURE: &str = "conflicting-fixture";
const ERROR_ROLE_POLICY: &str = "role-policy";
const ERROR_MODEL_POLICY: &str = "model-policy";
const ERROR_CHECK_REGISTRY: &str = "check-registry";
const ERROR_HEAD_STATE: &str = "head-state";
const HASH_SEPARATOR: &str = "|";
const NON_CLAIMS: &[&str] = &[
    "check-oracle-soundness",
    "honest-external-execution",
    "live-provider",
    "verifier-soundness",
    "human-independence",
    "mutation-execution",
    "release-readiness",
    "tui",
    "daemon",
    "production-readiness",
];

#[derive(Clone, Debug, Eq, PartialEq)]
struct SmokeArgs {
    role_policy_path: String,
    model_policy_path: String,
    check_registry_path: String,
    subject: String,
    head_state: HeadState,
    fixture: CompletionFixture,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum CompletionFixture {
    Admitted,
    JudgeVeto,
    RoleSelfAttestation,
    MissingResult,
    StaleSubject,
    MissingCairn,
}

impl CompletionFixture {
    pub(super) const fn issue(self) -> &'static str {
        match self {
            Self::Admitted => "completion",
            Self::JudgeVeto => "judge-veto",
            Self::RoleSelfAttestation => "role-self-attestation",
            Self::MissingResult => "missing-result",
            Self::StaleSubject => "stale-subject",
            Self::MissingCairn => "missing-cairn",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum HeadState {
    Clean,
    Dirty,
}

/// Run the packaged authoritative-completion smoke from explicit CLI values.
///
/// # Errors
///
/// Returns a stable bounded issue when parsing, loading, role composition,
/// selected-check evidence, Cairn validation, or completion reduction fails.
pub fn run_packaged_authoritative_completion_smoke(args: &[String]) -> Result<String, String> {
    let parsed = parse_args(args)?;
    let roles = super::load_agent_role_profiles(parsed.role_policy_path.as_str())
        .map_err(|_failure| error(ERROR_ROLE_POLICY))?;
    let models = super::load_model_runtime_profiles(parsed.model_policy_path.as_str())
        .map_err(|_failure| error(ERROR_MODEL_POLICY))?;
    let registry = super::load_agent_check_registry(Path::new(parsed.check_registry_path.as_str()))
        .map_err(|_failure| error(ERROR_CHECK_REGISTRY))?;
    let report = run_completion_workflow(LoadedCompletionInput {
        roles: &roles,
        model_profiles: &models,
        registry: &registry,
        subject: parsed.subject.as_str(),
        head_state: parsed.head_state,
        fixture: parsed.fixture,
    })?;
    Ok(render_report(&report))
}

fn parse_args(args: &[String]) -> Result<SmokeArgs, String> {
    let mut cursor = ArgCursor::new(args);
    let mut role_policy_path = None;
    let mut model_policy_path = None;
    let mut check_registry_path = None;
    let mut subject = None;
    let mut head_state = None;
    let mut fixture = CompletionFixture::Admitted;
    while let Some(argument) = cursor.next() {
        match argument.as_str() {
            ARG_ROLE_POLICY => role_policy_path = Some(cursor.next_value()?),
            ARG_MODEL_POLICY => model_policy_path = Some(cursor.next_value()?),
            ARG_CHECK_REGISTRY => check_registry_path = Some(cursor.next_value()?),
            ARG_SUBJECT => subject = Some(cursor.next_value()?),
            ARG_HEAD_STATE => {
                let selected = parse_head_state(cursor.next_value()?.as_str())?;
                head_state = Some(select_head_state(head_state, selected)?);
            }
            ARG_JUDGE_VETO => {
                fixture = select_fixture(fixture, CompletionFixture::JudgeVeto)?;
            }
            ARG_ROLE_SELF_ATTESTATION => {
                fixture = select_fixture(fixture, CompletionFixture::RoleSelfAttestation)?;
            }
            ARG_MISSING_RESULT => {
                fixture = select_fixture(fixture, CompletionFixture::MissingResult)?;
            }
            ARG_STALE_SUBJECT => {
                fixture = select_fixture(fixture, CompletionFixture::StaleSubject)?;
            }
            ARG_MISSING_CAIRN => {
                fixture = select_fixture(fixture, CompletionFixture::MissingCairn)?;
            }
            _unknown => return Err(error(ERROR_UNKNOWN_ARGUMENT)),
        }
    }
    Ok(SmokeArgs {
        role_policy_path: required(role_policy_path)?,
        model_policy_path: required(model_policy_path)?,
        check_registry_path: required(check_registry_path)?,
        subject: required(subject)?,
        head_state: head_state.ok_or_else(|| error(ERROR_MISSING_ARGUMENT))?,
        fixture,
    })
}

fn select_head_state(current: Option<HeadState>, selected: HeadState) -> Result<HeadState, String> {
    if current.is_none() {
        return Ok(selected);
    }
    Err(error(ERROR_HEAD_STATE))
}

fn parse_head_state(value: &str) -> Result<HeadState, String> {
    match value {
        HEAD_CLEAN => Ok(HeadState::Clean),
        HEAD_DIRTY => Ok(HeadState::Dirty),
        _unsupported => Err(error(ERROR_HEAD_STATE)),
    }
}

fn select_fixture(
    current: CompletionFixture,
    selected: CompletionFixture,
) -> Result<CompletionFixture, String> {
    if current == CompletionFixture::Admitted {
        return Ok(selected);
    }
    Err(error(ERROR_CONFLICTING_FIXTURE))
}

fn render_report(report: &CompletionReport) -> String {
    // r[impl agent_harness_runtime.authoritative_completion_host_smoke.receipt]
    let mut lines = vec![format!(
        "{RECORD_HEADER} schema={SMOKE_SCHEMA} status={STATUS_COMPLETED} authoritative_completion={} judge_decision={} required_checks={} passed_checks={} role_advisory_hash={} registry_hash={} selection_hash={} final_subject_hash={} execution_evidence_hash={} cairn_receipt_hash={} candidate_hash={} completion_receipt_hash={} artifact_observer_calls={} subject_hasher_calls={} check_executor_calls={} artifact_persister_calls={} cairn_validator_calls={}",
        report.authoritative_completion,
        report.judge_decision,
        report.required_check_count,
        report.passed_check_count,
        report.role_advisory_hash,
        report.registry_hash,
        report.selection_hash,
        report.final_subject_hash,
        report.execution_evidence_hash,
        report.cairn_receipt_hash,
        report.candidate_hash,
        report.completion_receipt_hash,
        report.service_calls.artifact_observer,
        report.service_calls.subject_hasher,
        report.service_calls.check_executor,
        report.service_calls.artifact_persister,
        report.service_calls.cairn_validator,
    )];
    for surface in NON_CLAIMS {
        lines.push(format!("{RECORD_NON_CLAIM} surface={surface}"));
    }
    lines.push(String::new());
    lines.join("\n")
}

fn required(value: Option<String>) -> Result<String, String> {
    value.ok_or_else(|| error(ERROR_MISSING_ARGUMENT))
}

pub(super) fn error(issue: &str) -> String {
    format!("{RECORD_HEADER} status={STATUS_FAILED} issue={issue}")
}

pub(super) fn domain_hash(domain: &str, parts: &[&str]) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(domain.as_bytes());
    for part in parts {
        hasher.update(HASH_SEPARATOR.as_bytes());
        hasher.update(part.as_bytes());
    }
    hasher.finalize().to_hex().to_string()
}

struct ArgCursor<'a> {
    args: &'a [String],
    index: usize,
}

impl<'a> ArgCursor<'a> {
    const fn new(args: &'a [String]) -> Self {
        Self { args, index: 0 }
    }

    fn next(&mut self) -> Option<&'a String> {
        let argument = self.args.get(self.index)?;
        self.index = self.index.saturating_add(1);
        Some(argument)
    }

    fn next_value(&mut self) -> Result<String, String> {
        let value = self
            .next()
            .ok_or_else(|| error(ERROR_MISSING_ARGUMENT_VALUE))?;
        Ok(value.clone())
    }
}

#[cfg(test)]
mod tests;
