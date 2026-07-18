use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::{Component, Path};

use serde::Deserialize;

use crate::model::{
    ARTIFACT_DOMAIN, AuthorizationBinding, BUBBLEWRAP_COHORT, EnvironmentBinding, ExecutableFacts,
    ExecutionObservation, GuardrailError, GuardrailErrorKind, GuardrailPlan, GuardrailProfile,
    GuardrailReceipt, NON_CLAIMS, NetworkPosture, PLAN_DOMAIN, PLAN_SCHEMA, PROFILE_DOMAIN,
    PROFILE_SCHEMA, PlanOperation, PreflightRequest, PreflightResult, PreflightStatus, ProfileRoot,
    RECEIPT_DOMAIN, RECEIPT_SCHEMA, REDACTION_MARKER, RootBinding, RootMode,
};

const FIELD_PROFILE: &str = "profile";
const FIELD_AUTHORIZATION: &str = "authorization";
const FIELD_ROOT: &str = "root";
const FIELD_ENVIRONMENT: &str = "environment";
const FIELD_EXECUTABLE: &str = "executable";
const FIELD_PLAN: &str = "plan";
const FIELD_RECEIPT: &str = "receipt";
const BACKEND_LABEL: &str = "bubblewrap-agent-guardrail";
const TRUST_SOURCE: &str = "central-reviewed";
const MAX_SAFE_LABEL_BYTES: usize = 128;
const HASH_HEX_BYTES: usize = 64;
const MINIMUM_ROOT_TARGET_DEPTH: usize = 1;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedProfile {
    schema: String,
    label: String,
    backend: String,
    cohort: String,
    trust_source: String,
    roots: Vec<ProfileRoot>,
    network: NetworkPosture,
    devices: bool,
    host_sockets: bool,
    environment_keys: Vec<String>,
    bounds: GeneratedBounds,
    escape_hatches: BTreeMap<String, bool>,
    identity: GeneratedIdentity,
    redaction: GeneratedRedaction,
    non_claims: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedBounds {
    #[serde(rename = "max_roots")]
    roots: u64,
    #[serde(rename = "max_environment")]
    environment: u64,
    #[serde(rename = "max_options")]
    options: u64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedIdentity {
    algorithm: String,
    profile_domain: String,
    plan_domain: String,
    executable_domain: String,
    receipt_domain: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedRedaction {
    marker: String,
    raw_payloads: bool,
}

/// Loads and validates the checked Nickel projection.
///
/// # Errors
///
/// Returns [`GuardrailError`] for malformed JSON, trust/cohort drift, unsafe root declarations,
/// escape-hatch widening, invalid bounds, or weakened redaction and non-claims.
pub fn load_generated_profile(json: &str) -> Result<GuardrailProfile, GuardrailError> {
    let generated = serde_json::from_str::<GeneratedProfile>(json)
        .map_err(|_error| error(GuardrailErrorKind::Profile, FIELD_PROFILE))?;
    validate_generated_profile(&generated)?;
    let max_roots = usize_value(generated.bounds.roots)?;
    let max_environment = usize_value(generated.bounds.environment)?;
    let max_options = usize_value(generated.bounds.options)?;
    if generated.roots.len() > max_roots || generated.environment_keys.len() > max_environment {
        return Err(error(GuardrailErrorKind::Profile, FIELD_PROFILE));
    }
    validate_profile_roots(generated.roots.as_slice())?;
    validate_unique_safe_values(generated.environment_keys.as_slice(), FIELD_ENVIRONMENT)?;
    Ok(GuardrailProfile {
        identity: domain_hash(PROFILE_DOMAIN, json.as_bytes()),
        label: generated.label,
        cohort: generated.cohort,
        trust_source: generated.trust_source,
        roots: generated.roots,
        network: generated.network,
        environment_keys: generated.environment_keys,
        max_roots,
        max_environment,
        max_options,
    })
}

fn validate_generated_profile(profile: &GeneratedProfile) -> Result<(), GuardrailError> {
    let expected_escape_keys = [
        "raw_arguments",
        "command_prefix",
        "passthrough",
        "command_environment_source",
        "implicit_shell",
        "unguarded_fallback",
        "privileged_sidecar",
        "project_local_policy",
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    let observed_escape_keys = profile
        .escape_hatches
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let exact = profile.schema == PROFILE_SCHEMA
        && profile.backend == BACKEND_LABEL
        && profile.cohort == BUBBLEWRAP_COHORT
        && profile.trust_source == TRUST_SOURCE
        && !profile.devices
        && !profile.host_sockets
        && observed_escape_keys == expected_escape_keys
        && profile.escape_hatches.values().all(|enabled| !enabled)
        && profile.identity.algorithm == "blake3"
        && profile.identity.profile_domain == PROFILE_DOMAIN
        && profile.identity.plan_domain == PLAN_DOMAIN
        && profile.identity.executable_domain == ARTIFACT_DOMAIN
        && profile.identity.receipt_domain == RECEIPT_DOMAIN
        && profile.redaction.marker == REDACTION_MARKER
        && !profile.redaction.raw_payloads
        && profile.non_claims == NON_CLAIMS
        && profile.bounds.roots > 0
        && profile.bounds.environment > 0
        && profile.bounds.options > 0;
    if exact {
        validate_safe_label(profile.label.as_str(), FIELD_PROFILE)
    } else {
        Err(error(GuardrailErrorKind::Profile, FIELD_PROFILE))
    }
}

fn validate_profile_roots(roots: &[ProfileRoot]) -> Result<(), GuardrailError> {
    let mut labels = BTreeSet::new();
    let mut targets = BTreeSet::new();
    for root in roots {
        validate_safe_label(root.label.as_str(), FIELD_ROOT)?;
        validate_target(root.target.as_path())?;
        if !labels.insert(root.label.as_str()) || !targets.insert(root.target.as_path()) {
            return Err(error(GuardrailErrorKind::Profile, FIELD_ROOT));
        }
    }
    Ok(())
}

/// Constructs one deterministic typed plan from explicit checked facts.
///
/// # Errors
///
/// Returns [`GuardrailError`] for denied or drifted authorization, incompatible executable facts,
/// missing/escaping/conflicting roots, unadmitted environment values, or plan identity drift.
pub fn build_plan(
    profile: &GuardrailProfile,
    request: &PreflightRequest,
) -> Result<GuardrailPlan, GuardrailError> {
    validate_authorization(profile, &request.context.authorization, request)?;
    validate_executable(profile, &request.executable)?;
    let (root_operations, root_labels) = plan_roots(profile, request.roots.as_slice())?;
    let mut operations = Vec::with_capacity(root_operations.len().saturating_add(1));
    operations.push(PlanOperation::PrivateRoot);
    operations.extend(root_operations);
    operations.push(PlanOperation::Proc);
    operations.push(PlanOperation::PrivateTmp);
    operations.push(PlanOperation::PrivateDev);
    operations.push(PlanOperation::ClearEnvironment);
    let environment = plan_environment(profile, request.environment.as_slice())?;
    operations.extend(environment);
    if profile.network == NetworkPosture::Inherit {
        operations.push(PlanOperation::ShareNetwork);
    }
    operations.push(PlanOperation::LockRoot);
    if operations.len() > profile.max_options {
        return Err(error(GuardrailErrorKind::Plan, FIELD_PLAN));
    }
    let mut enforced_capabilities = request.context.requested_capabilities.clone();
    validate_unique_safe_values(enforced_capabilities.as_slice(), FIELD_PLAN)?;
    enforced_capabilities.sort();
    let mut plan = GuardrailPlan {
        schema: PLAN_SCHEMA,
        profile_identity: profile.identity.clone(),
        authorization_identity: request.context.authorization.identity.clone(),
        descriptor_identity: request.context.descriptor_identity.clone(),
        policy_identity: request.context.policy_identity.clone(),
        executable_identity: request.executable.identity.clone(),
        network: profile.network,
        operations,
        enforced_capabilities,
        root_labels,
        identity: String::new(),
    };
    plan.identity = plan_identity(&plan);
    if request
        .expected_plan_identity
        .as_deref()
        .is_some_and(|expected| expected != plan.identity)
    {
        return Err(error(GuardrailErrorKind::Plan, FIELD_PLAN));
    }
    Ok(plan)
}

fn validate_authorization(
    profile: &GuardrailProfile,
    authorization: &AuthorizationBinding,
    request: &PreflightRequest,
) -> Result<(), GuardrailError> {
    let context = &request.context;
    let exact = authorization.allowed
        && authorization.action_identity == context.action_identity
        && authorization.subject_identity == context.subject_identity
        && authorization.target_identity == context.target_identity
        && authorization.descriptor_identity == context.descriptor_identity
        && authorization.policy_identity == context.policy_identity
        && authorization.profile_identity == profile.identity;
    for value in [
        authorization.identity.as_str(),
        context.action_identity.as_str(),
        context.subject_identity.as_str(),
        context.target_identity.as_str(),
        context.descriptor_identity.as_str(),
        context.policy_identity.as_str(),
        context.context_identity.as_str(),
    ] {
        validate_safe_label(value, FIELD_AUTHORIZATION)?;
    }
    if exact {
        Ok(())
    } else {
        Err(error(
            GuardrailErrorKind::Authorization,
            FIELD_AUTHORIZATION,
        ))
    }
}

fn validate_executable(
    profile: &GuardrailProfile,
    executable: &ExecutableFacts,
) -> Result<(), GuardrailError> {
    if executable.path.is_absolute()
        && validate_hash(executable.identity.as_str())
        && executable.cohort == profile.cohort
        && executable.args_supported
    {
        Ok(())
    } else {
        Err(error(GuardrailErrorKind::Executable, FIELD_EXECUTABLE))
    }
}

fn plan_roots(
    profile: &GuardrailProfile,
    bindings: &[RootBinding],
) -> Result<(Vec<PlanOperation>, Vec<String>), GuardrailError> {
    if bindings.len() > profile.max_roots {
        return Err(error(GuardrailErrorKind::Root, FIELD_ROOT));
    }
    let declarations = profile
        .roots
        .iter()
        .map(|root| (root.label.as_str(), root))
        .collect::<BTreeMap<_, _>>();
    let mut by_target = BTreeMap::<String, (&RootBinding, RootMode)>::new();
    let mut seen_labels = BTreeSet::new();
    for binding in bindings {
        let declaration = declarations
            .get(binding.label.as_str())
            .ok_or_else(|| error(GuardrailErrorKind::Root, FIELD_ROOT))?;
        validate_binding(declaration, binding)?;
        if !seen_labels.insert(binding.label.as_str()) {
            return Err(error(GuardrailErrorKind::Root, FIELD_ROOT));
        }
        let key = normalized_path(binding.target.as_path())?;
        if let Some((existing, mode)) = by_target.get(key.as_str()) {
            if existing.source != binding.source || *mode != declaration.mode {
                return Err(error(GuardrailErrorKind::Root, FIELD_ROOT));
            }
            continue;
        }
        by_target.insert(key, (binding, declaration.mode));
    }
    for declaration in &profile.roots {
        if declaration.required && !seen_labels.contains(declaration.label.as_str()) {
            return Err(error(GuardrailErrorKind::Root, FIELD_ROOT));
        }
    }
    let mut ordered = by_target.into_values().collect::<Vec<_>>();
    ordered.sort_by(|(left, _left_mode), (right, _right_mode)| {
        path_depth(left.target.as_path())
            .cmp(&path_depth(right.target.as_path()))
            .then_with(|| left.target.cmp(&right.target))
    });
    let mut operations = Vec::with_capacity(ordered.len());
    let mut labels = Vec::with_capacity(ordered.len());
    for (binding, mode) in ordered {
        let operation = match mode {
            RootMode::ReadOnly => PlanOperation::ReadOnlyBind {
                source: binding.source.clone(),
                target: binding.target.clone(),
            },
            RootMode::Writable => PlanOperation::WritableBind {
                source: binding.source.clone(),
                target: binding.target.clone(),
            },
        };
        operations.push(operation);
        labels.push(binding.label.clone());
    }
    Ok((operations, labels))
}

fn validate_binding(
    declaration: &ProfileRoot,
    binding: &RootBinding,
) -> Result<(), GuardrailError> {
    let exact = binding.exists
        && binding.symlink_within_root
        && binding.source_root.is_absolute()
        && binding.source.is_absolute()
        && binding.source.starts_with(binding.source_root.as_path())
        && binding.target == declaration.target
        && binding.source_kind == declaration.source_kind;
    if !exact {
        return Err(error(GuardrailErrorKind::Root, FIELD_ROOT));
    }
    validate_target(binding.target.as_path())?;
    if binding
        .source
        .components()
        .any(|component| component == Component::ParentDir)
    {
        return Err(error(GuardrailErrorKind::Root, FIELD_ROOT));
    }
    Ok(())
}

fn plan_environment(
    profile: &GuardrailProfile,
    bindings: &[EnvironmentBinding],
) -> Result<Vec<PlanOperation>, GuardrailError> {
    if bindings.len() > profile.max_environment {
        return Err(error(GuardrailErrorKind::Environment, FIELD_ENVIRONMENT));
    }
    let allowed = profile
        .environment_keys
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let mut values = BTreeMap::<&str, &str>::new();
    for binding in bindings {
        validate_environment_key(binding.key.as_str())?;
        validate_environment_value(binding.value.as_str())?;
        if !allowed.contains(binding.key.as_str())
            || values
                .insert(binding.key.as_str(), binding.value.as_str())
                .is_some()
        {
            return Err(error(GuardrailErrorKind::Environment, FIELD_ENVIRONMENT));
        }
    }
    Ok(values
        .into_iter()
        .map(|(key, value)| PlanOperation::SetEnvironment {
            key: String::from(key),
            value: String::from(value),
        })
        .collect())
}

/// Performs the exact side-effect-free active-context preflight.
#[must_use]
pub fn preflight(profile: &GuardrailProfile, request: &PreflightRequest) -> PreflightResult {
    match build_plan(profile, request) {
        Ok(plan) => PreflightResult {
            status: PreflightStatus::Valid,
            issue: None,
            plan: Some(plan),
        },
        Err(failure) => {
            let (status, issue) = match failure.kind() {
                GuardrailErrorKind::Authorization => {
                    (PreflightStatus::Denied, "authorization-denied")
                }
                GuardrailErrorKind::Executable => {
                    (PreflightStatus::Unsupported, "executable-unsupported")
                }
                GuardrailErrorKind::Root => {
                    let missing = request.roots.iter().any(|root| !root.exists);
                    if missing {
                        (PreflightStatus::MissingSource, "root-missing")
                    } else {
                        (PreflightStatus::Invalid, "root-invalid")
                    }
                }
                GuardrailErrorKind::Plan => (PreflightStatus::Drifted, "plan-drifted"),
                _other => (PreflightStatus::Invalid, "preflight-invalid"),
            };
            PreflightResult {
                status,
                issue: Some(issue),
                plan: None,
            }
        }
    }
}

/// Projects a metadata-only guardrail receipt from an admitted plan and bounded observation.
///
/// # Errors
///
/// Returns [`GuardrailError`] when a receipt label leaks raw or unbounded material.
pub fn build_receipt(
    profile: &GuardrailProfile,
    request: &PreflightRequest,
    plan: &GuardrailPlan,
    observation: &ExecutionObservation,
) -> Result<GuardrailReceipt, GuardrailError> {
    if plan.profile_identity != profile.identity
        || plan.descriptor_identity != request.context.descriptor_identity
        || plan.policy_identity != request.context.policy_identity
        || plan.authorization_identity != request.context.authorization.identity
        || plan.executable_identity != request.executable.identity
        || plan.identity != plan_identity(plan)
        || observation.option_count > profile.max_options
    {
        return Err(error(GuardrailErrorKind::Receipt, FIELD_RECEIPT));
    }
    validate_safe_label(observation.status.as_str(), FIELD_RECEIPT)?;
    validate_safe_label(observation.cleanup.as_str(), FIELD_RECEIPT)?;
    if let Some(issue) = observation.issue.as_deref() {
        validate_safe_label(issue, FIELD_RECEIPT)?;
    }
    validate_unique_safe_values(
        request.context.requested_capabilities.as_slice(),
        FIELD_RECEIPT,
    )?;
    validate_unique_safe_values(plan.enforced_capabilities.as_slice(), FIELD_RECEIPT)?;
    validate_unique_safe_values(plan.root_labels.as_slice(), FIELD_RECEIPT)?;
    let body = format!(
        "{RECEIPT_SCHEMA}\0{}\0{}\0{}\0{}\0{}\0{}\0{}\0{}\0{}\0{}\0{}",
        profile.identity,
        plan.descriptor_identity,
        plan.policy_identity,
        plan.authorization_identity,
        plan.executable_identity,
        plan.identity,
        plan.network.as_str(),
        observation.status,
        observation.issue.as_deref().unwrap_or("none"),
        observation.exit.as_str(),
        observation.option_count,
    );
    Ok(GuardrailReceipt {
        schema: RECEIPT_SCHEMA,
        backend: BACKEND_LABEL,
        cohort: BUBBLEWRAP_COHORT,
        profile_identity: profile.identity.clone(),
        descriptor_identity: plan.descriptor_identity.clone(),
        policy_identity: plan.policy_identity.clone(),
        authorization_identity: plan.authorization_identity.clone(),
        executable_identity: plan.executable_identity.clone(),
        plan_identity: plan.identity.clone(),
        requested_capabilities: request.context.requested_capabilities.clone(),
        enforced_capabilities: plan.enforced_capabilities.clone(),
        network: plan.network,
        root_labels: plan.root_labels.clone(),
        status: observation.status.clone(),
        issue: observation.issue.clone(),
        exit: observation.exit,
        cleanup: observation.cleanup.clone(),
        option_count: observation.option_count,
        redaction: REDACTION_MARKER,
        non_claims: NON_CLAIMS,
        identity: domain_hash(RECEIPT_DOMAIN, body.as_bytes()),
    })
}

pub(crate) fn executable_identity(bytes: &[u8]) -> String {
    domain_hash(ARTIFACT_DOMAIN, bytes)
}

pub(crate) fn domain_hash(domain: &str, bytes: &[u8]) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(domain.as_bytes());
    hasher.update(&[0]);
    hasher.update(bytes);
    hasher.finalize().to_hex().to_string()
}

fn plan_identity(plan: &GuardrailPlan) -> String {
    let mut body = format!(
        "{}\0{}\0{}\0{}\0{}\0{}\0{}",
        plan.schema,
        plan.profile_identity,
        plan.authorization_identity,
        plan.descriptor_identity,
        plan.policy_identity,
        plan.executable_identity,
        plan.network.as_str(),
    );
    for operation in &plan.operations {
        match operation {
            PlanOperation::PrivateRoot => body.push_str("\0private-root"),
            PlanOperation::ReadOnlyBind { source, target } => {
                write!(body, "\0ro\0{}\0{}", source.display(), target.display())
                    .expect("writing to String cannot fail");
            }
            PlanOperation::WritableBind { source, target } => {
                write!(body, "\0rw\0{}\0{}", source.display(), target.display())
                    .expect("writing to String cannot fail");
            }
            PlanOperation::Proc => body.push_str("\0proc"),
            PlanOperation::PrivateTmp => body.push_str("\0private-tmp"),
            PlanOperation::PrivateDev => body.push_str("\0private-dev"),
            PlanOperation::ClearEnvironment => body.push_str("\0clearenv"),
            PlanOperation::SetEnvironment { key, value } => {
                write!(body, "\0env\0{key}\0{value}").expect("writing to String cannot fail");
            }
            PlanOperation::ShareNetwork => body.push_str("\0share-net"),
            PlanOperation::LockRoot => body.push_str("\0lock-root"),
        }
    }
    domain_hash(PLAN_DOMAIN, body.as_bytes())
}

fn validate_target(path: &Path) -> Result<(), GuardrailError> {
    let valid = path.is_absolute()
        && path_depth(path) >= MINIMUM_ROOT_TARGET_DEPTH
        && path != Path::new("/")
        && !path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::CurDir | Component::Prefix(_)
            )
        })
        && !["/proc", "/dev", "/tmp"]
            .iter()
            .any(|reserved| path == Path::new(reserved) || path.starts_with(reserved));
    if valid {
        Ok(())
    } else {
        Err(error(GuardrailErrorKind::Root, FIELD_ROOT))
    }
}

fn normalized_path(path: &Path) -> Result<String, GuardrailError> {
    validate_target(path)?;
    Ok(path.to_string_lossy().into_owned())
}

fn path_depth(path: &Path) -> usize {
    path.components()
        .filter(|component| matches!(component, Component::Normal(_)))
        .count()
}

fn validate_environment_key(value: &str) -> Result<(), GuardrailError> {
    let valid = !value.is_empty()
        && value.len() <= MAX_SAFE_LABEL_BYTES
        && value
            .bytes()
            .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'_')
        && value.as_bytes()[0].is_ascii_uppercase();
    if valid {
        Ok(())
    } else {
        Err(error(GuardrailErrorKind::Environment, FIELD_ENVIRONMENT))
    }
}

fn validate_environment_value(value: &str) -> Result<(), GuardrailError> {
    if value.is_empty() || value.contains('\0') || value.contains('\n') || value.contains('\r') {
        Err(error(GuardrailErrorKind::Environment, FIELD_ENVIRONMENT))
    } else {
        Ok(())
    }
}

fn validate_unique_safe_values(
    values: &[String],
    field: &'static str,
) -> Result<(), GuardrailError> {
    let mut unique = BTreeSet::new();
    for value in values {
        validate_safe_label(value.as_str(), field)?;
        if !unique.insert(value.as_str()) {
            return Err(error(GuardrailErrorKind::Receipt, field));
        }
    }
    Ok(())
}

fn validate_safe_label(value: &str, field: &'static str) -> Result<(), GuardrailError> {
    let lower = value.to_ascii_lowercase();
    let unsafe_marker = ['/', '\\', '\n', '\r', '\0', '=', ':', ' ', '\t', '"', '\'']
        .iter()
        .any(|marker| value.contains(*marker))
        || [
            "prompt",
            "credential",
            "secret",
            "token",
            "password",
            "stderr",
            "stdout",
            "command",
            "mount",
            "debug",
        ]
        .iter()
        .any(|marker| lower.contains(marker));
    let valid = !value.is_empty()
        && value.len() <= MAX_SAFE_LABEL_BYTES
        && !unsafe_marker
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'));
    if valid {
        Ok(())
    } else {
        Err(error(GuardrailErrorKind::Receipt, field))
    }
}

fn validate_hash(value: &str) -> bool {
    value.len() == HASH_HEX_BYTES
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn usize_value(value: u64) -> Result<usize, GuardrailError> {
    usize::try_from(value).map_err(|_error| error(GuardrailErrorKind::Profile, FIELD_PROFILE))
}

const fn error(kind: GuardrailErrorKind, field: &'static str) -> GuardrailError {
    GuardrailError::new(kind, field)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        ActiveContext, EnvironmentBinding, ExecutableFacts, RootBinding, SourceKind,
    };

    const GENERATED: &str = include_str!("../../../config/generated/agent-guardrail-profile.json");
    const HASH_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const HASH_B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    const HASH_C: &str = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";
    const HASH_D: &str = "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd";
    const EXPECTED_ROOTS: usize = 2;

    fn profile() -> GuardrailProfile {
        load_generated_profile(GENERATED).expect("checked profile")
    }

    fn request(profile: &GuardrailProfile) -> PreflightRequest {
        let authorization = AuthorizationBinding {
            allowed: true,
            identity: String::from("authorization-id"),
            action_identity: String::from("action-id"),
            subject_identity: String::from("subject-id"),
            target_identity: String::from("target-id"),
            descriptor_identity: String::from("descriptor-id"),
            policy_identity: String::from("policy-id"),
            profile_identity: profile.identity.clone(),
        };
        PreflightRequest {
            context: ActiveContext {
                action_identity: authorization.action_identity.clone(),
                subject_identity: authorization.subject_identity.clone(),
                target_identity: authorization.target_identity.clone(),
                descriptor_identity: authorization.descriptor_identity.clone(),
                policy_identity: authorization.policy_identity.clone(),
                context_identity: String::from("context-id"),
                requested_capabilities: vec![String::from("filesystem-read")],
                authorization,
            },
            roots: vec![
                RootBinding {
                    label: String::from("tool-store"),
                    source_root: "/nix".into(),
                    source: "/nix/store".into(),
                    target: "/nix/store".into(),
                    source_kind: SourceKind::Directory,
                    exists: true,
                    symlink_within_root: true,
                },
                RootBinding {
                    label: String::from("worktree"),
                    source_root: "/workspace".into(),
                    source: "/workspace/project".into(),
                    target: "/workspace".into(),
                    source_kind: SourceKind::Directory,
                    exists: true,
                    symlink_within_root: true,
                },
            ],
            environment: vec![EnvironmentBinding {
                key: String::from("TERM"),
                value: String::from("xterm"),
            }],
            executable: ExecutableFacts {
                path: "/nix/store/bwrap/bin/bwrap".into(),
                identity: String::from(HASH_A),
                cohort: String::from(BUBBLEWRAP_COHORT),
                args_supported: true,
            },
            expected_plan_identity: None,
        }
    }

    #[test]
    fn checked_profile_and_plan_are_deterministic_and_read_only_by_default() {
        let profile = profile();
        let request = request(&profile);
        let first = build_plan(&profile, &request).expect("first plan");
        let second = build_plan(&profile, &request).expect("second plan");
        assert_eq!(first, second);
        assert_eq!(first.root_labels.len(), EXPECTED_ROOTS);
        assert!(
            first
                .operations
                .iter()
                .any(|operation| matches!(operation, PlanOperation::ReadOnlyBind { .. }))
        );
        assert!(
            first
                .operations
                .iter()
                .any(|operation| matches!(operation, PlanOperation::WritableBind { .. }))
        );
        assert_eq!(first.root_labels, ["worktree", "tool-store"]);
        assert_eq!(preflight(&profile, &request).status, PreflightStatus::Valid);
    }

    #[test]
    fn inherited_network_is_explicit_and_identity_bound() {
        let mut profile = profile();
        profile.network = NetworkPosture::Inherit;
        let request = request(&profile);
        let plan = build_plan(&profile, &request).expect("network plan");
        assert!(plan.operations.contains(&PlanOperation::ShareNetwork));
        assert_eq!(plan.network, NetworkPosture::Inherit);
    }

    #[test]
    fn authorization_root_environment_and_plan_drift_fail_closed() {
        let profile = profile();
        let mut denied = request(&profile);
        denied.context.authorization.allowed = false;
        assert_eq!(preflight(&profile, &denied).status, PreflightStatus::Denied);

        let mut escaping = request(&profile);
        escaping.roots[1].source = "/outside/project".into();
        assert_eq!(
            preflight(&profile, &escaping).status,
            PreflightStatus::Invalid
        );

        let mut symlink = request(&profile);
        symlink.roots[1].symlink_within_root = false;
        assert_eq!(
            preflight(&profile, &symlink).status,
            PreflightStatus::Invalid
        );

        let mut environment = request(&profile);
        environment.environment.push(EnvironmentBinding {
            key: String::from("AWS_SECRET_ACCESS_KEY"),
            value: String::from("hidden"),
        });
        assert_eq!(
            preflight(&profile, &environment).status,
            PreflightStatus::Invalid
        );

        let mut drifted = request(&profile);
        drifted.expected_plan_identity = Some(String::from(HASH_B));
        assert_eq!(
            preflight(&profile, &drifted).status,
            PreflightStatus::Drifted
        );
    }

    #[test]
    fn malformed_profiles_and_escape_hatches_are_rejected() {
        let mutations = [
            ("/cohort", serde_json::json!("bubblewrap-unknown")),
            ("/trust_source", serde_json::json!("project-local")),
            ("/devices", serde_json::json!(true)),
            ("/host_sockets", serde_json::json!(true)),
            ("/escape_hatches/raw_arguments", serde_json::json!(true)),
            ("/escape_hatches/command_prefix", serde_json::json!(true)),
            ("/escape_hatches/passthrough", serde_json::json!(true)),
            (
                "/escape_hatches/command_environment_source",
                serde_json::json!(true),
            ),
            ("/escape_hatches/implicit_shell", serde_json::json!(true)),
            (
                "/escape_hatches/unguarded_fallback",
                serde_json::json!(true),
            ),
            (
                "/escape_hatches/privileged_sidecar",
                serde_json::json!(true),
            ),
            (
                "/escape_hatches/project_local_policy",
                serde_json::json!(true),
            ),
            ("/bounds/max_roots", serde_json::json!(0)),
            ("/network", serde_json::json!("filtered")),
        ];
        for (pointer, replacement) in mutations {
            let mut value =
                serde_json::from_str::<serde_json::Value>(GENERATED).expect("profile JSON");
            *value.pointer_mut(pointer).expect("fixture pointer") = replacement;
            assert!(
                load_generated_profile(value.to_string().as_str()).is_err(),
                "mutation {pointer} must fail"
            );
        }
    }

    #[test]
    fn conflicting_targets_and_missing_sources_are_distinct() {
        let profile = profile();
        let mut conflict = request(&profile);
        conflict.roots[1].target = "/nix/store".into();
        assert_eq!(
            preflight(&profile, &conflict).status,
            PreflightStatus::Invalid
        );
        let mut missing = request(&profile);
        missing.roots[1].exists = false;
        assert_eq!(
            preflight(&profile, &missing).status,
            PreflightStatus::MissingSource
        );
    }

    #[test]
    fn safe_receipt_binds_metadata_and_rejects_leaks() {
        let profile = profile();
        let request = request(&profile);
        let plan = build_plan(&profile, &request).expect("plan");
        let observation = ExecutionObservation {
            status: String::from("passed"),
            issue: None,
            exit: crate::ExitClass::Success,
            cleanup: String::from("completed"),
            option_count: plan.operations.len(),
        };
        let receipt = build_receipt(&profile, &request, &plan, &observation).expect("receipt");
        assert_eq!(receipt.plan_identity, plan.identity);
        assert_eq!(receipt.non_claims, NON_CLAIMS);
        let mut leaked = observation;
        leaked.issue = Some(String::from("/home/operator/secret"));
        assert!(build_receipt(&profile, &request, &plan, &leaked).is_err());
    }

    #[test]
    fn executable_identity_is_domain_separated() {
        assert_ne!(
            executable_identity(b"same"),
            domain_hash(PLAN_DOMAIN, b"same")
        );
        assert_ne!(HASH_C, HASH_D);
    }
}
