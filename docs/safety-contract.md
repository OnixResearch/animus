# Safety Contract

Animus is structured around a functional-core / imperative-shell (FCIS) split. This contract is repo-owned and must be updated with any intentional change to the safety model.

## FCIS split

- `crates/animus-core` is the functional core. It owns pure deterministic logic, small typed state, and the minimal validation error surface.
- `crates/animus-adapters` is the component-boundary crate. It owns implementation-free port traits and typed DTOs that separate the harness from swappable model, tool, memory, and host integrations.
- `crates/animus-preserves` is the optional Preserves serialization bridge. It owns cross-language text/binary encoding for adapter boundary messages and must not become a required dependency of the core or adapter-contract crates.
- `crates/animus-adapter-rlm-kernel` is the pure no-std contract core for the non-default model-facing Steel environment. It owns bounded policy, state metadata, typed host requests, child admission, compaction preservation, and metadata-only receipts without evaluator or host authority.
- `crates/animus-adapter-change-request` is the pure no-std Remora-derived change-request family. It owns forge-neutral review meaning, exact bindings, separate ports, and metadata-only receipts without VCS, model, network, credential, forge, clock, scheduler, or persistence authority.
- `crates/animus-adapter-maintenance` is the pure no-std Remora-derived maintenance family. It owns inventory, candidate, advisory, grouping, update-plan, validation-profile, port, binding, and receipt meaning without registry, advisory, manifest, workspace, validation, VCS, forge, clock, scheduler, credential, or persistence authority.
- `crates/animus-steel` is the canonical Steel Scheme loop fabric. It also owns the optional transactional `SteelRlmKernel` evaluator and `SteelRlmToolAdapter`. Both directions revalidate every selected action through Rust contracts before dispatch.
- `crates/animus-std-lean-loop` is a non-default process shell implementing the same neutral `LoopFabric` boundary for one exact compiled Lean 4 child. It verifies explicit artifact/profile identities, owns bounded pipes/deadlines/cancellation/cleanup, and treats every Lean result as an untrusted candidate for current Rust DTO reconstruction. It owns no session, transcript, provider, tool executor, or completion authority.
- `crates/animus-std-agent-guardrail` is a non-default Linux Bubblewrap shell. Its pure core plans only from checked central policy, prior matching authorization, and explicit host facts; its shell owns explicit file/executable observation, sealed memfd preparation, and guarded launch. It owns no portable sandbox policy, project-policy discovery, ambient roots/environment, privileged sidecar, or fallback authority, and its evidence remains weaker than Sandlock or Mantle claims.
- `crates/animus-runtime` is the reusable agent-loop runtime. It owns pure turn-state transitions, ordered runtime events/reports, max-turn handling, typed workflow command manifest/registry/binding/output contracts, the metadata-only tool catalog and pure executor admission/receipt core, safe provenance ledger/receipt/artifact DTOs, host-injected ledger/session replay and restore planning, pure durable-session framing/commit/recovery/receipt contracts, pure active-turn acceptance/checkpoint/steering/recovery/receipt contracts, pure snapshot-first conversation projection and disposable-cache contracts, pure bounded branch-explorer topology, annotation, selection, comparison, equivalence, and TUI contracts, canonical metadata-only session authority profiles and durable bindings, the pure coding-session lifecycle/queue/commit-order/recovery/event/receipt reducer, bounded worker-team task/report contracts, and thin generic shells that call injected loop fabrics, tool executors, worker services, and model/tool adapters without constructing concrete providers or host services.
- `crates/animus-std` is the imperative host compatibility shell. It owns environment parsing, runtime setup, a pure authority catalog/fail-closed planner, async work, I/O, facade re-exports, and other process-facing concerns. Its non-default `durable-session-store` block alone owns capability-rooted private journals, writer leases, file permissions, synchronization, and reopen orchestration. Its non-default `active-turn-recovery` block loads explicit generated policy and supplies a deterministic transaction fixture. Its non-default `coding-session-host` block composes only explicitly injected provider, tool, persistence, policy, current-authority, cancellation, timer, summarizer, and event services. Its non-default `codebase-graph-host` block alone owns capability-scoped repository reads and tree-sitter Rust parsing; graph DTOs and reducers remain no-std, and syntactic results grant no mutation, VCS, process, network, or tool authority. The separate `codebase-tool-host` feature binds exact request values and adapts only that graph to one read-only executor with bounded output and metadata-only receipts.
- `crates/animus-std-observability` is an authority-scoped std block for deterministic no-op and in-memory sinks. It depends only on observability adapter contracts and owns no filesystem, process, network, credential, clock, terminal, global subscriber, or persistence authority.
- The root `animus` package is a thin compatibility layer. It must not accumulate non-trivial business logic.
- `docs/request-lifecycle.md` is the maintained ownership map for runtime input, core state, Steel decisions, adapter dispatch, status decisions, reports, receipts, and future ledger seams.
- `docs/status-authority.md` owns semantic agent status precedence: host integrations provide typed observations, while reusable runtime selection remains pure and presentation metadata stays visual-only.
- `docs/runtime-facade-inventory.md` and `.cairn/runtime-facade-inventory.ncl` classify runtime-facing public exports as reusable contracts, host-injection surfaces, or shell adapters.
- `docs/component-catalog.md` and `.cairn/component-catalog.ncl` map component ids to owners, feature gates, authority posture, dependency links, capability families, and focused checks.
- Shell-owned runtime inputs must not leak into exported core state types. Component implementations must be reached through adapter contracts rather than direct cross-crate coupling. Serialized boundary payloads and Steel loop decisions must revalidate through adapter DTO constructors after decoding/evaluation. Multimodal image inputs use `animus-adapters` DTOs for safe labels, media type, byte count, BLAKE3 identity, source trust, provider capability gates, injected image-service handles, provider-native fixtures, and safe image receipts; raw image bytes, base64/data URLs, private paths, credentials, provider payloads, and image loading/transcoding authority remain host/provider-owned.

## Autonomous-gate boundary

The optional `animus-runtime/autonomous-gates` feature receives only typed gate results, host-supplied counters, and fingerprints over an explicit admitted file set. It does not execute commands, read files, inspect clocks, or persist state. Failed-gate output is bounded before prompt use. `gates_passed` and `limit_reached` remain scoped outcomes and never prove task success, objective completion, production readiness, or release eligibility.

## Schedule reliability boundary

The optional `animus-runtime::schedule_reliability` family is a pure session-schedule boundary. It accepts caller-loaded durable jobs and host-injected logical ticks. It returns mutation, claim, coalescing, recovery, delivery-admission, and metadata-only receipt decisions.

The host must persist an exact claim before delivery. Recovery does not deliver a claimed tick again. Missed ticks produce one bounded policy decision, not one delivery per tick. Clocks, timers, storage, prompts, queues, and delivery remain host-owned.

This family does not establish exactly-once execution, real-time delivery, timer accuracy, prompt outcome correctness, production readiness, or release eligibility. See [schedule reliability](schedule-reliability.md).

## Session event replay boundary

The optional `animus-runtime::session_event_replay` family is a pure reconnect-planning boundary. It consumes caller-loaded facts from the existing admitted ledger and runtime order. It does not create a second event store.

A cursor binds session, stream, generation, branch, sequence, policy, and redaction identities. Gaps, duplicates, drift, rebuilds, truncation, no-op, and terminal states are explicit outcomes. Live-only deltas are never replayed as completed semantic content.

The host retains storage, authentication, transport, reconnection, retries, timers, and delivery. This family does not prove transcript truth, transport delivery, durable storage, conversation continuity, production readiness, or release eligibility. See [session event replay](session-event-replay.md).

## Continual-harness refinement boundary

The optional `animus-runtime::persistent_goals` family is a pure goal-state boundary. It accepts only metadata-only objective identities, explicit user or host actions, typed budgets, and host-injected accounting. It rejects model or inferred creation, model lifecycle control, invalid transitions, second active goals, budget-as-success claims, objective text in receipts, and ambient clock or persistence requests. It emits bounded state and receipts only. It does not prove objective achievement, work quality, automatic inference, production readiness, or release eligibility.

The optional `animus-adapters::messaging` family is a no-std pure messaging boundary. It accepts only host-derived family rosters, typed message drafts, supplied target state, counters, policy, and delivery attestations. Sender claims in content, unknown or cross-family targets, denied modes, size/rate/pending violations, ambient transport requests, and unattested delivery fail closed. It emits only routing decisions, limit decisions, and metadata-only `delivered` or `queued` receipts. Transport, interrupt classification, queue storage, network, process, credentials, read/acted state, ordering, exactly-once delivery, comprehension, and persistence remain host-owned and unclaimed.

The optional `animus-adapter-continual` crate is a no-std pure admission boundary. It accepts only typed proposals, current session-local supplemental state, and typed ledger evidence. It admits supplemental prompts, memories, skill descriptions, and subagent specifications under Nickel-authored size, count, conflict, evidence, and retention bounds. Base-prompt, repo-owned skill, accepted-spec, policy-file, evidence-free, and promotion-request proposals fail closed.

Every admitted update plans a BLAKE3 snapshot before the next state. Hosts own snapshot storage and apply the plan. Rollback accepts only retained, unmodified snapshots. Promotion requires a separate explicit host action and produces only a plan. Receipts contain metadata and required non-claims. They reject lesson bodies, prompt text, memory contents, skill bodies, trajectory spans, and improvement or authority claims.

## no_std core boundary

`animus-core`, `animus-adapters`, `animus-adapter-rlm-kernel`, `animus-adapter-change-request`, and `animus-adapter-maintenance` are `#![no_std]` and may use `alloc` for owned data. The root `animus` crate must also build as a no-std facade when default features are disabled, re-exporting only the deterministic reusable surface. Do not use `std`, process environment, filesystem, networking, clocks, randomness, interpreters such as Steel, serialization backends such as Preserves, or global mutable state in the no-std-supported surface.

## Wasm compile boundary

The initial Wasm compile boundary targets `wasm32-unknown-unknown` and covers `animus-core`, `animus-adapters`, `animus-router`, and root `animus --no-default-features`. The rail is a compile-surface check, not a runtime support claim. It excludes `animus-runtime`, `animus-std`, `animus-steel`, `animus-tui`, `animus-preserves`, terminal UI, provider clients, native persistence, process execution, filesystem authority, credentials, browser APIs, WASI host behavior, plugin loading, and host shell orchestration unless a later Cairn change adds a target-specific contract and validation evidence.

The separate `spacewasm-embedded-compat` profile is a non-default diagnostic over one test guest, exact Octet static profile, and exact Mantle/SpaceWasm cohort. Its injected shell grants no reusable authority and a pass cannot be promoted to general runtime, plugin, WASI, sandbox, production, or release support. See [SpaceWasm embedded compatibility](spacewasm-embedded-compatibility.md).

## Evaluation harness boundary

The non-default Harbor-compatible evaluation lane accepts only checked local profiles, pure bounded planning, validated ATIF-v1.7 metadata projection, finite attributed rewards, separate verifier identities, allowlisted artifact manifests, and caller-injected services. It imports no Python, Pydantic, Harbor, container, cloud, filesystem, process, network, credential, clock, scheduler, or persistence authority into reusable crates. Scores and judge outputs remain advisory unless existing Cairn completion policy independently admits exact current evidence. See [Harbor evaluation harness](harbor-evaluation-harness.md).

The focused [Harbor ATIF JSON adapter](harbor-atif-json-adapter.md) consumes caller bytes only. It keeps complete v1.7 validation, reference classification, import/export drafts, semantic identities, and safe receipts in a no-std pure crate. Image, continuation, and subagent locators never grant access. `session_id` never becomes a resolution key. Export does not invent unavailable facts or a terminal wire field. Receipts exclude all trajectory content and overclaims.

The non-default [native evaluation core](native-evaluation-core.md) is `#![no_std]` plus `alloc`. It plans over caller-supplied immutable identities and keeps episodes as canonical ledger references. It cannot load tasks, store traces, call models, tools, or verifiers, move artifacts, use clocks, observe cancellation, schedule runs, or persist results. Shared judges remain advisory. Only full exact terminal resume evidence can suppress work. Native receipts reject raw evaluation content and completion, benchmark, training, production, or release claims.

The non-default [native evaluation adapter family](evaluation-adapter-family.md) is also `#![no_std]` plus `alloc`. It splits each effect into a separate injected port. Catalog admission enforces one capability ceiling per port. Full binding identities prevent cross-plan, cross-task, cross-attempt, cross-policy, cross-artifact, and cross-implementation reuse. Terminal stream evidence cannot pass before exact commit evidence. Equal content cannot authorize retry coalescing. The family performs no transport, execution, discovery, cancellation, clock, artifact, or storage effect.

The non-default [native evaluation host](native-evaluation-host.md) is a standard-library shell. It accepts only checked policy, exact plans, explicit service catalogs, separate gateway capabilities, logical storage targets, and allowlisted artifacts. Preflight rejects missing services, unsafe binds, shared capabilities, links, hidden tests, stale evidence, and excessive bounds before service effects. The shell does not prove confinement, durability, exactly-once effects, verifier soundness, benchmark validity, completion, production readiness, or release eligibility.

The non-default [evaluation composition conformance](evaluation-composition-conformance.md) suite calls public production APIs with finite scripted fakes. It pairs every mechanism family with expected-negative coverage. It rejects private fixture data and unsupported claims. Fixture success does not prove live behavior, confinement, durability, verifier soundness, benchmark validity, completion, production readiness, or release eligibility.

## Remora-derived maintenance boundary

The non-default [Remora-derived adapter families](remora-adapter-families.md) adapt one exact Remora source cohort. The adaptation includes pure domain data, planning, separate ports, failure classes, exact binding, and metadata-only evidence.

Security and major updates stay isolated. Routine groups require matching ecosystem and manifest identities. Trusted validation profiles replace raw shell commands.

Inline review comments require observed commit, file, side, line, and range facts. Publication observations must match the exact planned comment count and summary state.

Missing services fail closed. Denied, unsupported, unavailable, or stale services retain their exact typed availability class. No ambient implementation or fallback is discovered.

Receipts reject raw dependencies, versions, manifests, diffs, comments, commands, logs, credentials, provider payloads, and physical paths. They also reject unsupported readiness claims.

The families do not prove registry freshness, advisory completeness, update correctness, validation sufficiency, VCS isolation, forge delivery, review quality, merge safety, production readiness, or release eligibility.

## Security review boundary

`animus-adapters` owns the reusable Goose-inspired security review DTOs and pure helpers. Review inputs keep only task labels, bounded context summaries, safe tool labels, active policy labels, and BLAKE3 evidence hashes; raw prompts, credentials, provider payloads, private paths, and unbounded tool arguments stay outside stored review state and receipts.

Deterministic scanner findings and optional classifier/adversary-review outcomes are evidence, not authority. They can block or require confirmation, but they cannot grant missing authority, widen capabilities, bypass sandbox policy, suppress required confirmation, or dispatch provider/tool/host effects. Review outage handling is explicit by action risk class: high-risk actions fail closed or require confirmation according to policy, while low-risk advisory outages may degrade to completed deterministic scanner evidence only.

## Multimodal image boundary

`animus-adapters` owns provider-neutral image content descriptors, prompt-order preservation outcomes, model capability gates, host-injected image payload service traits, provider-native fixture DTOs, and safe image receipts. It must not load files, decode images, sniff MIME types, resize/transcode media, persist raw pixels, serialize live provider payloads, or call providers. Hosts and provider adapters own those side effects behind injected services, and reusable validation rejects raw bytes, base64/data URLs, absolute private paths, credentials, raw provider payloads, and unbounded metadata.

## Provider video boundary

`animus-adapters` owns typed video descriptors, ordered mixed-media assembly, exact video capability decisions, provider-projection loss checks, safe handle DTOs, and metadata-only receipts. Video is separate from image capability. Required video fails closed when its model class, media limits, injected service, or exact provider projection is absent. Reusable code does not resolve paths or URLs, inspect containers, decode frames, transcode media, upload files, refresh credentials, or call providers. Receipt checks reject bytes, base64, data and remote URLs, private paths, credentials, upload material, provider bodies, frame bodies, audio bodies, and unbounded metadata.

## Terminal image boundary

The non-default `animus-tui/terminal-image` feature owns pure operation-level planning and a thin injected shell. The planner receives safe metadata, exact protocol rows, geometry, lifecycle state, and named limits. It has no byte, environment, file, terminal, process, network, clock, credential, or persistence authority. The shell rechecks resolved bytes against the admitted BLAKE3 identity and calls the pinned upstream `rat-image` preparation and writer seam. Query, placement, replacement, deletion, and cleanup remain host-injected operations. Missing rows fail or use visible bounded text under policy. Partial writes and failed cleanup cannot produce success. Receipts reject raw or encoded bytes, escape payloads, paths, environment values, terminal responses, provider payloads, and credentials. See [terminal image protocols](terminal-image-protocols.md).

## Terminal video boundary

The non-default `animus-tui/terminal-video` feature adds pure scheduling over explicit ticks and bounded decoded-frame metadata. It does not decode media, open paths or URLs, spawn processes, read clocks, play audio, or retain frame bodies. An injected frame source returns one transient bounded static frame. The shell rechecks its BLAKE3 identity and uses the terminal-image writer. Exact host rows own Kitty animation controls, static replacement, fallbacks, and cleanup. Backpressure acts before queue retention. Cancellation and terminal loss stop requests and create exact cleanup obligations. See [bounded terminal video playback](terminal-video-playback.md).

## Redacted session export boundary

`animus-runtime::session_tree_export` may project and render only already-loaded validated safe metadata. It must not open journals, infer provider formats, accept raw transcript/tool/reasoning/provider/credential/environment/path/error bodies, or perform destination I/O. `animus-std` owns explicit capability-scoped writes; success requires private mode, bounded bytes, exact artifact identity, commit, and synchronization evidence. Export receipts remain metadata-only and do not prove session correctness, omitted-content completeness, storage durability, release eligibility, or training-dataset suitability. See [redacted session-tree export](redacted-session-tree-export.md).

## Durable private-session boundary

`animus-runtime::durable_session_store` may process bounded replay-private bytes only inside private frame values; its sidecars and receipts expose metadata and BLAKE3 identities, never those bodies. It remains pure and cannot discover or open storage. `animus-std/durable-session-store` requires an explicit capability root and relative names, rejects ambient fallback, serializes one writer, reports durable success only after `sync_all`, and fails closed on committed corruption or ambiguous lineage. Encryption, authentication, power-loss behavior, filesystem correctness, backup, replication, and production recovery remain non-claims. See [durable local session store](durable-session-store.md).

## Session branch explorer boundary

`animus_runtime::branch_explorer` derives topology from one validated ledger snapshot. Sidecar order never changes replay. Activation is leaf-only and revision-fenced. Fork plans are append-only. External-world isolation requires separate supplied evidence. See [session branch explorer](session-branch-explorer.md).

## Canonical conversation projection boundary

`animus_runtime::conversation_projection` consumes only already-redacted ledger facts. It cannot read storage, mutate ledger state, contact transports, or render a UI. Caches remain disposable and cannot become model context. See [canonical conversation projection](canonical-conversation-view-projection.md).

## Active-turn recovery boundary

`animus_runtime::active_turn_recovery` plans durable acceptance, atomic checkpoint replacement, steering delivery, and fail-closed recovery from supplied facts. It cannot read storage, call providers, run tools, inspect clocks, or clean state. The std fixture host does not prove store correctness or power-loss durability. See [active-turn recovery](active-turn-recovery.md).

## Execution backend selection boundary

The optional `animus_runtime::execution_backend_planner` family is a pure planning boundary. It selects the first fully compatible candidate in explicit Nickel policy order. It does not assign hidden numeric authority ranks or infer requirements from commands and agent labels.

An unavailable selected backend requires an explicit new plan. The planner does not silently use a broader candidate. The thin std seam calls one injected current-enforcement service and checks exact backend, adapter, application-host binding, runtime-authority, effect-permission, resource, and enforcement identities before effects.

The host retains backend discovery, effect admission, filesystem, process, network, sandbox, execution, cancellation, cleanup, and result authority. A plan does not prove current availability, sandbox enforcement, completion, correctness, production readiness, or release eligibility. See [execution backend selection](execution-backend-selection.md).

## Composition activation boundary

`animus_session_core::composition_activation_graph` is a pure `no_std` gate over caller-supplied graph and readiness facts. It cannot discover providers, read host state, open stores, inspect credentials, or execute effects.

Maintained product roots must run this gate before credential, store, provider, process, filesystem, network, terminal, daemon, or ready-endpoint effects. Missing required providers, invalid consumers, mismatched authority, stale readiness, and required event gaps block startup without fallback. The inspection snapshot is metadata-only and grants no authority. See [composition activation graph](composition-activation-graph.md).

## Component-scope boundary

`animus_session_core::component_scope` is a pure `no_std` registration, resolution, and teardown reducer. It accepts only explicit scopes, generations, registrations, dependencies, commands, and host observations.

A scope lease grants no effect authority. Close revokes new resolution before delivery, cancellation, quiescence, release, or receipt effects.

The local daemon shell calls five injected ports. Unresolved cleanup remains visible as `failed_cleanup`. Receipts cannot prove cancellation, process death, resource reclamation, sandboxing, or external liveness. See [generation-fenced component scopes](component-scopes.md).

## Coding-session host boundary

`animus_runtime::coding_session_host` owns pure deterministic run, queue, active-leaf, durable-cursor, pending-effect, recovery, cancellation, event, and receipt reductions. It accepts only already-expanded and admitted input. It cannot discover resources or sessions, interpret commands, call providers or tools, read credentials or clocks, open storage, access a terminal, or perform persistence. The non-default `animus_std::coding_session_host` shell calls only explicit injected services and may emit a dependent provider or tool request only after the prerequisite metadata-only authority binding and message commit succeed and current enforcement is independently observed. Exact profile equality detects continuity but cannot authorize, probe, or enforce anything. Pending queue bodies remain non-transcript state until selected and committed. Resume validates the explicit store and admitted repair before continuation. Cancellation does not prove rollback. Receipts reject raw prompts, provider payloads, tool output, credentials, paths, and raw errors. See [coding-session host facade](coding-session-host.md) and [session runtime authority profiles](session-runtime-authority-profile.md).

## Optional Basalt authority boundary

The generic authority-provider contract is `no_std` plus `alloc` and exposes no Basalt or UCAN type. Display names are never principals. Stable agent-definition BLAKE3 identities bind explicitly to audience DIDs. The optional `AGPL-3.0-or-later` shell accepts only the closed reviewed document read/write mappings and caller-injected key, proof, revocation, caveat, replay, time, limit, and commit services. Basalt can deny but cannot widen any existing local gate. The shell retains live authority through commit-before-effect dispatch. Receipts are metadata-only and reject credentials, payloads, paths, diagnostics, and trust or completion overclaims. See [optional Basalt authority adapter](basalt-authority-adapter.md).

## Optional Pijul VCS boundary

The Pijul DTO and service contracts are `no_std` plus `alloc`. They do not link Pijul database crates or gain process, filesystem, environment, identity, key, network, remote, or persistence authority. Pure runtime validation admits only the exact beta.18 cohort. The optional std shell calls a caller-injected bounded executor with explicit argv and no environment inheritance. It uses hash-only log JSON because rich beta.18 logs can read identity files. Protected mutations and remote operations stop before executor dispatch. Receipts omit paths, diffs, messages, authors, process output, and unrestricted diagnostics. See [Pijul VCS adapter](pijul-vcs-adapter.md).

## Optional SecretSpec credential boundary

The credential request contract is `no_std` plus `alloc` and contains public metadata only. It rejects implicit selection, provider fallback, and writes before provider access. The optional host clears ambient environment variables, verifies the exact manifest identity, disables SecretSpec audit persistence, and runs resolution in an owned bounded worker. SOPS uses a second read-only bounded guard. Secret material is zeroizing and can cross only a borrowed injected delivery call. Receipts omit values, value hashes, credentials, paths, and process output. Resolution does not grant caller authority. See [SecretSpec host resolution](secretspec-host-resolution.md).

## Skill-source lifecycle boundary

Frozen skill sources bind exact revisions, BLAKE3 trees, and manifests. Tracked observations can create inactive candidates only. Promotion requires exact review facts and the existing host asset mutation gate. Fetching, paths, credentials, extraction, review, storage, and activation remain host owned. See [Skill-source lifecycle](skill-source-lifecycle.md).

## Optional Trajectory transcript boundary

The Trajectory DTO and service contracts add no Node, Bun, npm, TypeScript, filesystem, process, environment, network, SQLite, credential, clock, or persistence authority. Pure Rust revalidates the exact trajectory-v1 result. The non-default std shell calls only a caller-injected bounded executor. Reasoning, tool bodies, source metadata, and diagnostic prose are omitted by default. Projection creates existing neutral-ledger entries and batch drafts only. It does not persist, export, evaluate, or authorize the supplied content. See [Trajectory transcript adapter](trajectory-transcript-adapter.md).

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
scripts/cairn/check-provider-video-payloads --root .
scripts/cairn/check-remora-adapter-families --root .
scripts/cairn/check-basalt-authority-adapter --root .
scripts/cairn/check-pijul-vcs-adapter --root .
scripts/cairn/check-terminal-image-protocols --root .
scripts/cairn/check-terminal-video-playback --root .
cargo test -p animus-preserves
cargo test -p animus-steel
cargo test -p animus-runtime
cargo test -p animus-runtime tool_executor_kit --locked
cargo test -p animus-runtime worker_team --locked
cargo test -p animus-std
scripts/cairn/check-durable-session-store --root .
scripts/cairn/check-active-turn-recovery --root .
scripts/cairn/check-conversation-projection --root .
scripts/cairn/check-session-branch-explorer --root .
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

`scripts/cairn/check-remora-adapter-families --root .` verifies the exact Remora cohort, pure plans, separate ports, bindings, receipts, no-std and Wasm facades, catalogs, and dependency isolation. `scripts/cairn/check-session-tree-export --root .` verifies generated redaction policy, malformed topology rejection, inert deterministic artifacts, explicit destination writes, and metadata-only receipts. `cargo tigerstyle check` is the local executable Tiger Style policy rail backed by workspace metadata. `nix run .#cargo-tigerstyle -- check` exposes the pinned flake app for the same check. `check-nickel-configs` and `nix run .#check-nickel-configs` verify that generated TOML/YAML configs match `config/repo.ncl`. `scripts/cairn/check-clankers-product-surfaces --root .` verifies that bounded Clankers documentation remains allowed while reusable crates stay free of rejected Clankers product-surface dependencies and runtime assumptions. `scripts/cairn/check-request-lifecycle --root .` verifies that the lifecycle ownership map keeps required boundary, owner, forbidden-dependency, and validation markers. `cargo test -p animus-runtime worker_team --locked` verifies bounded worker delegation contracts, minimized context packets, injected worker service fail-closed behavior, report merge, and no-bypass mutation handling. `scripts/cairn/check-steel-runtime-policy --root .` verifies current Steel runtime activation and Rust revalidation with positive and negative program fixtures, and records explicit policy-drift source handling. `scripts/cairn/check-rlm-kernel --root .` verifies the optional persistent `steel_repl` adapter, typed host requests, bounded child spawn, compaction preservation, metadata-only receipts, and no-std/Wasm isolation. `scripts/cairn/check-injected-services --root .` verifies missing, denied, unavailable, cancelled, timed-out, malformed, and adapter-failure outcomes fail closed behind injected services. `scripts/cairn/check-composition-conformance --root .` verifies the cross-module conformance matrix, fake-backed positive composition, fail-closed negative composition, and bounded evidence receipts. `scripts/cairn/check-composition-activation-graph --root .` verifies catalog closure, generated maps, pure preflight, startup ordering, safe inspection, and no ambient fallback. `scripts/cairn/check-component-scopes --root .` verifies generation-fenced registration, explicit resolution, ordered teardown, daemon mapping, unresolved cleanup, and metadata-only receipts. `scripts/cairn/check-runtime-facade-inventory --root .` verifies Nickel-owned public export classification, owner metadata, fixture coverage, and class side-effect consistency. `scripts/cairn/check-component-catalog --root .` verifies the Nickel component catalog, generated docs projection, dependency links, cycles, authority labels, focused checks, safe evidence labels, and positive/negative fixtures. `scripts/cairn/check-runtime-kernel-split --root .` verifies the no-default runtime kernel build, `animus_runtime::kernel` facade parity, forbidden dependency/source leaks, and kernel split fixtures. `scripts/cairn/check-ast-grep-agent-boundary-audits --root .` verifies inventory-only ast-grep profiles, rule fixtures, BLAKE3 rule-bundle identity, bounded finding counts, and structural-audit non-claims without treating a clean scan as a safety proof. `cargo test -p animus-runtime session_persistence --locked` verifies host-injected ledger/session service descriptors, pure replay/restore planning, safe receipts, and fail-closed missing-service, stale, conflicting, malformed, ambient-fallback, and leak cases. `scripts/cairn/check-wasm-surface --root .` verifies the credential-free `wasm32-unknown-unknown` compile surface for core, adapters, router, and the no-default root facade while rejecting excluded shell/runtime packages in the no-default root facade tree. The `nickel-configs` and `octet-lints` pre-commit hooks in `.pre-commit-config.yaml` call `scripts/config/check-nickel-configs` and `scripts/hooks/octet-lints`. Keep this document, `config/repo.ncl`, generated config files, `workspace.metadata.tigerstyle`, `workspace.metadata.octet`, the flake apps, and the pre-commit hooks in sync with the Octet command.
