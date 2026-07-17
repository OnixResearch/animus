## Phase 1: Feature and pure launch contract

- [x] [serial] Verify upstream license/source provenance, add the non-default Linux-only `sandlock-host` feature, pin the reviewed cohort, and prove default/non-Linux dependency isolation. r[agent_harness_runtime.sandlock_host_adapter.feature_gate]
- [x] [serial] Define owned authorization binding, safe root binding, capability probe, launch plan, execution fact, and receipt input types without exposing Sandlock or kernel types through reusable crates. r[agent_harness_runtime.sandlock_host_adapter.translation] r[agent_harness_runtime.sandlock_host_adapter.authority]
- [x] [serial] Implement pure descriptor-to-launch translation, authorization identity matching, required-capability derivation, preflight decisions, and safe receipt projection. r[agent_harness_runtime.sandlock_host_adapter.translation] r[agent_harness_runtime.sandlock_host_adapter.preflight] r[agent_harness_runtime.sandlock_host_adapter.receipts]

## Phase 2: Thin Linux shell

- [x] [serial] Implement capability-root resolution and exact requested-mechanism probes for the pinned Sandlock cohort. r[agent_harness_runtime.sandlock_host_adapter.preflight]
- [x] [serial] Implement the `sandlock-core` process shell with bounded input/output handling, cancellation/timeout propagation, COW disposition, cleanup, and no partial-enforcement fallback. r[agent_harness_runtime.sandlock_host_adapter.execution]
- [x] [serial] Wire adapter dispatch only after the existing Animus/Basalt authorization gate and keep denial/enforcement outcomes distinct. r[agent_harness_runtime.sandlock_host_adapter.authority] r[agent_harness_runtime.sandlock_host_adapter.execution]

## Phase 3: Positive and negative evidence

- [x] [parallel] Add pure positive tests for complete authorization binding, root translation, full capability support, deterministic planning, successful outcome classification, and redacted receipt identity. r[agent_harness_runtime.sandlock_host_adapter.verification]
- [x] [parallel] Add negative tests for a disabled feature, non-Linux selection, stale authorization, root escape, missing mechanisms, partial enforcement, launch failure, timeout, cleanup failure, and receipt leak attempts. r[agent_harness_runtime.sandlock_host_adapter.verification]
- [x] [serial] Add an opt-in Linux behavior rail covering filesystem denial, network denial, dry-run/COW discard, bounded resources, clean environment, and unsupported-capability blocking. r[agent_harness_runtime.sandlock_host_adapter.preflight] r[agent_harness_runtime.sandlock_host_adapter.verification]

## Phase 4: Documentation and closeout

- [x] [parallel] Update the sandbox documentation, feature matrix, supported Sandlock cohort, operator prerequisites, upgrade process, and authority/non-claim boundaries. r[agent_harness_runtime.sandlock_host_adapter.feature_gate] r[agent_harness_runtime.sandlock_host_adapter.receipts]
- [x] [serial] Run focused `animus-std` and `animus-adapters` tests, default/no-default feature checks, Linux behavior smoke when available, formatting, clippy, and receipt-safety checks. r[agent_harness_runtime.sandlock_host_adapter.verification]
- [x] [serial] Run Cairn validation and proposal/design/tasks gates; sync and archive only with recorded positive and negative evidence and explicit blockers for unavailable kernel rails. r[agent_harness_runtime.sandlock_host_adapter.verification]
