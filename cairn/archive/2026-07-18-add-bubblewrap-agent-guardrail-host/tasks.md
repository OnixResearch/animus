## Phase 1: Boundary, provenance, and feature isolation

- [x] [serial] Record the reviewed `isolate` reference cohort, license/provenance, adopted mechanisms, rejected escape hatches, and distinct Bubblewrap/Sandlock/Mantle claim classes. r[agent_harness_runtime.bubblewrap_agent_guardrail.feature_gate]
- [x] [serial] Add the focused non-default Linux host component and thin `animus-agent-guardrail` executable after the std-host-adapter split, proving default, reusable, Wasm, and non-Linux dependency isolation. r[agent_harness_runtime.bubblewrap_agent_guardrail.feature_gate]
- [x] [serial] Define typed Nickel guardrail profile contracts, checked runtime projections, export-drift checks, and central/project-trust boundaries. r[agent_harness_runtime.bubblewrap_agent_guardrail.profile]

## Phase 2: Pure planning and validation core

- [x] [serial] Define owned active-context facts, authorization binding, safe root binding, typed filesystem operation, network/device/socket/environment posture, executable cohort, launch plan, execution fact, and receipt input types without exposing raw Bubblewrap or host-path types through reusable crates. r[agent_harness_runtime.bubblewrap_agent_guardrail.plan] r[agent_harness_runtime.bubblewrap_agent_guardrail.authority]
- [x] [serial] Implement pure profile normalization, read-only-default mount planning, writable upgrades, ancestor ordering, duplicate normalization, conflict rejection, symlink/root escape checks, and domain-separated BLAKE3 plan identity. r[agent_harness_runtime.bubblewrap_agent_guardrail.plan]
- [x] [serial] Implement active-context preflight decisions that reuse the planner and distinguish valid, denied, unsupported, missing-source, incompatible-cohort, and drifted-plan outcomes without mutation. r[agent_harness_runtime.bubblewrap_agent_guardrail.preflight]
- [x] [serial] Implement pure safe receipt projection and leak validation over bounded execution observations. r[agent_harness_runtime.bubblewrap_agent_guardrail.receipts]

## Phase 3: Thin Bubblewrap shell

- [x] [serial] Implement host path/cohort/capability observation and checked profile loading without project-local policy discovery, configured path creation, shell evaluation, command environment sources, or raw escape arguments. r[agent_harness_runtime.bubblewrap_agent_guardrail.profile] r[agent_harness_runtime.bubblewrap_agent_guardrail.preflight] r[agent_harness_runtime.bubblewrap_agent_guardrail.authority]
- [x] [serial] Implement NUL-delimited sealed-memfd option transport, compatible `bwrap --args` launch, descriptor lifecycle, explicit network mode, child process replacement, and fail-closed no-argv/no-unguarded fallback behavior. r[agent_harness_runtime.bubblewrap_agent_guardrail.sealed_transport]
- [x] [serial] Expose bounded `describe`, `validate`, and `exec` command surfaces with expected-plan identity checking, safe diagnostics, and truthful nested guardrail state. r[agent_harness_runtime.bubblewrap_agent_guardrail.preflight] r[agent_harness_runtime.bubblewrap_agent_guardrail.sealed_transport] r[agent_harness_runtime.bubblewrap_agent_guardrail.receipts]
- [x] [serial] Wire execution only after matching Animus/Basalt authorization and keep denied authorization, unavailable enforcement, launch failure, child exit, and receipt rejection as distinct outcomes. r[agent_harness_runtime.bubblewrap_agent_guardrail.authority] r[agent_harness_runtime.bubblewrap_agent_guardrail.receipts]

## Phase 4: Positive and negative evidence

- [x] [parallel] Add positive pure tests for deterministic profile selection, read-only and writable root planning, explicit inherited networking, authorization binding, stable BLAKE3 identities, and safe successful receipts. r[agent_harness_runtime.bubblewrap_agent_guardrail.verification]
- [x] [parallel] Add negative pure tests for malformed profiles, project-local policy injection, duplicate/conflicting targets, root and symlink escape, implicit writable/device/socket exposure, stale authorization, unsupported network filtering claims, raw arguments, prefixes, passthrough, command sources, plan drift, and receipt leaks. r[agent_harness_runtime.bubblewrap_agent_guardrail.verification]
- [x] [parallel] Add fake-Bubblewrap and executable-shell tests for missing/incompatible executables, memfd preparation and sealing failure, reserved parser controls, descriptor inheritance failure, no argv fallback, non-zero exits, signals, and safe diagnostics. r[agent_harness_runtime.bubblewrap_agent_guardrail.sealed_transport] r[agent_harness_runtime.bubblewrap_agent_guardrail.verification]
- [x] [serial] Add an opt-in Linux behavior rail proving admitted worktree use plus credential-read, sibling-read, outside-write, symlink-escape, and network-deny rejection; unavailable host prerequisites remain blocked with a next check. r[agent_harness_runtime.bubblewrap_agent_guardrail.verification]

## Phase 5: Documentation and closeout

- [x] [parallel] Document operator profile authoring, executable handshake, Tile/PTY integration, backend selection, update process, same-UID limitations, inherited-network caveats, and explicit non-claims. r[agent_harness_runtime.bubblewrap_agent_guardrail.feature_gate] r[agent_harness_runtime.bubblewrap_agent_guardrail.receipts]
- [x] [serial] Run focused std-host, adapters, Nickel, default/no-default/non-Linux, formatting, clippy, safe-receipt, component-catalog, roadmap, and optional Linux behavior checks. r[agent_harness_runtime.bubblewrap_agent_guardrail.verification]
- [x] [serial] Run Cairn validation and proposal/design/tasks gates; sync and archive only after accepted requirement IDs are present and positive/negative evidence or exact Linux blockers are recorded. r[agent_harness_runtime.bubblewrap_agent_guardrail.verification]
