//! Explicit local supervisor and worker process shell.
//!
//! The pure core owns lifecycle and routing decisions. The shell owns Unix
//! sockets, processes, clocks, and capability-relative registry publication.

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

mod component_scope;
mod config;
mod core;
mod process;
mod registry;
mod service;
mod transport;
mod worker;

pub use component_scope::{
    ClientDeliveryScopeLease, ComponentScopeCancellationPort, ComponentScopeDeliveryPort,
    ComponentScopeHost, ComponentScopeHostBounds, ComponentScopeHostError,
    ComponentScopeHostOutcome, ComponentScopePorts, ComponentScopeQuiescencePort,
    ComponentScopeReceiptPort, ComponentScopeReleasePort, ScopeBoundPortRequest,
    ScopeMemberPortRequest, ScopePortStatus, ScopeReceiptPortRequest, WorkerRootScopePlan,
    plan_client_delivery_scope_lease, plan_worker_root_scope,
};
pub use config::{LocalDaemonConfigError, ValidatedLocalDaemonConfig, parse_local_daemon_config};
pub use core::{
    Attachment, HealthFacts, LocalDaemonError, LocalDaemonReceipt, LocalDaemonReceiptCounts,
    RecoveryDecision, RecoveryKind, RestoreStatus, RoutingDecision, RoutingRequest, StopDecision,
    StopFacts, WorkerLifecycle, WorkerRegistry, WorkerRegistryRecord, admit_ready, decide_recovery,
    decide_routing, decide_stop, next_generation,
};
pub use process::{
    ProcessIdentity, ProcessObservation, ProcessTreeLauncher, SpawnedWorker, WorkerLaunchRequest,
    WorkerProcessError,
};
pub use registry::{RegistryError, RegistryStore};
pub use service::{
    EventBatchRequest, LocalSupervisorService, ServiceError, StateInspectionIdentityFacts,
    WorkerCommand, WorkerEvent, WorkerSessionPort, WorkerSnapshot, WorkerStateInspection,
};
pub use transport::{
    LocalClient, LocalEndpoint, LocalListener, LocalPeerFacts, TransportError, TransportFrame,
};
pub use worker::{
    LiveWorkerError, LiveWorkerOutput, LiveWorkerReceipt, LiveWorkerRequest, run_live_worker,
};
