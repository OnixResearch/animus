//! Non-default capability-relative workspace edit host.
//!
//! The crate keeps snapshot, visibility, byte-application, freshness,
//! publication, and receipt decisions pure. The filesystem shell accepts one
//! caller-opened capability root. It grants no ambient root authority.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod concrete;
mod evidence;
mod model;
mod planning;

pub use crate::concrete::{
    HostCheckpoint, HostError, HostObservation, HostRequest, HostServiceIssue, HostServices,
    StageNameService, WorkspaceEditHost, WriterLease, WriterRelease, logical_resource_identity,
};
pub use crate::evidence::{
    EditReceipt, PrivateMaterialClass, ReceiptDraft, ReceiptIssue, ReceiptLeakFacts,
    UnsupportedClaimClass, project_receipt,
};
pub use crate::model::{
    AdmittedSnapshot, BoundarySide, DurabilityState, EditOperationDraft, EditPlan,
    FinalNewlineState, FreshnessObservation, GateDecision, GateObservations, HostIssue,
    LifecycleGate, LineEndingMode, MAX_EDIT_OPERATIONS, MAX_REPLACEMENT_BYTES, MAX_SOURCE_BYTES,
    MAX_VISIBILITY_ITEMS, PublicationOutcome, PublicationState, ReplacementEnding, SnapshotDraft,
    StaleField, VisibleBoundaryDraft, VisibleSpanDraft,
};
pub use crate::planning::{
    admit_snapshot, plan_edit, reduce_publication, validate_freshness, validate_host_gates,
};
