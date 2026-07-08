# agent-harness-runtime Change Spec: Dogfoodable host readiness roadmap

## Requirements

### Requirement: Dogfoodability gap inventory

r[agent_harness_runtime.dogfood_readiness.gap_inventory] Animus MUST maintain a Clankers-informed dogfoodability gap inventory that classifies each compared operational surface as an actionable blocker, intentional non-claim, rejected default product surface, or optional future host adapter.

#### Scenario: Clankers gaps are classified without copying product defaults

r[agent_harness_runtime.dogfood_readiness.gap_inventory.classified]
- GIVEN maintainers compare Animus against Clankers product behavior
- WHEN the dogfoodability inventory is updated
- THEN missing CLI, provider, tool, persistence, TUI, daemon, plugin, documentation, and readiness-harness surfaces are each classified
- AND rejected Clankers product surfaces remain rejected unless a focused successor Cairn changes their status.

### Requirement: Non-default product-surface scope

r[agent_harness_runtime.dogfood_readiness.non_default_scope] Dogfoodability work MUST stay non-default and host-owned, and MUST NOT make daemon, TUI, plugin, provider-router, OAuth, Matrix/P2P, global session-store, self-evolution, or release-tagging product behavior required by reusable Animus crates.

#### Scenario: Host-owned dogfood shell preserves reusable boundaries

r[agent_harness_runtime.dogfood_readiness.non_default_scope.boundary]
- GIVEN a host shell is added to make Animus dogfoodable
- WHEN reusable crate dependencies and exports are reviewed
- THEN daemon, TUI, plugin, router, OAuth, Matrix, global-state, and self-evolution behavior are absent from reusable default crates
- AND any such behavior is feature-gated or delegated to an explicit host-owned adapter.

### Requirement: Minimum dogfood host shell

r[agent_harness_runtime.dogfood_readiness.minimum_host_shell] The first dogfood target SHOULD be a non-default headless or CLI host shell that demonstrates typed prompt input, injected model dispatch, admitted tool metadata, ordered runtime events, final output selection, and safe receipt metadata using deterministic adapters before live providers.

#### Scenario: Deterministic headless smoke exercises the request lifecycle

r[agent_harness_runtime.dogfood_readiness.minimum_host_shell.fake_smoke]
- GIVEN a deterministic fake model adapter and safe tool catalog are configured
- WHEN the non-default headless host shell runs a prompt
- THEN it emits ordered runtime events, safe final output metadata, and a bounded receipt
- AND it does not read credentials, discover providers, open the network, spawn processes, or persist sessions by ambient default.

### Requirement: Operational readiness matrix

r[agent_harness_runtime.dogfood_readiness.readiness_matrix] Animus MUST maintain an operational readiness matrix mapping provider execution, tool execution, session persistence, receipts, documentation/examples, TUI, daemon/attach, plugin/MCP runtime, and release evidence to prerequisite accepted contracts and successor Cairns.

#### Scenario: Operational gap maps to prerequisites

r[agent_harness_runtime.dogfood_readiness.readiness_matrix.prerequisites]
- GIVEN a maintainer wants to claim a live provider, built-in tool, persistent session, TUI, daemon, plugin/MCP, or release-readiness surface
- WHEN the readiness matrix is checked
- THEN the surface names accepted adapter, injected-service, receipt, ledger, policy, host-shell, and validation prerequisites
- AND missing prerequisites block the readiness claim or require an explicit waiver with a replacement requirement ID.

### Requirement: Successor Cairn ordering

r[agent_harness_runtime.dogfood_readiness.successor_order] Successor Cairns for operational dogfood surfaces MUST declare prerequisite accepted requirement IDs and MUST land reusable runtime, injected service, safe receipt, ledger, policy, and concrete-adapter contracts before product-surface readiness claims.

#### Scenario: Product-surface change cannot bypass safety contracts

r[agent_harness_runtime.dogfood_readiness.successor_order.blocks_shortcut]
- GIVEN a successor Cairn proposes live providers, daemon/attach, TUI dogfood, plugin runtime, or persistent sessions
- WHEN the roadmap checks prerequisites
- THEN the change is blocked unless prerequisite contracts are accepted, archived in the same batch, or explicitly waived with safe replacement requirement IDs
- AND no product-surface readiness claim is accepted from implementation code alone.

### Requirement: Readiness evidence ladder

r[agent_harness_runtime.dogfood_readiness.evidence_ladder] Animus readiness claims MUST identify the highest completed evidence rung: deterministic fixture checks, fake-backed integration, packaged headless smoke, optional live-provider smoke, optional TUI dogfood, or current-head release evidence index.

#### Scenario: Evidence rung limits the claim

r[agent_harness_runtime.dogfood_readiness.evidence_ladder.limited_claim]
- GIVEN only deterministic fixture checks and fake-backed integration have passed
- WHEN documentation or release evidence describes the surface
- THEN it may claim deterministic and fake-backed coverage only
- AND it MUST NOT claim live-provider, TUI dogfood, daemon, plugin-runtime, production, or Clankers parity readiness.

### Requirement: Claim hygiene validation

r[agent_harness_runtime.dogfood_readiness.claim_hygiene] Animus SHOULD provide focused validation fixtures or a rail that rejects Clankers parity, production readiness, live-provider, daemon, plugin-runtime, persistent-session, or TUI dogfood claims when matching indexed evidence is absent.

#### Scenario: Unsupported parity claim fails validation

r[agent_harness_runtime.dogfood_readiness.claim_hygiene.rejects_overclaim]
- GIVEN a doc, evidence index, or release note claims Clankers parity or a product surface without matching evidence
- WHEN claim-hygiene validation runs
- THEN validation fails with deterministic issue codes
- AND diagnostics name the missing evidence class without exposing raw prompts, credentials, provider payloads, full paths, or unbounded logs.

### Requirement: Dogfoodability roadmap verification

r[agent_harness_runtime.dogfood_readiness.verification] The dogfoodability roadmap change MUST pass Cairn validation, proposal/design/tasks gates, roadmap validation, and any claim-hygiene focused rail before archive, and MUST record unavailable checks as explicit non-claims with blockers and next-best validation.

#### Scenario: Closeout evidence is reviewable

r[agent_harness_runtime.dogfood_readiness.verification.closeout]
- GIVEN maintainers intend to archive the dogfoodability roadmap change
- WHEN closeout evidence is reviewed
- THEN it names Cairn validation, proposal/design/tasks gates, roadmap validation, and claim-hygiene validation results
- AND any skipped live-provider, TUI, daemon, plugin, VM, or release-index check is recorded as a non-claim.
