//! Borrowed authority available only inside the guarded dispatch callback.

/// Actual admitted holder authority and committed evidence for this dispatch.
///
/// This view has private construction and no Clone, Debug, or serde surface.
/// Copied metadata is evidence, not a new live capability. Verification-time
/// authority does not prove later revocation freshness, custody, or safe effects.
///
/// ```compile_fail
/// fn duplicate(view: animus_std_basalt_authority::ScopedAuthority<'_>) {
///     let _copy = view.clone();
/// }
/// ```
///
/// ```compile_fail
/// use animus_std_basalt_authority::*;
/// fn escape<'a, C: HostAuthorityDecisionCommit>(context: BasaltDispatchContext<'_, '_, C>)
///     -> Result<BasaltAuthorizedDispatch<ScopedAuthority<'a>>, BasaltAuthorityFailure>
/// {
///     dispatch_with_basalt_scoped(context, Ok)
/// }
/// ```
///
/// ```compile_fail
/// use animus_std_basalt_authority::*;
/// fn escape_holder<'a, C: HostAuthorityDecisionCommit>(context: BasaltDispatchContext<'_, '_, C>)
///     -> Result<BasaltAuthorizedDispatch<&'a basalt::ucan::HolderInvocationAuthorized>, BasaltAuthorityFailure>
/// {
///     dispatch_with_basalt_scoped(context, |view| Ok(view.holder()))
/// }
/// ```
///
/// ```compile_fail
/// fn forge<'a>(
///     holder: &'a basalt::ucan::HolderInvocationAuthorized,
///     decision: &'a animus_adapters::HostAuthorityDecision,
///     receipt: &'a animus_adapters::HostAuthorityReceipt,
///     commit_identity: &'a str,
/// ) -> animus_std_basalt_authority::ScopedAuthority<'a> {
///     animus_std_basalt_authority::ScopedAuthority { holder, decision, receipt, commit_identity }
/// }
/// ```
///
/// ```compile_fail
/// fn diagnostic(view: animus_std_basalt_authority::ScopedAuthority<'_>) {
///     let _private = format!("{view:?}");
/// }
/// ```
/// A recheck observation cannot escape through the callback result either.
///
/// ```compile_fail
/// use animus_std_basalt_authority::*;
/// use basalt::ucan::*;
/// fn escape_observation<'a, C: HostAuthorityDecisionCommit>(
///     context: BasaltDispatchContext<'_, '_, C>,
///     token: &str,
///     verification: &HolderTokenVerificationContext<'_>,
///     policies: &HolderRecheckPolicies<'_>,
/// ) -> Result<BasaltAuthorizedDispatch<HolderAuthorizationObservation<'a>>, BasaltAuthorityFailure> {
///     dispatch_with_basalt_scoped(context, |view| {
///         recheck_holder_authorization(view.holder(), token, verification, policies)
///             .map_err(|_| animus_adapters::HostAuthorityIssue::ProviderUnavailable)
///     })
/// }
/// ```
// r[impl agent_harness_runtime.scoped_authority.view]
// r[verify agent_harness_runtime.holder_observation.boundary]
pub struct ScopedAuthority<'live> {
    pub(crate) holder: &'live basalt::ucan::HolderInvocationAuthorized,
    pub(crate) decision: &'live animus_adapters::HostAuthorityDecision,
    pub(crate) receipt: &'live animus_adapters::HostAuthorityReceipt,
    pub(crate) commit_identity: &'live str,
}

impl<'live> ScopedAuthority<'live> {
    /// Actual verified holder binding, operation, and effective token time bounds.
    /// This is not reconstructed from receipt fields or a decoded token alone.
    #[must_use]
    pub const fn holder(&self) -> &'live basalt::ucan::HolderInvocationAuthorized {
        self.holder
    }

    /// Safe decision produced by the same live verification.
    #[must_use]
    pub const fn decision(&self) -> &'live animus_adapters::HostAuthorityDecision {
        self.decision
    }

    /// Metadata-only receipt successfully committed before this callback.
    #[must_use]
    pub const fn receipt(&self) -> &'live animus_adapters::HostAuthorityReceipt {
        self.receipt
    }

    /// Validated identity returned by the caller-owned successful commit service.
    #[must_use]
    pub const fn commit_identity(&self) -> &'live str {
        self.commit_identity
    }
}
