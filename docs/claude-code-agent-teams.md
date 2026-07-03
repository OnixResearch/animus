# Claude Code agent-team reference boundary

Animus adapts Claude Code subagent and background-agent patterns as bounded worker-task contracts. The reusable runtime owns only typed delegation data, pure validation, injected service ports, and deterministic report merge logic.

## Scope

Animus supports these reusable concepts:

- scoped worker task drafts with objective, scope, context packet, allowed capabilities, budgets, output schema, parent trace metadata, and authority metadata;
- minimized context packets assembled from selected parent messages, summaries, file observations, memory descriptors, and bounded tool-result summaries;
- host-injected worker services for enqueue, status, cancel, and collect operations;
- evidence-bearing worker reports with safe fields, claims, evidence references, and unresolved mutation intents;
- pure coordinator merge logic that records structured conflicts and denied mutation intents before any effect can happen.

Animus does not adopt Claude Code cloud sessions, full parent transcript sharing, reusable-core process spawning, direct shell fallback, or permission bypass. Worker output is data until normal policy, capability, and mutation-coordination checks accept a later action.

## Runtime boundary

`animus-runtime` owns the worker-team contract surface:

- `WorkerTaskDraft` is untrusted input assembled by a host or coordinator.
- `validate_task` returns a `WorkerTask` only after required fields, parent policy retention, scoped capabilities, budgets, parent trace metadata, and minimized context pass validation.
- `WorkerService` is a caller-supplied port. `enqueue_task`, `request_status`, `request_cancel`, and `collect_report` fail closed with `WorkerServiceUnavailable` when no service is injected.
- `validate_context_packet` rejects full parent transcripts, raw hidden memory, credentials, provider payloads, unbounded tool output, and unsafe raw markers by default.
- `merge_reports` validates each report against the task output schema, preserves evidence references, returns structured conflicts for incompatible claims or mutation intents, and never applies mutations.

## Context minimization

Context packets are safe by default when they contain selected parent-message excerpts, summaries, file observations, memory descriptors, or bounded tool-result summaries. Unsafe context classes require explicit admission and still pass text leak validation. Durable evidence should cite labels, counts, summaries, and BLAKE3 hashes rather than raw prompts, provider payloads, credentials, full paths, or unbounded tool output.

## Non-bypass guarantee

Workers cannot grant themselves authority. `BypassParentPolicy`, wildcard capabilities, missing capability declarations, omitted output schemas, missing parent trace metadata, and `parent_policy_required = false` are rejected before enqueue. Mutation intents outside declared capabilities are recorded as denied data, not dispatched.

## Verification

Focused coverage lives in `crates/animus-runtime/src/worker_team/tests/`:

```sh
cargo test -p animus-runtime worker_team --locked
scripts/cairn/check-runtime-facade-inventory --root .
nix run .#cairn -- validate --root .
```

The tests include positive task/context/service/merge cases and negative missing-field, context-leak, missing-service, malformed-report, conflict, and policy-bypass cases.
