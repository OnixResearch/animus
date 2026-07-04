# agent-harness-runtime Change Spec: Adapter conformance rail and contracts index

## Requirements

### Requirement: Contracts index

r[agent_harness_runtime.adapter_conformance.contracts_index] Animus MUST maintain a contracts index for public adapter families that names owner modules, stability or version posture, authority boundary, safe receipt behavior, serialization bridge status, and focused validation checks.

#### Scenario: Contract family has discoverable ownership

r[agent_harness_runtime.adapter_conformance.contracts_index.validation]
- GIVEN a public adapter family is maintained by Animus
- WHEN maintainers inspect the contracts index
- THEN the family has an owner module, authority-boundary note, receipt behavior summary, and focused validation rail
- AND the index does not require concrete host providers, filesystem access, network access, process execution, credentials, or plugin loading to evaluate.

### Requirement: Adapter conformance matrix

r[agent_harness_runtime.adapter_conformance.matrix] Public adapter families MUST declare conformance expectations for happy path, missing service, denied capability, unavailable service, timeout, cancellation, malformed output, oversized output, unsafe reference, schema mismatch, duplicate field, unknown field, and leak rejection cases, or provide explicit safe non-applicability reasons.

#### Scenario: Family coverage includes positive and negative cases

r[agent_harness_runtime.adapter_conformance.matrix.coverage]
- GIVEN a maintained adapter family appears in the conformance matrix
- WHEN focused conformance checks run
- THEN the family has positive coverage and relevant negative coverage for fail-closed behavior
- AND missing negative coverage without a safe non-applicability reason fails the rail.

### Requirement: Focused adapter surfaces

r[agent_harness_runtime.adapter_conformance.focused_preludes] Animus SHOULD provide focused adapter preludes or documented module-level export sets for model, tool, host/status, workspace, security/policy, and receipt surfaces while preserving the flat compatibility facade until a planned migration changes it.

#### Scenario: Embedder can use the smallest adapter surface

r[agent_harness_runtime.adapter_conformance.focused_preludes.validation]
- GIVEN an embedder needs only one adapter family
- WHEN they use the focused prelude or documented export set for that family
- THEN representative DTOs can be constructed without importing unrelated flat-facade types
- AND no concrete provider, filesystem, process, network, credential, UI, or plugin dependency is introduced.

### Requirement: Adapter conformance verification

r[agent_harness_runtime.adapter_conformance.verification] The adapter conformance rail and contracts index change MUST include focused positive and negative tests or inventory checks for contracts indexing, conformance coverage, focused adapter surfaces, facade inventory, workspace checks, and Cairn gates before archive.

#### Scenario: Closeout evidence covers adapter conformance

r[agent_harness_runtime.adapter_conformance.verification.closeout]
- GIVEN maintainers intend to archive adapter conformance indexing
- WHEN closeout validation runs
- THEN evidence names contracts-index, conformance matrix, positive/negative coverage, focused surfaces, inventory, workspace, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
