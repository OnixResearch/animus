## Why

Animus already owns portable Sandlock-inspired sandbox descriptors, pure normalization, policy checks, platform-capability matching, and redacted receipts. It does not yet have a host implementation that turns an admitted descriptor into real Linux confinement, so the current contract cannot protect an actual untrusted agent process.

A first integration should use the reviewed Sandlock Rust core behind an opt-in `animus-std` feature, preserve Basalt and Animus authorization as the authority boundary, and fail closed whenever the requested Linux mechanisms cannot be proven available. It must not make Sandlock, Linux, or a specific kernel ABI a default runtime dependency.

## What Changes

- Add a non-default Linux-only Sandlock host-adapter feature pinned to a reviewed upstream compatibility cohort. r[agent_harness_runtime.sandlock_host_adapter.feature_gate]
- Translate normalized Animus sandbox intent and host capability-root grants into a deterministic Sandlock launch plan without exposing kernel or host-path details through reusable DTOs. r[agent_harness_runtime.sandlock_host_adapter.translation]
- Probe every requested Landlock, seccomp, supervisor, COW, resource, network, HTTP, environment, and determinism capability before process launch and reject partial enforcement. r[agent_harness_runtime.sandlock_host_adapter.preflight]
- Require an existing Animus/Basalt authorization result before sandbox preparation, while keeping sandbox success unable to create or widen authority. r[agent_harness_runtime.sandlock_host_adapter.authority]
- Execute through the optional host shell, map bounded outcomes back into existing Animus receipts, and keep raw paths, output, payloads, and credentials out of durable evidence. r[agent_harness_runtime.sandlock_host_adapter.execution] r[agent_harness_runtime.sandlock_host_adapter.receipts]
- Add positive, negative, fake-host, and opt-in Linux behavior tests. r[agent_harness_runtime.sandlock_host_adapter.verification]

## Impact

- **Rust**: `animus-std` gains an optional Sandlock host module and feature; portable descriptors remain in `animus-adapters`.
- **Policy**: Nickel-authored sandbox policy and Basalt authorization remain prerequisites rather than outputs of the adapter.
- **Platforms**: default builds and non-Linux hosts remain unaffected; selecting the adapter on an unsupported host fails before execution.
- **Dependencies**: the reviewed Sandlock source is pinned and optional, not vendored or transitively enabled by default.
- **Claims**: a passing receipt proves only that one process was launched under the recorded requested/enforced cohort. It does not prove kernel correctness, complete containment, authorization, build hermeticity, or cross-platform parity.
