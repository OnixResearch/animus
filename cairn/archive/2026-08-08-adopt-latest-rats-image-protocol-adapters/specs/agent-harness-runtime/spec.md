# Agent Harness Runtime

## ADDED Requirements

### Requirement: Terminal image support uses the latest reviewed Rats cohort

r[agent_harness_runtime.terminal_image_protocols.rats_cohort] Animus MUST select the canonical Rats default-branch head observed at implementation baseline time, review changes from the current pin, bind one exact immutable revision across Cargo and Nix inputs, and record the selected revision and review evidence.

#### Scenario: Latest upstream cohort is selected

r[agent_harness_runtime.terminal_image_protocols.rats_cohort.latest]
- GIVEN the canonical Rats remote default branch and the current Animus pin are available
- WHEN the dependency baseline is selected
- THEN Animus MUST fetch and record the observed upstream head
- AND every Rats Cargo and Nix reference MUST resolve to that reviewed immutable revision.

#### Scenario: Dependency references drift

r[agent_harness_runtime.terminal_image_protocols.rats_cohort.drift]
- GIVEN Cargo, Nix, generated lock state, or evidence names different Rats revisions
- WHEN the cohort rail runs
- THEN it MUST fail with a diagnostic that names each divergent reference
- AND terminal-image support MUST NOT be declared ready.

### Requirement: Terminal image authority stays in the TUI shell

r[agent_harness_runtime.terminal_image_protocols.boundary] Terminal image protocol dependencies, byte resolution, capability probing, terminal handles, writes, and cleanup MUST remain non-default host-owned behavior under `animus-tui`; reusable, default, no-std, Wasm, runtime, and Steel surfaces MUST remain terminal-image free.

#### Scenario: Host renders an approved image

r[agent_harness_runtime.terminal_image_protocols.boundary.host]
- GIVEN a host enables the terminal-image feature and supplies approved bytes, capabilities, a writer, layout, and policy
- WHEN an admitted image plan executes
- THEN the host shell MAY call the selected upstream Rats protocol adapter
- AND reusable Animus crates MUST receive no terminal handle, raw bytes, environment state, or escape sequence.

#### Scenario: Feature is absent

r[agent_harness_runtime.terminal_image_protocols.boundary.absent]
- GIVEN the terminal-image feature is disabled
- WHEN default, no-std, Wasm, runtime, Steel, or unrelated host profiles build
- THEN they MUST acquire no `rat-image`, image decoder, base64, terminal writer, environment, or filesystem implementation
- AND image transcript entries MUST remain usable as bounded metadata or text.

### Requirement: Terminal image planning is pure and bounded

r[agent_harness_runtime.terminal_image_protocols.planning] Animus MUST derive terminal image protocol, operations, geometry, bounds, fallback, image identities, and cleanup obligations through deterministic logic over explicit typed inputs and named policy values.

#### Scenario: Capabilities admit a static image

r[agent_harness_runtime.terminal_image_protocols.planning.admit]
- GIVEN valid image metadata, terminal capabilities, layout cells, policy bounds, and lifecycle state
- WHEN pure image planning runs
- THEN it MUST return an ordered bounded plan with an explicit protocol row, image identity, operations, and cleanup obligation
- AND the plan MUST be reproducible without terminal, environment, filesystem, process, network, credential, clock, or persistence access.

#### Scenario: Input exceeds a bound

r[agent_harness_runtime.terminal_image_protocols.planning.reject]
- GIVEN image bytes, encoded size, dimensions, cells, chunks, operations, or lifecycle entries exceed a named bound
- WHEN pure image planning runs
- THEN it MUST return a typed bounded rejection or admitted text fallback
- AND it MUST NOT clamp silently, allocate an unbounded buffer, or produce a protocol write plan.

### Requirement: Protocol support is operation-specific

r[agent_harness_runtime.terminal_image_protocols.protocols] Animus MUST maintain an operation-level conformance matrix for Kitty, iTerm2, Sixel, and text fallback, and MUST NOT convert one successful operation into a full-protocol claim.

#### Scenario: Kitty static lifecycle succeeds

r[agent_harness_runtime.terminal_image_protocols.protocols.kitty]
- GIVEN explicit Kitty capability evidence, admitted image bytes, valid geometry, a writer, and a fresh image identity
- WHEN the selected static lifecycle runs
- THEN it MUST perform only admitted query, transmit, placement, replacement, deletion, and cleanup operations in deterministic order
- AND it MUST report typed operation outcomes without retaining the encoded payload.

#### Scenario: Protocol capability is absent

r[agent_harness_runtime.terminal_image_protocols.protocols.fallback]
- GIVEN no admitted graphics protocol row or a policy that denies protocol output
- WHEN display planning runs
- THEN Animus MUST produce bounded alt text or a typed unsupported outcome according to policy
- AND it MUST NOT emit guessed escape sequences or write them to stdout.

#### Scenario: iTerm2 or Sixel is selected

r[agent_harness_runtime.terminal_image_protocols.protocols.alternate]
- GIVEN explicit iTerm2 or Sixel capability evidence and valid bounded image input
- WHEN the selected adapter executes
- THEN it MUST use only the admitted operation row for that protocol
- AND unsupported placement, replacement, deletion, or animation behavior MUST remain explicit.

### Requirement: Terminal image failures clean up safely

r[agent_harness_runtime.terminal_image_protocols.cleanup] Partial writes, stale identities, cancellation, replacement, layout eviction, terminal loss, and shutdown MUST produce explicit cleanup plans and typed cleanup outcomes.

#### Scenario: Transmission fails after allocation

r[agent_harness_runtime.terminal_image_protocols.cleanup.partial]
- GIVEN a protocol image identity was allocated and a later transmission or placement operation fails
- WHEN failure reduction runs
- THEN it MUST retain a bounded cleanup obligation for the exact image identity
- AND a successful render receipt MUST NOT be emitted before cleanup state is known.

#### Scenario: Cleanup fails

r[agent_harness_runtime.terminal_image_protocols.cleanup.failure]
- GIVEN deletion or terminal cleanup returns a failure
- WHEN the host records the result
- THEN the outcome MUST remain failed or cleanup-incomplete
- AND Animus MUST NOT claim that terminal resources were removed.

### Requirement: Terminal image evidence is metadata-only

r[agent_harness_runtime.terminal_image_protocols.evidence] Terminal image receipts MUST bind the selected Rats revision, policy, protocol row, safe image identity, BLAKE3 content identity, geometry, bounded counts, operation outcomes, fallback, and cleanup status without retaining raw or encoded content.

#### Scenario: Safe receipt is produced

r[agent_harness_runtime.terminal_image_protocols.evidence.safe]
- GIVEN an admitted terminal image operation reaches a terminal outcome
- WHEN receipt projection runs
- THEN it MUST emit only approved metadata, bounded counts, safe labels, BLAKE3 identities, status, and explicit non-claims
- AND receipt identity MUST be deterministic over the admitted typed fields.

#### Scenario: Receipt contains terminal content

r[agent_harness_runtime.terminal_image_protocols.evidence.leak]
- GIVEN a receipt candidate contains raw image bytes, base64, escape sequences, private paths, environment values, terminal responses, provider payloads, credentials, or unbounded text
- WHEN receipt validation runs
- THEN it MUST reject the candidate with a typed leak diagnostic
- AND the candidate MUST NOT become durable evidence.

### Requirement: Terminal image verification covers positive and negative paths

r[agent_harness_runtime.terminal_image_protocols.verification] Terminal image support MUST include positive and negative pure-planner, protocol-fixture, dependency-isolation, bounds, failure, cleanup, fallback, receipt, documentation, and lifecycle checks before archive.

#### Scenario: Change reaches closeout

r[agent_harness_runtime.terminal_image_protocols.verification.closeout]
- GIVEN implementation and focused fixtures are complete
- WHEN closeout runs
- THEN selected TUI tests, protocol fixtures, dependency guards, Nickel checks, Cairn validation, and all Cairn gates MUST pass
- AND skipped live-terminal, emulator-parity, video, audio, production, and release checks MUST remain explicit non-claims.
