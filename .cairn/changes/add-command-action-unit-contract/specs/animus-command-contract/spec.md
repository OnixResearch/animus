# Animus Command Contract

## ADDED Requirements

### Requirement: Command contracts are authority-free

r[animus.command_contract.boundary] The command action-unit family MUST validate command identity, declared schema, complete-before-apply structure, effect-intent-only output, and generation facts without model, tool, process, filesystem, network, clock, terminal, provider, persistence, or product mutation effects.

#### Scenario: Supplied command facts are validated

- GIVEN a consumer supplies in-memory command, schema, effect-plan, and generation values
- WHEN the family validates them
- THEN it MUST return deterministic structural facts or typed issues
- AND it MUST perform no host effect.

### Requirement: Commands use typed bounded values

r[animus.command_contract.model] The family MUST define opaque command identity, action-unit identity, declared schema descriptor, generation, request, result, decision, issue, transition, limit, and metadata-only receipt values.

#### Scenario: Complete command is represented

- GIVEN a bounded command binds one action-unit identity, one declared schema, and bounded input values
- WHEN model validation runs
- THEN the family MUST admit the typed structure for evaluation
- AND no value may contain a service handle or product payload.

#### Scenario: Duplicate command is rejected

- GIVEN a command set repeats one action-unit identity under one generation
- WHEN model validation runs
- THEN the family MUST reject the set.

### Requirement: Admission fails closed on mismatch

r[animus.command_contract.admission] A valid command MUST pair an admitted declared schema with an exact bounded input set, one current generation, a complete post-command subject projection, and an effect-intent-only plan. Missing, duplicate, undeclared, mixed, stale, malformed, over-limit, or conflicting facts MUST fail closed.

#### Scenario: Complete command is admitted

- GIVEN the command cites the exact current generation and a declared schema, and the inputs satisfy the schema
- WHEN command admission runs
- THEN it MUST return an admitted command with a post-command subject projection, its effect-intent-only plan, and a deterministic BLAKE3 command identity.

#### Scenario: Plan claims to mutate the subject

- GIVEN an effect plan declares a write to the submitted state surface instead of an effect intent
- WHEN command admission runs
- THEN the family MUST reject the plan.

#### Scenario: Schema is undeclared

- GIVEN a request cites an action-unit identity with no declared schema descriptor
- WHEN command admission runs
- THEN the family MUST return an undeclared-schema denial.

### Requirement: Command replacement is generation-fenced

r[animus.command_contract.generation] Command replacement and handoff facts MUST create an explicit new generation. Old-generation references MUST NOT silently join the replacement command set.

#### Scenario: Replacement generation is current

- GIVEN a consumer supplies an admitted predecessor, a distinct next generation, and exact replacement command facts
- WHEN transition validation runs
- THEN the family MAY return a structurally valid transition
- AND continuity and effect decisions MUST remain consumer-owned.

#### Scenario: Old reference is reused silently

- GIVEN a replacement command set includes a reference bound only to the prior generation
- WHEN transition validation runs
- THEN the family MUST reject the set.

### Requirement: Consumer mappings retain product ownership

r[animus.command_contract.consumers] Lattice, Pi tool hosts, and other consumers MUST retain action truth, dispatch, execution, scheduling, effect delivery, authorization, credentials, lifecycle, persistence, evidence, and product claims.

#### Scenario: Lattice admits a command-bound node

- GIVEN Lattice supplies bounded command and current-generation facts for one orchestration node
- WHEN the mapping fixture runs
- THEN structural admission MAY succeed
- AND the family MUST gain no Lattice effect, persistence, or scheduling authority.

#### Scenario: Pi tool host dispatches by declared schema

- GIVEN a Pi tool host supplies a declared schema and bounded inputs
- WHEN the mapping fixture runs
- THEN the family MAY admit the command
- AND the host MUST NOT execute or authorize the command.

### Requirement: Command contracts have positive and negative verification

r[animus.command_contract.verification] The focused rail MUST cover valid complete commands, replacement generations, codecs, consumer mappings, and negative missing, duplicate, undeclared, mixed, stale, malformed, over-limit, plan-mutation, and authority-overclaim cases.

#### Scenario: Command-contract rail passes

- GIVEN all required no-std, Wasm, codec, consumer, positive, and negative fixtures are present
- WHEN the focused rail runs
- THEN structural decisions MUST be deterministic
- AND invalid facts MUST fail without effects or promoted claims.
