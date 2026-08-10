//! Packaged deterministic host smoke for four capability-bound semantic roles.

mod composition;
mod workflow;

#[cfg(feature = "authoritative-completion-smoke")]
pub(crate) use workflow::JudgeDecision;
#[cfg(feature = "authoritative-completion-smoke")]
use workflow::{JudgeWorkflowInput, run_workflow_with_judge};
use workflow::{WorkflowReport, run_workflow};

const ARG_ROLE_POLICY: &str = "--role-policy";
const ARG_MODEL_POLICY: &str = "--model-policy";
const ARG_SUBJECT: &str = "--subject";
const ARG_COLLAPSE_REVIEWER_ACTOR: &str = "--collapse-reviewer-actor";
const ARG_DENY_JUDGE_VETO: &str = "--deny-judge-veto";
const ARG_REQUIRE_REVIEWER_MUTATION: &str = "--require-reviewer-mutation";
const ARG_CLAIM_COMPLETION: &str = "--claim-completion";
const RECORD_HEADER: &str = "ANIMUS_MULTI_ROLE_SMOKE";
const RECORD_ROLE: &str = "ROLE";
const RECORD_NON_CLAIM: &str = "NON_CLAIM";
const SMOKE_SCHEMA: &str = "animus.multi-role-smoke.v1";
const STATUS_COMPLETED: &str = "completed";
const STATUS_FAILED: &str = "failed";
const ERROR_MISSING_ARGUMENT: &str = "missing-argument";
const ERROR_MISSING_ARGUMENT_VALUE: &str = "missing-argument-value";
const ERROR_UNKNOWN_ARGUMENT: &str = "unknown-argument";
const ERROR_CONFLICTING_FIXTURE: &str = "conflicting-fixture";
const ERROR_ROLE_POLICY: &str = "role-policy";
const ERROR_MODEL_POLICY: &str = "model-policy";
#[cfg(feature = "authoritative-completion-smoke")]
const ERROR_ADVISORY: &str = "advisory";
const SUBJECT_HASH_DOMAIN: &str = "animus.multi-role-smoke.subject.v1";
const HASH_SEPARATOR: &str = "|";
const EXPECTED_ROLE_COUNT: usize = 4;
const NON_CLAIMS: &[&str] = &[
    "provider-quality",
    "verifier-soundness",
    "human-independence",
    "mutation-execution",
    "authoritative-completion",
    "tui",
    "daemon",
    "production-readiness",
];

#[derive(Clone, Debug, Eq, PartialEq)]
struct SmokeArgs {
    role_policy_path: String,
    model_policy_path: String,
    subject: String,
    fixture: SmokeFixture,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum SmokeFixture {
    Admitted,
    CollapseReviewerActor,
    DenyJudgeVeto,
    RequireReviewerMutation,
    ClaimCompletion,
}

#[cfg(feature = "authoritative-completion-smoke")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct MultiRoleAdvisory {
    pub advisory_hash: String,
    pub judge_decision: JudgeDecision,
    pub role_count: usize,
    pub authoritative_completion: bool,
}

#[cfg(feature = "authoritative-completion-smoke")]
pub(crate) fn compose_multi_role_advisory(
    roles: &animus_runtime::agent_roles::AgentRoleCatalog,
    model_profiles: &animus_runtime::ModelRuntimeProfileSet,
    subject_hash: &str,
    judge_decision: JudgeDecision,
) -> Result<MultiRoleAdvisory, String> {
    // r[impl agent_harness_runtime.authoritative_completion_host_smoke.role_boundary]
    let report = run_workflow_with_judge(JudgeWorkflowInput {
        roles,
        model_profiles,
        subject_hash,
        fixture: SmokeFixture::Admitted,
        judge_decision,
    })?;
    let judge = report.roles.last().ok_or_else(|| error(ERROR_ADVISORY))?;
    if judge.output_effect != judge_decision.as_str() || report.authoritative_completion {
        return Err(error(ERROR_ADVISORY));
    }
    let rendered = render_report(&report);
    Ok(MultiRoleAdvisory {
        advisory_hash: domain_hash("animus.multi-role-smoke.advisory.v1", rendered.as_str()),
        judge_decision,
        role_count: report.roles.len(),
        authoritative_completion: report.authoritative_completion,
    })
}

/// Run the packaged four-role smoke from explicit CLI arguments.
///
/// # Errors
///
/// Returns a stable bounded error when argument parsing, explicit policy loading,
/// exact role composition, separation, output admission, completion-boundary
/// rejection, or safe receipt projection fails.
pub fn run_packaged_multi_role_smoke(args: &[String]) -> Result<String, String> {
    let parsed = parse_args(args)?;
    let roles = super::load_agent_role_profiles(parsed.role_policy_path.as_str())
        .map_err(|_failure| error(ERROR_ROLE_POLICY))?;
    let model_profiles = super::load_model_runtime_profiles(parsed.model_policy_path.as_str())
        .map_err(|_failure| error(ERROR_MODEL_POLICY))?;
    let subject_hash = domain_hash(SUBJECT_HASH_DOMAIN, parsed.subject.as_str());
    let report = run_workflow(
        &roles,
        &model_profiles,
        subject_hash.as_str(),
        parsed.fixture,
    )?;
    Ok(render_report(&report))
}

fn parse_args(args: &[String]) -> Result<SmokeArgs, String> {
    let mut cursor = ArgCursor::new(args);
    let mut role_policy_path = None;
    let mut model_policy_path = None;
    let mut subject = None;
    let mut fixture = SmokeFixture::Admitted;

    while let Some(argument) = cursor.next() {
        match argument.as_str() {
            ARG_ROLE_POLICY => role_policy_path = Some(cursor.next_value()?),
            ARG_MODEL_POLICY => model_policy_path = Some(cursor.next_value()?),
            ARG_SUBJECT => subject = Some(cursor.next_value()?),
            ARG_COLLAPSE_REVIEWER_ACTOR => {
                fixture = select_fixture(fixture, SmokeFixture::CollapseReviewerActor)?;
            }
            ARG_DENY_JUDGE_VETO => {
                fixture = select_fixture(fixture, SmokeFixture::DenyJudgeVeto)?;
            }
            ARG_REQUIRE_REVIEWER_MUTATION => {
                fixture = select_fixture(fixture, SmokeFixture::RequireReviewerMutation)?;
            }
            ARG_CLAIM_COMPLETION => {
                fixture = select_fixture(fixture, SmokeFixture::ClaimCompletion)?;
            }
            _unknown => return Err(error(ERROR_UNKNOWN_ARGUMENT)),
        }
    }

    Ok(SmokeArgs {
        role_policy_path: required(role_policy_path)?,
        model_policy_path: required(model_policy_path)?,
        subject: required(subject)?,
        fixture,
    })
}

fn select_fixture(current: SmokeFixture, selected: SmokeFixture) -> Result<SmokeFixture, String> {
    if current == SmokeFixture::Admitted {
        return Ok(selected);
    }
    Err(error(ERROR_CONFLICTING_FIXTURE))
}

fn render_report(report: &WorkflowReport) -> String {
    // r[impl agent_harness_runtime.multi_role_host_smoke.receipt]
    assert_eq!(report.roles.len(), EXPECTED_ROLE_COUNT);
    let mut lines = vec![format!(
        "{RECORD_HEADER} schema={SMOKE_SCHEMA} status={STATUS_COMPLETED} role_count={} reviewer_mutation_rejected={} completion_claim_rejected={} completion_candidate_rejected={} authoritative_completion={}",
        report.roles.len(),
        report.boundaries.reviewer_mutation_rejected,
        report.boundaries.completion_claim_rejected,
        report.boundaries.completion_candidate_rejected,
        report.authoritative_completion,
    )];
    for role in &report.roles {
        lines.push(format!(
            "{RECORD_ROLE} role_id={} class={} model_profile_id={} model_plan_hash={} assignment_hash={} separation_hash={} capability_count={} output_kind={} effect={} output_hash={} receipt_hash={}",
            role.role_id,
            role.role_class,
            role.model_profile_id,
            role.model_plan_hash,
            role.assignment_hash,
            role.separation_hash,
            role.effective_capability_count,
            role.output_kind,
            role.output_effect,
            role.output_hash,
            role.receipt_hash,
        ));
    }
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

pub(super) fn domain_hash(domain: &str, value: &str) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(domain.as_bytes());
    hasher.update(HASH_SEPARATOR.as_bytes());
    hasher.update(value.as_bytes());
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
