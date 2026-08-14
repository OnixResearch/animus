# Proposal: Add generation-fenced component scopes

## Why

Animus composes injected services and optional host blocks explicitly, but reusable lifecycle contracts do not provide one scoped registration model with deterministic visibility loss, cancellation, quiescence, and reverse-order teardown.

Without this contract, a live host can remove a provider from its own registry while stale listeners, child tasks, or retained service handles still accept work. Existing daemon generations and lifecycle hooks cover related cases, but they do not define a reusable component-scope state machine.

DeepSeek Harness commit `47f943859bef60e4160492346772ded9b24f765a` provides a useful scoped-registration and disposable-lifecycle pattern. Animus should adapt the lifetime rules without adopting a mutable ambient context or making policy and authority owners replaceable plugins.

## What Changes

- Add no-std component-scope, registration, lease, generation, parent, dependency, and lifecycle DTOs. r[agent_harness_runtime.component_scope.registration]
- Add pure explicit-scope resolution that rejects ambient, stale, closing, unauthorized, and out-of-scope lookup. r[agent_harness_runtime.component_scope.resolution]
- Add a teardown state machine that removes new resolution first, closes delivery, requests cancellation, awaits quiescence, and releases resources in reverse dependency order. r[agent_harness_runtime.component_scope.teardown]
- Make repeated or concurrent close requests converge on one generation-bound terminal outcome. r[agent_harness_runtime.component_scope.concurrency]
- Add metadata-only lifecycle receipts and exact unresolved-cleanup outcomes. r[agent_harness_runtime.component_scope.evidence]
- Add positive and negative core, shell, and daemon-composition tests. r[agent_harness_runtime.component_scope.verification]

## Prerequisites

- `generate-composition-activation-graph`
- `agent_harness_runtime.turn_lifecycle_hooks.fsm`
- `agent_harness_runtime.agent_control.cancel`
- `agent_harness_runtime.daemon_handoff.generation`

## Impact

- **Pure core**: a deterministic state machine and effect plans in `animus-session-core` with runtime re-exports.
- **Ports**: narrow host-injected cancellation, quiescence-observation, delivery-close, and release services.
- **Shells**: `add-local-daemon-supervisor-host` and future live hosts use explicit scope leases.
- **Evidence**: generation, component identities, transition classes, counts, unresolved cleanup, and non-claims only.

## Out of Scope

- Thread spawning, task runtimes, process teardown mechanics, global dependency injection, plugin discovery, hidden cancellation, or a general-purpose async framework.
- Treating scope ownership as effect authority, authorization, sandboxing, or proof that an external resource ended.

## Affected Specs

- `agent-harness-runtime`: registration, resolution, teardown, concurrency, evidence, and verification.
