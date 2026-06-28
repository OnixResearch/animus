# agent-harness-runtime Change Spec: Clankers release evidence index evaluation

## Requirements

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
