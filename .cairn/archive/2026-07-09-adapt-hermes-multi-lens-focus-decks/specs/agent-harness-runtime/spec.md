# agent-harness-runtime Change Spec: Hermes multi-lens focus deck contracts

## Requirements

### Requirement: Multi-lens focus decks are typed optional descriptors

r[agent_harness_runtime.hermes_multi_lens.focus_decks] Animus MUST represent multi-lens analysis as optional typed descriptors for focus decks, focus items, lens task packets, lens reports, synthesis reports, contradictions, and review criteria.

#### Scenario: Host-defined focus deck validates

r[agent_harness_runtime.hermes_multi_lens.focus_decks.host_defined]
- GIVEN a host defines a security-review deck with focus IDs, descriptions, look-for criteria, retrieval budget references, and output contracts
- WHEN focus deck validation runs
- THEN Animus accepts the deck as typed data
- AND it does not require Hermes' built-in focus lists, specific agent names, or parallel execution.

### Requirement: Focus deck validation bounds analysis inputs

r[agent_harness_runtime.hermes_multi_lens.validation] Focus deck and lens task validation MUST reject duplicate focus IDs, empty or oversized descriptions, missing evidence expectations, unsupported output contracts, unsafe context packets, untrusted deck bodies, invalid retrieval-budget references, and prompt bodies that contain credentials, provider payloads, full paths, or unbounded source material.

#### Scenario: Unsafe lens task context is rejected

r[agent_harness_runtime.hermes_multi_lens.validation.unsafe_context]
- GIVEN a lens task packet attempts to include a full transcript, raw provider response, credential, or unbounded file body
- WHEN task packet validation runs
- THEN validation returns a typed unsafe-context diagnostic
- AND the lens task is not admitted for worker dispatch or prompt assembly.

### Requirement: Lens execution remains host-owned

r[agent_harness_runtime.hermes_multi_lens.host_owned] Lens execution MUST be planned through injected worker, checklist, or external analysis services, and reusable Animus crates MUST NOT spawn subagents, call models, create processes, read repositories, open networks, or enforce parallelism by default.

#### Scenario: Missing worker service returns a plan failure

r[agent_harness_runtime.hermes_multi_lens.host_owned.missing_service]
- GIVEN a focus deck requests parallel worker execution
- AND the host injects no worker or analysis service
- WHEN execution planning runs
- THEN Animus returns a missing-service or sequential-unavailable diagnostic
- AND it does not fall back to local model calls, shell commands, threads, or hidden task execution.

### Requirement: Synthesis preserves provenance and contradictions

r[agent_harness_runtime.hermes_multi_lens.synthesis] Multi-lens synthesis MUST preserve lens IDs, inspected source labels, evidence hashes, assumptions, skipped lenses, contradictions, confidence labels, and unresolved evidence gaps, and MUST NOT collapse disagreement into an unsupported final claim.

#### Scenario: Conflicting reports stay explicit

r[agent_harness_runtime.hermes_multi_lens.synthesis.contradiction]
- GIVEN two lens reports disagree about whether a boundary is host-owned
- WHEN synthesis runs without resolving evidence
- THEN the synthesis report records the contradiction, involved lens IDs, safe evidence identities, and unresolved status
- AND it does not mark the finding as confirmed.

### Requirement: Multi-lens verification

r[agent_harness_runtime.hermes_multi_lens.verification] The multi-lens adaptation MUST include focused positive and negative tests for valid focus decks, custom deck families, deck bounds, retrieval-budget links, lens task packets, missing execution services, unsafe context rejection, synthesis provenance, contradiction handling, skipped lenses, and no baked-in Hermes focus set.

#### Scenario: Closeout evidence covers optional analysis contracts

r[agent_harness_runtime.hermes_multi_lens.verification.closeout]
- GIVEN maintainers intend to archive the multi-lens adaptation
- WHEN closeout validation runs
- THEN evidence names deck DTO, validation, host-owned execution, synthesis, docs, Cairn validation, and Cairn gate checks
- AND it records as non-claims any built-in subagent spawning, mandatory focus decks, property-discovery default behavior, and live analysis service support.
