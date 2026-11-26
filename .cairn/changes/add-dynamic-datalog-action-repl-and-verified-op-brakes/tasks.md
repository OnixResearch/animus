# Tasks

## Phase 1 — Session Dynamic Datalog Action Contract
- [ ] Add pure `no_std` bounded session Datalog action & `Certain`/`Shadow` + `Active`/`Dead`/`Absent` delta contract in `crates/animus-adapter-rlm-kernel/src/lib.rs` (or sibling adapter crate).

## Phase 2 — Framing & Verified Operation Brakes
- [ ] Add `UntrustedToolOutput` framing and Protocolis post-action observed-diff verification in `crates/animus-acp-stdio-host/src/permission.rs` and `crates/animus-std-agent-guardrail`.
- [ ] Add regression tests covering relation reuse, non-promotion of untrusted tool output, and post-action divergence refusal; pass workspace clippy & tests for touched crates.
