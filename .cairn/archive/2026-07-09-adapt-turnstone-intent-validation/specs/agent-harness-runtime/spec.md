# agent-harness-runtime Change Spec: Turnstone intent validation contracts

## Requirements

### Requirement: Intent verdicts describe pending tool-call risk

r[agent_harness_runtime.turnstone_intent_validation.dto] Animus MUST represent intent verdicts with typed call identity, batch identity, intent summary labels, risk level, confidence, recommendation, detector tier, safe evidence references, latency bucket, and reviewer labels.

#### Scenario: Read-only verdict is represented safely

r[agent_harness_runtime.turnstone_intent_validation.dto.read_only]
- GIVEN a pending read-only tool call has a deterministic low-risk verdict
- WHEN verdict DTO validation runs
- THEN Animus accepts the call identity, risk level, confidence, recommendation, detector tier, and safe evidence labels
- AND durable verdict metadata excludes raw arguments, raw prompts, provider payloads, credentials, and full paths.

### Requirement: Heuristic rules are typed and fixture-backed

r[agent_harness_runtime.turnstone_intent_validation.heuristics] Intent heuristic rules MUST carry stable IDs, action classes, match classes, risk levels, recommendations, safe rationale labels, and positive and negative examples.

#### Scenario: Dangerous process command rule is validated

r[agent_harness_runtime.turnstone_intent_validation.heuristics.fixtures]
- GIVEN a heuristic rule describes a dangerous process command class
- WHEN rule validation runs
- THEN positive examples match the declared class and negative examples do not match it
- AND malformed match classes, missing examples, unknown risks, and unsafe rationale text are rejected.

### Requirement: Heuristic evaluation is deterministic

r[agent_harness_runtime.turnstone_intent_validation.evaluate] Animus MUST evaluate deterministic intent heuristics for read-only, mutation, process, network, secret-adjacent, and persistent control-plane tool classes without model, filesystem, network, process, clock, or provider access.

#### Scenario: Secret-adjacent access requires review or denial

r[agent_harness_runtime.turnstone_intent_validation.evaluate.secret_adjacent]
- GIVEN a pending tool call targets a secret-adjacent resource class
- WHEN deterministic heuristic evaluation runs
- THEN Animus returns a review or deny recommendation according to policy
- AND no executor, provider, network, filesystem, or shell effect occurs during evaluation.

### Requirement: Optional reviewers can only tighten admission

r[agent_harness_runtime.turnstone_intent_validation.reviewer] Optional reviewer verdicts MUST NOT approve calls denied by deterministic policy, lower deterministic critical findings, grant missing authority, suppress required confirmation, or bypass sandbox and capability checks.

#### Scenario: Reviewer approval cannot override deterministic denial

r[agent_harness_runtime.turnstone_intent_validation.reviewer.no_override]
- GIVEN deterministic heuristics produce a deny verdict for a critical tool call
- AND an optional reviewer recommends approval
- WHEN verdict merge runs
- THEN the merged outcome remains denied or review-required
- AND the reviewer result is recorded only as advisory metadata.

### Requirement: Batch admission considers cross-call risk

r[agent_harness_runtime.turnstone_intent_validation.batch] Intent validation MUST bind verdicts to a tool-call batch identity and allow batch-level policy to require review when individually allowed calls combine into a risky flow.

#### Scenario: Read and network calls require batch review

r[agent_harness_runtime.turnstone_intent_validation.batch.interaction]
- GIVEN one pending call reads sensitive-adjacent data and another pending call performs network output
- WHEN batch intent admission evaluates the call set
- THEN Animus can require review for the batch before any call executes
- AND partial approval cannot dispatch a risky suffix before batch admission completes.

### Requirement: Intent verdict receipts are safe metadata only

r[agent_harness_runtime.turnstone_intent_validation.receipts] Intent-validation receipts and runtime events MUST store safe labels, counts, policy hashes, redaction markers, BLAKE3 evidence identities, recommendation classes, and decision classes only.

#### Scenario: Receipt rejects raw command body

r[agent_harness_runtime.turnstone_intent_validation.receipts.no_raw_command]
- GIVEN a verdict receipt draft includes a raw command body or raw tool arguments
- WHEN receipt validation runs
- THEN Animus rejects the receipt with a leak diagnostic
- AND durable evidence stores only command fingerprints, safe labels, and decision classes.

### Requirement: Intent validation docs reject smart approvals

r[agent_harness_runtime.turnstone_intent_validation.docs] Documentation for the intent-validation adaptation MUST state that smart approvals, live judge execution, approval bypass, and raw command or tool bodies in durable evidence are non-claims.

#### Scenario: Architecture note preserves gate authority

r[agent_harness_runtime.turnstone_intent_validation.docs.gate]
- GIVEN maintainers review intent-validation documentation
- WHEN they inspect the authority boundary
- THEN it states that verdicts are evidence and not authority grants
- AND normal capability, policy, confirmation, sandbox, mutation, and executor checks remain required before dispatch.

### Requirement: Intent validation verification covers positive and negative cases

r[agent_harness_runtime.turnstone_intent_validation.verification] The intent-validation change MUST include positive low-risk/read-only and review-required mutation tests plus negative critical-command, secret-adjacent, stale-verdict, classifier-downgrade, malformed-confidence, missing-evidence, and receipt-leak tests before archive.

#### Scenario: Closeout evidence names no-approval-bypass tests

r[agent_harness_runtime.turnstone_intent_validation.verification.closeout]
- GIVEN maintainers prepare to archive intent-validation work
- WHEN closeout evidence is reviewed
- THEN it names DTOs, heuristics, evaluation, reviewer merge, batch admission, receipts, docs, positive tests, negative tests, Cairn validation, and Cairn gates
- AND it records smart approvals and live LLM judge execution as non-claims.
