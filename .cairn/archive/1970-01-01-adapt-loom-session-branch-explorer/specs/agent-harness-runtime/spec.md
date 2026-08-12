# Agent Harness Runtime

## ADDED Requirements

### Requirement: Session branch exploration preserves ledger authority

r[agent_harness_runtime.session_branch_explorer.boundary] Animus MUST derive branch-explorer facts from one validated session ledger. Explorer state MUST NOT become model context, ledger truth, tool authority, or completion evidence.

#### Scenario: Validated ledger enters branch projection

r[agent_harness_runtime.session_branch_explorer.boundary.validated]
- GIVEN a caller supplies one validated session ledger, revision, projection policy, annotations, and expected active leaf
- WHEN pure branch-explorer validation runs
- THEN Animus MUST return a deterministic projection, operation plan, diagnostic, or blocked outcome
- AND reusable code MUST NOT read storage, inspect clocks, contact providers, render terminals, or mutate the ledger.

#### Scenario: Independent DAG is offered as conversation history

r[agent_harness_runtime.session_branch_explorer.boundary.rejects_dag]
- GIVEN a caller offers a multi-parent node, evidence edge, comparison edge, or equivalence edge as model-history order
- WHEN branch-explorer admission runs
- THEN Animus MUST reject the topology before replay or host mutation
- AND evidence and synthesis links MUST remain separate from the single-parent conversation tree.

### Requirement: Session branch projection is bounded and redaction-first

r[agent_harness_runtime.session_branch_explorer.projection] Animus MUST produce a bounded branch projection with exact source identity, revision, parent, storage sequence, child count, leaf state, active-path state, safe annotation, artifact-reference, and truncation facts.

#### Scenario: Complete bounded tree is projected

r[agent_harness_runtime.session_branch_explorer.projection.complete]
- GIVEN one valid tree fits the admitted node, depth, label, and reference bounds
- WHEN branch projection runs
- THEN every projected node MUST bind its exact source identity and structural facts under the current policy
- AND repeated projection over identical inputs MUST preserve node identities, order, active-path marks, and diagnostics.

#### Scenario: Projection input is unsafe or overbound

r[agent_harness_runtime.session_branch_explorer.projection.rejects]
- GIVEN input has a missing parent, cycle, duplicate identity, unsupported node, false leaf, unsafe field, or exceeded bound
- WHEN branch projection runs
- THEN Animus MUST return explicit truncation or a bounded typed rejection according to policy
- AND it MUST NOT expose transcript text, reasoning, raw tool bodies, provider payloads, credentials, physical paths, or host handles.

### Requirement: Branch annotations cannot change replay

r[agent_harness_runtime.session_branch_explorer.annotations] Animus MUST validate bookmarks, safe labels, and sibling presentation order as sidecar metadata. Annotations MUST NOT change storage order, parentage, active-path replay, model context, provenance, or tool correlation.

#### Scenario: Partial sibling order is admitted

r[agent_harness_runtime.session_branch_explorer.annotations.partial_order]
- GIVEN an annotation names distinct current children of one parent in a partial presentation order
- WHEN annotation validation runs
- THEN named children MUST appear in that order and omitted children MUST retain deterministic ledger order
- AND the ledger bytes and replay order MUST remain unchanged.

#### Scenario: Annotation targets invalid topology

r[agent_harness_runtime.session_branch_explorer.annotations.rejects]
- GIVEN an annotation names an unknown entry, repeats a child, crosses parent boundaries, carries an unsafe label, or binds a stale revision
- WHEN annotation validation runs
- THEN Animus MUST reject the annotation with a bounded diagnostic
- AND it MUST NOT repair, reparent, remove, reorder, or activate any ledger node.

### Requirement: Branch selection distinguishes inspect, activate, and fork

r[agent_harness_runtime.session_branch_explorer.selection] Animus MUST plan inspection, active-leaf selection, and fork-from-node as distinct operations with exact source revision, expected active leaf, target, policy, and external-state posture.

#### Scenario: Current leaf is activated

r[agent_harness_runtime.session_branch_explorer.selection.activate]
- GIVEN the target is a true leaf in the expected current revision and policy admits selection
- WHEN active-leaf planning runs
- THEN Animus MUST return one compare-and-set host plan that changes only the selected active leaf
- AND the plan MUST NOT replay tools, rewrite entries, claim world-state rollback, or infer effect isolation.

#### Scenario: Internal node becomes a fork point

r[agent_harness_runtime.session_branch_explorer.selection.fork]
- GIVEN the target is an admitted internal node and the operator requests continuation from that point
- WHEN fork planning runs
- THEN Animus MUST return an append-only child-lineage plan through accepted branch controls
- AND it MUST preserve the original branch without destructive split, merge, reparent, suffix deletion, or content rewrite.

#### Scenario: Selection facts are stale or ambiguous

r[agent_harness_runtime.session_branch_explorer.selection.rejects]
- GIVEN the source revision changed, expected active leaf differs, target is unknown, activation target has children, or external-state posture is missing
- WHEN selection planning runs
- THEN Animus MUST return a stale, invalid-target, or unknown-effect blocked outcome before host mutation
- AND it MUST NOT dispatch model, tool, filesystem, network, provider, or persistence effects.

### Requirement: Branch comparison reports exact bounded facts

r[agent_harness_runtime.session_branch_explorer.comparison] Animus MUST compare two admitted branch targets through their exact common ancestor, ordered suffix identities, bounded entry-class counts, usage summaries, model-profile identities, safe references, annotations, and unavailable facts.

#### Scenario: Two branches share an ancestor

r[agent_harness_runtime.session_branch_explorer.comparison.shared]
- GIVEN two current targets belong to one admitted session tree and share one exact ancestor
- WHEN branch comparison runs
- THEN Animus MUST return the ancestor identity and both ordered suffixes with bounded safe metadata
- AND it MUST NOT claim response quality, correctness, semantic superiority, effect equivalence, or winner status.

#### Scenario: Comparison targets cannot be joined safely

r[agent_harness_runtime.session_branch_explorer.comparison.rejects]
- GIVEN targets cross session or generation boundaries, lack a visible common ancestor, exceed bounds, or require unauthorized content
- WHEN branch comparison runs
- THEN Animus MUST return a bounded blocked or truncated outcome
- AND it MUST NOT guess joins from timestamps, equal content, labels, row positions, or model names.

### Requirement: Sibling equivalence preserves every branch

r[agent_harness_runtime.session_branch_explorer.equivalence] Animus MAY mark sibling leaves equivalent only through a domain-separated BLAKE3 generation identity over complete admitted source, model, parameter, tool-plan, effect-receipt, and response-artifact facts.

#### Scenario: Complete generation identities match

r[agent_harness_runtime.session_branch_explorer.equivalence.matches]
- GIVEN two sibling leaves have equal complete generation identities under current policy
- WHEN equivalence validation runs
- THEN Animus MAY emit one equivalence sidecar that keeps both leaves present and selectable
- AND no delete, merge, rewrite, reparent, or canonical-winner action may result.

#### Scenario: Equal response text hides different effects

r[agent_harness_runtime.session_branch_explorer.equivalence.different_effects]
- GIVEN two siblings have equal visible response text but different tool plans, effect receipts, model profiles, parameters, or response artifacts
- WHEN equivalence validation runs
- THEN Animus MUST keep their generation identities distinct or report incomplete evidence
- AND text equality MUST NOT trigger deduplication or history removal.

### Requirement: The reference TUI remains a pure presentation host

r[agent_harness_runtime.session_branch_explorer.tui] `animus-tui` MUST present bounded read and map views through a pure reducer. It MUST dispatch typed operations through host ports without direct ledger, persistence, model, tool, or transport authority.

#### Scenario: Operator navigates the branch map

r[agent_harness_runtime.session_branch_explorer.tui.navigate]
- GIVEN one admitted branch projection and selected canonical conversation projection
- WHEN the operator focuses, collapses, inspects, bookmarks, compares, refreshes, or requests activation
- THEN the pure reducer MUST produce deterministic view state and typed host intents
- AND deterministic text snapshots MUST expose active, leaf, bookmark, truncation, stale, and external-state indicators.

#### Scenario: Host operation fails or projection becomes stale

r[agent_harness_runtime.session_branch_explorer.tui.failure]
- GIVEN a host port rejects an operation, drops a response, or reports a newer ledger revision
- WHEN the reducer applies the outcome
- THEN the TUI MUST preserve prior validated state and show one bounded diagnostic or refresh requirement
- AND it MUST NOT assume that a bookmark, activation, fork, comparison, or continuation succeeded.

### Requirement: Branch-explorer evidence is metadata-only

r[agent_harness_runtime.session_branch_explorer.evidence] Branch-explorer receipts MUST bind policy, source revision, projection, annotation, selection, comparison, equivalence, TUI action, external-state posture, outcome, redaction, and non-claim metadata only.

#### Scenario: Safe explorer receipt is emitted

r[agent_harness_runtime.session_branch_explorer.evidence.safe]
- GIVEN projection or operation planning completes with supplied host evidence
- WHEN receipt projection runs
- THEN the receipt MUST bind exact identities, bounded counts, operation class, result class, and external-state posture
- AND it MUST describe only validated branch-explorer facts.

#### Scenario: Receipt leaks private data or overclaims effects

r[agent_harness_runtime.session_branch_explorer.evidence.rejects]
- GIVEN a receipt draft includes transcript text, reasoning, raw tool bodies, provider payloads, credentials, environment values, physical paths, raw UI state, rollback claims, or quality claims
- WHEN receipt validation runs
- THEN Animus MUST reject the draft without echoing the private value
- AND the draft MUST NOT contribute to model context, authority, completion, lifecycle, or release evidence.

### Requirement: Session branch exploration has positive and negative evidence

r[agent_harness_runtime.session_branch_explorer.verification] Session branch exploration MUST include deterministic positive and negative tests for topology, bounds, annotations, selection, comparison, equivalence, TUI behavior, redaction, receipts, and dependency isolation.

#### Scenario: Positive branch-explorer matrix passes

r[agent_harness_runtime.session_branch_explorer.verification.positive]
- GIVEN fixtures cover roots, internal nodes, leaves, active paths, forks, bookmarks, partial order, common ancestors, equivalence, refresh, and text snapshots
- WHEN the focused branch-explorer rail runs
- THEN production reducers and host entrypoints MUST return the expected deterministic identities, plans, views, and diagnostics
- AND repeated runs over identical facts MUST return identical results.

#### Scenario: Adversarial branch operation fails

r[agent_harness_runtime.session_branch_explorer.verification.negative]
- GIVEN a fixture attempts a cycle, missing parent, false leaf, stale activation, cross-parent order, unsafe label, guessed comparison, content deduplication, direct ledger write, private receipt, or rollback claim
- WHEN the focused branch-explorer rail runs
- THEN the attempt MUST fail before the prohibited mutation, disclosure, or claim
- AND ledger, model, tool, authority, persistence, UI, and consumer boundaries MUST remain intact.

#### Scenario: Change is ready to archive

r[agent_harness_runtime.session_branch_explorer.verification.closeout]
- GIVEN maintainers intend to sync and archive the session branch explorer
- WHEN closeout validation runs
- THEN focused core, adapter, std-host, TUI, Nickel, redaction, receipt, Cairn, and relevant Nix checks MUST pass
- AND transcript truth, effect rollback, world isolation, semantic quality, UI delivery, accessibility, production readiness, and release eligibility MUST remain explicit non-claims.
