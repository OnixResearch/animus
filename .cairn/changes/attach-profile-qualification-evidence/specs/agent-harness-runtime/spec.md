## ADDED Requirements

### Requirement: Authority-free qualification boundary
r[agent_harness_runtime.qualification_attachment.boundary] Animus MUST expose qualification attachments as the pure `animus_runtime::qualification` module, compiled under every `animus-runtime` feature set including `--no-default-features --features steel`, with no clock, filesystem, process, network, credential, provider, or persistence authority and no dependency beyond `blake3`.

#### Scenario: Consumer feature set
- GIVEN a consumer that depends on `animus-runtime` with `default-features = false, features = ["steel"]`
- WHEN it builds
- THEN the validator, applicability, and replay functions are available without a new dependency

### Requirement: Complete versioned qualification subject
r[agent_harness_runtime.qualification_attachment.subject] An `animus.qualification-subject.v1` subject MUST carry the Lattice definition identity spelled `blake3:<64 lowercase hex>`, the Lattice profile revision spelled as bare 64 lowercase hex, a task family that follows Lattice's capability name rule, and every closed component kind exactly once (agent definition, semantic role, prompt, skills, strategy, memory policy, harness, tool schemas, tool implementation, executable, environment, model route, context, output and sampling controls, permissions, isolation, network, task protocol, corpus, verifier, selection policy, budget, topology, evaluation configuration), each declared, observed, or unavailable. Hosted-model facts MUST be provider-declared or unavailable and MUST NOT claim a verified weight identity.

#### Scenario: Other spelling of a subject key
- GIVEN a subject whose profile revision carries a `blake3:` prefix, or whose definition identity lacks it
- WHEN the attachment is encoded or validated
- THEN it is rejected as `malformed`

### Requirement: Canonical evidence attachments
r[agent_harness_runtime.qualification_attachment.attachment] An `animus.qualification-attachment.v1` attachment MUST be canonical JSON-subset bytes whose identity is `blake3:` over those bytes, and MUST hold one independent evidence class (`declared_support`, `contract_conformance`, `task_family_evaluation`, or `deployment_admission`) with exact protocol, verifier, corpus, and source identities, outcome counts whose parts sum to the planned denominator, scope, limitations, missing modes, provenance with freshness, and bounded references. Evaluation and conformance evidence MUST bind its protocol, verifier, and corpus to observed subject components. Unknown fields, non-canonical spellings, and out-of-shape classes MUST be rejected as `malformed`.

#### Scenario: Authority or throughput field
- GIVEN an attachment with an extra `activate_revision` or throughput field
- WHEN it is decoded
- THEN it is rejected as `malformed`

### Requirement: Redacted bounded exports
r[agent_harness_runtime.qualification_attachment.redaction] Every attachment string MUST be a bounded lowercase token or a `blake3:` identity, so raw prompts, hidden tests, credentials, provider bodies, and host paths cannot be represented. Credential-looking tokens MUST be rejected.

#### Scenario: Host path in a limitation
- GIVEN an attachment whose limitation is an absolute host path
- WHEN it is decoded
- THEN it is rejected as `malformed`

### Requirement: Complete measures with honest denominators
r[agent_harness_runtime.qualification_attachment.measures] Task-family measures MUST keep failed, cancelled, and incomplete trials in the denominator, MUST record unmeasured costs and times as unknown rather than zero, and MUST leave cost per accepted completion undefined when nothing was accepted or any cost is unknown. Equal-budget comparisons MUST require identical task family, denominator, and control components and MUST compare independently accepted completions only.

#### Scenario: Faster route with fewer accepted completions
- GIVEN a candidate with fewer accepted completions under identical controls
- WHEN it is compared with the baseline
- THEN the comparison reports no accepted gain

### Requirement: Applicability over the complete subject
r[agent_harness_runtime.qualification_attachment.applicability] Applicability MUST extend evaluation-feedback freshness to every subject component: a changed component, profile revision, or expired freshness makes evidence historical, and an unavailable required component makes it unusable. Reuse across revisions or components MUST require an explicit compatibility decision scoped to the exact attachment, revision pair, task family, and class that waives every changed component and no evaluator component, and such reuse MUST NOT strengthen claims or become current.

#### Scenario: Changed prompt under a new revision
- GIVEN evidence for one revision and a current subject with a new revision and prompt
- WHEN applicability is classified without a decision
- THEN the evidence is historical

### Requirement: Pure consumer validator
r[agent_harness_runtime.qualification_attachment.validator] `validate_attachment(bytes, expected)` MUST return the attachment only when its subject keys equal the expected keys exactly, its task family is the expected family, its class is the required class, no required or current component is unavailable, it is fresh, and no supplied current component changed. Otherwise it MUST return one closed reason: `malformed`, `subject_mismatch`, `scope_mismatch`, `insufficient_class`, `unusable`, `stale`, or `conflicting`.

#### Scenario: Cross-profile attachment
- GIVEN an attachment for another profile revision
- WHEN it is validated for the current revision
- THEN it is rejected as `subject_mismatch`

### Requirement: Replay without rerun
r[agent_harness_runtime.qualification_attachment.replay] Replay and set qualification MUST only decode stored bytes: exact duplicates collapse to one identity, distinct attachments that claim one source for the same subject are `conflicting`, and no model, verifier, or evaluation runs and no missing fact is filled in.

#### Scenario: Conflicting claims over one run
- GIVEN two attachments for the same subject and source with different outcome counts
- WHEN they are qualified together
- THEN the answer is `conflicting`

### Requirement: Recommendations carry no authority
r[agent_harness_runtime.qualification_attachment.authority] A favorable attachment MUST NOT activate a revision, change runtime selection policy, widen capabilities, bypass current revocation, or replace per-candidate acceptance; current evidence may only request separate review or admission.

#### Scenario: Current favorable evidence
- GIVEN a current task-family evaluation
- WHEN its review authority is evaluated
- THEN it may request review but activates nothing

### Requirement: One existing producer lane
r[agent_harness_runtime.qualification_attachment.producer] The campaign-manager evaluation lane MUST produce `task_family_evaluation` attachments from a finished job report, its task set, and a host binding of subject keys and freshness, counting accepted completions from the separate verifier's reduced rewards, without running another trial or evaluation engine.

#### Scenario: Scripted job
- GIVEN the scripted job over the checked task set
- WHEN attachments are produced for the scripted routes
- THEN they equal the checked fixtures byte for byte
