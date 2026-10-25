//! Shared support of the campaign-manager integration tests: a real campaign over Choregraph
//! history, doubles for the model and the durable store, and steering helpers.

#![allow(
    dead_code,
    unused_imports,
    reason = "each integration-test binary uses a subset of the shared support"
)]

mod doubles;
mod history;
mod steering;

pub use doubles::{MemoryStore, ScriptedModel};
pub use history::{Fixture, admission_rule, frontier, id};
pub use steering::{advise_once, full_terms, handle, kind, no_menu, publish, selected, setup};

/// Rank dimensions of the tests' scores.
pub const RANK_DIMENSIONS: u32 = 2;

/// Why a test step failed.
#[derive(Debug)]
pub enum Failure {
    /// A Campaign, Choregraph history, or control-session step failed.
    Setup(campaign_choregraph::AdapterError),
    /// A manager turn was refused or its model failed.
    Turn(animus_campaign_manager::TurnFailure<&'static str>),
    /// A publication stopped after `committed` events.
    Publication {
        /// Events committed before the failure.
        committed: u64,
        /// Why publication stopped.
        cause: animus_campaign_manager::PublicationCause<&'static str>,
    },
    /// A step had another outcome than the test expects.
    Unexpected(&'static str),
}

impl From<campaign_choregraph::AdapterError> for Failure {
    fn from(error: campaign_choregraph::AdapterError) -> Self {
        Self::Setup(error)
    }
}

impl From<campaign_core::CampaignError> for Failure {
    fn from(error: campaign_core::CampaignError) -> Self {
        Self::Setup(campaign_choregraph::AdapterError::from(error))
    }
}

impl From<campaign_choregraph::HistoryError> for Failure {
    fn from(error: campaign_choregraph::HistoryError) -> Self {
        Self::Setup(campaign_choregraph::AdapterError::from(error))
    }
}

impl From<animus_campaign_manager::TurnFailure<&'static str>> for Failure {
    fn from(failure: animus_campaign_manager::TurnFailure<&'static str>) -> Self {
        Self::Turn(failure)
    }
}

impl From<Box<animus_campaign_manager::PublicationFailure<&'static str>>> for Failure {
    fn from(failure: Box<animus_campaign_manager::PublicationFailure<&'static str>>) -> Self {
        Self::Publication {
            committed: failure.committed,
            cause: failure.cause,
        }
    }
}
