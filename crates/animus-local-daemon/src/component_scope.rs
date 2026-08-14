//! Injected component-scope effect shell and daemon generation mapping.

use animus_session_core::component_scope::{
    ComponentScope, ComponentScopeError, ComponentScopeReceipt, ScopeCleanupFailure,
    ScopeCleanupIssueKind, ScopeCloseRequest, ScopeEffectRequest, ScopeGeneration,
    ScopeObservation, ScopeTerminalOutcome, fail_component_scope_cleanup,
    project_component_scope_receipt, reduce_component_scope_observation,
    request_component_scope_close,
};
use blake3::Hasher;
use std::collections::VecDeque;

const WORKER_SCOPE_DOMAIN: &str = "animus/local-daemon-worker-scope/v1";
const CLIENT_DELIVERY_LEASE_DOMAIN: &str = "animus/local-daemon-client-delivery-lease/v1";
const HASH_SEPARATOR: u8 = 0;
const MINIMUM_HOST_EFFECTS: u16 = 1;
const MAPPING_LABEL_COUNT: usize = 3;

/// Explicit upper bound for one scope-close host drive.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComponentScopeHostBounds {
    /// Maximum number of reducer-requested effects executed in one call.
    pub max_effects: u16,
}

impl ComponentScopeHostBounds {
    /// Validates a non-zero effect bound.
    ///
    /// # Errors
    ///
    /// Returns [`ComponentScopeHostError::InvalidBounds`] for zero.
    pub const fn validate(self) -> Result<Self, ComponentScopeHostError> {
        if self.max_effects < MINIMUM_HOST_EFFECTS {
            return Err(ComponentScopeHostError::InvalidBounds);
        }
        Ok(self)
    }
}

/// Safe completion class returned by an injected scope port.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScopePortStatus {
    /// The requested exact effect completed.
    Complete,
    /// The requested effect remains explicitly unresolved.
    Unresolved,
}

/// Generation-bound scope request passed to the delivery port.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScopeBoundPortRequest<'request> {
    /// Scope identity.
    pub scope_id: &'request str,
    /// Exact generation.
    pub generation: ScopeGeneration,
    /// Stable close-plan identity.
    pub plan_identity: &'request str,
}

/// Generation-bound member request passed to member lifecycle ports.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScopeMemberPortRequest<'request> {
    /// Scope identity.
    pub scope_id: &'request str,
    /// Exact generation.
    pub generation: ScopeGeneration,
    /// Stable close-plan identity.
    pub plan_identity: &'request str,
    /// Exact owned member identity.
    pub member_id: &'request str,
}

/// Generation-bound terminal receipt commit request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScopeReceiptPortRequest<'request> {
    /// Scope identity.
    pub scope_id: &'request str,
    /// Exact generation.
    pub generation: ScopeGeneration,
    /// Stable close-plan identity.
    pub plan_identity: &'request str,
    /// Exact metadata-only receipt identity.
    pub receipt_identity: &'request str,
    /// Terminal outcome.
    pub outcome: ScopeTerminalOutcome,
}

/// Injected owner of new-delivery closure.
pub trait ComponentScopeDeliveryPort {
    /// Closes new event or message delivery for the exact generation.
    fn close_delivery(&mut self, request: ScopeBoundPortRequest<'_>) -> ScopePortStatus;
}

/// Injected owner of cancellation requests.
pub trait ComponentScopeCancellationPort {
    /// Requests cancellation for one exact owned member.
    fn request_cancellation(&mut self, request: ScopeMemberPortRequest<'_>) -> ScopePortStatus;
}

/// Injected owner of quiescence observations.
pub trait ComponentScopeQuiescencePort {
    /// Observes whether one exact member is quiescent under host deadline policy.
    fn observe_quiescence(&mut self, request: ScopeMemberPortRequest<'_>) -> ScopePortStatus;
}

/// Injected owner of component and child-scope release effects.
pub trait ComponentScopeReleasePort {
    /// Releases one exact member requested by the reducer.
    fn release_member(&mut self, request: ScopeMemberPortRequest<'_>) -> ScopePortStatus;
}

/// Injected owner of metadata-only terminal receipt publication.
pub trait ComponentScopeReceiptPort {
    /// Commits one exact terminal receipt identity.
    fn commit_receipt(&mut self, request: ScopeReceiptPortRequest<'_>) -> ScopePortStatus;
}

/// Successful host outcome for one clean or explicitly failed cleanup sequence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComponentScopeHostOutcome {
    scope: ComponentScope,
    receipt: ComponentScopeReceipt,
}

impl ComponentScopeHostOutcome {
    /// Returns the final pure scope state.
    #[must_use]
    pub const fn scope(&self) -> &ComponentScope {
        &self.scope
    }

    /// Returns the committed metadata-only receipt.
    #[must_use]
    pub const fn receipt(&self) -> &ComponentScopeReceipt {
        &self.receipt
    }
}

/// Five narrow caller-injected component-scope ports.
pub struct ComponentScopePorts<Delivery, Cancellation, Quiescence, Release, Receipt> {
    /// New-delivery closure port.
    pub delivery: Delivery,
    /// Cancellation request port.
    pub cancellation: Cancellation,
    /// Quiescence observation port.
    pub quiescence: Quiescence,
    /// Member release port.
    pub release: Release,
    /// Terminal receipt commit port.
    pub receipt: Receipt,
}

/// Thin imperative shell over five caller-injected component-scope ports.
pub struct ComponentScopeHost<Delivery, Cancellation, Quiescence, Release, Receipt> {
    delivery: Delivery,
    cancellation: Cancellation,
    quiescence: Quiescence,
    release: Release,
    receipt: Receipt,
    bounds: ComponentScopeHostBounds,
}

impl<Delivery, Cancellation, Quiescence, Release, Receipt>
    ComponentScopeHost<Delivery, Cancellation, Quiescence, Release, Receipt>
where
    Delivery: ComponentScopeDeliveryPort,
    Cancellation: ComponentScopeCancellationPort,
    Quiescence: ComponentScopeQuiescencePort,
    Release: ComponentScopeReleasePort,
    Receipt: ComponentScopeReceiptPort,
{
    /// Creates a shell from explicit ports and a finite effect bound.
    ///
    /// # Errors
    ///
    /// Returns [`ComponentScopeHostError::InvalidBounds`] for a zero bound.
    pub fn new(
        ports: ComponentScopePorts<Delivery, Cancellation, Quiescence, Release, Receipt>,
        bounds: ComponentScopeHostBounds,
    ) -> Result<Self, ComponentScopeHostError> {
        Ok(Self {
            delivery: ports.delivery,
            cancellation: ports.cancellation,
            quiescence: ports.quiescence,
            release: ports.release,
            receipt: ports.receipt,
            bounds: bounds.validate()?,
        })
    }

    /// Drives only effects requested by the pure reducer.
    ///
    /// An unresolved delivery, cancellation, quiescence, or release request is
    /// converted into an explicit failed-cleanup receipt. The shell does not
    /// substitute another provider or claim cleanup.
    ///
    /// # Errors
    ///
    /// Returns a typed error for core rejection, effect-bound exhaustion,
    /// unresolved receipt commit, or an incomplete terminal state.
    pub fn close_scope(
        &mut self,
        scope: &ComponentScope,
    ) -> Result<ComponentScopeHostOutcome, ComponentScopeHostError> {
        let transition = request_component_scope_close(
            scope,
            &ScopeCloseRequest {
                scope_id: String::from(scope.scope_id()),
                generation: scope.generation(),
            },
        )
        .map_err(|_error| ComponentScopeHostError::Core)?;
        let (mut current, effects) = transition.into_parts();
        let mut pending: VecDeque<ScopeEffectRequest> = effects.into();
        let mut executed_effects = 0_u16;

        while let Some(effect) = pending.pop_front() {
            if executed_effects >= self.bounds.max_effects {
                return Err(ComponentScopeHostError::EffectBoundExceeded);
            }
            executed_effects = executed_effects
                .checked_add(1)
                .ok_or(ComponentScopeHostError::EffectBoundExceeded)?;
            let result = self.execute_effect(&effect)?;
            match result {
                EffectExecution::NoObservation => {}
                EffectExecution::Observation(observation) => {
                    let transition = reduce_component_scope_observation(&current, &observation)
                        .map_err(|_error| ComponentScopeHostError::Core)?;
                    let (next, effects) = transition.into_parts();
                    current = next;
                    pending.extend(effects);
                }
                EffectExecution::Unresolved(failure) => {
                    let transition = fail_component_scope_cleanup(&current, &failure)
                        .map_err(|_error| ComponentScopeHostError::Core)?;
                    let (next, effects) = transition.into_parts();
                    current = next;
                    pending.clear();
                    pending.extend(effects);
                }
            }
        }

        if !current.terminal_receipt_committed() {
            return Err(ComponentScopeHostError::InvalidTerminal);
        }
        let receipt = project_component_scope_receipt(&current)
            .map_err(|_error| ComponentScopeHostError::Core)?;
        Ok(ComponentScopeHostOutcome {
            scope: current,
            receipt,
        })
    }

    fn execute_effect(
        &mut self,
        effect: &ScopeEffectRequest,
    ) -> Result<EffectExecution, ComponentScopeHostError> {
        match effect {
            ScopeEffectRequest::CloseDelivery {
                scope_id,
                generation,
                plan_identity,
            } => Ok(self.execute_delivery(ScopeBoundPortRequest {
                scope_id,
                generation: *generation,
                plan_identity,
            })),
            ScopeEffectRequest::RequestCancellation {
                scope_id,
                generation,
                plan_identity,
                member_id,
            } => Ok(self.execute_cancellation(member_request(
                scope_id,
                *generation,
                plan_identity,
                member_id,
            ))),
            ScopeEffectRequest::ObserveQuiescence {
                scope_id,
                generation,
                plan_identity,
                member_id,
            } => Ok(self.execute_quiescence(member_request(
                scope_id,
                *generation,
                plan_identity,
                member_id,
            ))),
            ScopeEffectRequest::ReleaseMember {
                scope_id,
                generation,
                plan_identity,
                member_id,
            } => Ok(self.execute_release(member_request(
                scope_id,
                *generation,
                plan_identity,
                member_id,
            ))),
            ScopeEffectRequest::CommitTerminalReceipt {
                scope_id,
                generation,
                plan_identity,
                receipt_identity,
                outcome,
            } => self.execute_receipt_commit(ScopeReceiptPortRequest {
                scope_id,
                generation: *generation,
                plan_identity,
                receipt_identity,
                outcome: *outcome,
            }),
        }
    }

    fn execute_delivery(&mut self, request: ScopeBoundPortRequest<'_>) -> EffectExecution {
        match self.delivery.close_delivery(request) {
            ScopePortStatus::Complete => {
                EffectExecution::Observation(ScopeObservation::DeliveryClosed {
                    scope_id: String::from(request.scope_id),
                    generation: request.generation,
                    plan_identity: String::from(request.plan_identity),
                })
            }
            ScopePortStatus::Unresolved => EffectExecution::Unresolved(bound_cleanup_failure(
                request,
                ScopeCleanupIssueKind::DeliveryClose,
                request.scope_id,
            )),
        }
    }

    fn execute_cancellation(&mut self, request: ScopeMemberPortRequest<'_>) -> EffectExecution {
        match self.cancellation.request_cancellation(request) {
            ScopePortStatus::Complete => EffectExecution::NoObservation,
            ScopePortStatus::Unresolved => EffectExecution::Unresolved(member_cleanup_failure(
                request,
                ScopeCleanupIssueKind::CancellationRequest,
            )),
        }
    }

    fn execute_quiescence(&mut self, request: ScopeMemberPortRequest<'_>) -> EffectExecution {
        match self.quiescence.observe_quiescence(request) {
            ScopePortStatus::Complete => {
                EffectExecution::Observation(ScopeObservation::MemberQuiescent {
                    scope_id: String::from(request.scope_id),
                    generation: request.generation,
                    plan_identity: String::from(request.plan_identity),
                    member_id: String::from(request.member_id),
                })
            }
            ScopePortStatus::Unresolved => EffectExecution::Unresolved(member_cleanup_failure(
                request,
                ScopeCleanupIssueKind::QuiescenceDeadline,
            )),
        }
    }

    fn execute_release(&mut self, request: ScopeMemberPortRequest<'_>) -> EffectExecution {
        match self.release.release_member(request) {
            ScopePortStatus::Complete => {
                EffectExecution::Observation(ScopeObservation::MemberReleased {
                    scope_id: String::from(request.scope_id),
                    generation: request.generation,
                    plan_identity: String::from(request.plan_identity),
                    member_id: String::from(request.member_id),
                })
            }
            ScopePortStatus::Unresolved => EffectExecution::Unresolved(member_cleanup_failure(
                request,
                ScopeCleanupIssueKind::ReleaseObservation,
            )),
        }
    }

    fn execute_receipt_commit(
        &mut self,
        request: ScopeReceiptPortRequest<'_>,
    ) -> Result<EffectExecution, ComponentScopeHostError> {
        if self.receipt.commit_receipt(request) == ScopePortStatus::Unresolved {
            return Err(ComponentScopeHostError::ReceiptCommitUnresolved);
        }
        Ok(EffectExecution::Observation(
            ScopeObservation::TerminalReceiptCommitted {
                scope_id: String::from(request.scope_id),
                generation: request.generation,
                plan_identity: String::from(request.plan_identity),
                receipt_identity: String::from(request.receipt_identity),
            },
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum EffectExecution {
    NoObservation,
    Observation(ScopeObservation),
    Unresolved(ScopeCleanupFailure),
}

fn member_request<'request>(
    scope_id: &'request str,
    generation: ScopeGeneration,
    plan_identity: &'request str,
    member_id: &'request str,
) -> ScopeMemberPortRequest<'request> {
    ScopeMemberPortRequest {
        scope_id,
        generation,
        plan_identity,
        member_id,
    }
}

fn bound_cleanup_failure(
    request: ScopeBoundPortRequest<'_>,
    kind: ScopeCleanupIssueKind,
    subject: &str,
) -> ScopeCleanupFailure {
    ScopeCleanupFailure {
        scope_id: String::from(request.scope_id),
        generation: request.generation,
        plan_identity: String::from(request.plan_identity),
        kind,
        subject: String::from(subject),
    }
}

fn member_cleanup_failure(
    request: ScopeMemberPortRequest<'_>,
    kind: ScopeCleanupIssueKind,
) -> ScopeCleanupFailure {
    bound_cleanup_failure(
        ScopeBoundPortRequest {
            scope_id: request.scope_id,
            generation: request.generation,
            plan_identity: request.plan_identity,
        },
        kind,
        request.member_id,
    )
}

/// Exact worker-generation to root-scope mapping plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkerRootScopePlan {
    /// Root session identity.
    pub root_id: String,
    /// Worker identity.
    pub worker_id: String,
    /// Exact worker and scope generation.
    pub generation: ScopeGeneration,
    /// Deterministic root component-scope identity.
    pub scope_id: String,
    /// BLAKE3 mapping identity.
    pub identity: String,
}

/// Client-owned delivery lease that does not own the worker root scope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientDeliveryScopeLease {
    /// Deterministic worker root-scope identity.
    pub worker_scope_id: String,
    /// Exact worker generation.
    pub worker_generation: ScopeGeneration,
    /// Client identity.
    pub client_id: String,
    /// BLAKE3 client lease identity.
    pub identity: String,
    worker_scope_owned: bool,
}

impl ClientDeliveryScopeLease {
    /// Returns false because detach owns only this client delivery lease.
    #[must_use]
    pub const fn owns_worker_scope(&self) -> bool {
        self.worker_scope_owned
    }
}

/// Maps one worker registry record to one exact root component-scope plan.
///
/// # Errors
///
/// Returns [`ComponentScopeHostError::Mapping`] for malformed identities or generation.
pub fn plan_worker_root_scope(
    record: &crate::WorkerRegistryRecord,
) -> Result<WorkerRootScopePlan, ComponentScopeHostError> {
    validate_mapping_label(record.root_id.as_str())?;
    validate_mapping_label(record.worker_id.as_str())?;
    let generation = ScopeGeneration::new(record.generation)
        .map_err(|_error| ComponentScopeHostError::Mapping)?;
    let scope_id = worker_scope_identity(
        record.root_id.as_str(),
        record.worker_id.as_str(),
        generation,
    );
    let identity = mapping_identity(
        WORKER_SCOPE_DOMAIN,
        [
            scope_id.as_str(),
            record.root_id.as_str(),
            record.worker_id.as_str(),
        ],
        generation,
    );
    Ok(WorkerRootScopePlan {
        root_id: record.root_id.clone(),
        worker_id: record.worker_id.clone(),
        generation,
        scope_id,
        identity,
    })
}

/// Creates one client delivery lease bound to the worker root scope.
///
/// # Errors
///
/// Returns [`ComponentScopeHostError::Mapping`] for malformed identities.
pub fn plan_client_delivery_scope_lease(
    record: &crate::WorkerRegistryRecord,
    client_id: &str,
) -> Result<ClientDeliveryScopeLease, ComponentScopeHostError> {
    validate_mapping_label(client_id)?;
    let worker = plan_worker_root_scope(record)?;
    let identity = mapping_identity(
        CLIENT_DELIVERY_LEASE_DOMAIN,
        [worker.scope_id.as_str(), record.root_id.as_str(), client_id],
        worker.generation,
    );
    Ok(ClientDeliveryScopeLease {
        worker_scope_id: worker.scope_id,
        worker_generation: worker.generation,
        client_id: String::from(client_id),
        identity,
        worker_scope_owned: false,
    })
}

fn worker_scope_identity(root_id: &str, worker_id: &str, generation: ScopeGeneration) -> String {
    mapping_identity(
        WORKER_SCOPE_DOMAIN,
        [root_id, worker_id, "root-scope"],
        generation,
    )
}

fn mapping_identity(
    domain: &str,
    labels: [&str; MAPPING_LABEL_COUNT],
    generation: ScopeGeneration,
) -> String {
    let mut hasher = Hasher::new();
    for value in core::iter::once(domain).chain(labels) {
        hasher.update(value.as_bytes());
        hasher.update(&[HASH_SEPARATOR]);
    }
    hasher.update(&generation.get().to_le_bytes());
    hasher.finalize().to_hex().to_string()
}

fn validate_mapping_label(value: &str) -> Result<(), ComponentScopeHostError> {
    const MAX_MAPPING_LABEL_BYTES: usize = 256;
    if value.is_empty()
        || value.len() > MAX_MAPPING_LABEL_BYTES
        || value
            .bytes()
            .any(|byte| byte == HASH_SEPARATOR || byte.is_ascii_control())
    {
        return Err(ComponentScopeHostError::Mapping);
    }
    Ok(())
}

/// Safe host-shell failure class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentScopeHostError {
    /// Pure core rejected a transition or observation.
    Core,
    /// Effect bound is zero.
    InvalidBounds,
    /// Reducer-requested effects exceeded the explicit host bound.
    EffectBoundExceeded,
    /// Metadata-only terminal receipt publication remained unresolved.
    ReceiptCommitUnresolved,
    /// The reducer did not reach a receipt-committed terminal state.
    InvalidTerminal,
    /// Worker or client mapping metadata was malformed.
    Mapping,
}

impl core::fmt::Display for ComponentScopeHostError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "component scope host failed: {self:?}")
    }
}

impl std::error::Error for ComponentScopeHostError {}

impl From<ComponentScopeError> for ComponentScopeHostError {
    fn from(_error: ComponentScopeError) -> Self {
        Self::Core
    }
}
