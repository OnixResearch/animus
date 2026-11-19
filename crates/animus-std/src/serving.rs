//! Non-default serving-readiness and request-admission host path.
//!
//! One complete host path from a supplied availability snapshot through
//! eligibility screening, existing recipe selection, serving-owner admission,
//! a current recheck, the Mesh provider shell, response provenance, and a
//! metadata-only usage receipt. The serving owner and the transport are
//! injected; this module discovers, probes, allocates, and retries nothing.

mod codec;
mod fixture;
#[cfg(test)]
mod tests;

pub use codec::{CodecError, MAX_SNAPSHOT_BYTES, parse_profile, parse_snapshot, receipt_json};
pub use fixture::{FixtureOwner, OwnerCall};

use animus_adapters::serving::{
    AdmissionQuery, AdmissionRecord, CallerBinding, ContractIssue, CurrentReading, CurrentState,
    DispatchOutcome, DispatchPermit, EligibleRoute, EvidenceLabel, GateInput, GateVerdict,
    InvocationReceipt, NotDispatched, OwnerReply, Readiness, ReceiptInput, ResponseProvenance,
    ResponseRejection, ScreenRequest, Screening, UncertainCause, Usage,
};

use crate::mesh::{InferenceRequest, MeshHostError, MeshTransport, ProviderObservation};

/// Owned current serving observation returned by an [`OwnerPort`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrentRecord {
    /// Current service identifier.
    pub service_id: String,
    /// Current endpoint identity.
    pub endpoint_identity: String,
    /// Current deployment identifier.
    pub deployment_id: String,
    /// Current deployment generation.
    pub deployment_generation: u64,
    /// Currently served model.
    pub model_id: String,
    /// Currently served configuration.
    pub configuration_ref: String,
    /// Current readiness.
    pub readiness: Readiness,
    /// Current owner admission epoch.
    pub admission_epoch: u64,
    /// Current owner logical tick; `None` is unknown.
    pub now_tick: Option<u64>,
    /// Currently loaded rollout policy generation, if established.
    pub policy_generation: Option<u64>,
}

impl CurrentRecord {
    /// Borrowed view for the pure gate.
    #[must_use]
    pub fn state(&self) -> CurrentState<'_> {
        CurrentState {
            service_id: &self.service_id,
            endpoint_identity: &self.endpoint_identity,
            deployment_id: &self.deployment_id,
            deployment_generation: self.deployment_generation,
            model_id: &self.model_id,
            configuration_ref: &self.configuration_ref,
            readiness: self.readiness,
            admission_epoch: self.admission_epoch,
            now_tick: self.now_tick,
            policy_generation: self.policy_generation,
        }
    }
}

/// The host's reply to a current-state recheck.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CurrentReply {
    /// The host observed the current state.
    Observed(CurrentRecord),
    /// No recheck service was reachable.
    Unavailable,
    /// The recheck port failed.
    PortIssue,
}

impl CurrentReply {
    fn reading(&self) -> CurrentReading<'_> {
        match self {
            Self::Observed(record) => CurrentReading::Observed(record.state()),
            Self::Unavailable => CurrentReading::Unavailable,
            Self::PortIssue => CurrentReading::PortIssue,
        }
    }
}

/// Injected serving-owner authority. Implementations own the network,
/// credentials, deadlines, and the owner's admission ledger.
pub trait OwnerPort {
    /// Asks the serving owner to admit one request. A snapshot's capacity is
    /// never a reservation; only this reply can grant a request slot.
    fn admit(&mut self, query: &AdmissionQuery) -> OwnerReply;

    /// Observes the current endpoint, deployment generation, model,
    /// readiness, admission epoch, and owner tick immediately before
    /// provider contact.
    fn recheck(&mut self, query: &AdmissionQuery) -> CurrentReply;
}

/// The standalone owner port: every admission and recheck is unavailable, so
/// protected dispatch fails closed before any provider effect.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UnavailableOwner;

impl OwnerPort for UnavailableOwner {
    fn admit(&mut self, _query: &AdmissionQuery) -> OwnerReply {
        OwnerReply::Unavailable
    }

    fn recheck(&mut self, _query: &AdmissionQuery) -> CurrentReply {
        CurrentReply::Unavailable
    }
}

/// Existing router selection inputs. Selection only sees the eligible set.
#[derive(Clone, Copy, Debug)]
pub struct Selection<'a> {
    /// Projected request signals.
    pub signals: &'a animus_router::Signals,
    /// Validated recipes.
    pub recipes: &'a [animus_router::Recipe],
    /// Budget and backend allowlist.
    pub budget: &'a animus_router::Budget,
    /// Optional host-owned rotation snapshot.
    pub rotation: Option<&'a animus_router::RouterRotationSnapshot>,
}

/// One invocation through the host path.
#[derive(Clone, Copy, Debug)]
pub struct Invocation<'a> {
    /// Fixture or live-endpoint label carried onto the receipt.
    pub evidence: EvidenceLabel,
    /// Profile, snapshot, freshness, privacy, and named routes.
    pub screen: ScreenRequest<'a>,
    /// Existing recipe selection inputs.
    pub selection: Selection<'a>,
    /// Caller identities.
    pub caller: &'a CallerBinding,
    /// Request identity.
    pub request_id: &'a str,
    /// Existing profile, privacy, budget, authority, remote, deadline,
    /// cancellation, and transport decisions.
    pub existing: animus_adapters::mesh::DispatchAdmission,
    /// Safe provider label for the existing Mesh receipt.
    pub provider_label: &'a str,
    /// Reviewed projected request bytes; never retained or hashed into receipts.
    pub projected_body: &'a [u8],
}

/// Result of one invocation. It has no `Debug` so bounded provider bytes are
/// never formatted by accident.
pub struct InvocationReport {
    /// Screening behind the selection.
    pub screening: Screening,
    /// Metadata-only receipt.
    pub receipt: InvocationReceipt,
    /// Bounded provider observation when the provider answered.
    pub observation: Option<ProviderObservation>,
    /// Existing Mesh receipt when one could be projected.
    pub provider_receipt: Option<animus_adapters::mesh::Receipt>,
}

/// Host-path failure on invalid inputs. Every runtime refusal is a receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InvocationError {
    /// A candidate, request identity, or receipt contract failed.
    Contract(ContractIssue),
}

impl From<ContractIssue> for InvocationError {
    fn from(issue: ContractIssue) -> Self {
        Self::Contract(issue)
    }
}

/// Runs one invocation: screen, select, admit, recheck, gate, dispatch, and record.
///
/// # Errors
///
/// Returns [`InvocationError::Contract`] only for invalid inputs. Refusals,
/// blocks, uncertain dispatches, and rejected responses are receipts.
pub fn invoke<O: OwnerPort, T: MeshTransport>(
    owner: &mut O,
    transport: &mut T,
    invocation: &Invocation<'_>,
) -> Result<InvocationReport, InvocationError> {
    let screening = animus_adapters::serving::screen_routes(&invocation.screen)?;
    let Some(route) = select(&screening, &invocation.selection) else {
        let reason = if screening.eligible().is_empty() {
            NotDispatched::NoEligibleRoute
        } else {
            NotDispatched::SelectionDenied
        };
        return refused(
            invocation,
            screening,
            None,
            &(AdmissionRecord::NotRequested, reason),
        );
    };
    let query = animus_adapters::serving::admission_query(
        &route,
        invocation.caller,
        invocation.request_id,
    )?;
    let reply = owner.admit(&query);
    let admission = AdmissionRecord::from_reply(&reply);
    let current = match reply {
        OwnerReply::Granted(_) => owner.recheck(&query),
        OwnerReply::Rejected(_) | OwnerReply::Unavailable | OwnerReply::PortIssue => {
            CurrentReply::Unavailable
        }
    };
    let verdict = animus_adapters::serving::gate_dispatch(&GateInput {
        profile: invocation.screen.profile,
        query: &query,
        owner: &reply,
        current: current.reading(),
        existing: invocation.existing,
    });
    let permit = match verdict {
        GateVerdict::Admitted(permit) => permit,
        GateVerdict::Rejected(rejection) => {
            let reason = NotDispatched::Rejected(rejection);
            return refused(invocation, screening, Some(&route), &(admission, reason));
        }
        GateVerdict::Blocked(block) => {
            let reason = NotDispatched::Blocked(block);
            return refused(invocation, screening, Some(&route), &(admission, reason));
        }
    };
    dispatch(
        transport,
        invocation,
        Dispatch {
            screening,
            route,
            query,
            admission,
            permit,
        },
    )
}

/// Selects one eligible route through existing recipe selection and rotation.
fn select(screening: &Screening, selection: &Selection<'_>) -> Option<EligibleRoute> {
    if screening.eligible().is_empty() {
        return None;
    }
    let catalog = screening.eligible_catalog();
    let plan = match selection.rotation {
        None => animus_router::select_recipe(
            selection.signals,
            selection.recipes,
            &catalog,
            selection.budget,
        ),
        Some(rotation) => {
            animus_router::select_recipe_with_rotation(animus_router::RotatingSelectionRequest {
                signals: selection.signals,
                recipes: selection.recipes,
                catalog: &catalog,
                budget: selection.budget,
                rotation,
            })
        }
    };
    plan.ok()?
        .selected_models()
        .first()
        .and_then(|label| screening.route_for_label(label))
        .cloned()
}

struct Dispatch {
    screening: Screening,
    route: EligibleRoute,
    query: AdmissionQuery,
    admission: AdmissionRecord,
    permit: DispatchPermit,
}

fn dispatch<T: MeshTransport>(
    transport: &mut T,
    invocation: &Invocation<'_>,
    state: Dispatch,
) -> Result<InvocationReport, InvocationError> {
    let outcome = crate::mesh::execute(
        transport,
        &InferenceRequest {
            admission: &invocation.existing,
            endpoint_identity: state.route.deployment().endpoint_identity(),
            exact_model: state.route.model().model_id(),
            projected_body: invocation.projected_body,
        },
    );
    let classified = classify(&state.query, invocation.existing, outcome);
    let provider_receipt = classified
        .observation
        .as_ref()
        .and_then(|observation| mesh_receipt(invocation, &state, observation, classified.outcome));
    let permit = classified.outcome.is_dispatched().then_some(&state.permit);
    let receipt = animus_adapters::serving::project_receipt(&ReceiptInput {
        evidence: invocation.evidence,
        request_id: invocation.request_id,
        caller: invocation.caller,
        screening: &state.screening,
        selected: Some(&state.route),
        admission: &state.admission,
        permit,
        outcome: classified.outcome,
        response: classified.response,
        usage: classified.usage,
        provider_receipt_identity: provider_receipt
            .as_ref()
            .map(|receipt| receipt.identity.as_str()),
    })?;
    Ok(InvocationReport {
        screening: state.screening,
        receipt,
        observation: classified.observation,
        provider_receipt,
    })
}

struct Classified {
    outcome: DispatchOutcome,
    response: Option<ResponseProvenance>,
    usage: Usage,
    observation: Option<ProviderObservation>,
}

/// Maps the Mesh shell result. Errors after possible provider contact are
/// uncertain and never retried here.
fn classify(
    query: &AdmissionQuery,
    existing: animus_adapters::mesh::DispatchAdmission,
    outcome: Result<ProviderObservation, MeshHostError>,
) -> Classified {
    match outcome {
        Ok(observation) => {
            let response = animus_adapters::serving::response_provenance(
                query,
                Some(query.model().model_id()),
                &observation.facts,
            );
            Classified {
                outcome: response.rejection().map_or(
                    DispatchOutcome::Completed,
                    DispatchOutcome::ResponseRejected,
                ),
                response: Some(response),
                usage: observation.facts.usage,
                observation: Some(observation),
            }
        }
        Err(error) => Classified {
            outcome: error_outcome(error, existing),
            response: None,
            usage: Usage::UNKNOWN,
            observation: None,
        },
    }
}

/// `AdmissionDenied` and `MissingTransport` happen before provider contact.
fn error_outcome(
    error: MeshHostError,
    existing: animus_adapters::mesh::DispatchAdmission,
) -> DispatchOutcome {
    let decision = match error {
        MeshHostError::MissingTransport => {
            animus_adapters::mesh::DispatchDecision::MissingTransport
        }
        MeshHostError::AdmissionDenied => animus_adapters::mesh::admit_dispatch(&existing),
        MeshHostError::ResponseTooLarge => {
            return DispatchOutcome::ResponseRejected(ResponseRejection::Oversized);
        }
        MeshHostError::ModelSubstitution => {
            return DispatchOutcome::ResponseRejected(ResponseRejection::ModelSubstituted);
        }
        MeshHostError::TransportFailed => {
            return DispatchOutcome::Uncertain(UncertainCause::TransportFailed);
        }
        MeshHostError::Cancelled => return DispatchOutcome::Uncertain(UncertainCause::Cancelled),
        MeshHostError::TimedOut => return DispatchOutcome::Uncertain(UncertainCause::TimedOut),
    };
    DispatchOutcome::NotDispatched(NotDispatched::Rejected(
        animus_adapters::serving::GateRejection::ExistingPolicy(decision),
    ))
}

/// Projects the existing metadata-only Mesh receipt, binding the endpoint,
/// configuration, snapshot, request, response hash, and grant. Labels the
/// Mesh receipt cannot carry leave it absent rather than altered.
fn mesh_receipt(
    invocation: &Invocation<'_>,
    state: &Dispatch,
    observation: &ProviderObservation,
    outcome: DispatchOutcome,
) -> Option<animus_adapters::mesh::Receipt> {
    let response_identity = blake3::hash(&observation.body).to_hex();
    let status = match outcome {
        DispatchOutcome::Completed => "passed",
        DispatchOutcome::NotDispatched(_)
        | DispatchOutcome::ResponseRejected(_)
        | DispatchOutcome::Uncertain(_) => "rejected",
    };
    animus_adapters::mesh::project_receipt(&animus_adapters::mesh::ReceiptInput {
        provider_label: invocation.provider_label,
        model_label: state.route.model().model_id(),
        endpoint_identity: state.route.deployment().endpoint_identity(),
        config_identity: state.route.model().configuration_ref(),
        catalog_identity: state.screening.snapshot_identity(),
        request_identity: invocation.request_id,
        response_identity: response_identity.as_str(),
        authority_identity: state.permit.grant_ref(),
        api_style: animus_adapters::mesh::ApiStyle::Responses,
        status,
    })
    .ok()
}

fn refused(
    invocation: &Invocation<'_>,
    screening: Screening,
    route: Option<&EligibleRoute>,
    refusal: &(AdmissionRecord, NotDispatched),
) -> Result<InvocationReport, InvocationError> {
    let receipt = animus_adapters::serving::project_receipt(&ReceiptInput {
        evidence: invocation.evidence,
        request_id: invocation.request_id,
        caller: invocation.caller,
        screening: &screening,
        selected: route,
        admission: &refusal.0,
        permit: None,
        outcome: DispatchOutcome::NotDispatched(refusal.1),
        response: None,
        usage: Usage::UNKNOWN,
        provider_receipt_identity: None,
    })?;
    Ok(InvocationReport {
        screening,
        receipt,
        observation: None,
        provider_receipt: None,
    })
}
