//! Deterministic serving-owner fixture for tests and fixture-labelled evidence.
//!
//! It holds a fixed number of request slots, grants for its current
//! deployment, and records every query. It is not a serving owner and its
//! results are never live evidence.

use animus_adapters::serving::{AdmissionQuery, Grant, GrantDraft, OwnerRejection, OwnerReply};

use super::{CurrentRecord, CurrentReply, OwnerPort};

const MAX_RECORDED_CALLS: usize = 64;

/// One recorded owner call.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OwnerCall {
    /// An admission query for this request.
    Admit(String),
    /// A recheck for this request.
    Recheck(String),
}

/// Deterministic owner with scripted replies.
#[derive(Clone, Debug)]
pub struct FixtureOwner {
    /// Remaining request slots.
    pub free_slots: u32,
    /// State reported by rechecks; grants name its deployment and generation.
    pub current: CurrentRecord,
    /// Grant `(issued_tick, valid_until_tick)`.
    pub grant_window: (u64, u64),
    /// Replica the owner places requests on, if exposed.
    pub replica: Option<String>,
    /// Scripted admission reply replacing the slot ledger.
    pub admit_override: Option<OwnerReply>,
    /// Scripted recheck reply.
    pub recheck_override: Option<CurrentReply>,
    /// State that replaces `current` after an admission, modelling a change
    /// between admission and recheck.
    pub after_admission: Option<CurrentRecord>,
    calls: Vec<OwnerCall>,
    issued: u32,
}

impl FixtureOwner {
    /// Creates an owner that reports `current` and holds `free_slots`.
    #[must_use]
    pub const fn new(current: CurrentRecord, free_slots: u32, grant_window: (u64, u64)) -> Self {
        Self {
            free_slots,
            current,
            grant_window,
            replica: None,
            admit_override: None,
            recheck_override: None,
            after_admission: None,
            calls: Vec::new(),
            issued: 0,
        }
    }

    /// Recorded calls, oldest first; at most 64 are kept.
    #[must_use]
    pub fn calls(&self) -> &[OwnerCall] {
        &self.calls
    }

    fn record(&mut self, call: OwnerCall) {
        if self.calls.len() < MAX_RECORDED_CALLS {
            self.calls.push(call);
        }
    }

    fn grant(&mut self, query: &AdmissionQuery) -> OwnerReply {
        let Some(remaining) = self.free_slots.checked_sub(1) else {
            return OwnerReply::Rejected(OwnerRejection::Saturated);
        };
        self.issued = self.issued.saturating_add(1);
        let grant_ref = format!("fixture-grant-{}", self.issued);
        let current = &self.current;
        let grant = Grant::new(&GrantDraft {
            grant_ref: &grant_ref,
            request_id: query.request_id(),
            service_id: &current.service_id,
            deployment_id: &current.deployment_id,
            deployment_generation: current.deployment_generation,
            model_id: &current.model_id,
            configuration_ref: &current.configuration_ref,
            admission_epoch: current.admission_epoch,
            issued_tick: self.grant_window.0,
            valid_until_tick: self.grant_window.1,
            replica: self.replica.as_deref(),
            policy_generation: current.policy_generation,
        });
        match grant {
            Ok(grant) => {
                self.free_slots = remaining;
                OwnerReply::granted(grant)
            }
            Err(_) => OwnerReply::PortIssue,
        }
    }
}

impl OwnerPort for FixtureOwner {
    fn admit(&mut self, query: &AdmissionQuery) -> OwnerReply {
        self.record(OwnerCall::Admit(query.request_id().to_owned()));
        let reply = match self.admit_override.clone() {
            Some(reply) => reply,
            None => self.grant(query),
        };
        if let Some(next) = self.after_admission.take() {
            self.current = next;
        }
        reply
    }

    fn recheck(&mut self, query: &AdmissionQuery) -> CurrentReply {
        self.record(OwnerCall::Recheck(query.request_id().to_owned()));
        self.recheck_override
            .clone()
            .unwrap_or_else(|| CurrentReply::Observed(self.current.clone()))
    }
}
