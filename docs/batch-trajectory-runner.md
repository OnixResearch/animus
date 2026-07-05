# Batch trajectory runner contract

Animus adapts the Clankers batch trajectory pattern as reusable runtime contracts, not as a product CLI or daemon. The batch surface is pure by default: planning, resume decisions, output ordering, and receipt validation operate over typed in-memory DTOs. Host shells own file reads/writes, provider/session execution, and renderer persistence.

## Contract map

- `r[agent_harness_runtime.clankers_batch_runner.planning]`: `BatchRequestDraft` validates into `BatchPlan` with deterministic run identity, ordered jobs, output format, concurrency policy, execution mode, redaction policy, and safe output labels.
- `r[agent_harness_runtime.clankers_batch_runner.planning.remote_rejected]`: batch planning rejects `http://`, `https://`, `s3://`, generic URI-scheme, and UNC-style remote input/output locations before execution or rendering.
- `r[agent_harness_runtime.clankers_batch_runner.execution]`: `BatchExecutionService` is an injected host port; reusable runtime code never constructs providers, daemons, filesystems, or shells.
- `r[agent_harness_runtime.clankers_batch_runner.execution.unavailable]`: `execute_batch_plan` fails closed with `ExecutionServiceUnavailable` when no service is injected.
- `r[agent_harness_runtime.clankers_batch_runner.resume]`: `evaluate_resume_manifest` skips only jobs whose run ID, plan hash, output format, redaction policy, completed status, and expected BLAKE3 output evidence match the current plan.
- `r[agent_harness_runtime.clankers_batch_runner.output]`: `render_plan_order_records` normalizes host outcomes into deterministic plan order even if completion order differs.
- `r[agent_harness_runtime.clankers_batch_runner.metadata]`: `BatchReceipt` records safe counts, statuses, prompt byte counts, handles, objective summaries, output labels, redaction markers, and BLAKE3 hashes without raw prompts, provider payloads, credentials, full private paths, remote destinations, or unbounded output.

## Non-claims

This contract does not add a batch CLI, cloud batch API, provider-specific eval runner, daemon/session persistence, filesystem traversal, dataset exporter, performance guarantee, or production-ready scheduler. Those remain host-shell or future-change scope.

## Focused verification

Use `cargo test -p animus-runtime --offline` for the focused Rust fixtures. The batch tests include positive local planning, injected execution, matching resume, stable output ordering, and safe receipt construction, plus negative remote-location, missing-service, stale-manifest, missing/unknown outcome, and metadata leak fixtures.
