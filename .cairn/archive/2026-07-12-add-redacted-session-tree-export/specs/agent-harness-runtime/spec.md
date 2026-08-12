# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: Session-tree export starts from validated Animus data

r[agent_harness_runtime.session_tree_export.source] Human-readable session-tree export MUST consume an already-validated Animus session snapshot, replay report, active-leaf selection, safe ledger projection, and optional safe receipt references, and MUST NOT read journals or external provider/agent session formats inside pure export logic.

#### Scenario: Validated session source is supplied

- GIVEN a validated branchable Animus session snapshot and current active leaf
- WHEN export source admission runs
- THEN it MAY produce a typed export source for projection
- AND no filesystem, provider, Tau, Pi, Claude Code, or other external transcript read may occur.

#### Scenario: Raw journal path is supplied as source

- GIVEN the caller supplies a file path or unvalidated raw private journal instead of a validated source
- WHEN pure export admission runs
- THEN it MUST reject the source
- AND it MUST not infer the format or read the path.

### Requirement: Session-tree export policy is redaction-first

r[agent_harness_runtime.session_tree_export.policy] Animus MUST author session-tree export policy in typed Nickel, and the default and initial supported profile MUST exclude raw system, user, assistant, reasoning, tool, provider, credential, environment, private-path, error, and private custom-entry material.

#### Scenario: Default redacted policy projects a private session

- GIVEN a validated session contains private message and tool bodies
- WHEN the default export policy is applied
- THEN projection MUST emit only admitted safe summaries, kinds, counts, statuses, omissions, and BLAKE3 identities
- AND private bodies MUST be absent from the artifact and receipt.

#### Scenario: Policy requests unsupported full fidelity

- GIVEN a policy requests raw prompts, tool arguments or outputs, hidden reasoning, provider payloads, credentials, or private paths
- WHEN policy validation runs for the initial exporter
- THEN validation MUST fail as unsupported or unsafe
- AND it MUST not silently widen the redacted profile.

### Requirement: Session-tree projection preserves topology and storage order

r[agent_harness_runtime.session_tree_export.projection] Pure export projection MUST deterministically represent roots, parent-child links, branch points, active path, active leaf, compaction and branch summaries, repair markers, admitted statuses, omissions, and a storage-order detail view over validated entries.

#### Scenario: Diverged session is projected

- GIVEN a validated session has one root, multiple sibling branches, and a selected active leaf
- WHEN projection runs
- THEN the tree view MUST mark the active path and preserve sibling relationships
- AND the detail view MUST preserve validated storage order without flattening siblings into active context.

#### Scenario: Session topology is malformed

- GIVEN entries contain duplicate IDs, a cycle, missing required parent, stale active leaf, stale repair target, unsupported kind, or inconsistent hash
- WHEN projection validation runs
- THEN it MUST return a bounded diagnostics-only result or error according to policy
- AND it MUST not render the data as a valid session tree.

### Requirement: HTML export is escaped, bounded, and offline

r[agent_harness_runtime.session_tree_export.html] Session-tree HTML MUST be self-contained, script-free, remote-resource-free, semantically structured, fully escaped for text and attributes, and bounded by validated tree, entry, depth, summary, and artifact-size limits.

#### Scenario: Safe tree renders offline

- GIVEN a valid redacted projection within bounds
- WHEN HTML rendering runs
- THEN the artifact MUST contain coordinated tree and storage-order views with fixed semantic markup and no embedded styles
- AND it MUST require no JavaScript, remote font, image, stylesheet, network request, or dynamic code.

#### Scenario: Entry contains markup and script payloads

- GIVEN any safe label or summary contains HTML, script, style, URL, quote, or control-character injection text
- WHEN HTML rendering runs
- THEN the value MUST be escaped or rejected according to policy
- AND it MUST not create executable markup, external navigation, or attribute injection.

### Requirement: Session-tree artifacts are deterministic

r[agent_harness_runtime.session_tree_export.determinism] Identical validated source, policy, safe title/source labels, renderer profile, and format MUST produce byte-identical session-tree artifacts and domain-separated BLAKE3 artifact identities independent of clock, locale, terminal width, environment, random IDs, and host paths.

#### Scenario: Export repeats under different host state

- GIVEN identical export inputs are rendered at different times and under different locale, terminal, and environment settings
- WHEN deterministic rendering runs
- THEN output bytes and artifact identity MUST match
- AND ambient generation timestamps or paths MUST be absent.

### Requirement: Export writes require explicit destination authority

r[agent_harness_runtime.session_tree_export.host_write] Artifact writes MUST occur only in a host shell with an explicit destination capability, overwrite posture, file mode, and byte budget, and write, rename, synchronization, or permission failure MUST not produce a successful receipt.

#### Scenario: Explicit destination write succeeds

- GIVEN rendered bytes are within budget and an admitted destination capability permits create or overwrite according to policy
- WHEN the host writer completes the declared write boundary
- THEN it MAY return a successful write outcome bound to the artifact identity
- AND no source-store or destination path may enter the safe receipt.

#### Scenario: No destination is supplied

- GIVEN no destination capability is present
- WHEN export writing is requested
- THEN the host MUST fail before filesystem mutation
- AND it MUST not default to the current directory, home directory, session directory, or global export directory.

### Requirement: Session-tree export receipts are safe

r[agent_harness_runtime.session_tree_export.receipts] Export receipts MUST contain only schema, policy, snapshot, projection, and artifact identities, admitted and omitted counts, branch and active-path counts, bounded diagnostic classes, format, byte count, redaction status, host-write status, and explicit non-claims.

#### Scenario: Redacted export receipt is built

- GIVEN a valid projection and successful explicit host write
- WHEN receipt projection runs
- THEN it MUST bind the exact policy, snapshot, projection, artifact, format, counts, and redaction state
- AND it MUST state that export is not session correctness, behavioral proof, release evidence, or a training dataset.

#### Scenario: Receipt includes private location or content

- GIVEN receipt input contains source or destination paths, raw prompts, tool data, reasoning, provider payloads, credentials, environment values, private custom entries, or unbounded errors
- WHEN receipt validation runs
- THEN it MUST reject the receipt without echoing the private material.

### Requirement: Session-tree export has adversarial evidence

r[agent_harness_runtime.session_tree_export.verification] Session-tree export MUST include positive and negative tests for linear and branched sessions, active-path selection, compaction, repair, omissions, malformed topology, redaction, injection, bounds, determinism, explicit host writes, failures, and receipt safety.

#### Scenario: Export change is ready for archive

- GIVEN maintainers intend to sync and archive the export change
- WHEN closeout validation runs
- THEN focused runtime, export, session, ledger, compaction, repair, policy, escaping, determinism, host-write, receipt, Cairn, and relevant Nix checks MUST pass
- AND raw-journal backup, dataset export, release evidence, and interactive web serving MUST remain explicit non-goals.
