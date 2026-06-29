# agent-harness-runtime Change Spec: Zerostack context file layering adaptation

## Requirements

### Requirement: Typed context file classes

r[agent_harness_runtime.zerostack_context_layering.classes] Animus MUST represent discovered context files as typed descriptors with context class, source identity, trust scope, precedence, freshness metadata, size metadata, redaction classification, and prompt-inclusion policy.

#### Scenario: Unknown context class is withheld

r[agent_harness_runtime.zerostack_context_layering.classes.unknown]
- GIVEN a discovered context descriptor has an unknown class or missing source identity
- WHEN prompt assembly evaluates it
- THEN the descriptor is rejected or withheld with a typed diagnostic
- AND its body is not injected into model-visible context.

### Requirement: Host-owned discovery plans

r[agent_harness_runtime.zerostack_context_layering.discovery] Directory walking, file reading, path canonicalization, ignore handling, and global/project context discovery MUST remain host-owned behavior that produces in-memory descriptors for reusable runtime logic.

#### Scenario: Core performs no ancestor walk

r[agent_harness_runtime.zerostack_context_layering.discovery.no_io]
- GIVEN reusable prompt assembly receives context descriptors in memory
- WHEN it assembles prompt sections
- THEN it does not inspect the current directory, walk ancestors, read files, or consult environment variables
- AND missing discovery data is represented as an injected-service or descriptor absence.

### Requirement: Admission before prompt influence

r[agent_harness_runtime.zerostack_context_layering.admission] Discovered context files MUST pass trust, size, freshness, redaction, and precedence policy before their body text can influence model-visible prompt sections.

#### Scenario: Secret-looking body is redacted or rejected

r[agent_harness_runtime.zerostack_context_layering.admission.secret]
- GIVEN a discovered context body matches policy-defined sensitive-content signals
- WHEN admission evaluates the descriptor
- THEN the context is rejected or redacted according to policy
- AND raw sensitive body text is not included in prompt receipts.

### Requirement: Deterministic precedence and conflict handling

r[agent_harness_runtime.zerostack_context_layering.precedence] Prompt assembly MUST apply deterministic precedence when multiple admitted context descriptors overlap, and MUST expose conflict diagnostics when policy requires review.

#### Scenario: Higher precedence guidance wins deterministically

r[agent_harness_runtime.zerostack_context_layering.precedence.conflict]
- GIVEN two admitted context descriptors provide conflicting guidance for the same prompt section
- WHEN precedence policy selects a winner
- THEN the selected descriptor is deterministic
- AND the omitted descriptor is recorded with a conflict or lower-precedence reason.

### Requirement: Context layering receipts

r[agent_harness_runtime.zerostack_context_layering.receipts] Context layering receipts MUST record included and omitted context descriptor IDs, context classes, source identity class, BLAKE3 body hashes, precedence decisions, and omission diagnostics without embedding raw private file bodies or full private paths by default.

#### Scenario: Omitted ancestor file is auditable

r[agent_harness_runtime.zerostack_context_layering.receipts.omitted]
- GIVEN an ancestor context file is discovered but omitted as untrusted, stale, oversized, disabled, malformed, or unsafe
- WHEN a context layering receipt is emitted
- THEN the omission reason and safe source identity are recorded
- AND the raw file body and full private path are omitted unless policy explicitly permits them.
