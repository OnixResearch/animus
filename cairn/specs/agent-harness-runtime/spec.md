# Agent Harness Runtime Specification

## Purpose

Defines the accepted baseline for Animus as a modular Rust agent-harness framework with explicit crate barriers, typed adapter contracts, a Steel decision fabric, and standard-library host shells.

## Requirements

### Requirement: Crate boundary baseline

r[agent_harness_runtime.baseline.crate_boundaries] The workspace MUST preserve explicit crate boundaries between pure core state, implementation-free adapter contracts, optional Preserves boundary serialization, Steel turn decisions, standard-library host orchestration, optional terminal UI hosting, and the root compatibility facade.

#### Scenario: Embedders choose the smallest surface

r[agent_harness_runtime.baseline.crate_boundaries.smallest_dependency]
- GIVEN an embedder needs only deterministic state logic, adapter DTOs, serialization, Steel scripting, std shell orchestration, or TUI hosting
- WHEN they select workspace crates
- THEN each reusable crate exposes only its owned boundary
- AND concrete host, provider, filesystem, process, network, and UI behavior stays outside unrelated reusable crates.

### Requirement: Functional core and imperative shell split

r[agent_harness_runtime.baseline.functional_core] Non-trivial Animus behavior MUST be decomposed into pure deterministic core logic over explicit inputs and thin imperative shells for filesystem, process, environment, network, terminal, credential, clock, and persistence effects.

#### Scenario: Core logic is testable without standing up the world

r[agent_harness_runtime.baseline.functional_core.testable]
- GIVEN runtime, policy, adapter, prompt, provenance, or serialization logic contains branch decisions
- WHEN maintainers add or change that behavior
- THEN the branch decisions are represented as deterministic functions over in-memory values
- AND tests can exercise positive and negative cases without real providers, shells, terminals, credentials, or storage backends.

### Requirement: Adapter-first integration boundary

r[agent_harness_runtime.baseline.adapters] Model, tool, memory, ledger, queue, policy, confirmation, provider, workspace, and host integrations MUST cross typed adapter or service contracts rather than being constructed by reusable runtime core code.

#### Scenario: Missing host services fail closed

r[agent_harness_runtime.baseline.adapters.missing_service]
- GIVEN reusable runtime logic requires a host capability
- WHEN the corresponding adapter or service is absent, denied, unavailable, malformed, or cancelled
- THEN runtime returns a typed failure before claiming completed effects
- AND it does not discover ambient config, read credentials, spawn commands, or contact providers as a fallback.

### Requirement: Runtime dispatch uses injected services

r[agent_harness_runtime.clankers_injected_services.contracts] Animus runtime dispatch MUST consume injected model/provider, tool, host-effect, capability, cancellation, confirmation, and runtime-policy services through typed contracts rather than constructing concrete clients or reading ambient host state.

#### Scenario: Service contracts are explicit

r[agent_harness_runtime.clankers_injected_services.contracts.explicit]
- GIVEN runtime code needs model, provider, tool, host-effect, confirmation, cancellation, or policy behavior
- WHEN the boundary is reviewed
- THEN the behavior is represented by an injected trait, adapter, or typed service handle
- AND reusable runtime code does not depend on concrete provider clients, credential stores, shell tools, plugin runtimes, or host persistence.

### Requirement: Missing or denied services fail closed

r[agent_harness_runtime.clankers_injected_services.fail_closed] Missing, unavailable, denied, cancelled, timed-out, malformed-output, and completed service outcomes MUST be typed and MUST prevent false success receipts or later side effects when the outcome is not completed.

#### Scenario: Non-completed services block effects

r[agent_harness_runtime.clankers_injected_services.fail_closed.blocks]
- GIVEN a runtime action requires an injected service
- WHEN the service is absent, unavailable, denied, cancelled, timed out, or returns malformed output
- THEN runtime returns the corresponding typed outcome
- AND no receipt or provenance fact claims the effect completed.

### Requirement: Confirmation denies by default

r[agent_harness_runtime.clankers_injected_services.confirmation] Protected actions that require confirmation MUST deny by default when no broker is injected or when the broker reports unavailable, timed-out, or cancelled.

#### Scenario: Missing confirmation broker prevents protected action

r[agent_harness_runtime.clankers_injected_services.confirmation.default_deny]
- GIVEN a protected action requires human or host confirmation
- WHEN no confirmation broker is available
- THEN runtime records a denied decision
- AND the protected side effect is not executed.

### Requirement: Provider boundary forbids ambient clients

r[agent_harness_runtime.clankers_injected_services.provider_boundary] Provider dispatch tests or rails MUST prove reusable runtime code does not construct provider clients, read credentials, discover provider config, or open network connections directly.

#### Scenario: Provider dispatch goes through the injected service

r[agent_harness_runtime.clankers_injected_services.provider_boundary.injected]
- GIVEN a model/provider request is dispatched
- WHEN focused boundary tests run
- THEN the runtime calls only the injected service with a validated adapter-neutral request
- AND ambient credential lookup, provider discovery, network opening, and concrete client construction are absent from reusable runtime code.

### Requirement: Tool and effect boundaries fail before unsafe execution

r[agent_harness_runtime.clankers_injected_services.tool_boundary] Tool/effect dispatch tests MUST cover missing executor, unavailable service, capability denial, cancellation, timeout, and malformed output.

#### Scenario: Tool denial stops the turn safely

r[agent_harness_runtime.clankers_injected_services.tool_boundary.denied]
- GIVEN a loop decision requests a tool or effect denied by capability policy
- WHEN dispatch evaluates the decision
- THEN a typed capability-denied outcome is returned
- AND later model, provider, or tool calls for that denied effect are not executed.

### Requirement: Effect receipts are redacted

r[agent_harness_runtime.clankers_injected_services.receipts] Service and effect receipts MUST record safe outcome classes and MUST omit raw provider bodies, credentials, environment values, full paths, raw tool arguments, and raw tool output.

#### Scenario: Injected-service closeout is evidenced

r[agent_harness_runtime.clankers_injected_services.closeout]
- GIVEN the injected service change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes provider-boundary tests, tool/effect negative tests, receipt redaction tests, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Safe receipt DTOs

r[agent_harness_runtime.clankers_safe_receipts.dto] Animus runtime and provenance receipts MUST expose safe metadata only, including schema, status, issue code, labels, bounded counts, BLAKE3 hashes, effect or mutation kind, backend/interchange labels, and explicit redaction markers.

#### Scenario: Positive receipts expose only safe evidence

r[agent_harness_runtime.clankers_safe_receipts.positive]
- GIVEN model, tool, authorization, runtime block, final response, adapter error, or provenance observations contain raw sensitive inputs and outputs
- WHEN safe receipts are constructed
- THEN receipts expose only safe labels, bounded counts, status, issue class, redaction markers, and BLAKE3 hashes
- AND the raw sensitive material is omitted.

### Requirement: Receipt leak rejection

r[agent_harness_runtime.clankers_safe_receipts.rejects_leaks] Receipt validation MUST reject or redact raw prompt text, provider JSON, credential-looking values, absolute path dumps, raw tool input, raw tool output, raw model output, and unbounded output before evidence is accepted.

#### Scenario: Leak fixtures fail closed

r[agent_harness_runtime.clankers_safe_receipts.rejects_leaks.fixture]
- GIVEN a receipt fixture includes raw prompt text, provider JSON, token-like values, absolute paths, raw tool/model output, or an oversized body
- WHEN receipt validation runs
- THEN validation returns a typed redaction error or redacted output
- AND the unsafe fixture is not accepted as review or release evidence.

### Requirement: BLAKE3 canonical artifact envelopes

r[agent_harness_runtime.clankers_artifact_identity.envelope] Animus-owned artifact identity MUST use BLAKE3 over versioned canonical envelopes containing artifact kind, envelope version, redaction class, normalized payload, and sorted/deduplicated dependency hashes.

#### Scenario: Artifact identity is deterministic

r[agent_harness_runtime.clankers_artifact_identity.deterministic]
- GIVEN equivalent artifact envelopes with dependencies provided in different orders and with duplicates
- WHEN artifact identity is computed
- THEN dependencies are sorted and deduplicated before hashing
- AND the resulting BLAKE3 hash is stable and round-trips through display/parse validation.

### Requirement: Redaction classes govern review output

r[agent_harness_runtime.clankers_artifact_identity.redaction] Artifact redaction classes MUST determine whether payload bodies may appear in receipts, CLI, or review output.

#### Scenario: Non-public artifacts hide payloads

r[agent_harness_runtime.clankers_artifact_identity.redaction.display]
- GIVEN an artifact is metadata-only, redacted-payload, or secret
- WHEN a receipt or review output is rendered
- THEN payload bodies are omitted or replaced with explicit redaction markers
- AND only public artifacts may expose payload bodies by default.

### Requirement: Focused receipt and artifact rails

r[agent_harness_runtime.clankers_safe_receipts.rails] Animus MUST provide focused verification rails for safe receipt validation, leak rejection, artifact hash determinism, and redaction-class display behavior before claiming the Clankers receipt/artifact pattern adopted.

#### Scenario: Receipt and artifact closeout is evidenced

r[agent_harness_runtime.clankers_safe_receipts.closeout]
- GIVEN the safe receipt and artifact identity change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes focused receipt/artifact rails, positive and negative tests, dependency checks, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Animus-owned neutral ledger DTOs

r[agent_harness_runtime.clankers_neutral_ledger.dto] Animus MUST define its own neutral session-ledger DTOs for model-visible messages, summaries, usage observations, safe receipts, unsupported entries, replay metadata, and typed replay errors without depending on Clankers session, daemon, TUI, provider, Automerge, or concrete storage types.

#### Scenario: DTOs stay storage-neutral

r[agent_harness_runtime.clankers_neutral_ledger.dto.storage_neutral]
- GIVEN reusable Animus ledger DTOs are reviewed
- WHEN dependency and field boundaries are inspected
- THEN the DTOs do not require concrete storage, filesystem paths, wall-clock construction, daemon events, TUI types, provider-native payloads, or Clankers transcript IDs
- AND host persistence can wrap them without changing pure replay semantics.

### Requirement: Pure deterministic replay

r[agent_harness_runtime.clankers_neutral_ledger.replay] Ledger replay MUST be a pure deterministic function over in-memory entries that returns model-visible context plus safe metadata counts or a typed error.

#### Scenario: Supported entries replay in order

r[agent_harness_runtime.clankers_neutral_ledger.replay_positive]
- GIVEN an in-memory ledger containing user, assistant, tool, summary, usage, and safe receipt entries
- WHEN replay runs
- THEN model-visible user, assistant, tool, and summary context is returned in deterministic order
- AND usage and receipt entries contribute safe metadata counts without leaking raw storage details.

### Requirement: Unsupported entries fail closed

r[agent_harness_runtime.clankers_neutral_ledger.unsupported] Replay MUST fail closed on unsupported or adapter-owned ledger entries rather than silently dropping or exposing them.

#### Scenario: Unsupported payloads do not leak

r[agent_harness_runtime.clankers_neutral_ledger.unsupported.redacted]
- GIVEN replay encounters an unsupported or adapter-owned entry
- WHEN replay is required before a model request
- THEN replay returns a typed unsupported-entry error
- AND the error includes only a safe reason class, not the raw unsupported payload.

### Requirement: Resume requires an injected ledger service

r[agent_harness_runtime.clankers_neutral_ledger.missing_service] Runtime paths that require session resume MUST consume an injected ledger service and MUST fail before adapter dispatch when the service or requested session is unavailable.

#### Scenario: Missing ledger service blocks dispatch

r[agent_harness_runtime.clankers_neutral_ledger.missing_service.blocks]
- GIVEN a runtime request requires resume semantics
- WHEN no ledger service is injected or the requested session is missing
- THEN runtime returns a typed unavailable-service or missing-session failure
- AND no model or tool adapter call is made.

### Requirement: Pure branch query helpers when parent links exist

r[agent_harness_runtime.clankers_neutral_ledger.branch_queries] If Animus ledger entries include parent/head relationships, branch query helpers MUST be pure and MUST detect duplicate IDs, missing parents, cycles, active branch ancestry, and abandoned/dead-end attempts.

#### Scenario: Malformed graphs fail without panic

r[agent_harness_runtime.clankers_neutral_ledger.branch_queries.invalid]
- GIVEN a ledger graph has duplicate IDs, a missing parent, a cycle, or reversed-order assumptions
- WHEN query preparation runs
- THEN it returns a typed graph validation error
- AND no panic, infinite traversal, or silent ancestry corruption occurs.

#### Scenario: Ledger closeout is evidenced

r[agent_harness_runtime.clankers_neutral_ledger.closeout]
- GIVEN the neutral ledger change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes focused replay tests, unsupported-entry negative tests, missing-service tests, dependency-isolation checks, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Steel decision fabric baseline

r[agent_harness_runtime.baseline.steel_fabric] Animus SHOULD use Steel Scheme as the default scriptable turn-decision fabric while Rust boundary code MUST revalidate every selected model, tool, memory, response, or host action before dispatch.

#### Scenario: Script decisions are revalidated

r[agent_harness_runtime.baseline.steel_fabric.revalidated]
- GIVEN a Steel script returns a loop decision
- WHEN Rust runtime code receives that decision
- THEN the decision is decoded through typed boundary DTOs
- AND malformed payloads, unsupported actions, unauthorized effects, or invalid text fail closed before adapter dispatch.

### Requirement: Nickel-owned repository configuration

r[agent_harness_runtime.baseline.nickel_config] Repo-owned tool and policy configuration MUST use Nickel as the source of truth when Animus owns the config shape, while generated tool-native artifacts are kept fresh for consumers that require JSON, TOML, YAML, or other native formats.

#### Scenario: Generated config drift is detectable

r[agent_harness_runtime.baseline.nickel_config.drift]
- GIVEN maintainers change repo-owned Nickel configuration
- WHEN generated native config files are checked
- THEN drift is reported deterministically
- AND stale generated config is not treated as authoritative evidence.

### Requirement: Verification safety rails

r[agent_harness_runtime.baseline.verification] The workspace MUST maintain focused positive and negative tests, formatting, linting, Tiger Style, generated-config drift checks, feature checks, and lifecycle validation evidence for behavior that crosses public API, adapter, policy, serialization, or host boundaries.

#### Scenario: Boundary changes include reviewable evidence

r[agent_harness_runtime.baseline.verification.evidence]
- GIVEN a boundary-affecting change is ready for review
- WHEN maintainers inspect its evidence
- THEN the evidence names the focused checks that cover the changed boundary
- AND unavailable checks are recorded with a blocker and next best validation path.

### Requirement: Focused rail convention

r[agent_harness_runtime.clankers_focused_rails.convention] Animus MUST define a focused verification rail convention where each rail validates one boundary, uses deterministic fixtures, emits safe versioned evidence, and avoids live credentials or host-specific state by default.

#### Scenario: Rails have bounded evidence

r[agent_harness_runtime.clankers_focused_rails.convention.evidence]
- GIVEN a focused rail validates an Animus boundary
- WHEN the rail writes evidence
- THEN evidence includes schema, status, fixture names, hashed artifacts, and safe metadata
- AND it omits raw prompts, credentials, provider payloads, full paths, raw tool output, and live host state.

### Requirement: Lifecycle documentation rail

r[agent_harness_runtime.clankers_focused_rails.lifecycle] Animus MUST provide a lifecycle-doc marker rail that checks boundary names, owner markers, forbidden dependency sections, and validation command references.

#### Scenario: Lifecycle rail catches drift

r[agent_harness_runtime.clankers_focused_rails.lifecycle.drift]
- GIVEN the lifecycle document loses a required boundary, owner, forbidden dependency section, or validation marker
- WHEN the lifecycle rail runs
- THEN it fails with an actionable diagnostic
- AND it does not require provider credentials or live services.

### Requirement: Steel runtime policy rail

r[agent_harness_runtime.clankers_focused_rails.steel_policy] Animus MUST provide a focused Steel runtime policy rail that covers Nickel-authored policy export drift, generated-policy activation, current Steel activation, Rust adapter-boundary revalidation, invalid Steel program rejection before adapter dispatch, and an explicit policy-drift hook when a canonical policy source is supplied.

#### Scenario: Steel rail covers positive and negative activation

r[agent_harness_runtime.clankers_focused_rails.steel_policy.fixtures]
- GIVEN current positive and negative Steel runtime fixtures exist
- WHEN the Steel runtime policy rail runs
- THEN valid Steel programs activate safely through Rust DTO validation
- AND invalid Steel programs fail before model or tool adapter dispatch
- AND policy-drift checking is either executed against an explicit canonical source or reported as skipped with a next-best validation path.

### Requirement: Nickel-authored Steel runtime policy

r[agent_harness_runtime.clankers_steel_policy.nickel] Animus Steel runtime profiles MUST be authored in Nickel with typed contracts for profile identity, script bindings, BLAKE3 script hashes, runtime budgets, allowed host actions, fallback mode, and receipt redaction.

#### Scenario: Policy contract accepts valid profiles

r[agent_harness_runtime.clankers_steel_policy.nickel.valid]
- GIVEN a maintained Steel runtime profile under `policy/steel-runtime/`
- WHEN Nickel validation runs
- THEN valid profile identity, script binding, hash, budget, host-action, fallback, and receipt-redaction fields pass
- AND the policy remains reviewable before runtime use.

### Requirement: Checked export and invalid fixtures

r[agent_harness_runtime.clankers_steel_policy.export] Rust runtime tests MUST consume checked policy data or typed fixtures and MUST NOT evaluate Nickel during turn execution.

#### Scenario: Policy export drift fails

r[agent_harness_runtime.clankers_steel_policy.export.drift]
- GIVEN generated policy data exists for Rust consumption
- WHEN the drift check runs
- THEN it fails if generated data differs from Nickel source
- AND it reports a deterministic update path.

#### Scenario: Invalid policies fail before activation

r[agent_harness_runtime.clankers_steel_policy.invalid_fixtures]
- GIVEN a policy fixture has a missing script, bad name, non-positive budget, unknown host action, hash mismatch, path escape, or unsupported fallback mode
- WHEN policy validation runs
- THEN validation returns a typed policy error
- AND Steel is not loaded or evaluated.

### Requirement: Policy enforcement before Steel evaluation

r[agent_harness_runtime.clankers_steel_policy.enforced] Animus MUST validate selected profile, script hash, runtime budget, action allowlist, fallback mode, and receipt redaction before Steel loop evaluation can influence model or tool dispatch.

#### Scenario: Policy denial blocks adapter calls

r[agent_harness_runtime.clankers_steel_policy.enforced.blocks]
- GIVEN a runtime turn selects a Steel script that fails policy activation
- WHEN the runtime prepares the turn
- THEN it returns a typed policy-denied failure
- AND no model or tool adapter call is made.

### Requirement: Redacted policy receipts and rails

r[agent_harness_runtime.clankers_steel_policy.receipts] Steel policy activation receipts MUST expose safe status, profile labels, script hash, policy hash, denied reason class, fallback decision, and receipt hash only.

#### Scenario: Receipts omit raw sensitive material

r[agent_harness_runtime.clankers_steel_policy.receipts.redacted]
- GIVEN activation observes prompt text, script source, provider payloads, credentials, or tool bodies
- WHEN a receipt is rendered
- THEN those values are omitted or represented by redaction markers and BLAKE3 hashes
- AND the receipt remains usable as review evidence.

#### Scenario: Focused rails gate closeout

r[agent_harness_runtime.clankers_steel_policy.rails]
- GIVEN maintainers claim Steel runtime policy is adopted
- WHEN focused policy drift and activation rails run
- THEN they cover valid policy, invalid fixtures, enforcement-before-evaluation, and receipt redaction
- AND closeout evidence names the successful rails or explicit blockers.

#### Scenario: Steel policy closeout is evidenced

r[agent_harness_runtime.clankers_steel_policy.closeout]
- GIVEN the Steel policy change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes focused policy rails, Steel tests, Nickel/config drift checks, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Receipt, artifact, ledger, and service rails

r[agent_harness_runtime.clankers_focused_rails.receipts_artifacts] Animus MUST provide focused rails for safe receipt validation, leak rejection, BLAKE3 artifact determinism, and redaction-class display behavior.

r[agent_harness_runtime.clankers_focused_rails.ledger] Animus MUST provide focused rails for neutral ledger replay, unsupported-entry failure, safe metadata counts, and missing ledger service behavior.

r[agent_harness_runtime.clankers_focused_rails.injected_services] Animus MUST provide focused rails or tests for injected provider/tool service boundaries, absence of ambient provider behavior, and fail-closed missing/denied/cancelled/malformed outcomes.

#### Scenario: Boundary rails cover negative cases

r[agent_harness_runtime.clankers_focused_rails.boundary_negative]
- GIVEN receipt, artifact, ledger, or injected-service boundary fixtures include unsafe or unavailable cases
- WHEN focused rails run
- THEN each unsafe or unavailable case returns a typed failure or redacted result
- AND no rail accepts leaked sensitive material or false success evidence.

### Requirement: Stable rails wire into Nix checks

r[agent_harness_runtime.clankers_focused_rails.nix_wiring] Stable credential-free focused rails SHOULD be wired into named Nix checks or the default flake check once deterministic; rails not wired into Nix MUST document the blocker.

#### Scenario: Closeout separates evidence from non-claims

r[agent_harness_runtime.clankers_focused_rails.closeout]
- GIVEN a Clankers-pattern adoption is ready for archive
- WHEN closeout evidence is prepared
- THEN it names successful focused rails, workspace tests, clippy, Nickel/config drift checks when applicable, Cairn validation, and Cairn gates
- AND it explicitly identifies skipped rails and non-claims such as live providers, TUI dogfood, daemon tests, VM checks, or production readiness when not covered.

### Requirement: Roadmap taxonomy

r[agent_harness_runtime.roadmap_sequence.taxonomy] Animus MUST classify active Cairn changes by roadmap role, including foundational lifecycle, core-runtime contract, host-adapter, optional-reference, product-surface, and evaluation-only roles.

#### Scenario: Active changes have roadmap roles

r[agent_harness_runtime.roadmap_sequence.taxonomy.classified]
- GIVEN the Cairn tree contains active changes
- WHEN the roadmap classification is checked
- THEN each active change has exactly one primary role
- AND optional secondary relationships are represented as explicit prerequisite or related-change links.

### Requirement: Dependency order

r[agent_harness_runtime.roadmap_sequence.dependency_order] The roadmap MUST define prerequisite tiers that land accepted baseline specs and local validation wiring before runtime behavior, reusable runtime contracts before concrete host adapters, and core safety contracts before product surfaces.

#### Scenario: Dependent change waits for prerequisite contracts

r[agent_harness_runtime.roadmap_sequence.dependency_order.blocks_dependents]
- GIVEN a change depends on runtime loop, injected services, receipts, ledger, event, policy, or host-adapter contracts
- WHEN the prerequisite requirement IDs are not accepted, not in the same archive batch, and not explicitly waived
- THEN roadmap validation reports the dependency as blocked
- AND the dependent change is not marked archive-ready.

### Requirement: Overlap resolution

r[agent_harness_runtime.roadmap_sequence.overlap_resolution] The roadmap MUST assign ownership for overlapping ledger, receipt, provider, hook, manifest, catalog, and tool-contract concerns so duplicate changes converge on shared requirement IDs instead of creating incompatible contracts.

#### Scenario: Overlapping changes share owners

r[agent_harness_runtime.roadmap_sequence.overlap_resolution.shared_owner]
- GIVEN two active changes describe similar ledger, receipt, provider, hook, manifest, catalog, or tool behavior
- WHEN maintainers classify the changes
- THEN one owning concept and prerequisite path is recorded
- AND the non-owning change links to that owner or narrows itself to evaluation-only scope.

### Requirement: Roadmap validation

r[agent_harness_runtime.roadmap_sequence.validation] Animus SHOULD provide a deterministic roadmap fixture or static check that detects unknown change IDs, unknown prerequisite requirement IDs, dependency cycles, unclassified active changes, and product-surface changes that bypass foundational safety contracts.

#### Scenario: Invalid roadmap fails closed

r[agent_harness_runtime.roadmap_sequence.validation.negative]
- GIVEN roadmap fixtures include an unknown change ID, unknown prerequisite ID, dependency cycle, unclassified change, or unsafe product-surface shortcut
- WHEN roadmap validation runs
- THEN validation fails with deterministic issue codes
- AND the diagnostics do not depend on filesystem ordering or ambient state.

### Requirement: Roadmap closeout evidence

r[agent_harness_runtime.roadmap_sequence.closeout] Roadmap sequencing work MUST produce reviewable evidence that lifecycle validation, roadmap validation, and Cairn gates pass before the change is archived.

#### Scenario: Sequencing evidence is reviewable

r[agent_harness_runtime.roadmap_sequence.closeout.evidence]
- GIVEN the roadmap sequencing change is ready to archive
- WHEN reviewers inspect the evidence
- THEN the evidence names successful lifecycle validation, focused roadmap checks, and Cairn proposal/design/tasks gates
- AND any unavailable check has a blocker and a next best validation path.

### Requirement: Generated policy artifact

r[agent_harness_runtime.cairn_policy.generated_artifact] Animus MUST provide repo-local Cairn generated policy data at the path expected by default Cairn runtime validation, or an equivalent repo-owned wrapper that resolves the policy without hidden session-specific absolute paths.

#### Scenario: Default validation finds policy

r[agent_harness_runtime.cairn_policy.generated_artifact.default_path]
- GIVEN a developer runs Cairn validation from the Animus repository root
- WHEN no explicit external policy path is supplied
- THEN validation loads the repo-owned policy artifact or wrapper-resolved policy
- AND it does not depend on a private checkout path.

### Requirement: Policy refresh documentation

r[agent_harness_runtime.cairn_policy.refresh_docs] Animus MUST document the policy source, refresh command, check command, and expected default validation command for Cairn lifecycle artifacts.

#### Scenario: Maintainer refreshes policy intentionally

r[agent_harness_runtime.cairn_policy.refresh_docs.reproducible]
- GIVEN the canonical Cairn policy source changes
- WHEN a maintainer refreshes Animus policy wiring
- THEN the documentation names the source identity and commands used
- AND the generated artifact can be reviewed as an intentional lifecycle dependency update.

### Requirement: Policy drift check

r[agent_harness_runtime.cairn_policy.drift_check] Animus SHOULD provide a deterministic policy drift check or documented fixture that compares the checked-in generated policy to the canonical exported policy when the source is available.

#### Scenario: Stale policy is detected

r[agent_harness_runtime.cairn_policy.drift_check.stale]
- GIVEN the checked-in generated policy differs from the canonical exported policy
- WHEN the drift check runs with the source available
- THEN the check fails with a deterministic diagnostic
- AND it reports the source identity without embedding secrets or local-only full paths in durable evidence.

### Requirement: Default Cairn validation

r[agent_harness_runtime.cairn_policy.default_validation] Animus lifecycle validation MUST be runnable through the default Cairn validation command from the repository root without requiring a developer to remember an external policy argument.

#### Scenario: Validation is local and reproducible

r[agent_harness_runtime.cairn_policy.default_validation.local]
- GIVEN the repository contains lifecycle artifacts and policy wiring
- WHEN `cairn validate --root .` or the pinned equivalent runs
- THEN validation succeeds or fails only on lifecycle/policy content
- AND missing external checkout paths are not reported as validation blockers.

### Requirement: Policy evidence

r[agent_harness_runtime.cairn_policy.evidence] Cairn policy validation evidence SHOULD record policy source identity, policy hash when available, validation command, and validation result.

#### Scenario: Reviewers can audit policy used for validation

r[agent_harness_runtime.cairn_policy.evidence.reviewable]
- GIVEN lifecycle validation evidence is attached to a change
- WHEN reviewers inspect it
- THEN the evidence identifies which policy artifact was used
- AND it includes enough stable identity to reproduce or challenge the validation result.

### Requirement: Policy wiring closeout

r[agent_harness_runtime.cairn_policy.closeout] Policy wiring work MUST run default validation, explicit-policy validation when available, and Cairn proposal/design/tasks gates before archive.

#### Scenario: Policy wiring closeout is reviewable

r[agent_harness_runtime.cairn_policy.closeout.evidence]
- GIVEN the policy wiring change is ready to archive
- WHEN reviewers inspect closeout evidence
- THEN default validation, explicit-policy validation, and relevant Cairn gates are named with their status
- AND unavailable checks include a blocker and next best validation path.

### Requirement: Lifecycle layout

r[agent_harness_runtime.cairn_dogfood.lifecycle_layout] Animus MUST treat `cairn/specs/` as the accepted requirements baseline, `cairn/changes/` as active change deltas, and `cairn/archive/` as completed lifecycle history once changes are accepted and archived.

#### Scenario: Accepted spec is source of current requirements

r[agent_harness_runtime.cairn_dogfood.lifecycle_layout.accepted_baseline]
- GIVEN a requirement has been accepted and synced
- WHEN maintainers inspect current lifecycle requirements
- THEN the requirement appears under `cairn/specs/`
- AND the completed change package is archived rather than left as active planning state.

### Requirement: Change threshold guidance

r[agent_harness_runtime.cairn_dogfood.change_threshold] Animus MUST document when work requires a Cairn change package, including public behavior, compatibility semantics, mutation behavior, architecture, receipt/schema contracts, release/evidence contracts, roadmap scope, and safety-boundary changes.

#### Scenario: Small safe edits can stay direct

r[agent_harness_runtime.cairn_dogfood.change_threshold.direct_safe_edit]
- GIVEN a change is limited to typo repair, fixture hardening, or narrow documentation cleanup
- WHEN it does not affect public behavior, architecture, compatibility, mutation, schema, receipt, or roadmap scope
- THEN maintainers may apply it directly
- AND ordinary repository checks still provide evidence.

### Requirement: Gate workflow

r[agent_harness_runtime.cairn_dogfood.gate_workflow] Animus MUST define a lifecycle workflow for proposal, design, tasks, validation, sync dry-run, archive dry-run, and executed archive closeout using Cairn commands or pinned equivalents.

#### Scenario: Change cannot archive without gates

r[agent_harness_runtime.cairn_dogfood.gate_workflow.blocks_incomplete]
- GIVEN an active Cairn change lacks passing proposal, design, tasks, validation, or sync/archive dry-run evidence
- WHEN maintainers attempt closeout
- THEN the change is not considered archive-ready
- AND blockers or waivers are recorded explicitly.

### Requirement: Bounded evidence contract

r[agent_harness_runtime.cairn_dogfood.evidence_contract] Cairn lifecycle evidence SHOULD record command identity, status, issue codes, requirement/change IDs, policy source identity, policy hash when available, output artifact hashes, and unavailable-check blockers while keeping evidence bounded and reviewable.

#### Scenario: Policy identity is visible in evidence

r[agent_harness_runtime.cairn_dogfood.evidence_contract.policy]
- GIVEN Cairn validation or gates run under a policy artifact
- WHEN evidence is recorded
- THEN the evidence identifies the policy source or artifact and hash when available
- AND reviewers can tell which policy accepted the lifecycle content.

### Requirement: Safe evidence summaries

r[agent_harness_runtime.cairn_dogfood.safe_evidence] Durable Cairn evidence summaries MUST omit raw prompts, credentials, provider payloads, full private paths, raw private file bodies, and unbounded command output by default.

#### Scenario: Evidence summary redacts private material

r[agent_harness_runtime.cairn_dogfood.safe_evidence.redacted]
- GIVEN validation, gate, sync, archive, release-readiness, or workspace-check output includes private or unbounded material
- WHEN durable evidence is summarized
- THEN it includes bounded status, counts, issue codes, hashes, and safe relative labels
- AND raw sensitive material is omitted or replaced by approved redaction markers.

### Requirement: Cairn dogfood closeout

r[agent_harness_runtime.cairn_dogfood.closeout] Cairn dogfood work MUST run validation, proposal/design/tasks gates, and sync/archive dry runs for the change itself before archive.

#### Scenario: Dogfood evidence is reviewable

r[agent_harness_runtime.cairn_dogfood.closeout.evidence]
- GIVEN Cairn dogfood work is ready to archive
- WHEN reviewers inspect evidence
- THEN validation, gates, sync dry-run, archive dry-run, and evidence-safety checks are named with status
- AND unavailable checks include a blocker and next best validation path.

### Requirement: Clankers product surfaces are rejected as defaults

r[agent_harness_runtime.clankers_product_rejection.docs] Animus MUST document that it adapts bounded Clankers patterns while rejecting Clankers product surfaces as default reusable Animus behavior.

#### Scenario: Rejected surfaces are explicit

r[agent_harness_runtime.clankers_product_rejection.docs.explicit]
- GIVEN maintainers review Clankers references in Animus
- WHEN they inspect the architecture note
- THEN it explicitly rejects default adoption of Clankers TUI, daemon/attach protocol, Matrix/P2P, global `.clankers` state, provider router/OAuth, plugin subprocess runtime, Wasm/Extism loading, Automerge desktop storage, self-evolution apply/rollback, live dogfood harnesses, product tools, and release tagging workflow
- AND it distinguishes those surfaces from adopted boundary patterns.

### Requirement: Reusable crates remain product-surface free

r[agent_harness_runtime.clankers_product_rejection.dependency_isolation] Reusable Animus crates and default std-shell contracts MUST NOT require Clankers product crates, daemon/TUI concepts, global state directories, Matrix/P2P, plugin runtimes, Automerge storage, OAuth/provider router, self-evolution workflows, product tools, or dogfood harnesses.

#### Scenario: Dependency isolation rail rejects product leakage

r[agent_harness_runtime.clankers_product_rejection.dependency_isolation.rail]
- GIVEN reusable Animus crate dependencies and source markers are checked
- WHEN a rejected Clankers product surface appears as a runtime dependency or assumption
- THEN the rail fails with an actionable diagnostic
- AND bounded-reference documentation mentions remain allowed only in documented reference sections.

### Requirement: Future exceptions require explicit Cairn scope

r[agent_harness_runtime.clankers_product_rejection.future_exception] Any future change that adopts a previously rejected Clankers product surface MUST create a focused Cairn change that names host ownership, injected service boundaries, safety policy, redacted receipts, non-default feature gating, and focused validation rails.

#### Scenario: Future exception is gated

r[agent_harness_runtime.clankers_product_rejection.future_exception.gated]
- GIVEN a maintainer wants to add a daemon, TUI, plugin runtime, provider router, Matrix/P2P, global state, or self-evolution surface
- WHEN the proposal is reviewed
- THEN it includes a focused Cairn change with host ownership, injected services, safety policy, redacted receipts, feature gating, and validation rails
- AND the surface remains non-default unless the change explicitly justifies default behavior.

### Requirement: Marker rail guards non-adoption

r[agent_harness_runtime.clankers_product_rejection.marker_rail] Animus MUST provide a focused marker or dependency rail that allows bounded-reference docs while rejecting product-surface dependencies or runtime assumptions in reusable crates.

#### Scenario: Product rejection closeout is evidenced

r[agent_harness_runtime.clankers_product_rejection.closeout]
- GIVEN the product-surface rejection change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes docs checks, dependency/marker rail results, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Wasm-supported surface

r[agent_harness_runtime.wasm_compile.surface] Animus MUST define a Wasm-supported crate and feature surface for deterministic embedders, initially targeting `wasm32-unknown-unknown` and including only crates that do not require ambient std shell services.

#### Scenario: Embedders select a portable surface

r[agent_harness_runtime.wasm_compile.surface.portable]
- GIVEN an embedder wants Animus logic in a Wasm module
- WHEN they select the documented Wasm-supported crate and feature profile
- THEN the selected surface includes deterministic reusable contracts such as core state and adapter DTOs
- AND it excludes std shell orchestration, terminal UI, provider clients, native persistence, process execution, and filesystem authority unless a later target-specific contract accepts them.

### Requirement: Root facade supports a no-std Wasm profile

r[agent_harness_runtime.wasm_compile.root_facade] The root `animus` facade MUST be able to expose a no-std-compatible profile for the Wasm-supported surface without requiring `animus-std`, terminal, process, filesystem, provider, or native persistence dependencies.

#### Scenario: Root no-std profile avoids shell dependencies

r[agent_harness_runtime.wasm_compile.root_facade.no_shell]
- GIVEN the root facade is built for the documented Wasm target without std shell features
- WHEN dependency resolution and compile checks run
- THEN shell-only crates are not required by that build
- AND std-backed convenience APIs remain available only under documented host-shell features or host-target builds.

### Requirement: Wasm compile rail

r[agent_harness_runtime.wasm_compile.rail] Animus MUST provide a credential-free Wasm compile rail that checks the documented supported surface against the accepted Wasm target and reports deterministic evidence for review.

#### Scenario: Compile rail catches portability drift

r[agent_harness_runtime.wasm_compile.rail.drift]
- GIVEN a change introduces an incompatible dependency or target assumption into the Wasm-supported surface
- WHEN the Wasm compile rail runs
- THEN the rail fails before review claims Wasm support
- AND the diagnostic identifies the crate or feature profile that no longer compiles.

### Requirement: Shell isolation for Wasm support

r[agent_harness_runtime.wasm_compile.shell_isolation] Animus MUST keep std shell, terminal, Steel runtime, provider, native persistence, process, filesystem, network, credential, clock, and host execution dependencies outside the Wasm-supported surface unless guarded by target or feature gates that the Wasm rail does not enable.

#### Scenario: Host behavior remains explicit

r[agent_harness_runtime.wasm_compile.shell_isolation.explicit]
- GIVEN a host wants process, filesystem, provider, terminal, browser, WASI, plugin, or persistence behavior
- WHEN it composes Animus with host services
- THEN that behavior crosses explicit adapter or shell contracts
- AND Wasm compile support alone does not imply those host behaviors are present or authorized.

### Requirement: Optional bridge participation is explicit

r[agent_harness_runtime.wasm_compile.optional_bridges] Optional bridge crates such as Preserves MAY join the Wasm rail only after their target compatibility, dependency behavior, and adapter-DTO revalidation path are documented and tested.

#### Scenario: Optional bridge support is not assumed

r[agent_harness_runtime.wasm_compile.optional_bridges.evidence]
- GIVEN an optional bridge is considered for Wasm support
- WHEN closeout evidence is prepared
- THEN it either includes successful Wasm compile evidence and positive/negative boundary tests for that bridge
- OR records the blocker and excludes the bridge from the supported Wasm surface.

### Requirement: Wasm closeout evidence

r[agent_harness_runtime.wasm_compile.closeout] Boundary-affecting Wasm support changes MUST include reviewable evidence for the focused Wasm rail, relevant crate tests, workspace checks, Cairn validation, and Cairn gates, with unavailable checks recorded alongside blockers and next-best validation.

#### Scenario: Reviewers can distinguish support from non-claims

r[agent_harness_runtime.wasm_compile.closeout.non_claims]
- GIVEN a Wasm support change is ready to archive
- WHEN reviewers inspect its evidence
- THEN the evidence names the exact crate and feature surface that compiles for Wasm
- AND it explicitly lists excluded runtime claims such as browser APIs, WASI host behavior, plugin loading, providers, terminal UI, process execution, filesystem access, credentials, and native persistence.

### Requirement: Animus lifecycle map

r[agent_harness_runtime.clankers_lifecycle_doc.map] Animus MUST document the runtime request lifecycle from typed runtime input through deterministic core state, Steel loop decisions, adapter dispatch, event/report projection, safe receipt construction, and host ledger or persistence seams.

#### Scenario: Golden path is reviewable

r[agent_harness_runtime.clankers_lifecycle_doc.map.reviewable]
- GIVEN a maintainer needs to change runtime request behavior
- WHEN they inspect `docs/request-lifecycle.md`
- THEN the document describes the ordered Animus runtime path
- AND it distinguishes reusable core/adapters/Steel behavior from std-shell or host-owned behavior.

### Requirement: Boundary owners and source-of-truth types

r[agent_harness_runtime.clankers_lifecycle_doc.owners] Each lifecycle boundary MUST name the owning crate and source-of-truth type or trait for the state transition.

#### Scenario: Owner names prevent duplicate state machines

r[agent_harness_runtime.clankers_lifecycle_doc.owners.no_duplicate_state]
- GIVEN a change touches runtime input, Steel decisions, model/tool dispatch, reports, events, receipts, or ledger replay
- WHEN reviewers inspect the lifecycle document
- THEN the owning crate and source-of-truth type or trait are named
- AND the change does not introduce an undocumented duplicate owner for the same transition.

### Requirement: Forbidden dependency rules

r[agent_harness_runtime.clankers_lifecycle_doc.forbidden_dependencies] The lifecycle document MUST identify forbidden dependencies and side effects for reusable core, adapter, Steel, std-shell, receipt, and future ledger boundaries.

#### Scenario: Forbidden dependencies stay visible

r[agent_harness_runtime.clankers_lifecycle_doc.forbidden_dependencies.visible]
- GIVEN a reusable Animus crate boundary is reviewed
- WHEN the lifecycle document is consulted
- THEN it identifies which boundaries must not read environment, filesystem, credentials, provider config, network, clocks, or concrete host persistence
- AND it points to the validation rail that checks those constraints when one exists.

### Requirement: Lifecycle links and marker rail

r[agent_harness_runtime.clankers_lifecycle_doc.links] Animus README or architecture docs MUST link to the lifecycle document so maintainers can find it before changing runtime behavior.

#### Scenario: Focused rail guards lifecycle drift

r[agent_harness_runtime.clankers_lifecycle_doc.rail]
- GIVEN the lifecycle document is part of the maintained architecture contract
- WHEN the focused marker rail runs
- THEN it fails if required boundary names, owner markers, forbidden dependency sections, or validation command references are missing
- AND it emits reviewable evidence without requiring live providers or host credentials.

#### Scenario: Lifecycle closeout is evidenced

r[agent_harness_runtime.clankers_lifecycle_doc.closeout]
- GIVEN the lifecycle change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes the focused lifecycle rail, relevant docs checks, Cairn validation, and Cairn proposal/design/tasks gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Bounded runtime facade inventory adoption

r[agent_harness_runtime.clankers_runtime_facade_inventory.scope] Animus MUST adopt Clankers' runtime-facade inventory pattern only as a bounded architecture and verification rail, not as a dependency on Clankers runtime crates or product surfaces.

#### Scenario: Inventory adoption rejects product surfaces

r[agent_harness_runtime.clankers_runtime_facade_inventory.scope.rejects_product]
- GIVEN maintainers adapt Clankers runtime-facade inventory ideas
- WHEN the adoption scope is documented
- THEN Animus distinguishes export-classification discipline from Clankers daemon, TUI, provider router, plugin runtime, auth flow, process manager, desktop storage, and product shell behavior
- AND reusable Animus crates do not import Clankers runtime crates for the inventory.

### Requirement: Public export inventory schema

r[agent_harness_runtime.clankers_runtime_facade_inventory.schema] Animus MUST define an inventory record schema for runtime-facing public exports that records export path, owning crate, source module or marker, class, side-effect allowance, stability intent, review note, and safe evidence metadata.

#### Scenario: Export records name owners and authority class

r[agent_harness_runtime.clankers_runtime_facade_inventory.schema.owner_class]
- GIVEN a runtime-facing public export is included in the inventory
- WHEN reviewers inspect the record
- THEN the record names the owning crate and source location or marker
- AND it classifies the export as a reusable contract, host-injection surface, or shell adapter.

### Requirement: Export classes preserve reusable boundaries

r[agent_harness_runtime.clankers_runtime_facade_inventory.classification] Runtime-facing public exports MUST be classified as reusable contracts, host-injection surfaces, or shell adapters, and each class MUST define forbidden side effects.

#### Scenario: Reusable contract remains no-ambient

r[agent_harness_runtime.clankers_runtime_facade_inventory.classification.reusable_no_ambient]
- GIVEN an export is classified as a reusable contract
- WHEN the inventory rail checks it
- THEN the export does not require ambient filesystem, environment, credentials, clocks, randomness, network, provider discovery, TUI state, daemon frames, plugin launch, process execution, or concrete persistence
- AND any needed authority is represented through a host-injection surface or shell adapter instead.

#### Scenario: Host injection fails closed by default

r[agent_harness_runtime.clankers_runtime_facade_inventory.classification.fail_closed]
- GIVEN an export is classified as a host-injection surface
- WHEN the default service or adapter is absent, unavailable, denied, cancelled, or unsupported
- THEN it fails closed with typed status or denial metadata
- AND it does not silently create a shell adapter, read ambient config, discover providers, or contact backends.

### Requirement: Focused inventory rail

r[agent_harness_runtime.clankers_runtime_facade_inventory.rail] Animus MUST provide a focused static rail or equivalent deterministic test that fails on unclassified runtime-facing public exports, missing owner metadata, class and side-effect contradictions, and unsafe evidence fields.

r[agent_harness_runtime.clankers_runtime_facade_inventory.positive] The inventory rail MUST accept fixtures or real exports for pure DTOs, validation helpers, adapter traits, and deny-by-default host service seams.

r[agent_harness_runtime.clankers_runtime_facade_inventory.negative] The inventory rail MUST reject fixtures or real exports that classify ambient filesystem, environment, credential, clock, network, provider, TUI, daemon, plugin, process, or persistence behavior as reusable contracts.

#### Scenario: Positive and negative fixtures guard classification

r[agent_harness_runtime.clankers_runtime_facade_inventory.rail.fixtures]
- GIVEN inventory fixtures or real exports include reusable contracts, host-injection seams, and unsafe shell-authority examples
- WHEN the focused inventory rail runs
- THEN valid reusable and host-injection surfaces are accepted
- AND unsafe ambient-authority examples are rejected as reusable contracts.

#### Scenario: Unclassified exports fail review

r[agent_harness_runtime.clankers_runtime_facade_inventory.rail.unclassified]
- GIVEN a runtime-facing crate adds a public export without inventory classification
- WHEN the focused inventory rail runs
- THEN it fails with an actionable diagnostic naming the missing classification
- AND it does not require live providers, host credentials, plugin runtimes, daemon sessions, or TUI state.

### Requirement: Safe inventory evidence and docs

r[agent_harness_runtime.clankers_runtime_facade_inventory.evidence] Inventory evidence MUST include schema/status, checked crates, class counts, fixture names, safe diagnostics, and content hashes when needed, and MUST omit raw prompts, provider payloads, credentials, environment values, full host paths, and tool output.

r[agent_harness_runtime.clankers_runtime_facade_inventory.docs] Animus README or architecture docs MUST link to an inventory summary that tells embedders which surfaces are default-safe reusable contracts, host-owned injection points, or shell-only adapters.

#### Scenario: Closeout evidence separates claims from non-claims

r[agent_harness_runtime.clankers_runtime_facade_inventory.closeout]
- GIVEN the runtime facade inventory change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes the focused inventory rail, positive and negative fixtures, dependency checks, docs links, Cairn validation, and Cairn gates
- AND skipped checks or non-claims such as live providers, TUI dogfood, daemon tests, plugin runtime tests, VM checks, or production readiness are identified explicitly.

### Requirement: Plugin manifests are bounded descriptor references

r[agent_harness_runtime.clankers_plugin_manifest.scope] Animus MUST treat Clankers plugin manifests as bounded descriptor references and MUST NOT adopt plugin execution, stdio supervision, Wasm loading, Zellij, MCP, or sandbox enforcement in this change.

#### Scenario: Descriptor scope rejects runtime adoption

r[agent_harness_runtime.clankers_plugin_manifest.scope.rejects_runtime]
- GIVEN maintainers evaluate Clankers plugin manifests
- WHEN the adoption scope is reviewed
- THEN it documents descriptor fields separately from runtime execution
- AND it does not add plugin process launch, Wasm loading, MCP/Zellij integration, or sandbox enforcement as default Animus behavior.

### Requirement: Optional manifest schema

r[agent_harness_runtime.clankers_plugin_manifest.schema] Animus MAY define plugin descriptor fields for identity, runtime kind, declared tools, capabilities, launch policy metadata, sandbox requirements, environment allowlist names, writable roots, network policy, and redaction.

#### Scenario: Manifest descriptors declare authority needs

r[agent_harness_runtime.clankers_plugin_manifest.schema.authority]
- GIVEN a plugin descriptor is parsed
- WHEN validation inspects it
- THEN declared tools, capabilities, runtime kind, launch policy metadata, sandbox requirements, environment allowlist names, writable roots, network policy, and redaction are explicit
- AND missing required descriptor fields fail validation.

### Requirement: Manifest validation rejects unsafe declarations

r[agent_harness_runtime.clankers_plugin_manifest.validation] Manifest validation MUST reject unknown runtime kinds, missing required fields, duplicate or built-in tool collisions, path escapes, absolute writable roots, environment value leakage, and unsafe network/sandbox combinations.

#### Scenario: Invalid manifest fixtures fail

r[agent_harness_runtime.clankers_plugin_manifest.fixtures]
- GIVEN manifest fixtures include path escapes, absolute writable roots, env values, missing commands, unknown runtime kinds, tool collisions, or unsafe network/sandbox combinations
- WHEN validation runs
- THEN each invalid fixture fails with an actionable diagnostic
- AND safe descriptors normalize deterministically.

### Requirement: Manifest parsing is runtime-neutral

r[agent_harness_runtime.clankers_plugin_manifest.runtime_neutral] Manifest parsing MUST NOT launch stdio processes, load Wasm, inherit environment values, read secrets, connect to network, or execute tools.

#### Scenario: Runtime-neutrality rail catches plugin side effects

r[agent_harness_runtime.clankers_plugin_manifest.runtime_neutral.rail]
- GIVEN plugin descriptor parsing is validated
- WHEN the runtime-neutrality rail runs
- THEN it proves parsing is data-only
- AND no plugin execution, Wasm load, secret read, environment inheritance, or network access is required.

#### Scenario: Plugin manifest closeout is evidenced

r[agent_harness_runtime.clankers_plugin_manifest.closeout]
- GIVEN the plugin manifest boundary evaluation is ready to archive
- WHEN closeout validation runs
- THEN evidence includes scope docs, manifest fixtures, runtime-neutrality checks, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Release evidence index is a bounded future pattern

r[agent_harness_runtime.clankers_evidence_index.scope] Animus MUST treat Clankers release evidence indexing as a bounded future pattern and MUST NOT adopt the full Clankers harness, dogfood tests, live model checks, VM gates, or release tagging workflow in this evaluation change.

#### Scenario: Evidence index scope avoids harness adoption

r[agent_harness_runtime.clankers_evidence_index.scope.bounded]
- GIVEN maintainers evaluate Clankers release evidence
- WHEN the adoption note is reviewed
- THEN it documents evidence-index concepts separately from Clankers' full harness and dogfood suites
- AND it keeps live model checks, VM gates, TUI/daemon dogfood, and release tagging out of scope.

### Requirement: Evidence index schema composes receipts

r[agent_harness_runtime.clankers_evidence_index.schema] A future Animus evidence index SHOULD compose focused rail receipts, workspace tests, clippy, flake checks, Nickel/config drift checks, Cairn validation, and Cairn gates without rerunning missing modes by default.

#### Scenario: Index composes selected evidence

r[agent_harness_runtime.clankers_evidence_index.schema.compose]
- GIVEN multiple local receipts and check summaries exist
- WHEN the evidence index is generated
- THEN it selects and summarizes existing evidence by mode
- AND it does not imply missing modes ran.

### Requirement: Current-head evidence is explicit

r[agent_harness_runtime.clankers_evidence_index.current_head] An evidence index that makes current-head claims MUST record commit, branch, dirty status, receipt/head match, stale receipt status, and missing modes, with VCS collection kept in the imperative shell.

#### Scenario: Stale or dirty evidence is not current-head proof

r[agent_harness_runtime.clankers_evidence_index.current_head.stale]
- GIVEN selected receipts do not match the current head or the worktree is dirty
- WHEN the evidence index is reviewed
- THEN those receipts are labeled stale or dirty
- AND they are not treated as current-head proof.

### Requirement: Non-claims are explicit

r[agent_harness_runtime.clankers_evidence_index.non_claims] Evidence indexes MUST separate pure credential-free checks from optional host-dependent checks and MUST state non-claims for live providers, TUI/daemon dogfood, VM checks, remote services, plugin runtime, and production readiness unless those checks explicitly ran.

#### Scenario: Overclaim fixtures fail

r[agent_harness_runtime.clankers_evidence_index.fixtures]
- GIVEN fixture evidence omits live provider, daemon/TUI, VM, plugin, or production-readiness checks
- WHEN index validation runs
- THEN claims for those omitted surfaces are rejected or labeled as non-claims
- AND pure evidence remains usable for the contracts it actually covers.

#### Scenario: Evidence index closeout is documented

r[agent_harness_runtime.clankers_evidence_index.closeout]
- GIVEN the evidence index evaluation is ready to archive
- WHEN closeout validation runs
- THEN evidence includes scope docs, schema fixtures, current-head fixtures, non-claim fixtures, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Runtime crate boundary

r[agent_harness_runtime.loop.contract] The workspace MUST provide an `animus-runtime` crate that owns reusable agent-loop orchestration without depending on concrete provider, tool, filesystem, process, network, UI, or Syndicate implementations.

#### Scenario: Runtime crate is embeddable

r[agent_harness_runtime.loop.contract.embeddable]
- GIVEN an embedder depends on `animus-runtime`, `animus-adapters`, and a chosen loop fabric
- WHEN they construct a runtime with model and tool adapters
- THEN they can execute the agent loop without pulling in concrete provider or tool crates
- AND the runtime public API exposes typed inputs, configuration, events, reports, and errors.

### Requirement: Pure runtime loop core

r[agent_harness_runtime.loop.core] Runtime decision handling MUST be decomposed into a pure deterministic core over explicit turn state, decisions, adapter outcomes, and configuration, plus a thin shell that calls fabrics and adapters.

#### Scenario: Runtime core is directly testable

r[agent_harness_runtime.loop.core.pure]
- GIVEN a modeled turn state, runtime configuration, and next decision
- WHEN the runtime core evaluates the transition
- THEN it returns typed next state, events, and completion status deterministically
- AND it does not read files, inspect environment, spawn processes, use clocks, log, mutate global state, or call adapters.

### Requirement: Steel default loop fabric

r[agent_harness_runtime.steel_fabric.default] The runtime SHOULD provide a default constructor or helper that uses `animus-steel` as the scriptable agent-loop fabric while preserving a swappable fabric boundary.

#### Scenario: Steel decision drives runtime turn

r[agent_harness_runtime.steel_fabric.default.turn]
- GIVEN a Steel program defines a valid `(animus-decide)` function
- WHEN the runtime evaluates a turn through the Steel fabric
- THEN the returned decision is validated by Rust boundary DTOs before dispatch
- AND invalid Steel load, evaluation, shape, or payload errors stop the turn with typed runtime failure.

### Requirement: Adapter decision dispatch

r[agent_harness_runtime.adapter_dispatch] Runtime dispatch MUST route validated model and tool decisions through injected `ModelAdapter` and `ToolAdapter` implementations and MUST NOT construct concrete provider or tool clients internally.

#### Scenario: Model decision uses injected adapter

r[agent_harness_runtime.adapter_dispatch.model]
- GIVEN the loop fabric returns a validated model request decision
- WHEN runtime dispatches the turn
- THEN it calls only the injected model adapter with the validated request
- AND records a typed model-requested and model-responded event or a typed adapter-error event.

#### Scenario: Tool decision uses injected adapter

r[agent_harness_runtime.adapter_dispatch.tool]
- GIVEN the loop fabric returns a validated tool request decision
- WHEN runtime dispatches the turn
- THEN it calls only the injected tool adapter with the validated request
- AND records a typed tool-requested and tool-responded event or a typed adapter-error event.

### Requirement: Runtime stop and max-turn behavior

r[agent_harness_runtime.loop.max_turns] Runtime configuration MUST include an explicit max-turn limit and MUST halt with typed status when the fabric returns stop, final response, adapter failure, invalid decision, or max-turn exhaustion.

#### Scenario: Max turns fail closed

r[agent_harness_runtime.loop.max_turns.exhausted]
- GIVEN runtime configuration allows a finite number of turns
- WHEN the loop reaches that limit without final response or stop
- THEN runtime halts with a max-turns-exceeded status
- AND no additional model or tool adapter calls occur.

### Requirement: Runtime positive tests

r[agent_harness_runtime.tests.positive] Runtime work MUST include positive tests for one-turn model calls, tool calls, final responses, explicit stop behavior, and multi-turn propagation of previous model/tool outputs into the next fabric turn.

#### Scenario: Multi-turn state is propagated

r[agent_harness_runtime.tests.positive.multi_turn]
- GIVEN a model adapter response or tool adapter output is produced in one turn
- WHEN the next turn is evaluated
- THEN the loop fabric receives the previous output through typed turn input
- AND the final report records the ordered event sequence.

### Requirement: Runtime negative tests

r[agent_harness_runtime.tests.negative] Runtime work MUST include negative tests for invalid Steel decisions, malformed inputs, model adapter failure, tool adapter failure, max-turn exhaustion, missing final response, and stop-without-response behavior.

#### Scenario: Adapter failure stops safely

r[agent_harness_runtime.tests.negative.adapter_failure]
- GIVEN an injected model or tool adapter returns an error
- WHEN runtime dispatch handles that adapter call
- THEN runtime records a typed adapter-error event
- AND halts without invoking later adapters or fabric turns.

### Requirement: Preserves runtime schema

r[agent_harness_runtime.preserves_schema.contract] Runtime boundary messages SHOULD have Preserves schema coverage for stable event, request, response, final-report, and error payloads before those payloads are treated as external wire contracts.

#### Scenario: Schema validates runtime payloads

r[agent_harness_runtime.preserves_schema.contract.round_trip]
- GIVEN a runtime event or final report is encoded for a boundary
- WHEN Preserves schema validation and round-trip tests run
- THEN valid payloads decode into equivalent typed Rust values
- AND malformed, wrong-kind, missing-field, and invalid-text payloads fail closed.

### Requirement: Runtime docs and validation

r[agent_harness_runtime.docs.validation] Runtime work MUST update architecture docs and safety contracts to describe `animus-runtime`, Steel loop-fabric use, adapter dispatch boundaries, and validation commands.

#### Scenario: Runtime closeout is reviewable

r[agent_harness_runtime.validation]
- GIVEN runtime work is ready to archive
- WHEN reviewers inspect the change evidence
- THEN successful logs show focused crate tests, positive and negative runtime tests, Preserves schema tests when added, workspace check/clippy/tests, pre-commit, Octet/Tigerstyle, flake checks, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Semantic agent status DTOs

r[agent_harness_runtime.agent_status_authority.semantic_state] Animus MUST define semantic agent status DTOs for `idle`, `working`, `blocked`, `done`, and `unknown` with source identity, agent label, optional native session reference, bounded message text, presentation metadata, sequence, and optional TTL fields.

#### Scenario: Lifecycle report is validated

r[agent_harness_runtime.agent_status_authority.semantic_state.validation]
- GIVEN a trusted host integration reports an agent label, source identifier, semantic state, target, and sequence
- WHEN Animus validates the status report
- THEN the report is accepted as a typed lifecycle observation
- AND malformed states, malformed source identifiers, missing targets, and unsupported session references are rejected with typed diagnostics.

### Requirement: Authority precedence is deterministic

r[agent_harness_runtime.agent_status_authority.authority_precedence] Status selection MUST choose one semantic authority for a target at a time, and a current trusted lifecycle authority MUST suppress heuristic fallback observations for that target.

#### Scenario: Lifecycle authority wins over heuristic fallback

r[agent_harness_runtime.agent_status_authority.authority_precedence.lifecycle_wins]
- GIVEN a current trusted lifecycle report marks an agent `working`
- AND a host heuristic observation would otherwise classify the same target as `idle`
- WHEN status selection runs
- THEN the selected semantic state is `working`
- AND the heuristic observation is recorded only as non-authoritative fallback evidence.

### Requirement: Presentation metadata is visual-only

r[agent_harness_runtime.agent_status_authority.presentation_only] Titles, display agent names, compact activity text, custom status labels, and state label overrides MUST be presentation-only metadata and MUST NOT change semantic lifecycle state, waits, notifications, rollups, or policy decisions.

#### Scenario: Custom label cannot escalate state

r[agent_harness_runtime.agent_status_authority.presentation_only.no_semantic_escalation]
- GIVEN a metadata report sets a custom visual label that says `needs input`
- AND the selected semantic state remains `working`
- WHEN waits, notifications, rollups, and policy decisions consume agent status
- THEN they use `working`
- AND the visual label is available only to presentation hosts.

### Requirement: Explainable status decisions are bounded

r[agent_harness_runtime.agent_status_authority.explain] Animus SHOULD expose bounded status-decision explanations that identify the selected authority, rejected candidates, freshness class, fallback reason, safe hashes, and redacted evidence summaries without embedding raw prompts, terminal snapshots, provider payloads, credentials, or unbounded detector output.

#### Scenario: Explanation rejects raw evidence leaks

r[agent_harness_runtime.agent_status_authority.explain.redacted]
- GIVEN a status explanation fixture contains raw terminal text, prompt text, provider JSON, a credential-looking value, or an unbounded detector transcript
- WHEN explanation validation runs
- THEN validation rejects the fixture with a typed leak diagnostic
- AND the explanation is not accepted as durable evidence.

### Requirement: Detection is host-owned

r[agent_harness_runtime.agent_status_authority.host_owned_detection] Reusable Animus runtime crates MUST NOT inspect processes, read terminals, parse screen buffers, or load detector manifests; they MUST consume typed host observations supplied through adapter contracts.

#### Scenario: Pure status selection has no ambient I/O

r[agent_harness_runtime.agent_status_authority.host_owned_detection.no_ambient_io]
- GIVEN status selection receives typed lifecycle and heuristic observations in memory
- WHEN it selects the current semantic state
- THEN the selection result is deterministic over those inputs
- AND the pure status core does not read terminals, processes, files, clocks, environment variables, or network state.

### Requirement: Status authority verification

r[agent_harness_runtime.agent_status_authority.verification] The adaptation MUST include focused positive and negative tests for report validation, authority precedence, stale sequence and TTL handling, presentation-only metadata, bounded explanations, leak rejection, host-owned detection, and pure status selection.

#### Scenario: Closeout evidence covers status authority rails

r[agent_harness_runtime.agent_status_authority.verification.closeout]
- GIVEN maintainers intend to archive the agent status authority adaptation
- WHEN closeout validation runs
- THEN evidence names focused status DTO, authority precedence, presentation metadata, explanation redaction, host-owned detection, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a blocker and next-best validation.
