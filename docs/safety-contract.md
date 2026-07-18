# Safety Contract

Animus is structured around a functional-core / imperative-shell (FCIS) split. This contract is repo-owned and must be updated with any intentional change to the safety model.

## FCIS split

- `crates/animus-core` is the functional core. It owns pure deterministic logic, small typed state, and the minimal validation error surface.
- `crates/animus-adapters` is the component-boundary crate. It owns implementation-free port traits and typed DTOs that separate the harness from swappable model, tool, memory, and host integrations.
- `crates/animus-preserves` is the optional Preserves serialization bridge. It owns cross-language text/binary encoding for adapter boundary messages and must not become a required dependency of the core or adapter-contract crates.
- `crates/animus-steel` is the canonical Steel Scheme loop fabric. It owns script evaluation for agent turn selection and must revalidate every selected action through Rust adapter DTOs before calling model, tool, or response boundaries.
- `crates/animus-std-lean-loop` is a non-default process shell implementing the same neutral `LoopFabric` boundary for one exact compiled Lean 4 child. It verifies explicit artifact/profile identities, owns bounded pipes/deadlines/cancellation/cleanup, and treats every Lean result as an untrusted candidate for current Rust DTO reconstruction. It owns no session, transcript, provider, tool executor, or completion authority.
- `crates/animus-std-agent-guardrail` is a non-default Linux Bubblewrap shell. Its pure core plans only from checked central policy, prior matching authorization, and explicit host facts; its shell owns explicit file/executable observation, sealed memfd preparation, and guarded launch. It owns no portable sandbox policy, project-policy discovery, ambient roots/environment, privileged sidecar, or fallback authority, and its evidence remains weaker than Sandlock or Mantle claims.
- `crates/animus-runtime` is the reusable agent-loop runtime. It owns pure turn-state transitions, ordered runtime events/reports, max-turn handling, typed workflow command manifest/registry/binding/output contracts, the metadata-only tool catalog and pure executor admission/receipt core, safe provenance ledger/receipt/artifact DTOs, host-injected ledger/session replay and restore planning, pure durable-session framing/commit/recovery/receipt contracts, canonical metadata-only session authority profiles and durable bindings, the pure coding-session lifecycle/queue/commit-order/recovery/event/receipt reducer, bounded worker-team task/report contracts, and thin generic shells that call injected loop fabrics, tool executors, worker services, and model/tool adapters without constructing concrete providers or host services.
- `crates/animus-std` is the imperative host compatibility shell. It owns environment parsing, runtime setup, a pure authority catalog/fail-closed planner, async work, I/O, facade re-exports, and other process-facing concerns. Its non-default `durable-session-store` block alone owns capability-rooted private journals, writer leases, file permissions, synchronization, and reopen orchestration. Its non-default `coding-session-host` block composes only explicitly injected provider, tool, persistence, policy, current-authority, cancellation, timer, summarizer, and event services.
- `crates/animus-std-observability` is an authority-scoped std block for deterministic no-op and in-memory sinks. It depends only on observability adapter contracts and owns no filesystem, process, network, credential, clock, terminal, global subscriber, or persistence authority.
- The root `animus` package is a thin compatibility layer. It must not accumulate non-trivial business logic.
- `docs/request-lifecycle.md` is the maintained ownership map for runtime input, core state, Steel decisions, adapter dispatch, status decisions, reports, receipts, and future ledger seams.
- `docs/status-authority.md` owns semantic agent status precedence: host integrations provide typed observations, while reusable runtime selection remains pure and presentation metadata stays visual-only.
- `docs/runtime-facade-inventory.md` and `cairn/runtime-facade-inventory.ncl` classify runtime-facing public exports as reusable contracts, host-injection surfaces, or shell adapters.
- `docs/component-catalog.md` and `cairn/component-catalog.ncl` map component ids to owners, feature gates, authority posture, dependency links, capability families, and focused checks.
- Shell-owned runtime inputs must not leak into exported core state types. Component implementations must be reached through adapter contracts rather than direct cross-crate coupling. Serialized boundary payloads and Steel loop decisions must revalidate through adapter DTO constructors after decoding/evaluation. Multimodal image inputs use `animus-adapters` DTOs for safe labels, media type, byte count, BLAKE3 identity, source trust, provider capability gates, injected image-service handles, provider-native fixtures, and safe image receipts; raw image bytes, base64/data URLs, private paths, credentials, provider payloads, and image loading/transcoding authority remain host/provider-owned.

## no_std core boundary

`animus-core` and `animus-adapters` are `#![no_std]` and may use `alloc` for owned data. The root `animus` crate must also build as a no-std facade when default features are disabled, re-exporting only the deterministic reusable surface. Do not use `std`, process environment, filesystem, networking, clocks, randomness, interpreters such as Steel, serialization backends such as Preserves, or global mutable state in the no-std-supported surface.

## Wasm compile boundary

The initial Wasm compile boundary targets `wasm32-unknown-unknown` and covers `animus-core`, `animus-adapters`, `animus-router`, and root `animus --no-default-features`. The rail is a compile-surface check, not a runtime support claim. It excludes `animus-runtime`, `animus-std`, `animus-steel`, `animus-tui`, `animus-preserves`, terminal UI, provider clients, native persistence, process execution, filesystem authority, credentials, browser APIs, WASI host behavior, plugin loading, and host shell orchestration unless a later Cairn change adds a target-specific contract and validation evidence.

## Security review boundary

`animus-adapters` owns the reusable Goose-inspired security review DTOs and pure helpers. Review inputs keep only task labels, bounded context summaries, safe tool labels, active policy labels, and BLAKE3 evidence hashes; raw prompts, credentials, provider payloads, private paths, and unbounded tool arguments stay outside stored review state and receipts.

Deterministic scanner findings and optional classifier/adversary-review outcomes are evidence, not authority. They can block or require confirmation, but they cannot grant missing authority, widen capabilities, bypass sandbox policy, suppress required confirmation, or dispatch provider/tool/host effects. Review outage handling is explicit by action risk class: high-risk actions fail closed or require confirmation according to policy, while low-risk advisory outages may degrade to completed deterministic scanner evidence only.

## Multimodal image boundary

`animus-adapters` owns provider-neutral image content descriptors, prompt-order preservation outcomes, model capability gates, host-injected image payload service traits, provider-native fixture DTOs, and safe image receipts. It must not load files, decode images, sniff MIME types, resize/transcode media, persist raw pixels, serialize live provider payloads, or call providers. Hosts and provider adapters own those side effects behind injected services, and reusable validation rejects raw bytes, base64/data URLs, absolute private paths, credentials, raw provider payloads, and unbounded metadata.

## Redacted session export boundary

`animus-runtime::session_tree_export` may project and render only already-loaded validated safe metadata. It must not open journals, infer provider formats, accept raw transcript/tool/reasoning/provider/credential/environment/path/error bodies, or perform destination I/O. `animus-std` owns explicit capability-scoped writes; success requires private mode, bounded bytes, exact artifact identity, commit, and synchronization evidence. Export receipts remain metadata-only and do not prove session correctness, omitted-content completeness, storage durability, release eligibility, or training-dataset suitability. See [redacted session-tree export](redacted-session-tree-export.md).

## Durable private-session boundary

`animus-runtime::durable_session_store` may process bounded replay-private bytes only inside private frame values; its sidecars and receipts expose metadata and BLAKE3 identities, never those bodies. It remains pure and cannot discover or open storage. `animus-std/durable-session-store` requires an explicit capability root and relative names, rejects ambient fallback, serializes one writer, reports durable success only after `sync_all`, and fails closed on committed corruption or ambiguous lineage. Encryption, authentication, power-loss behavior, filesystem correctness, backup, replication, and production recovery remain non-claims. See [durable local session store](durable-session-store.md).

## Coding-session host boundary

`animus_runtime::coding_session_host` owns pure deterministic run, queue, active-leaf, durable-cursor, pending-effect, recovery, cancellation, event, and receipt reductions. It accepts only already-expanded and admitted input. It cannot discover resources or sessions, interpret commands, call providers or tools, read credentials or clocks, open storage, access a terminal, or perform persistence. The non-default `animus_std::coding_session_host` shell calls only explicit injected services and may emit a dependent provider or tool request only after the prerequisite metadata-only authority binding and message commit succeed and current enforcement is independently observed. Exact profile equality detects continuity but cannot authorize, probe, or enforce anything. Pending queue bodies remain non-transcript state until selected and committed. Resume validates the explicit store and admitted repair before continuation. Cancellation does not prove rollback. Receipts reject raw prompts, provider payloads, tool output, credentials, paths, and raw errors. See [coding-session host facade](coding-session-host.md) and [session runtime authority profiles](session-runtime-authority-profile.md).

## Config ownership

Parse configuration once at the shell boundary, validate it, and keep the typed result in owned structs such as `animus-std::RuntimeConfig` for host input loading or `animus-runtime::RuntimeConfig` for loop execution limits. Do not validate-and-drop raw settings and then re-read or re-parse them later.

Repo-owned tool configuration must be authored in Nickel under `config/repo.ncl` with typed contracts. Keep tool-native TOML/YAML files generated from Nickel for consumers that require those formats, and run the generated-config drift check after every config change.

## Tests

Every test suite must include both positive and negative cases. Positive tests cover valid inputs and expected behavior. Negative tests cover invalid config, invalid input, missing fields, boundary failures, and shell-side error paths.

Run the smallest relevant baseline tests before modifying core logic, then run them again after the change. If no baseline is possible, record why in the change evidence.

Tests that touch process-global state, such as environment variables, clocks, or current directories, must add explicit isolation. Prefer deterministic helpers such as `RuntimeConfig::from_env_pairs` over mutating process globals.

## No magic numbers

Numeric literals that are not self-evident must be named constants, parameters, or configuration values. If a number needs a comment, it needs a name.

## Unsafe code

Workspace lints forbid unsafe code. If this policy is intentionally relaxed in the future, every unsafe block must include a clear safety explanation and must pass `clippy::undocumented_unsafe_blocks`.

## Octet lint levels

`config/repo.ncl` is the source of truth for the consumer-owned Octet configuration exported to `dylint.toml`. Every registered Octet lint for the pinned flake input must be listed under `octet.lint_levels` with level `"deny"`. Keep the list complete when `flake.lock` updates; any suppression must be narrow, justified, owned, and tested.

## ACP stdio safety boundary

The optional [ACP stdio host](acp-stdio-host.md) is non-default and local-stdio-only. Animus owns bounded newline framing, backpressure, correlation, permission-response admission, cancellation/deadline observation, write/flush/broken-pipe classification, safe stderr diagnostics, and cleanup. It has no socket, HTTP, WebSocket, TLS, provider discovery, credential discovery, implicit retry, or network fallback. ACP approval is additive and cannot widen policy, capability, sandbox, authority profile, or executor admission.

## Verification rails

Use these commands as the standard verification rails:

```sh
cargo test -p animus-core
cargo test -p animus-adapters
cargo test -p animus-adapters multimodal
cargo test -p animus-preserves
cargo test -p animus-steel
cargo test -p animus-runtime
cargo test -p animus-runtime tool_executor_kit --locked
cargo test -p animus-runtime worker_team --locked
cargo test -p animus-std
scripts/cairn/check-durable-session-store --root .
scripts/cairn/check-coding-session-host --root .
scripts/cairn/check-session-authority-profile --root .
cargo test -p animus-std-observability
cargo check -p animus-std --no-default-features
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo check -p animus --features preserves
cargo check -p animus --features steel
cargo check -p animus --features runtime
cargo tigerstyle check
check-nickel-configs
scripts/cairn/check-clankers-product-surfaces --root .
scripts/cairn/check-request-lifecycle --root .
scripts/cairn/check-provenance-ledger --root .
scripts/cairn/check-steel-runtime-policy --root .
scripts/cairn/check-injected-services --root .
scripts/cairn/check-composition-conformance --root .
scripts/cairn/check-runtime-facade-inventory --root .
scripts/cairn/check-component-catalog --root .
scripts/cairn/check-runtime-kernel-split --root .
scripts/cairn/check-ast-grep-agent-boundary-audits --root .
cargo test -p animus-runtime session_persistence --locked
scripts/cairn/check-wasm-surface --root .
nix run .#cargo-tigerstyle -- check
nix run .#check-nickel-configs
nix flake check
```

`scripts/cairn/check-session-tree-export --root .` verifies generated redaction policy, malformed topology rejection, inert deterministic artifacts, explicit destination writes, and metadata-only receipts. `cargo tigerstyle check` is the local executable Tiger Style policy rail backed by workspace metadata. `nix run .#cargo-tigerstyle -- check` exposes the pinned flake app for the same check. `check-nickel-configs` and `nix run .#check-nickel-configs` verify that generated TOML/YAML configs match `config/repo.ncl`. `scripts/cairn/check-clankers-product-surfaces --root .` verifies that bounded Clankers documentation remains allowed while reusable crates stay free of rejected Clankers product-surface dependencies and runtime assumptions. `scripts/cairn/check-request-lifecycle --root .` verifies that the lifecycle ownership map keeps required boundary, owner, forbidden-dependency, and validation markers. `cargo test -p animus-runtime worker_team --locked` verifies bounded worker delegation contracts, minimized context packets, injected worker service fail-closed behavior, report merge, and no-bypass mutation handling. `scripts/cairn/check-steel-runtime-policy --root .` verifies current Steel runtime activation and Rust revalidation with positive and negative program fixtures, and records explicit policy-drift source handling. `scripts/cairn/check-injected-services --root .` verifies missing, denied, unavailable, cancelled, timed-out, malformed, and adapter-failure outcomes fail closed behind injected services. `scripts/cairn/check-composition-conformance --root .` verifies the cross-module conformance matrix, fake-backed positive composition, fail-closed negative composition, and bounded evidence receipts. `scripts/cairn/check-runtime-facade-inventory --root .` verifies Nickel-owned public export classification, owner metadata, fixture coverage, and class side-effect consistency. `scripts/cairn/check-component-catalog --root .` verifies the Nickel component catalog, generated docs projection, dependency links, cycles, authority labels, focused checks, safe evidence labels, and positive/negative fixtures. `scripts/cairn/check-runtime-kernel-split --root .` verifies the no-default runtime kernel build, `animus_runtime::kernel` facade parity, forbidden dependency/source leaks, and kernel split fixtures. `scripts/cairn/check-ast-grep-agent-boundary-audits --root .` verifies inventory-only ast-grep profiles, rule fixtures, BLAKE3 rule-bundle identity, bounded finding counts, and structural-audit non-claims without treating a clean scan as a safety proof. `cargo test -p animus-runtime session_persistence --locked` verifies host-injected ledger/session service descriptors, pure replay/restore planning, safe receipts, and fail-closed missing-service, stale, conflicting, malformed, ambient-fallback, and leak cases. `scripts/cairn/check-wasm-surface --root .` verifies the credential-free `wasm32-unknown-unknown` compile surface for core, adapters, router, and the no-default root facade while rejecting excluded shell/runtime packages in the no-default root facade tree. The `nickel-configs` and `octet-lints` pre-commit hooks in `.pre-commit-config.yaml` call `scripts/config/check-nickel-configs` and `scripts/hooks/octet-lints`. Keep this document, `config/repo.ncl`, generated config files, `workspace.metadata.tigerstyle`, `workspace.metadata.octet`, the flake apps, and the pre-commit hooks in sync with the Octet command.
