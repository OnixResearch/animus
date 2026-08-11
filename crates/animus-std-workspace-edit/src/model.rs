mod admission;
mod gates;
pub(crate) mod mutation;
mod result;
pub(crate) mod text;

pub use admission::AdmittedSnapshot;
pub use gates::{
    FreshnessObservation, GateDecision, GateObservations, HostIssue, LifecycleGate, StaleField,
};
pub use mutation::{EditOperationDraft, EditPlan};
pub use result::{DurabilityState, PublicationOutcome, PublicationState};
pub use text::{
    BoundarySide, FinalNewlineState, LineEndingMode, MAX_EDIT_OPERATIONS, MAX_REPLACEMENT_BYTES,
    MAX_SOURCE_BYTES, MAX_VISIBILITY_ITEMS, ReplacementEnding, SnapshotDraft, VisibleBoundaryDraft,
    VisibleSpanDraft,
};
