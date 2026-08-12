# Verification: Tau interrupted tool-call repair adaptation

## Baseline

Before implementation, the proposal, design, and tasks gates passed for `adapt-tau-interrupted-tool-call-repair`. Focused baselines also passed for Pi compaction (6 tests), session persistence (6 tests), canonical model DTOs (9 tests), safe-receipt artifacts, provenance, and composition conformance. The baseline established that active-branch repair was absent while the surrounding ledger, compaction, provider-neutral DTO, persistence, and receipt contracts were healthy.

## Implemented boundary

The implementation lives in `crates/animus-runtime/src/interrupted_tool_repair/` and is exposed as `animus_runtime::interrupted_tool_repair` through the lint-neutral `session_recovery` implementation module. The reusable core:

- validates one already-loaded active branch with bounded entry, call, observation, and application-evidence collections;
- binds results to exact run identity, call ID, tool-definition BLAKE3 identity, transcript order, and current active leaf;
- distinguishes clean, repairable, and fail-closed malformed histories without inferring cancellation from absence;
- reduces only exact run-and-call observations to user cancellation, host cancellation, or unclean shutdown, otherwise using `unknown_interruption`;
- builds deterministic transcript-ordered tail or historical append-only plans with BLAKE3 snapshot, action, proposed-leaf, and plan identities;
- returns one host application effect, `AppendSyntheticFailures`, bound to the exact session, branch, old/new leaves, anchor, actions, and preserved suffix;
- validates durable complete application, clean replay, exact action coverage, current leaf, idempotent prior-plan evidence, and explicit executor non-invocation before provider continuation;
- projects only canonical provider-neutral synthetic tool-result messages after the gate passes; and
- projects metadata-only receipts while rejecting raw prompts, tool arguments/results, provider payloads, credentials, private paths, exception bodies, and transcripts.

Session discovery, storage, branch mutation, locks, clocks, provider dispatch, cancellation observation, and every tool executor remain host-owned. The repair API cannot construct a tool request, approval, retry, fallback, or mutation permit.

## Focused and integration evidence

The current implementation has passed:

- `cargo test -p animus-runtime --locked session_recovery --no-fail-fast` — 16 positive and negative recovery tests;
- `cargo clippy -p animus-runtime --all-targets --locked -- -D warnings`;
- `scripts/hooks/octet-lints` — Tiger Style/Octet passed after structural fixes for bounded growth, explicit owner paths, typed parameter groups, predicate naming, function size, and assertion density;
- `nix develop -c scripts/cairn/check-interrupted-tool-repair --root .` — focused runtime, compaction, persistence, composition, canonical-model, adapter-index, authority-boundary, and receipt rail;
- `nix develop -c scripts/cairn/check-adapter-conformance-index --root .` — 16 families and 208 required cases, including the interrupted-tool-repair family;
- `cargo test -p animus-runtime --locked pi_compaction --no-fail-fast` — 6 tests;
- `cargo test -p animus-runtime --locked session_persistence --no-fail-fast` — 6 tests;
- `cargo test -p animus-adapters --locked canonical --no-fail-fast` — 9 tests;
- `nix develop -c scripts/cairn/check-safe-receipts-artifacts --root .`;
- `nix develop -c scripts/cairn/check-provenance-ledger --root .` — 29 tests;
- `nix develop -c scripts/cairn/check-composition-conformance --root .` — 6 tests;
- component-catalog, runtime-facade-inventory, and request-lifecycle focused rails;
- `cargo test --workspace --locked --no-fail-fast`;
- `cargo check --workspace --all-targets --locked`;
- `cargo clippy --workspace --all-targets --locked -- -D warnings`;
- `nix build .#checks.x86_64-linux.interrupted-tool-repair -L`;
- `nix flake check -L` — all 52 compatible-system checks passed; and
- `pre-commit run --all-files` — Nickel generated configs, router policy conformance, and Octet all passed.

The configured Octet remote builder at `ssh-ng://root@10.10.10.1` was unavailable; Nix fell back to a successful local build. Earlier pre-commit invocations were killed without diagnostics and were treated as inconclusive. The isolated Octet run produced actionable findings, those findings were fixed structurally, and the complete post-fix pre-commit invocation passed.

After sync, repository-wide `cairn tracey coverage` remained globally invalid with 2,162 accepted requirements and 644 referenced markers because the existing report contains unrelated dangling and uncovered lifecycle IDs. None of the `agent_harness_runtime.tau_tool_repair.*` IDs appeared in the dangling or missing diagnostics, and the accepted delta has both implementation and verification markers. This change therefore claims focused marker coverage, not global Tracey closure.

## Bounded mechanism portfolio and adversarial audit

The design audit compared five mechanism families:

1. append exact synthetic terminal failures at an active tail;
2. fork an append-only repaired branch and replay an exact historical suffix;
3. hide gaps through compaction or summary replacement;
4. automatically retry interrupted tools; and
5. inject provider-specific continuation markers.

The implementation admits the first mechanism and uses the second only when later non-result context must be preserved. Summary replacement was rejected because it erases exact call/result structure. Automatic retry was rejected because it can duplicate external effects. Provider-specific markers were rejected because they move transcript correctness into provider adapters and weaken the provider-neutral boundary.

The adversarial pass covered duplicate and reversed bindings, wrong run IDs, wrong tool identities, partial batches, multiple missing calls, stale leaves and diagnoses, missing anchors, unsupported entries, false or conflicting cancellation evidence, partial/store-failed/replay-invalid application, prior-plan idempotence, executor invocation, oversized evidence collections, unsafe receipt material, and provider projection before repair. A local VibeThinker consultation proposed nonexistent functions, ambient signature/timestamp checks, and tolerance-based identity matching; those suggestions were rejected as inapplicable or authority-expanding. Repository tests and rails, not that advisory response, are the accepted evidence.

## Non-claims

The evidence does not prove why an interruption occurred, that an interrupted external effect did not happen, exactly-once tool effects, host-store correctness, host observation honesty, provider acceptance, arbitrary provider transcript compatibility, filesystem recovery, cross-process locking, live credentials, or production readiness. A passing receipt establishes only the bounded metadata and gate facts supplied to this pure core.

## Lifecycle closeout

Cairn sync dry-run completed without blockers, and executed sync merged all seven `agent_harness_runtime.tau_tool_repair.*` requirements into `cairn/specs/agent-harness-runtime/spec.md`. The accepted spec was inspected directly for the synced IDs. Archive dry-run completed without blockers, and archive execution used `CAIRN_ARCHIVE_DATE=2026-07-12`, producing this archive with mutation receipt `0aed3c80bf29768e069df67a763468c305ed9a39e6a80c88d4f11674d93e5760`.
