# Tool execution stages

Animus provides one fixed, typed stage contract around the existing tool executor kit. The stage core is pure. Host code selects and injects handlers before execution.

## Design reference

The bounded design reference is DeepSeek Harness revision `47f943859bef60e4160492346772ded9b24f765a`. The source is MIT-licensed and marked as a developer preview.

Animus selects staged interception semantics only. It does not adopt ambient contexts, hidden injection, provider-native storage, reorderable middleware, or plugin-owned authority.

## Baseline evidence

Before core changes, the tool-executor-kit, tool-catalog, coding-session, lifecycle-hook, authority-seal, composition-conformance, and evidence-safety rails passed. The post-change focused rail repeats the affected package and owner checks.

## Fixed order

Every plan uses this order:

```text
pre_admission -> final_admission -> pre_execution -> execution -> output_narrowing -> final_validation -> durable_commit -> post_commit_observation
```

Profiles cannot add, remove, or reorder a phase. Optional handlers can run only during `pre_admission`, `pre_execution`, `output_narrowing`, and `post_commit_observation`.

The fixed barriers keep their existing owners:

- `plan_tool_execution` owns final admission.
- `invoke_admitted_tool` owns the injected executor boundary.
- `validate_tool_executor_output` owns final validation.
- The coding-session host owns exact durable result commit.
- Product-owned observers can inspect metadata only after that commit.

A handler cannot replace one of these owners. A handler list is an ordered, bounded selection with exact BLAKE3 handler and cohort identities.

## Pure core

`animus_runtime::tool_execution_stages` owns deterministic plans, transitions, identities, and receipts. The plan binds these facts:

- phase order and selected handlers;
- tool request and schema identities;
- executor binding;
- runtime-authority seal identity;
- capability labels;
- mutation targets and posture;
- output limits and redaction posture;
- cancellation and deadline posture;
- durable commit-owner identity.

Every handler candidate must re-enter Rust validation. Validation rejects stale identities, changed handler sets, capability growth, mutation growth, bound growth, redaction removal, phase reordering, and outcome widening.

Denial is sticky. Failure, cancellation, and timeout cannot become success. Later output handlers can only redact, truncate, classify, or replace output with a safe narrower value.

The core has no filesystem, network, provider, clock, credential, executor, or persistence authority. It supports `wasm32-unknown-unknown` without default features.

## Host shell

The `animus-std` feature `tool-execution-stages` exports `animus_std::tool_execution_stage_host`. It defines phase-specific handler traits and two operations:

1. `execute_staged_tool` runs through final validation and returns `PendingStagedToolExecution`.
2. `complete_staged_tool_commit` accepts the exact durable commit observation, runs metadata-only observers, and projects the terminal receipt.

`NoToolExecutionStageHandlers` preserves the previous executor-kit behavior. It does not weaken any fixed barrier.

No provider continuation can observe a tool result before that result has an exact durable commit. Observer failure also blocks provider continuation.

## Coding-session ownership

`CodingSessionCommitRequest::tool_response_after_commit` carries the pending stage plan and state. The coding-session reducer validates the synchronized durable session commit before it returns `CodingSessionToolCommitObservation`.

`CodingSessionHost` sends that observation to `CodingSessionToolService::observe_committed`. This default-compatible hook gives the tool shell exact commit metadata. It does not give the observer result mutation authority.

## Live Steel route

The live Steel tool adapter builds an exact executor-kit catalog, capability set, runtime-authority seal, and commit-owner identity. It uses zero optional handlers by default.

The Steel shell retains the private pending stage and its safe-point journal. It persists the safe point explicitly, then waits for the coding-session result commit. `observe_committed` completes the staged transition before the shell exposes the tool result to provider continuation.

Raw arguments, output, handler errors, credentials, provider payloads, and private paths stay in the private shell. They do not enter public stage evidence.

## RLM route

`ToolKitRoute` accepts exact authority-seal and commit-owner identities. A product-owned `ToolRouteCommitService` performs the durable commit. The generic RLM host cannot claim durable completion by itself.

The route returns output only after commit validation and terminal receipt projection. Missing or mismatched commit evidence fails closed.

## Rejected alternatives

Animus does not use reorderable plugin middleware for tool authority. Such middleware could move admission, validation, or commit barriers.

Animus also rejects ambient handler discovery, fallback executors, plugin-owned commit authority, mutable shared handler state in the core, and public receipts with raw values.

The product composition root selects exact handlers and services. Missing required selections fail closed.

## Evidence boundary

`ToolExecutionStageReceipt` contains safe labels, BLAKE3 identities, status, outcome class, handler count, redaction and truncation facts, and bounded non-claims. It does not contain raw tool values.

The receipt does not prove tool correctness, rollback, handler honesty, sandboxing, task success, provider correctness, production readiness, or release eligibility.

## Verification

Run the focused rail:

```sh
scripts/cairn/check-tool-execution-stages --root .
```

Run the sandbox-safe Nix subset:

```sh
nix build path:$PWD#checks.x86_64-linux.tool-execution-stages --no-link -L --option builders '' --option max-jobs 1
```

These checks prove only the named source, test, lint, and catalog observations. They do not prove external effects or production safety.
