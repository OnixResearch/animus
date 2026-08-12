# Verification: Tau provider retry execution adaptation

## Baseline

Before implementation, the proposal, design, and tasks gates passed for `adapt-tau-provider-retry-execution`. Focused baselines also passed for provider dispatch/profile/receipt/composition contracts and established that provider streaming and routing explicitly did not own retry policy, backoff, or repeated dispatch.

## Implemented boundary

The implementation uses `config/provider-retry-policy.ncl` as the typed authoring source, exports `config/generated/provider-retry-policy.json`, revalidates it through `animus-std`, and exposes the pure runtime contract as `animus_runtime::provider_retry` through the lint-neutral `resilience` source module. The implementation:

- validates attempt, delay, cancellation, semantic-stream, canonical-class, provider-family mapping, uniqueness, and lowercase BLAKE3 policy invariants without runtime Nickel evaluation;
- recomputes the canonical policy identity from separator-safe ordered fields and length-unambiguous mapping parts;
- accepts only safe versioned provider-family/native-class observations while raw HTTP, SDK, body, header, endpoint, credential, and exception material remains host-private;
- defaults unknown and permanent classes to terminal and admits only explicitly configured transient classes;
- binds provider, model, canonical request, profile plan, prompt, tool catalog, response format, reasoning intent, stream intent, credential-handle class, and policy identities into one exact invocation hash;
- requires fresh composition for every changed invocation dimension and never selects router fallback;
- tracks canonical stream frames and denies ordinary retry after accepted content, reasoning, tool-call, structured-output, or other semantic frames;
- computes a bounded deterministic schedule without clocks, sleeps, polling, entropy, environment, network, or provider authority;
- creates an injected timer request plus exact threaded wait state bound to run, attempt, plan, and decision identities;
- closes completed, cancelled, timed-out, unavailable, and failed waits so later observations cannot reopen a dispatch gate;
- advances threaded attempt state only to the immediate successor and rejects applying the same dispatch request to already-advanced state;
- uses the existing injected `ComponentAdapter<ModelRequest, ModelResponse>` port after pure gate, plan, and retained-request hash checks instead of defining a second provider service;
- projects provider-neutral progress into `RuntimeEvent`, headless, validated Preserves round trips, and an explicit non-transcript TUI path without creating assistant transcript content; and
- projects metadata-only receipts while rejecting raw requests, provider bodies/headers, endpoints, credentials, exception bodies, prompts, hidden reasoning, tool bodies, and private paths.

The reusable runtime owns no file, process, environment, clock, sleep, timer, network, provider discovery, credential, router, storage, transcript, or completion authority. `animus-std` reads only an explicitly supplied generated-policy path and invokes only caller-supplied timer/model adapter implementations.

## Focused evidence

The current implementation has passed:

- `cargo test -p animus-runtime --locked resilience --no-fail-fast` — 17 positive and negative retry tests;
- `cargo test -p animus-std --features provider-retry --locked providerretry --no-fail-fast` — 8 generated-loader and injected-shell tests;
- `cargo test -p animus-preserves --locked provider_retry_progress --no-fail-fast` — 2 positive/negative metadata round-trip tests;
- `cargo test -p animus-tui --locked provider_retry_progress --no-fail-fast` — 1 negative transcript-projection test;
- focused runtime, std, Preserves, and TUI Clippy with `-D warnings`, plus runtime/std/root no-default-feature checks;
- `nix develop -c scripts/cairn/check-provider-retry --root .` — Nickel positive/negative diagnostics, pure-core authority scan, focused tests, Clippy, dependency-isolation checks, and BLAKE3-bound focused receipt;
- focused receipt `target/cairn/focused-rails/provider-retry.md` with source BLAKE3 `3768c44d18a1cc6fdcbffdc69bf9059f0460d292c8140bb5c7a6f44c7c2b83fc`;
- `nix build .#checks.x86_64-linux.provider-retry -L` and `scripts/hooks/octet-lints`;
- Nickel drift, provider registry/profile, provider adapter/std-provider, Preserves boundary, safe receipt, composition, dogfood-hygiene, catalog/inventory, adapter-conformance, and request-lifecycle companion rails;
- `cargo test --workspace --locked --no-fail-fast`, `cargo check --workspace --all-targets --locked`, and `cargo clippy --workspace --all-targets --locked -- -D warnings`; and
- Cairn validation plus proposal, design, and tasks gates;
- `cargo check -p animus --features provider-retry,preserves,tui --locked` compatibility composition;
- `nix develop -c pre-commit run --all-files`; and
- `nix flake check -L` — all 50 compatible-system checks passed, with only the configured unavailable remote builder warning before local fallback.

The repository-wide Tracey run remains pre-existingly invalid. Before sync it reported 2,162 requirements and 644 referenced markers, with the seven behavioral IDs only in the expected pre-sync dangling set and no retry ID in the missing set. After sync and the final verification marker, it reports 2,170 requirements and 652 referenced markers; no `tau_provider_retry` ID appears in either the dangling or missing diagnostics.

## Bounded adversarial audit

The implementation compared four mechanisms: stateless timer gates, threaded timer state, host-mutable retry loops, and provider-native retry/resume. Stateless gates were rejected because cancellation/completion races and duplicate observations could reopen dispatch. Threaded pure state was selected because one correlated terminal outcome closes the wait while preserving host-owned timer authority. Host-mutable hidden loops were rejected because attempt accounting would not be reviewable. Provider-native retry/resume was rejected because it couples correctness to SDK behavior and permits partial-stream replay outside the canonical stream contract.

A VibeThinker adversarial review identified cancellation/completion races, total elapsed-time budgeting, mapping-version drift, event-field preservation, canonicalization claims, receipt-hash confidentiality, and duplicate-dispatch concerns. The cancellation race was addressed with decision-bound threaded wait state and explicit race tests. Mapping-version mismatch and headless event field preservation received negative/positive tests. Documentation now preserves independent host deadline/total-budget ownership, states that BLAKE3 binds host-supplied canonical identities without proving canonicalization semantics, and treats hashes as correlation metadata rather than confidentiality. Claims of practical BLAKE3 collision bypass or request reconstruction from nested BLAKE3 identities were not accepted without a concrete counterexample. Exactly-once remote effects remain an explicit non-claim because hosts can clone or fork local evidence.

## Non-claims

The evidence does not prove provider behavior, provider availability, native status correctness, host observation honesty, host state-threading correctness, timer implementation correctness, request delivery, provider acceptance, billing behavior, remote cancellation, exactly-once local or remote effects, provider resume safety, router fallback, model quality, semantic completion, live credentials, live network behavior, or production readiness. Optional live-provider smoke remains separately indexed and absent live evidence does not weaken fixture-scoped claims.
