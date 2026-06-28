# agent-harness-runtime Change Spec: Clankers tool gateway delivery adaptation

## Requirements

### Requirement: Artifact delivery request DTOs

r[agent_harness_runtime.clankers_tool_gateway.delivery_dto] Animus MUST represent artifact delivery requests with typed artifact handle, artifact kind, target kind, active context label, capability/toolset metadata, policy decision, delivery attempt ID when available, retryability, and adapter outcome.

#### Scenario: Local artifact delivery request normalizes

r[agent_harness_runtime.clankers_tool_gateway.delivery_dto.local]
- GIVEN a tool produces a local artifact handle and requests session-local delivery
- WHEN delivery request normalization runs
- THEN Animus records artifact kind, target kind, safe artifact label, and required capability metadata
- AND raw file contents are not embedded in the request metadata.

### Requirement: Gateway policy rejects unsafe destinations

r[agent_harness_runtime.clankers_tool_gateway.policy] Tool Gateway policy MUST reject unsupported platform targets, missing active contexts, raw destinations, raw credentials, headers, cloud object URLs, webhook URLs, and raw platform payloads before adapter execution.

#### Scenario: Raw webhook destination rejected

r[agent_harness_runtime.clankers_tool_gateway.policy.raw_destination]
- GIVEN a delivery request includes a raw webhook URL or authorization header
- WHEN gateway policy validates the request
- THEN validation returns an unsafe-destination failure
- AND no delivery adapter is invoked.

### Requirement: Adapter-backed delivery attempts

r[agent_harness_runtime.clankers_tool_gateway.adapters] Approved delivery MUST execute through an injected delivery adapter and MUST fail closed when the selected adapter is missing, unavailable, denied, or returns malformed output.

#### Scenario: Missing adapter fails before success receipt

r[agent_harness_runtime.clankers_tool_gateway.adapters.missing]
- GIVEN policy approves a delivery target kind
- AND no adapter is injected for that target kind
- WHEN delivery execution starts
- THEN Animus records a missing-adapter failure
- AND it does not emit a successful delivery receipt.

### Requirement: Safe delivery outbox and retry

r[agent_harness_runtime.clankers_tool_gateway.outbox] Delivery attempts MUST be representable as bounded local outbox records and retry MUST reference attempt IDs, revalidate current policy, and execute only retryable adapter failures.

#### Scenario: Policy-denied attempt is not retryable

r[agent_harness_runtime.clankers_tool_gateway.outbox.non_retryable]
- GIVEN a prior delivery attempt failed because policy rejected the target or credentials
- WHEN retry is requested for that attempt ID
- THEN Animus refuses retry with a non-retryable failure
- AND preserves the original safe failure class.

### Requirement: Safe delivery receipts

r[agent_harness_runtime.clankers_tool_gateway.receipts] Delivery receipts MUST record safe metadata such as attempt ID, status, target kind, artifact kind, safe artifact label, retryability, adapter class, optional platform handle, error class, and BLAKE3 receipt hash, and MUST NOT include raw destinations, tokens, headers, full paths, raw artifact bytes, or platform payloads.

#### Scenario: Delivery receipt leak rejected

r[agent_harness_runtime.clankers_tool_gateway.receipts.rejects_leaks]
- GIVEN a delivery receipt fixture contains a raw destination URL, token, header, full path dump, artifact body, or platform payload
- WHEN receipt validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Tool Gateway delivery verification

r[agent_harness_runtime.clankers_tool_gateway.verification] The adaptation MUST include focused positive and negative tests for delivery DTOs, gateway policy, adapter-backed execution, missing adapters, outbox retry decisions, non-retryable failures, and receipt redaction.

#### Scenario: Closeout evidence covers delivery fixtures

r[agent_harness_runtime.clankers_tool_gateway.verification.closeout]
- GIVEN maintainers intend to archive the Tool Gateway delivery adaptation
- WHEN closeout validation runs
- THEN evidence names focused DTO, policy, adapter, outbox, receipt, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
