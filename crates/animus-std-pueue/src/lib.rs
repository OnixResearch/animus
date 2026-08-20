//! Non-default capability-scoped pueue host.
//!
//! The crate owns explicit pueue process calls, daemon lifecycle observations,
//! bounded backend parsing, and tool projection. It implements the
//! [`animus_adapters::BackgroundQueueService`] contract. Animus queue contracts and
//! process-job profile resolution retain deterministic policy authority.

#![cfg_attr(animus_octet, feature(register_tool))]
#![cfg_attr(animus_octet, register_tool(tigerstyle))]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

#[cfg_attr(
    animus_octet,
    allow(
        tigerstyle::assertion_density,
        tigerstyle::non_trait_imports,
        tigerstyle::path_segment_repetition,
        reason = "fallible command planning keeps one reviewed pueue argv cohort and explicit queue DTO names"
    )
)]
mod command;
#[cfg_attr(
    animus_octet,
    allow(
        tigerstyle::non_trait_imports,
        tigerstyle::path_segment_repetition,
        reason = "typed configuration imports and public pueue names keep authority ownership explicit"
    )
)]
mod config;
#[cfg_attr(
    animus_octet,
    allow(
        tigerstyle::non_trait_imports,
        tigerstyle::path_segment_repetition,
        reason = "one host module keeps bounded orchestration and explicit process ownership at the queue port shell"
    )
)]
mod host;
#[cfg_attr(
    animus_octet,
    allow(
        tigerstyle::non_trait_imports,
        tigerstyle::path_segment_repetition,
        reason = "pueue process DTO names distinguish the injected mechanism from queue domain outcomes"
    )
)]
mod process;
#[cfg_attr(
    animus_octet,
    allow(
        tigerstyle::assertion_density,
        tigerstyle::non_trait_imports,
        tigerstyle::path_segment_repetition,
        reason = "one reviewed output cohort keeps fallible parsing, normalization, bounds, and redaction adjacent"
    )
)]
mod projection;
#[cfg_attr(
    animus_octet,
    allow(
        tigerstyle::assertion_density,
        tigerstyle::non_trait_imports,
        tigerstyle::path_segment_repetition,
        reason = "one pueue tool cohort keeps fallible catalog, exact decoding, execution, and safe rendering together"
    )
)]
mod tool;

pub use config::{
    PueueHostConfig, PueueHostConfigError, PueueHostConfigIssue, PueueHostLimits,
    parse_pueue_host_config,
};
pub use host::PueueHost;
pub use process::{
    BoundedExecPueueProcessService, PueueProcessDisposition, PueueProcessError, PueueProcessOutput,
    PueueProcessRequest, PueueProcessService,
};
pub use projection::PueueProjectionError;
pub use tool::{
    PUEUE_TOOL_CAPABILITY, PUEUE_TOOL_EXECUTOR_BINDING, PUEUE_TOOL_NAME,
    PUEUE_TOOL_SCHEMA_IDENTITY, PueueToolError, PueueToolExecutor, PueueToolRequestDraft,
    build_pueue_tool_catalog, build_pueue_tool_request, execute_pueue_tool,
};
