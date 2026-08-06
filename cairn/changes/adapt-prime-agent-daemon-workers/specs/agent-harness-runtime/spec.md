# Agent Harness Runtime

## ADDED Requirements

### Requirement: Daemon worker roles remain an exact optional boundary

r[agent_harness_runtime.daemon_workers.boundary] Animus MUST provide the client, supervisor, and worker role split as a focused non-default contract family pinned to the reviewed prime-agent cohort, and reusable, default, no-std, and Wasm surfaces MUST NOT acquire process, socket, filesystem, network, or persistence authority from the daemon contracts.

#### Scenario: Daemon profile is selected

r[agent_harness_runtime.daemon_workers.boundary.selected]
- GIVEN an embedder selects the reviewed daemon profile and supplies host process, transport, and persistence services
- WHEN the daemon topology is composed
- THEN Animus MAY produce role, attachment, routing, and recovery decisions
- AND the existing host-control, session-restore, std-control, background-queue, messaging, and receipt owners MUST remain authoritative.

#### Scenario: Daemon contracts expect ambient authority

r[agent_harness_runtime.daemon_workers.boundary.no_ambient]
- GIVEN a caller expects the daemon layer to spawn processes, open sockets, or write transcripts directly
- WHEN a decision runs
- THEN Animus MUST return a typed unsupported-authority outcome
- AND it MUST NOT substitute an ambient implementation.

### Requirement: Daemon policy is Nickel-authored and revalidated

r[agent_harness_runtime.daemon_workers.profile] Human-authored daemon policy MUST use typed Nickel contracts for role authority sets, attachment limits, recovery rules, versioned envelope identities, receipt fields, and required non-claims, and generated runtime policy MUST be deterministically exported and revalidated by Rust before use.

#### Scenario: Reviewed bounded policy is admitted

r[agent_harness_runtime.daemon_workers.profile.valid]
- GIVEN a checked profile names the exact cohort, role authority sets, finite attachment limits, recovery rules, and required non-claims
- WHEN Rust revalidates the generated profile
- THEN it MUST return a deterministic policy decision and domain-separated BLAKE3 policy identity
- AND no role decision, attachment, or receipt may proceed before admission.

#### Scenario: Policy drifts or leaves an unsafe default implicit

r[agent_harness_runtime.daemon_workers.profile.invalid]
- GIVEN generated policy is stale, grants a role out-of-role authority, or weakens the containment boundary
- WHEN profile validation runs
- THEN validation MUST fail closed with a bounded typed issue
- AND no daemon decision may proceed.

### Requirement: Role authority is typed and exclusive

r[agent_harness_runtime.daemon_workers.ownership] The client role MUST own only rendering, input, and local UI preferences, the supervisor role MUST own only discovery, routing, attachments, and health, the worker role MUST own exactly one root session tree with its queue, schedules, kernel handles, and descendants, and cross-role traffic MUST use versioned envelopes.

#### Scenario: Each role performs its own duties

r[agent_harness_runtime.daemon_workers.ownership.in_role]
- GIVEN admitted role instances and versioned envelopes
- WHEN a client renders, a supervisor routes, or a worker executes its session tree
- THEN each action MUST type-check against the role authority set
- AND interactive, headless, and automation clients MUST reach the same worker execution path.

#### Scenario: A role attempts out-of-role authority

r[agent_harness_runtime.daemon_workers.ownership.out_of_role]
- GIVEN a client attempts prompt execution, a supervisor attempts session execution, or a worker attempts rendering or cross-worker tree access
- WHEN conformance validation runs
- THEN it MUST fail closed with a typed outcome
- AND no effect, envelope, or receipt may follow.

### Requirement: Detach never stops execution and reattach restores state

r[agent_harness_runtime.daemon_workers.detach] Session execution MUST NOT depend on an attached client, detach MUST end only the client's subscription, and reattach MUST admit a live stream or a recovery snapshot with typed freshness.

#### Scenario: Client detaches during active work

r[agent_harness_runtime.daemon_workers.detach.execution_continues]
- GIVEN a worker with active schedules, goals, or autonomous continuations and an attached client
- WHEN the client detaches
- THEN the worker MUST continue execution unchanged
- AND the detach MUST end only the client's event subscription and command channel.

#### Scenario: Client reattaches after downtime

r[agent_harness_runtime.daemon_workers.detach.reattach]
- GIVEN a running or recovered worker and a reattaching client
- WHEN reattach admission runs
- THEN the client MUST receive a live stream or a recovery snapshot with typed freshness
- AND a stale snapshot MUST be rejected or repaired through the session-restore contract before use.

### Requirement: Supervisor duties are typed decisions

r[agent_harness_runtime.daemon_workers.supervisor] Supervisor discovery, routing, attachment management, worker health classification, and catalog scans MUST be pure decisions over supplied state, and the supervisor MUST NOT execute session work.

#### Scenario: Supervisor routes a command

r[agent_harness_runtime.daemon_workers.supervisor.routing]
- GIVEN a versioned command envelope and supplied worker registry state
- WHEN the routing decision runs
- THEN it MUST return the target worker or a typed unknown-target outcome
- AND identical inputs MUST produce identical decisions.

#### Scenario: Supervisor attempts session execution

r[agent_harness_runtime.daemon_workers.supervisor.no_execution]
- GIVEN any path where the supervisor executes prompts, tools, or schedules directly
- WHEN conformance validation runs
- THEN it MUST fail closed with a typed outcome
- AND supervisor failure MUST degrade routing and attachment only, never worker execution.

### Requirement: Process isolation is lifecycle containment, not sandboxing

r[agent_harness_runtime.daemon_workers.containment] Worker and kernel process isolation MUST be documented and receipted as lifecycle and failure containment only, and no contract, receipt, or projection MAY claim security sandboxing, confinement, or attack-surface reduction from process isolation.

#### Scenario: Containment receipt states lifecycle properties

r[agent_harness_runtime.daemon_workers.containment.lifecycle_only]
- GIVEN worker or kernel process events complete under policy
- WHEN receipt projection runs
- THEN the receipt MUST state containment as crash-domain, teardown, and restart facts
- AND it MUST carry the explicit non-sandbox non-claim.

#### Scenario: Sandbox claim is made

r[agent_harness_runtime.daemon_workers.containment.no_sandbox_claim]
- GIVEN a contract, receipt, or projection claims sandboxing, confinement, or security isolation from process separation
- WHEN validation runs
- THEN it MUST reject the claim with a bounded typed issue
- AND the claim MUST NOT contribute to lifecycle, completion, or release evidence.

### Requirement: Daemon evidence is metadata-only

r[agent_harness_runtime.daemon_workers.evidence] Daemon receipts MUST bind only policy identity, role, envelope identity, attachment class, recovery class, health class, bounded counts, required non-claims, and domain-separated BLAKE3 identities, and MUST exclude transcript content, prompt bodies, and client input.

#### Scenario: Safe daemon receipt is projected

r[agent_harness_runtime.daemon_workers.evidence.safe]
- GIVEN attach, detach, recovery, routing, or health events complete under policy
- WHEN receipt projection runs
- THEN the receipt MUST bind safe identities, classes, counts, and required non-claims
- AND it MUST describe only bounded topology and lifecycle facts.

#### Scenario: Receipt contains content or overclaims

r[agent_harness_runtime.daemon_workers.evidence.rejects]
- GIVEN a receipt draft includes transcript content, prompt bodies, client input, or a sandbox, crash-freedom, or durability claim
- WHEN receipt validation runs
- THEN it MUST reject the draft with a bounded typed issue without echoing the material
- AND the rejected receipt MUST NOT contribute to lifecycle, completion, or release evidence.

### Requirement: Daemon adoption has positive and negative conformance evidence

r[agent_harness_runtime.daemon_workers.verification] The daemon contract MUST include positive and negative tests through production role, attachment, recovery, supervisor, and receipt entrypoints; cohort fixture parity; default and no-std dependency isolation; and a focused deterministic rail.

#### Scenario: Exact fixture cohort passes

r[agent_harness_runtime.daemon_workers.verification.positive]
- GIVEN fixtures cover role authority, attach/detach/reattach cycles, snapshot recovery, supervisor routing, and single-tree failure containment
- WHEN the focused conformance rail runs
- THEN production entrypoints MUST agree with the reviewed prime-agent semantics for the admitted subset
- AND fixture provenance and any semantic limitation MUST remain explicit.

#### Scenario: Adversarial fixture fails before widening

r[agent_harness_runtime.daemon_workers.verification.negative]
- GIVEN a fixture attempts out-of-role authority, cross-worker access, stale snapshot use, sandbox claims, receipt content leakage, or authority widening
- WHEN the focused conformance rail runs
- THEN the fixture MUST fail with the expected bounded typed diagnostic before the prohibited effect or claim
- AND default, reusable, no-std, and Wasm surfaces MUST remain free of new effect authority.

#### Scenario: Change is ready to archive

r[agent_harness_runtime.daemon_workers.verification.closeout]
- GIVEN maintainers intend to sync and archive the daemon change
- WHEN closeout validation runs
- THEN focused daemon, runtime, Nickel, no-std/Wasm, component/facade, composition, safe-receipt, Cairn, and relevant Nix checks MUST pass with current positive and negative fixtures
- AND security sandboxing, crash-freedom, multi-machine supervision, production readiness, and release eligibility MUST remain separate non-claims.
