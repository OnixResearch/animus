# agent-harness-runtime Change Spec: Ornith scaffold profile adaptation

## Requirements

### Requirement: Reviewed scaffold profile contract

r[agent_harness_runtime.ornith_scaffold.profile_contract] Animus MUST represent runtime scaffold strategies as typed reviewed data with stable profile identity, revision identity, task-class labels, supported strategy knobs, provenance, review status, route constraints, and BLAKE3 content hashes.

#### Scenario: Invalid scaffold profile fails closed

r[agent_harness_runtime.ornith_scaffold.profile_contract.invalid]
- GIVEN a scaffold profile fixture contains malformed IDs, duplicate active revisions, unsupported strategy knobs, missing provenance, an unreviewed active revision, or a mismatched content hash
- WHEN scaffold profile validation runs
- THEN validation returns a typed scaffold-profile error
- AND the profile is not eligible for prompt, Steel, runtime, or evaluation influence.

### Requirement: Deterministic scaffold selection

r[agent_harness_runtime.ornith_scaffold.selection] Animus MUST select scaffold profiles through pure deterministic planning over in-memory profiles, task-class input, host policy, and route capability labels before task execution begins.

#### Scenario: Route policy controls scaffold eligibility

r[agent_harness_runtime.ornith_scaffold.selection.policy]
- GIVEN accepted scaffold profiles have task-class labels and route capability constraints
- WHEN scaffold selection evaluates a task and model route
- THEN the selected profile is compatible with both the task class and route policy
- AND disabled, stale, or incompatible profiles are rejected with deterministic diagnostics.

### Requirement: Host-admitted scaffold revision proposals

r[agent_harness_runtime.ornith_scaffold.admission] Animus MAY allow runtime or evaluation flows to propose scaffold revisions, but proposed revisions MUST require an injected host admission outcome before persistence, activation, or future prompt influence.

#### Scenario: Unreviewed revision cannot influence future turns

r[agent_harness_runtime.ornith_scaffold.admission.unreviewed]
- GIVEN a model or evaluator proposes a new scaffold revision
- WHEN the host admission service is absent, denies the proposal, or reports a stale base revision
- THEN Animus records the unavailable, denied, or stale outcome
- AND the proposed revision is not persisted, activated, or included in future prompt/Steel inputs.

### Requirement: Scaffold influence receipts

r[agent_harness_runtime.ornith_scaffold.receipts] Runtime and evaluation receipts MUST record scaffold influence using safe metadata such as profile ID, revision ID, task class, review status, route labels, validation outcome, and BLAKE3 hashes, and MUST NOT include raw prompt bodies, scaffold bodies, provider payloads, credentials, full paths, raw tool bodies, or unbounded outputs.

#### Scenario: Scaffold receipt rejects sensitive bodies

r[agent_harness_runtime.ornith_scaffold.receipts.redacted]
- GIVEN a scaffold influence receipt fixture contains a raw prompt, scaffold body, provider payload, credential-looking value, absolute path dump, or unbounded tool output
- WHEN receipt validation runs
- THEN validation fails with a typed redaction diagnostic
- AND the receipt is not accepted as runtime or evaluation evidence.

### Requirement: Route-neutral scaffold scope

r[agent_harness_runtime.ornith_scaffold.scope] Animus scaffold profiles MUST remain route-neutral reviewed strategy data and MUST NOT require reusable runtime code to depend on a concrete provider, model family, training harness, or executable model-authored scaffold.

#### Scenario: Scaffold scope excludes executable harnesses

r[agent_harness_runtime.ornith_scaffold.scope.executable_rejected]
- GIVEN a scaffold profile attempts to include executable code, shell commands, provider-specific client construction, or verifier mutation instructions
- WHEN scaffold profile validation evaluates the profile
- THEN validation rejects the profile before runtime use
- AND no model, tool, provider, filesystem, process, or network adapter is called.

### Requirement: Scaffold adaptation verification

r[agent_harness_runtime.ornith_scaffold.verification] The Ornith scaffold adaptation MUST include focused positive and negative tests for profile validation, selection policy, admission outcomes, receipt redaction, route-neutral scope, and executable-harness rejection before archive.

#### Scenario: Closeout evidence covers scaffold profile behavior

r[agent_harness_runtime.ornith_scaffold.verification.closeout]
- GIVEN maintainers intend to archive the scaffold profile adaptation
- WHEN closeout validation runs
- THEN evidence names focused scaffold, admission, receipt, workspace, roadmap, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
