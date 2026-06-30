/// Plans one std-shell adapter effect from deterministic inputs.
///
/// The returned receipt contains only safe labels, counts, redaction markers, and
/// a BLAKE3 payload hash. Raw prompts, provider payloads, credentials, paths,
/// tool bodies, and model outputs are not copied into the receipt.
///
/// # Errors
///
/// Returns [`animus_adapters::SafeReceiptError`] when a caller supplied unsafe
/// metadata labels or count values that cannot be represented safely.
pub fn plan_adapter_effect(
    descriptor: super::taxonomy::AdapterDescriptor<'_>,
    prerequisites: super::gates::EffectPrerequisites,
    payload: &str,
) -> Result<super::outcome::AdapterPlan, animus_adapters::SafeReceiptError> {
    debug_assert!(descriptor.family().is_shell_owned());
    debug_assert!(payload.len() >= payload.chars().count());

    let issue = select_issue(prerequisites);
    let verdict = verdict_from_issue(issue);
    let receipt = super::receipt::build(descriptor, issue, payload)?;

    Ok(super::outcome::AdapterPlan::new(
        descriptor.family(),
        verdict,
        receipt,
    ))
}

fn verdict_from_issue(issue: super::outcome::AdapterIssue) -> super::outcome::AdapterVerdict {
    match issue {
        super::outcome::AdapterIssue::None => super::outcome::AdapterVerdict::Allowed,
        other => super::outcome::AdapterVerdict::Blocked { issue: other },
    }
}

fn select_issue(prerequisites: super::gates::EffectPrerequisites) -> super::outcome::AdapterIssue {
    debug_assert!(gate_state_is_known(prerequisites.provider_registry()));
    debug_assert!(gate_state_is_known(prerequisites.tool_contract()));
    debug_assert!(gate_state_is_known(prerequisites.policy()));
    debug_assert!(termination_state_is_known(prerequisites.termination()));

    match prerequisites.termination() {
        super::gates::TerminationState::TimedOut => return super::outcome::AdapterIssue::TimedOut,
        super::gates::TerminationState::Cancelled => {
            return super::outcome::AdapterIssue::Cancelled;
        }
        super::gates::TerminationState::Continue => {}
    }
    match prerequisites.provider_registry() {
        super::gates::GateState::Missing | super::gates::GateState::Mismatch => {
            return super::outcome::AdapterIssue::MissingProviderRegistry;
        }
        super::gates::GateState::Denied => return super::outcome::AdapterIssue::DeniedCapability,
        super::gates::GateState::Satisfied => {}
    }
    match prerequisites.tool_contract() {
        super::gates::GateState::Missing | super::gates::GateState::Mismatch => {
            return super::outcome::AdapterIssue::MissingToolContract;
        }
        super::gates::GateState::Denied => return super::outcome::AdapterIssue::DeniedCapability,
        super::gates::GateState::Satisfied => {}
    }
    match prerequisites.policy() {
        super::gates::GateState::Missing => return super::outcome::AdapterIssue::MissingPolicy,
        super::gates::GateState::Denied | super::gates::GateState::Mismatch => {
            return super::outcome::AdapterIssue::DeniedCapability;
        }
        super::gates::GateState::Satisfied => {}
    }
    select_issue_after_policy(prerequisites)
}

fn select_issue_after_policy(
    prerequisites: super::gates::EffectPrerequisites,
) -> super::outcome::AdapterIssue {
    debug_assert!(gate_state_is_known(prerequisites.sandbox()));
    debug_assert!(gate_state_is_known(prerequisites.receipt_contract()));
    debug_assert!(gate_state_is_known(prerequisites.event_contract()));

    match prerequisites.sandbox() {
        super::gates::GateState::Missing => return super::outcome::AdapterIssue::MissingSandbox,
        super::gates::GateState::Denied | super::gates::GateState::Mismatch => {
            return super::outcome::AdapterIssue::SandboxMismatch;
        }
        super::gates::GateState::Satisfied => {}
    }
    match prerequisites.receipt_contract() {
        super::gates::GateState::Missing | super::gates::GateState::Mismatch => {
            return super::outcome::AdapterIssue::MissingReceiptContract;
        }
        super::gates::GateState::Denied => return super::outcome::AdapterIssue::DeniedCapability,
        super::gates::GateState::Satisfied => {}
    }
    match prerequisites.event_contract() {
        super::gates::GateState::Missing | super::gates::GateState::Mismatch => {
            return super::outcome::AdapterIssue::MissingEventContract;
        }
        super::gates::GateState::Denied => return super::outcome::AdapterIssue::DeniedCapability,
        super::gates::GateState::Satisfied => {}
    }
    select_issue_after_contracts(prerequisites)
}

fn select_issue_after_contracts(
    prerequisites: super::gates::EffectPrerequisites,
) -> super::outcome::AdapterIssue {
    debug_assert!(gate_state_is_known(prerequisites.host_approval()));
    debug_assert!(credential_state_is_known(prerequisites.credentials()));
    debug_assert!(output_state_is_known(prerequisites.output()));

    match prerequisites.host_approval() {
        super::gates::GateState::Missing => {
            return super::outcome::AdapterIssue::MissingHostApproval;
        }
        super::gates::GateState::Denied | super::gates::GateState::Mismatch => {
            return super::outcome::AdapterIssue::DeniedCapability;
        }
        super::gates::GateState::Satisfied => {}
    }
    match prerequisites.credentials() {
        super::gates::CredentialState::Missing => {
            return super::outcome::AdapterIssue::MissingCredentials;
        }
        super::gates::CredentialState::NotRequired | super::gates::CredentialState::Present => {}
    }
    match prerequisites.output() {
        super::gates::OutputState::Malformed => super::outcome::AdapterIssue::MalformedOutput,
        super::gates::OutputState::Valid => super::outcome::AdapterIssue::None,
    }
}

const fn gate_state_is_known(state: super::gates::GateState) -> bool {
    match state {
        super::gates::GateState::Satisfied
        | super::gates::GateState::Missing
        | super::gates::GateState::Denied
        | super::gates::GateState::Mismatch => true,
    }
}

const fn credential_state_is_known(state: super::gates::CredentialState) -> bool {
    match state {
        super::gates::CredentialState::NotRequired
        | super::gates::CredentialState::Present
        | super::gates::CredentialState::Missing => true,
    }
}

const fn output_state_is_known(state: super::gates::OutputState) -> bool {
    match state {
        super::gates::OutputState::Valid | super::gates::OutputState::Malformed => true,
    }
}

const fn termination_state_is_known(state: super::gates::TerminationState) -> bool {
    match state {
        super::gates::TerminationState::Continue
        | super::gates::TerminationState::TimedOut
        | super::gates::TerminationState::Cancelled => true,
    }
}
