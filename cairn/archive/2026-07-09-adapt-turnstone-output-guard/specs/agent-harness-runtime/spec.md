# agent-harness-runtime Change Spec: Turnstone output guard contracts

## Requirements

### Requirement: Output assessments describe safe tool-result risk

r[agent_harness_runtime.turnstone_output_guard.dto] Animus MUST represent tool-result output assessments with typed risk levels, detector flags, annotation labels, redaction state, bounded counts, safe output references, detector tier labels, and BLAKE3 evidence hashes.

#### Scenario: Clean bounded output is assessed

r[agent_harness_runtime.turnstone_output_guard.dto.clean]
- GIVEN a tool response contains bounded clean text and a safe output reference
- WHEN output assessment DTO validation runs
- THEN Animus accepts low-risk metadata, zero detector flags, bounded counts, and the safe reference
- AND durable metadata excludes raw private paths, credentials, provider payloads, and unbounded output.

### Requirement: Deterministic scanner finds unsafe output classes

r[agent_harness_runtime.turnstone_output_guard.scanner] Animus MUST provide a deterministic scanner core for prompt-injection markers, credential leakage, encoded payloads, adversarial URLs, unsafe references, and over-budget output summaries over already-bounded host-supplied text.

#### Scenario: Prompt injection marker is detected

r[agent_harness_runtime.turnstone_output_guard.scanner.prompt_injection]
- GIVEN bounded tool output contains an instruction-override marker from untrusted content
- WHEN the deterministic scanner evaluates the output
- THEN Animus returns a prompt-injection flag with high risk
- AND it does not execute tools, fetch URLs, call providers, or persist the raw output body.

### Requirement: Optional classifier findings cannot lower deterministic findings

r[agent_harness_runtime.turnstone_output_guard.merge] Output-guard merge logic MUST preserve deterministic findings and credential redaction requirements when optional classifier or judge assessments are absent, failed, or lower risk.

#### Scenario: Classifier disagrees with deterministic credential finding

r[agent_harness_runtime.turnstone_output_guard.merge.no_downgrade]
- GIVEN the deterministic scanner flags a credential leak
- AND an optional classifier labels the output benign
- WHEN assessment merge runs
- THEN the merged assessment keeps the credential flag and redaction requirement
- AND the classifier disagreement is represented only as an advisory annotation.

### Requirement: Model-visible projection respects assessment policy

r[agent_harness_runtime.turnstone_output_guard.projection] Tool-output projection into model-visible context MUST require an allowed assessment outcome, satisfied redaction policy, bounded output status, and safe output-reference validation.

#### Scenario: Redacted credential output is projected safely

r[agent_harness_runtime.turnstone_output_guard.projection.redacted]
- GIVEN a tool result contains a credential-like value and policy allows redacted projection
- WHEN projection planning runs after assessment
- THEN Animus replaces the sensitive span with a redaction marker before model visibility
- AND the original credential value is not included in model-visible text or receipt text.

### Requirement: Output guard receipts are safe metadata only

r[agent_harness_runtime.turnstone_output_guard.receipts] Output-guard runtime events and receipts MUST contain detector flags, risk labels, bounded counts, output hashes, redaction markers, detector tier labels, and safe references only.

#### Scenario: Receipt rejects raw output body

r[agent_harness_runtime.turnstone_output_guard.receipts.no_raw_output]
- GIVEN a receipt draft includes raw tool output or an unsafe output reference body
- WHEN receipt validation runs
- THEN Animus rejects the receipt with a leak diagnostic
- AND no raw output body is accepted as durable evidence.

### Requirement: Output guard documentation scopes Turnstone adaptation

r[agent_harness_runtime.turnstone_output_guard.docs] Documentation for the Turnstone output-guard adaptation MUST state that Turnstone is a bounded reference and that product UI warning chips, live classifier calls, web fetching, shell execution, and raw output storage remain non-claims.

#### Scenario: Architecture note names non-claims

r[agent_harness_runtime.turnstone_output_guard.docs.non_claims]
- GIVEN maintainers review the output guard architecture note
- WHEN they inspect the non-claims section
- THEN it names product UI warnings, live classifiers, web fetching, shell execution, and raw output storage as outside reusable Animus scope
- AND it identifies the reusable surface as pure DTO validation, scanning, merge, projection planning, and receipt validation.

### Requirement: Output guard verification covers positive and negative cases

r[agent_harness_runtime.turnstone_output_guard.verification] The output-guard change MUST include positive clean-output and redacted-credential tests plus negative prompt-injection, encoded-payload, adversarial-URL, unsafe-reference, classifier-downgrade, over-budget, and receipt-leak tests before archive.

#### Scenario: Closeout evidence names scanner and leak tests

r[agent_harness_runtime.turnstone_output_guard.verification.closeout]
- GIVEN maintainers prepare to archive output-guard work
- WHEN closeout evidence is reviewed
- THEN it names DTO, scanner, merge, projection, receipt, documentation, positive tests, negative tests, Cairn validation, and Cairn gates
- AND it records live classifier execution and product UI behavior as non-claims.
