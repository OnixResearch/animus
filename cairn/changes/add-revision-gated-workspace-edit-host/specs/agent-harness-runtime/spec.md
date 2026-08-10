# Agent Harness Runtime

## ADDED Requirements

### Requirement: The concrete hashed-edit host is separate and capability-scoped

r[agent_harness_runtime.hashed_edit_host.boundary] Animus MUST provide concrete workspace-file mutation only through a separate non-default host with a pure admission and apply core plus a thin capability-relative filesystem shell. Reusable and default crates MUST NOT acquire filesystem, path, lock, publication, or durability authority.

#### Scenario: Explicit workspace-edit host is selected

r[agent_harness_runtime.hashed_edit_host.boundary.selected]
- GIVEN an embedder supplies an authorized capability root, logical relative resource, host policy, synchronization service, and publication services
- WHEN host construction and request admission run
- THEN Animus MAY create one resource-scoped edit operation
- AND unrelated default, no-std, Wasm, provider, TUI, and std profiles MUST remain filesystem-free.

#### Scenario: Path or authority is ambient

r[agent_harness_runtime.hashed_edit_host.boundary.ambient]
- GIVEN a request supplies an absolute path, parent escape, missing capability root, mismatched root identity, unsupported file kind, or absent host authority
- WHEN admission runs
- THEN the host MUST reject before opening, staging, or mutating the resource
- AND lexical path admission MUST NOT be presented as root authorization or safe file I/O.

### Requirement: Host snapshots bind revision, visibility, and exact text shape

r[agent_harness_runtime.hashed_edit_host.snapshot] A concrete host snapshot MUST bind the existing complete-content BLAKE3 hash and tagged lines with exact capability-root identity, logical resource identity, revision identity, visible tagged spans, line-ending mode, final-newline state, and writer generation.

#### Scenario: Complete visible snapshot is admitted

r[agent_harness_runtime.hashed_edit_host.snapshot.valid]
- GIVEN the host reads one authorized text file and supplies complete consistent snapshot, visibility, text-shape, and writer facts
- WHEN pure snapshot validation runs
- THEN Animus MUST return one canonical admitted snapshot and identity
- AND later apply and publication plans MUST bind that exact snapshot.

#### Scenario: Snapshot text shape is ambiguous

r[agent_harness_runtime.hashed_edit_host.snapshot.invalid]
- GIVEN line endings are mixed without supported policy, final-newline state is unknown, tags conflict, visible spans are malformed, or identities mismatch
- WHEN snapshot validation runs
- THEN Animus MUST reject before byte application or staging
- AND it MUST not normalize, guess, or silently repair the source.

### Requirement: Edits affect only visible lines and insertion boundaries

r[agent_harness_runtime.hashed_edit_host.visibility] Replacement and deletion MUST affect only lines visible under the exact supplied revision. Insertion MUST name an admitted visible adjacent boundary, and end insertion MUST require an explicit visible end boundary.

#### Scenario: Visible edit is admitted

r[agent_harness_runtime.hashed_edit_host.visibility.allowed]
- GIVEN every selected replacement, deletion, or insertion boundary is visible and current under the exact snapshot revision
- WHEN edit admission runs
- THEN Animus MAY produce one ordered non-overlapping apply plan
- AND the plan MUST retain the exact selected tags, spans, and visibility identity.

#### Scenario: Hidden or partially visible edit is requested

r[agent_harness_runtime.hashed_edit_host.visibility.denied]
- GIVEN an edit includes a hidden line, partial span, unknown boundary, stale visible tag, or unobserved end-of-file boundary
- WHEN edit admission runs
- THEN Animus MUST deny before byte application or host mutation
- AND a complete file hash MUST NOT substitute for caller visibility.

### Requirement: The host revalidates exact freshness before publication

r[agent_harness_runtime.hashed_edit_host.freshness] After pure plan construction and resource-lock acquisition, the shell MUST read the file again and require exact snapshot, revision, capability-root, metadata-policy, and writer-generation agreement before staging publication.

#### Scenario: Second read matches

r[agent_harness_runtime.hashed_edit_host.freshness.current]
- GIVEN the exact second read and current host observations match the admitted snapshot and writer generation
- WHEN pre-publication validation runs
- THEN the shell MAY stage the exact precomputed after bytes
- AND the staged bytes MUST match the pure after-snapshot identity.

#### Scenario: Resource drifts before publication

r[agent_harness_runtime.hashed_edit_host.freshness.stale]
- GIVEN content, revision, root, file metadata, policy, or writer generation changes before the second-read gate
- WHEN pre-publication validation runs
- THEN the operation MUST return stale before staging or rename
- AND it MUST not rebase, fuzzy-match, or apply the old plan to current bytes.

### Requirement: Publication and durability outcomes remain distinct

r[agent_harness_runtime.hashed_edit_host.publication] The host MUST represent prepared, staged, published, durability-pending, durability-complete, failed-before-publication, and uncertain-after-publication outcomes explicitly. It MUST serialize Animus-owned writers without claiming universal cross-process exclusion.

#### Scenario: Publication and durability complete

r[agent_harness_runtime.hashed_edit_host.publication.complete]
- GIVEN staging, exact replacement, file synchronization, and required directory synchronization all return matching evidence
- WHEN publication reduction runs
- THEN Animus MAY return durability-complete for the exact after identity
- AND it MUST bind the writer generation and every observed publication stage.

#### Scenario: Failure occurs before publication

r[agent_harness_runtime.hashed_edit_host.publication.before]
- GIVEN open, read, validation, lock, staging, or pre-rename work fails without changing the target
- WHEN outcome reduction runs
- THEN Animus MUST return failed-before-publication
- AND it MUST not claim that an edit became visible.

#### Scenario: Failure occurs after publication

r[agent_harness_runtime.hashed_edit_host.publication.after]
- GIVEN replacement made the new bytes visible but later synchronization or observation fails
- WHEN outcome reduction runs
- THEN Animus MUST return published, durability-pending, or uncertain-after-publication as supported by evidence
- AND it MUST not claim rollback, complete durability, or unchanged target state without separate proof.

#### Scenario: External writer races the operation

r[agent_harness_runtime.hashed_edit_host.publication.external_race]
- GIVEN another process can write outside the Animus resource lock after the final read
- WHEN receipts and documentation describe the operation
- THEN they MUST retain the external-writer race as an explicit non-claim
- AND the Animus writer generation MUST NOT be presented as a filesystem-wide compare-and-swap.

### Requirement: Concrete edit receipts are metadata-only

r[agent_harness_runtime.hashed_edit_host.evidence] Edit-host receipts MUST contain bounded root, resource, revision, before, after, plan, visibility, writer-generation, publication, durability, redaction, non-claim, and BLAKE3 identity metadata only.

#### Scenario: Safe edit-host receipt is projected

r[agent_harness_runtime.hashed_edit_host.evidence.safe]
- GIVEN an edit operation reaches a validated terminal publication outcome
- WHEN receipt projection runs
- THEN the receipt MUST bind only exact safe facts and mandatory non-claims
- AND it MUST not claim semantic correctness, universal exclusion, authorization, sandboxing, or production readiness.

#### Scenario: Receipt contains file or path material

r[agent_harness_runtime.hashed_edit_host.evidence.rejects]
- GIVEN a receipt candidate contains file bodies, replacement text, absolute paths, credentials, VCS data, process data, raw errors, or unsupported atomicity and durability claims
- WHEN receipt validation runs
- THEN Animus MUST reject it without echoing the value
- AND the candidate MUST NOT enter session, completion, lifecycle, or release evidence.

### Requirement: Concrete edit validation includes successful and failed filesystem behavior

r[agent_harness_runtime.hashed_edit_host.verification] The change MUST include positive and negative production-path tests for capability admission, snapshot text shape, visibility, pure byte application, second-read freshness, writer serialization, publication stages, durability failures, external races, safe receipts, feature isolation, and dependency boundaries.

#### Scenario: Complete edit-host corpus passes

r[agent_harness_runtime.hashed_edit_host.verification.complete]
- GIVEN fixtures cover visible edits, supported line endings, stale reads, hidden spans, path escape, policy denial, publication failures, uncertain durability, external races, and leaks
- WHEN focused pure-core, host-shell, filesystem, dependency, and receipt checks run
- THEN allowed cases MUST produce exact deterministic bytes and outcomes and denied cases MUST fail before prohibited mutation or claims
- AND repeated pure application over equal inputs MUST produce equal BLAKE3 identities.

#### Scenario: Closeout evidence is incomplete

r[agent_harness_runtime.hashed_edit_host.verification.incomplete]
- GIVEN maintainers intend to archive without exact dependency review, path denial, hidden-span, second-read, post-publication, external-race, leak, feature-isolation, Cairn, or relevant Nix evidence
- WHEN closeout validation runs
- THEN archive readiness MUST remain blocked with the missing evidence or exact unavailable-rail blocker
- AND universal compare-and-swap, semantic correctness, sandboxing, durable storage correctness, and production readiness MUST remain explicit non-claims.
