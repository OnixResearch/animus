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
