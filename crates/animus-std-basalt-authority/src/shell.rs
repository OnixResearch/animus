use animus_adapters::{
    EffectiveAuthorityDecision, GuardedAction, HOST_AUTHORITY_NON_CLAIMS, HostAuthorityDecision,
    HostAuthorityDecisionDraft, HostAuthorityFreshnessClass, HostAuthorityIssue,
    HostAuthorityReceipt, HostAuthorityReceiptDraft, HostAuthorityReplayClass,
    HostAuthorityRequest, HostAuthorityService, HostAuthorityStatus, RequiredHostGates,
    build_host_authority_decision, build_host_authority_receipt, intersect_host_authority,
};
use basalt::Policy;
use basalt::authority_core::BasaltPolicyRef;
use basalt::ucan::{AudienceDid, HolderInvocationNonce, SignerSignature};
use basalt::ucan_enforcement::{
    UcanEnforcementBoundaries, UcanEnforcementFailure, UcanEnforcementReason,
    UcanEnforcementReceipt, UcanEnforcementRequest, verify_ucan_enforcement_request,
};
use basalt::ucan_profile::BasaltUcanProtocolProfile;

use crate::core::{
    BASALT_POLICY_IDENTITY, BASALT_PROCESS_EXECUTION_POLICY_IDENTITY, basalt_provider_identity,
    map_basalt_reason, normalize_basalt_identity, plan_basalt_request, principal_hash,
    resource_hash,
};

const ALLOWED_CHECKED_FACT_COUNT: u32 = 8;
const DENIED_CHECKED_FACT_COUNT: u32 = 0;
const BLAKE3_HEX_BYTES: usize = 64;

/// Raw holder invocation and caller-owned verification services.
pub struct BasaltAuthorityInvocation<'a> {
    /// Raw compact UCAN supplied by the caller.
    pub compact_token: &'a str,
    /// Holder invocation nonce supplied by the caller.
    pub nonce: HolderInvocationNonce,
    /// Holder signature supplied by the caller.
    pub holder_signature: &'a SignerSignature,
    /// Expected audience DID admitted by the agent binding.
    pub audience: &'a AudienceDid,
    /// Already parsed checked Basalt policy.
    pub policy: &'a Policy,
    /// Exact checked Basalt policy reference.
    pub policy_ref: &'a BasaltPolicyRef,
    /// Caller-owned key, proof, revocation, caveat, replay, time, and limit services.
    pub boundaries: UcanEnforcementBoundaries<'a>,
}

/// Optional Basalt implementation of the generic Animus authority-provider port.
pub struct BasaltAuthorityService<'a> {
    invocation: BasaltAuthorityInvocation<'a>,
}

impl<'a> BasaltAuthorityService<'a> {
    /// Construct one request-scoped provider from explicit caller-owned boundaries.
    #[must_use]
    pub const fn new(invocation: BasaltAuthorityInvocation<'a>) -> Self {
        Self { invocation }
    }

    /// Verify raw holder authority and preserve safe denial evidence.
    ///
    /// # Errors
    ///
    /// Returns a metadata-only failure for unsupported mappings, mismatches,
    /// unavailable services, stale inputs, replay denial, or policy denial.
    pub fn authorize_with_evidence(
        &self,
        request: &HostAuthorityRequest,
    ) -> Result<HostAuthorityDecision, BasaltAuthorityFailure> {
        self.with_live_authority(request, |decision| Ok(decision.clone()))
    }

    fn with_live_authority<T, F>(
        &self,
        request: &HostAuthorityRequest,
        admitted: F,
    ) -> Result<T, BasaltAuthorityFailure>
    where
        F: FnOnce(&HostAuthorityDecision) -> Result<T, BasaltAuthorityFailure>,
    {
        preflight_invocation(request, &self.invocation)?;
        let plan = plan_basalt_request(request).map_err(BasaltAuthorityFailure::without_receipt)?;
        let raw = UcanEnforcementRequest::new(
            &plan.enforcement,
            self.invocation.compact_token,
            BasaltUcanProtocolProfile::OnixCompactEd25519V1,
            plan.action,
            self.invocation.nonce,
            self.invocation.holder_signature,
        );
        match verify_ucan_enforcement_request(
            self.invocation.policy,
            self.invocation.policy_ref,
            &raw,
            &self.invocation.boundaries,
        ) {
            Ok(live) => {
                let authorized = live.holder_authorization();
                if authorized.audience() != self.invocation.audience
                    || authorized.audience().as_str() != request.audience_did()
                    || authorized.resource() != request.resource()
                    || authorized.ability() != request.ability()
                {
                    return Err(BasaltAuthorityFailure::without_receipt(
                        HostAuthorityIssue::DecisionMismatch,
                    ));
                }
                let decision = build_allowed_decision(request, live.receipt())
                    .map_err(BasaltAuthorityFailure::without_receipt)?;
                admitted(&decision)
            }
            Err(failure) => Err(build_denied_failure(request, &failure)),
        }
    }
}

impl HostAuthorityService for BasaltAuthorityService<'_> {
    fn authorize(
        &mut self,
        request: &HostAuthorityRequest,
    ) -> Result<HostAuthorityDecision, HostAuthorityIssue> {
        self.authorize_with_evidence(request)
            .map_err(|failure| failure.issue())
    }
}

/// Durable safe-decision commit port. Implementations retain persistence authority.
pub trait HostAuthorityDecisionCommit {
    /// Commit one metadata-only decision receipt before its guarded effect.
    ///
    /// # Errors
    ///
    /// Returns a stable issue when durable commit is unavailable or fails.
    fn commit_decision(
        &mut self,
        receipt: &HostAuthorityReceipt,
    ) -> Result<String, HostAuthorityIssue>;
}

/// Successful guarded dispatch after provider, local-gate, and commit admission.
#[derive(Debug)]
pub struct BasaltAuthorizedDispatch<T> {
    /// Effect result.
    pub output: T,
    /// Metadata-only committed authority-decision receipt.
    pub authority_receipt: HostAuthorityReceipt,
    /// Caller-owned durable commit identity.
    pub commit_identity: String,
    /// Redacted Basalt receipt identity.
    pub basalt_receipt_identity: String,
}

/// Metadata-only fail-closed outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasaltAuthorityFailure {
    issue: HostAuthorityIssue,
    authority_receipt: Option<Box<HostAuthorityReceipt>>,
    basalt_receipt: Option<Box<UcanEnforcementReceipt>>,
}

impl BasaltAuthorityFailure {
    /// Return the stable generic issue.
    #[must_use]
    pub const fn issue(&self) -> HostAuthorityIssue {
        self.issue
    }

    /// Return optional generic metadata-only evidence.
    #[must_use]
    pub fn authority_receipt(&self) -> Option<&HostAuthorityReceipt> {
        self.authority_receipt.as_deref()
    }

    /// Return optional Basalt metadata-only evidence.
    #[must_use]
    pub fn basalt_receipt(&self) -> Option<&UcanEnforcementReceipt> {
        self.basalt_receipt.as_deref()
    }

    fn without_receipt(issue: HostAuthorityIssue) -> Self {
        Self {
            issue,
            authority_receipt: None,
            basalt_receipt: None,
        }
    }
}

/// Explicit shell inputs for one guarded Basalt dispatch.
pub struct BasaltDispatchContext<'a, 'provider, C> {
    /// Request-scoped live Basalt provider.
    pub service: &'a BasaltAuthorityService<'provider>,
    /// Exact validated generic request.
    pub request: &'a HostAuthorityRequest,
    /// Every required independent Animus gate.
    pub gates: RequiredHostGates,
    /// Caller-owned durable decision commit service.
    pub commit: &'a mut C,
}

/// Gate and dispatch one effect while live Basalt admission remains in scope.
///
/// The function calls the effect only after exact raw UCAN verification, every
/// required Animus gate, and durable commit of metadata-only safe evidence.
///
/// # Errors
///
/// Returns a fail-closed metadata-only outcome. The effect is not called after
/// any provider, local-gate, or commit failure.
pub fn dispatch_with_basalt<T, C, E>(
    context: BasaltDispatchContext<'_, '_, C>,
    effect: E,
) -> Result<BasaltAuthorizedDispatch<T>, BasaltAuthorityFailure>
where
    C: HostAuthorityDecisionCommit,
    E: FnOnce() -> Result<T, HostAuthorityIssue>,
{
    let BasaltDispatchContext {
        service,
        request,
        gates,
        commit,
    } = context;
    service.with_live_authority(request, |decision| {
        let effective = intersect_host_authority(decision, gates);
        if effective.status != HostAuthorityStatus::Allowed {
            return Err(build_local_failure(request, decision, effective));
        }
        let authority_receipt = build_host_authority_receipt(HostAuthorityReceiptDraft {
            decision,
            dispatch_class: "committed",
            non_claims: &HOST_AUTHORITY_NON_CLAIMS,
        })
        .map_err(BasaltAuthorityFailure::without_receipt)?;
        let commit_identity = commit
            .commit_decision(&authority_receipt)
            .map_err(BasaltAuthorityFailure::without_receipt)?;
        if !valid_blake3_identity(commit_identity.as_str()) {
            return Err(BasaltAuthorityFailure::without_receipt(
                HostAuthorityIssue::CommitFailed,
            ));
        }
        let output = effect().map_err(|issue| BasaltAuthorityFailure {
            issue,
            authority_receipt: Some(Box::new(authority_receipt.clone())),
            basalt_receipt: None,
        })?;
        Ok(BasaltAuthorizedDispatch {
            output,
            basalt_receipt_identity: String::from(decision.provider_receipt_identity()),
            authority_receipt,
            commit_identity,
        })
    })
}

fn preflight_invocation(
    request: &HostAuthorityRequest,
    invocation: &BasaltAuthorityInvocation<'_>,
) -> Result<(), BasaltAuthorityFailure> {
    let policy_identity = normalize_basalt_identity(invocation.policy_ref.artifact_ref.as_str());
    let required_policy_identity = match request.action() {
        GuardedAction::Exec => BASALT_PROCESS_EXECUTION_POLICY_IDENTITY,
        _ => BASALT_POLICY_IDENTITY,
    };
    if policy_identity.as_deref() != Some(request.policy_identity())
        || request.policy_identity() != required_policy_identity
        || invocation.policy_ref.contract_id != request.contract()
    {
        return Err(BasaltAuthorityFailure::without_receipt(
            HostAuthorityIssue::PolicyDenied,
        ));
    }
    if invocation.audience.as_str() != request.audience_did() {
        return Err(BasaltAuthorityFailure::without_receipt(
            HostAuthorityIssue::BindingMismatch,
        ));
    }
    Ok(())
}

fn build_allowed_decision(
    request: &HostAuthorityRequest,
    receipt: &UcanEnforcementReceipt,
) -> Result<HostAuthorityDecision, HostAuthorityIssue> {
    if !receipt.is_allowed() || receipt.reason != UcanEnforcementReason::Allowed {
        return Err(HostAuthorityIssue::DecisionMismatch);
    }
    let provider_identity = basalt_provider_identity();
    let principal = principal_hash(request.audience_did());
    let resource = resource_hash(request.resource());
    build_host_authority_decision(HostAuthorityDecisionDraft {
        request,
        provider_identity: provider_identity.as_str(),
        policy_identity: request.policy_identity(),
        status: HostAuthorityStatus::Allowed,
        issue: HostAuthorityIssue::None,
        principal_hash: principal.as_str(),
        resource_hash: resource.as_str(),
        checked_fact_count: ALLOWED_CHECKED_FACT_COUNT,
        freshness: HostAuthorityFreshnessClass::Current,
        replay: HostAuthorityReplayClass::Admitted,
        provider_receipt_identity: normalize_basalt_identity(receipt.receipt_blake3.as_str())
            .as_deref()
            .ok_or(HostAuthorityIssue::UnsafeReceipt)?,
    })
}

fn build_denied_failure(
    request: &HostAuthorityRequest,
    failure: &UcanEnforcementFailure,
) -> BasaltAuthorityFailure {
    let issue = map_basalt_reason(failure.reason());
    let receipt = failure.receipt().clone();
    let decision = build_denied_decision(request, issue, &receipt);
    let authority_receipt = decision.and_then(|decision| {
        build_host_authority_receipt(HostAuthorityReceiptDraft {
            decision: &decision,
            dispatch_class: "blocked",
            non_claims: &HOST_AUTHORITY_NON_CLAIMS,
        })
    });
    BasaltAuthorityFailure {
        issue,
        authority_receipt: authority_receipt.ok().map(Box::new),
        basalt_receipt: Some(Box::new(receipt)),
    }
}

fn build_denied_decision(
    request: &HostAuthorityRequest,
    issue: HostAuthorityIssue,
    receipt: &UcanEnforcementReceipt,
) -> Result<HostAuthorityDecision, HostAuthorityIssue> {
    let provider_identity = basalt_provider_identity();
    let principal = principal_hash(request.audience_did());
    let resource = resource_hash(request.resource());
    let provider_receipt_identity = normalize_basalt_identity(receipt.receipt_blake3.as_str())
        .ok_or(HostAuthorityIssue::UnsafeReceipt)?;
    let freshness = match failure_freshness(receipt.reason) {
        Some(value) => value,
        None => HostAuthorityFreshnessClass::Unavailable,
    };
    let replay = if receipt.reason == UcanEnforcementReason::ReplayDenied {
        HostAuthorityReplayClass::Denied
    } else {
        HostAuthorityReplayClass::Unavailable
    };
    build_host_authority_decision(HostAuthorityDecisionDraft {
        request,
        provider_identity: provider_identity.as_str(),
        policy_identity: request.policy_identity(),
        status: HostAuthorityStatus::Denied,
        issue,
        principal_hash: principal.as_str(),
        resource_hash: resource.as_str(),
        checked_fact_count: DENIED_CHECKED_FACT_COUNT,
        freshness,
        replay,
        provider_receipt_identity: provider_receipt_identity.as_str(),
    })
}

fn build_local_failure(
    request: &HostAuthorityRequest,
    allowed: &HostAuthorityDecision,
    effective: EffectiveAuthorityDecision,
) -> BasaltAuthorityFailure {
    let denied = build_host_authority_decision(HostAuthorityDecisionDraft {
        request,
        provider_identity: allowed.provider_identity(),
        policy_identity: allowed.policy_identity(),
        status: HostAuthorityStatus::Denied,
        issue: effective.issue,
        principal_hash: allowed.principal_hash(),
        resource_hash: allowed.resource_hash(),
        checked_fact_count: allowed.checked_fact_count(),
        freshness: allowed.freshness(),
        replay: allowed.replay(),
        provider_receipt_identity: allowed.provider_receipt_identity(),
    });
    let authority_receipt = denied.and_then(|decision| {
        build_host_authority_receipt(HostAuthorityReceiptDraft {
            decision: &decision,
            dispatch_class: "blocked",
            non_claims: &HOST_AUTHORITY_NON_CLAIMS,
        })
    });
    BasaltAuthorityFailure {
        issue: effective.issue,
        authority_receipt: authority_receipt.ok().map(Box::new),
        basalt_receipt: None,
    }
}

const fn failure_freshness(reason: UcanEnforcementReason) -> Option<HostAuthorityFreshnessClass> {
    match reason {
        UcanEnforcementReason::TimeDenied => Some(HostAuthorityFreshnessClass::Stale),
        UcanEnforcementReason::Allowed => Some(HostAuthorityFreshnessClass::Current),
        _ => None,
    }
}

fn valid_blake3_identity(value: &str) -> bool {
    value.strip_prefix("b3:").is_some_and(|hex| {
        hex.len() == BLAKE3_HEX_BYTES
            && hex
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    })
}
