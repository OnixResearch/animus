# Design: Add a command action-unit contract

## Context

Animus owns typed agent-loop fabric and adapter contracts. A command action-unit contract gives one agent a validated, reusable way to address another agent, with lifted effect intents owned by the runtime.

The family is authority-free. It validates structure and identity only; it does not execute.

## Success Contract

Given in-memory command and effect-plan facts, the family returns a deterministic admitted or blocked decision. A complete command binds one declared schema, one opaque action-unit identity, one generation, and an effect-intent-only plan.

## Decisions

### 1. Keep command identity opaque and product-neutral

`CommandId` is an opaque safe identifier. `CommandGeneration` fences replacement. `ActionUnitId` names the validated unit. `CommandRequest` binds a consumer-owned action-unit identity, declared schema descriptor, bounded input values, and expected result shape.

The family does not define model, tool, file, process, network, or provider semantics.

### 2. Model the complete-before-apply invariant structurally

The admitted command decision always includes the full post-command subject projection and the effect plan together. There is no later "apply directives" step inside the family. A command whose plan mutates the subject is rejected.

The pure decision returns the admitted command, safe issue classes, and a domain-separated BLAKE3 command identity.

### 3. Require declared effect-intent-only output

The effect plan carries typed effect intents only. Intents never write to the submitted state surface. Consumers own delivery, execution, authorization, and effect truth.

### 4. Fail closed on ambiguity and drift

Validation rejects missing, duplicate, unknown, undeclared, stale-generation, over-limit, malformed-schema, and conflicting-subject cases. An identity match does not imply execution, authorization, compatibility, or truth.

### 5. Bind replacement to generation transitions

A command replacement or handoff creates a new generation. Old-generation references cannot silently join the new command set.

Consumers decide whether to rebuild, reject, or degrade. The family only validates supplied transition facts.

### 6. Provide structural codecs only

Optional Serde and Preserves codecs encode and decode the command family. Decoded values re-enter core validation before use.

### 7. Reference the reviewed Jido discipline

Jido `cmd/2` at revision `548b2a345765ba33e687341c661bbbcbdda73d94` is a design reference for the three invariants: complete subject, effect-intent-only output, deterministic results. Animus adapts the invariants; it does not port Elixir or OTP.

## Alternatives Considered

### Reuse pi-tool-contracts as the command contract

Rejected. Tool contracts describe host-injected tool calls. A command intent describes one agent addressing another agent or a validated orchestration unit. The shapes and authority differ.

### Reuse Lattice operation contracts only

Rejected. Lattice owns durable workflow operations with product authority. The Animus family is authority-free and swarm-neutral, so agent harnesses can admit commands without a workflow runtime.

### Let every orchestrator define its own command shape

Rejected. The validation rule is product-neutral and every consumer needs compatible semantics.

## Risks and Controls

- Consumers can supply false action semantics. Non-claims state that the family validates structure only.
- Schema descriptors can drift from real dispatch. Consumers retain dispatch admission and current-head rechecks.
- Generation rollover can interrupt active work. Consumers retain transition and continuity policy.

## Non-Claims

This design does not prove action honesty, execution success, effect delivery, authorization, sandboxing, provider behavior, persistence, production readiness, or release eligibility.
