## ADDED Requirements

### Requirement: Strict live-host quality checks

r[agent_harness_runtime.live_host_quality.strict]

Both live host crates MUST pass the full pinned Octet catalog with all targets and all compatible features. Each crate MUST have an independent result. Final acceptance MUST include the combined selection. The gate MUST reject any lint exemption, warning budget, finding baseline, or omitted feature as a substitute for acceptance.

#### Scenario: Independent host results

- GIVEN the selected immutable source and pinned deny-level catalog.
- WHEN the provider and live-agent checks run independently.
- THEN each result identifies its complete package scope without inferring success from a dependency build.

#### Scenario: A remaining finding

- GIVEN a selected target with an Octet finding.
- WHEN the repair records acceptance.
- THEN the strict check fails and blocks completion and publication.

### Requirement: Coherent host components

r[agent_harness_runtime.live_host_quality.structure]

The repair MUST keep deterministic admission, policy, parsing, and transitions inside pure components. Transport, persistence, clocks, and effect execution MUST stay in explicit host adapters. Modules MUST satisfy the pinned layout rules without hidden owner aliases or new ambient capabilities.

#### Scenario: Component ownership

- GIVEN a split request, schema, parser, or session component.
- WHEN a caller uses the public facade.
- THEN its domain decisions remain deterministic and external effects remain behind the existing authority boundary.

### Requirement: Checked portable host behavior

r[agent_harness_runtime.live_host_quality.behavior]

The repair MUST preserve accepted and rejected runtime outcomes for the existing cohorts. Arithmetic and schema traversal MUST remain bounded and explicit. Public quantities MUST use portable numeric contracts. Any required source migration MUST identify affected callers and MUST NOT masquerade as source compatibility. Invalid input MUST return its appropriate error without a panic or state corruption.

#### Scenario: Accepted input

- GIVEN a valid admitted request, schema, or session operation.
- WHEN the repaired implementation processes it.
- THEN it produces the prior semantic result and preserves the existing authority and persistence order.

#### Scenario: Rejected boundary

- GIVEN malformed data, a crossed identity, an exceeded bound, or a value that cannot convert safely.
- WHEN the repaired implementation processes it.
- THEN it returns the appropriate error and leaves protected state unchanged.

### Requirement: Evidence-bound recovery and continuation

r[agent_harness_runtime.live_host_quality.recovery]

The host MUST retain protected pending state when a commit observation fails correlation checks. Successful execution MUST preserve the existing persistence order. Recovery MUST refuse a kernel/session pair whose evidence does not prove safe continuation. A failed continuation that can follow an external effect MUST NOT silently become a fresh or retryable request. These controls MUST NOT claim atomicity across separate journals or reversal of completed effects.

#### Scenario: Rejected observation followed by the exact commit

- GIVEN a pending staged tool result and its exact admitted binding.
- WHEN a crossed observation is rejected before the exact observation arrives.
- THEN the pending result remains available and only the exact observation can complete it.

#### Scenario: Interrupted journal pair

- GIVEN a kernel safe point committed before its matching session tool result.
- WHEN the host reopens the interrupted session.
- THEN recovery refuses unproved continuation without rerunning the tool or contacting the provider.

#### Scenario: Failed provider continuation

- GIVEN an admitted continuation whose snapshot, provider, or terminal persistence attempt fails.
- WHEN a caller attempts another dispatch through the same binding.
- THEN the host refuses that dispatch without an unproved provider retry or fresh-request fallback.

### Requirement: Exact host repair evidence

r[agent_harness_runtime.live_host_quality.evidence]

Evidence MUST identify the source revision, baseline, changed public contracts, accepted and rejected controls, feature scope, strict results, and unresolved limits. Advisory reports and task markers MUST NOT substitute for execution evidence. Publication MUST wait for all required gates.

#### Scenario: Verified repair subject

- GIVEN passing tests, feature checks, strict lint results, host rails, and Cairn validation.
- WHEN the repair records completion.
- THEN the record binds those results to the retained subject without claims about live providers, remote computation, child isolation, or fresh verification.
