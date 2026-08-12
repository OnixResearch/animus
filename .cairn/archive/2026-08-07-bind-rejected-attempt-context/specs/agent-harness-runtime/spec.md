# Rejected Attempt Context Delta

## ADDED Requirements

### Requirement: Rejected attempt context binds external review evidence

r[agent_harness_runtime.rejected_attempt.boundary] Animus MUST treat rejected-attempt status as supplied review evidence. A validated Cairn revision handoff or a policy-declared equivalent MUST bind the attempt before Animus projects rejection context.

#### Scenario: Cairn handoff is admitted

- GIVEN a Cairn revision handoff binds prior findings, dispositions, and one revision attempt
- WHEN Animus validates the rejected-attempt descriptor
- THEN it can use the handoff as bounded review evidence

#### Scenario: Commit text alone is insufficient

- GIVEN a VCS revision description contains the word rejected but no admitted review evidence exists
- WHEN Animus validates the descriptor
- THEN it denies rejected-attempt status

### Requirement: Attempt identities are nominal and VCS-neutral

r[agent_harness_runtime.rejected_attempt.model] Animus MUST distinguish task, run, attempt, VCS provider, revision, workspace snapshot, content, review handoff, summary, and receipt identities. Reusable contracts MUST NOT require `jj`, Git, a branch, or a pull request.

#### Scenario: A current `jj` attempt maps safely

- GIVEN a host supplies a current `jj` change identity and matching workspace facts
- WHEN the VCS-neutral descriptor is built
- THEN the descriptor preserves the provider and revision identity without granting VCS authority

#### Scenario: Rewritten attempt is stale

- GIVEN a descriptor names content or workspace facts that no longer match the supplied revision
- WHEN freshness validation runs
- THEN Animus denies the stale attempt

### Requirement: Rejection projection is bounded and redaction-first

r[agent_harness_runtime.rejected_attempt.policy] A versioned Nickel policy MUST control accepted review profiles, VCS providers, freshness, metadata fields, summary limits, raw diff admission, redaction, and non-claims. Raw diffs MUST be denied by default.

#### Scenario: Bounded summary is selected

- GIVEN review evidence includes a safe summary within policy limits
- WHEN context projection runs
- THEN Animus can include the summary and records all omissions

#### Scenario: Unsafe diff is denied

- GIVEN a raw diff contains denied classifications, exceeds limits, or lacks explicit capability admission
- WHEN context projection runs
- THEN Animus omits the diff and records the denial safely

### Requirement: Projection planning has no host effects

r[agent_harness_runtime.rejected_attempt.planning] The pure planner MUST operate on already-loaded review, VCS, classification, and policy facts. It MUST NOT inspect repositories, read diffs, mutate tasks, create runs, or assemble provider requests.

#### Scenario: Supplied facts produce a context plan

- GIVEN current admitted review and attempt facts
- WHEN the planner evaluates them
- THEN it returns a bounded context plan with no host effects

### Requirement: Host services retain review and VCS effects

r[agent_harness_runtime.rejected_attempt.host] Host adapters MUST own Cairn receipt loading, VCS observation, diff reading, redaction input, prompt assembly, and any mutation. Each effect MUST use explicit capabilities and limits.

#### Scenario: Missing host service fails closed

- GIVEN a context plan needs current VCS evidence but no service is available
- WHEN the host prepares the context
- THEN it denies the projection without ambient repository discovery

### Requirement: Rejection evidence grants no lifecycle authority

r[agent_harness_runtime.rejected_attempt.evidence] Rejected-attempt receipts MUST be metadata-only and MUST NOT mark a task open or complete, authorize retry, create a run, mutate VCS state, or prove that a later attempt fixes the findings.

#### Scenario: Receipt remains advisory

- GIVEN a valid rejected-attempt receipt
- WHEN another component evaluates lifecycle or execution authority
- THEN the receipt is supporting context only

### Requirement: Positive and negative rejection fixtures are required

r[agent_harness_runtime.rejected_attempt.verification] The focused rail MUST include positive handoff and bounded-summary cases plus negative stale, ambiguous, unsafe, oversized, missing-evidence, and overclaim cases.

#### Scenario: Focused rail passes

- GIVEN all required fixtures and host boundaries are present
- WHEN the focused rail runs
- THEN it reports exact supported behavior and non-claims
