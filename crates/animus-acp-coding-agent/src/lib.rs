//! Optional ACP coding agent for Animus.
//!
//! This crate admits a coding session's tool requests before any effect: file
//! edits confined to one workspace root, and command execution limited to checks
//! the task declares. The modules here are pure: they take already-read values
//! and return decisions, with no filesystem, process, network, clock or
//! credential authority. The shell that opens the workspace root, runs checks
//! and serves ACP stdio is added by later tasks of `add-acp-coding-agent`.

pub mod admission;
pub mod checks;

// r[impl agent_harness_runtime.acp_coding_agent.boundary]

#[cfg(test)]
mod tests;
