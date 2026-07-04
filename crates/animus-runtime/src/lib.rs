//! Reusable agent-loop runtime for `animus`.
//!
//! This crate owns typed orchestration over loop-fabric decisions and injected
//! model/tool adapters. It does not construct concrete providers, tools,
//! filesystem handles, processes, network clients, terminal UI, persistence, or
//! Syndicate dataspaces. The deterministic transition core is separated from the
//! thin imperative shell that calls fabrics and adapters.

#![cfg_attr(animus_octet, feature(register_tool))]
#![cfg_attr(animus_octet, register_tool(tigerstyle))]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_name_repetitions)]
#![allow(unknown_lints)]
#![allow(module_file_count)]
// Octet-only suppression keeps archived compatibility facades reviewable until
// their public APIs are split by owner without requiring nightly-only lint names
// in normal stable cargo builds.
#![cfg_attr(
    animus_octet,
    allow(
        tigerstyle::excessive_file_length,
        tigerstyle::platform_dependent_cast,
        tigerstyle::unbounded_loop,
        tigerstyle::unjustified_allow
    )
)]
#![forbid(unsafe_code)]

mod background;
#[allow(excessive_file_length)]
mod batch;
mod boundary;
mod context_layering;
mod contract;
#[allow(excessive_file_length)]
#[path = "continuation/mod.rs"]
mod controller;
mod dataset_export;
mod eval_feedback;
mod exports;
mod fabric;
mod fresh_context_tasks;
mod headless;
mod hooks;
#[allow(excessive_file_length)]
#[path = "kanban.rs"]
mod orchestration;
mod pi_compaction;
mod provenance;
mod provider_hardening;
mod quality_records;
mod researchspec;
mod scaffold;
mod shell;
mod status;
mod telemetry;
mod transition;
mod trust_boundary;
mod workcmd;
mod worker_team;
mod zerostack_midturn;

pub use exports::*;
