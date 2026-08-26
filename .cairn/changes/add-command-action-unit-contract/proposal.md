# Proposal: Add a command action-unit contract

## Why

Animus agents already route tools and model calls through typed adapter contracts. There is no product-neutral contract for a named, schema-validated action unit that one agent can invoke on another with an explicit effect plan.

Agent-to-agent coordination therefore invents ad-hoc message shapes per orchestration. Message shape disagreement cannot be detected at the boundary, and effect ownership is implied instead of declared.

Jido, the Elixir agent framework at `github.com/agentjido/jido` revision `548b2a345765ba33e687341c661bbbcbdda73d94`, formalizes the same rule with three invariants in its `cmd/2` operation:

- the returned agent is always complete; there is no later "apply directives" step;
- directives describe runtime-owned effects only and never mutate agent state;
- deterministic actions produce deterministic command results.

Animus should adapt only the authority-free command-contract discipline. Products retain action truth, execution, scheduling, persistence, retries, credentials, diagnostics, and product effects.

## What Changes

- Add an opaque command identity, action-unit descriptor, declared input schema descriptor, effect-plan output, result, and metadata-only command-receipt family. r[animus.command_contract.model]
- Add pure validation for declared schemas, exact command binding, complete-before-apply structure, effect-intent-only output, duplicate and missing member cases, and over-limit sets. r[animus.command_contract.admission]
- Bind command replacement to explicit generation transitions. r[animus.command_contract.generation]
- Add frozen Lattice orchestration and Pi tool-host mapping fixtures without runtime dependencies. r[animus.command_contract.consumers]
- Add positive and negative no-std, codec, Wasm, consumer, and Cairn verification. r[animus.command_contract.verification]

## Lifecycle Prerequisite

None. Animus already runs the native `.cairn/` lifecycle with current generated policy.

## Impact

- **Core**: a new `animus-adapter-command` no-std adapter family under `crates/` owns only DTOs, validators, decisions, and BLAKE3 identities over supplied safe values.
- **Consumers**: Lattice can admit a command-bound orchestration node. The Pi tool host can require a declared schema before dispatch. Animus sessions can route one agent command to another.
- **Boundary**: no product adapter or effect moves into the family.

## Out of Scope

- Model dispatch, tool execution, effect delivery, credentials, scheduling, persistence, retries, clocks, and product semantics.
- Proving that a declared action unit is honest, that an action completed, or that the effect plan was executed.
- Replacing pi-tool-contracts, tool-executor-kit, Lattice operation contracts, or model-request snapshots.

## Affected Specs

- `animus-command-contract`: model, admission, generations, consumer mappings, and verification.
