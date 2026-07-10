# Goose Code Mode typed tool planning

Animus adapts Goose Code Mode as metadata discovery plus declarative tool-plan DTOs. The reusable crates do not evaluate generated JavaScript, Deno programs, plugin code, shell scripts, or host callbacks. A planner can only work with safe tool metadata, and every planned step must re-enter typed tool admission before an injected host adapter may run it.

## Functional core

`animus-adapters::tool` owns the reusable DTOs and pure validators:

- `ToolDiscoveryRequestDraft`, `ToolDiscoverySet`, `ToolDiscoverySummary`, and `ToolDiscoveryDetail` expose bounded metadata for already-loaded active tool definitions.
- `ToolPlanFabricDraft` and `admit_tool_plan_fabric` admit Steel or typed fabrics by kind, profile, policy hash, allowed host actions, budget label, and safe receipt posture.
- `ToolPlanDraft`, `ToolPlanStepDraft`, and `admit_tool_plan` validate ordered batch steps, cancellation posture, timeout posture, exact capability labels, and schema identities against the admitted discovery set and loaded definitions.
- `ToolPlanIntermediateBindingDraft` rejects binary, image, secret, oversized, or unsupported intermediates when a later step tries to bind them as text.
- `ToolPlanReceipt` projects plan decisions into safe receipt metadata with BLAKE3 plan and receipt identities.

The core is deterministic and accepts only in-memory definitions, requests, discovery results, fabric metadata, and plan drafts. It performs no filesystem access, plugin loading, process execution, network access, credential lookup, clock reads, model dispatch, or tool dispatch.

## Host shell boundary

Hosts may provide a Steel script, typed fabric, UI picker, model planner, or other planning service, but host shells must admit that producer before it can influence discovery or plan output. Admission is fail-closed for unsupported fabric kinds, missing profile or policy hashes, missing Steel script hashes, unsupported host actions, missing budgets, or unsafe receipt posture.

Tool execution remains host-owned. After a plan is admitted, the imperative shell still owns executor injection, cancellation, timeout observation, mutation-window serialization, approval prompts, and adapter dispatch. The typed plan never grants authority by itself.

## Non-claims

This contract does not adopt Goose's JavaScript Code Mode, Deno/pctx runtime, dynamic package loading, automatic MCP execution, browser sandboxing, product UI, ambient plugin discovery, live-provider behavior, or process-backed batch executor. It is a reusable DTO and validation boundary only.

## Focused verification

Run the focused rail with:

```sh
cargo test -p animus-adapters tool::tests --locked
scripts/cairn/check-tool-catalog --root .
nix run .#cairn -- validate --root .
```

The tool tests include positive progressive discovery, Steel-fabric admission, declarative batch planning, bounded intermediate binding, and safe receipt cases, plus negative arbitrary host-action, undiscovered tool, cancelled step, timed-out step, binary-as-text intermediate, and receipt-leak cases.
