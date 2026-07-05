# agent-harness-runtime Change Spec: Clankers runtime pattern adaptation

## Requirements

### Requirement: Nickel-governed Steel runtime policy

r[agent_harness_runtime.steel_policy.nickel] Animus Steel runtime policy MUST be authored in Nickel with typed contracts for profile identity, script bindings, runtime budgets, allowed host actions, fallback mode, and receipt redaction.

#### Scenario: Steel policy exports checked runtime data

r[agent_harness_runtime.steel_policy.nickel.export]
- GIVEN a maintained Steel runtime policy under `policy/steel-runtime/`
- WHEN the policy export or drift check runs
- THEN the generated runtime policy data matches the Nickel source
- AND invalid profile names, missing script bindings, non-positive budgets, unknown host actions, or unsupported fallback modes fail before runtime use.

#### Scenario: Steel policy is enforced before execution

r[agent_harness_runtime.steel_policy.nickel.enforced]
- GIVEN a Steel script is selected for runtime turn planning
- WHEN runtime prepares to evaluate that script
- THEN the selected profile, script hash, budget, host-action allowlist, and fallback mode are validated before Steel evaluation
- AND a policy denial produces a typed runtime failure without calling model or tool adapters.

### Requirement: Runtime lifecycle ownership document

r[agent_harness_runtime.lifecycle.ownership] Animus documentation MUST define the request lifecycle and identify the source-of-truth type and owning crate for each transition from runtime input through loop decision, adapter dispatch, event emission, report construction, and receipt creation.

#### Scenario: Lifecycle ownership is reviewable

r[agent_harness_runtime.lifecycle.ownership.reviewable]
- GIVEN a maintainer changes runtime input, Steel decision, adapter dispatch, event, report, or receipt behavior
- WHEN they inspect the lifecycle document
- THEN the document identifies the owning crate, source-of-truth type, and forbidden dependencies for that boundary
- AND it names the focused validation rail that covers the changed boundary.

### Requirement: Safe runtime receipts

r[agent_harness_runtime.receipts.safe_metadata] Runtime and adapter observation receipts MUST expose safe metadata only, including status, issue code, labels, counts, bounded sizes, and BLAKE3 content hashes, and MUST NOT include raw prompts, provider payloads, credentials, full paths, raw tool bodies, or unbounded outputs.

#### Scenario: Receipt redaction is deterministic

r[agent_harness_runtime.receipts.safe_metadata.redacted]
- GIVEN a runtime turn includes prompt text, model output, tool input, tool output, or adapter error details
- WHEN a receipt is constructed
- THEN the receipt contains only approved safe fields and BLAKE3 hashes for sensitive content identity
- AND raw sensitive fields are omitted or replaced with explicit redaction markers.

#### Scenario: Receipt validation rejects leaks

r[agent_harness_runtime.receipts.safe_metadata.rejects_leaks]
- GIVEN a receipt fixture contains raw prompt text, a credential-looking field, an absolute path dump, raw provider payload, or unbounded tool output
- WHEN receipt validation runs
- THEN validation fails with a typed redaction error
- AND no receipt is accepted as release evidence.

### Requirement: Neutral session ledger and resume contract

r[agent_harness_runtime.session_ledger.neutral] Animus MUST define a neutral session ledger contract for model-visible messages, tool results, summaries, replay metadata, and resume errors without depending on daemon, TUI, database, filesystem, provider, or concrete JSONL session-store crates.

#### Scenario: Ledger rebuilds model-visible context

r[agent_harness_runtime.session_ledger.neutral.replay]
- GIVEN an in-memory ledger containing user, assistant, tool, and summary entries
- WHEN the runtime rebuilds model-visible context for a resumed turn
- THEN the resulting request history preserves entry order and supported content semantics
- AND replay metadata remains safe receipt metadata rather than raw storage implementation details.

#### Scenario: Missing ledger service fails closed

r[agent_harness_runtime.session_ledger.neutral.missing_service]
- GIVEN a runtime request requires resume semantics
- WHEN no session ledger service is injected or the requested session is missing
- THEN runtime returns a typed missing-session or unavailable-service failure
- AND no model or tool adapter is called.

### Requirement: Injected provider and tool dispatch services

r[agent_harness_runtime.dispatch.injected_services] Runtime dispatch MUST consume injected model, provider, and tool services through typed adapter contracts and MUST fail closed when a required service is absent, unavailable, capability-denied, cancelled, or returns malformed output.

#### Scenario: Provider dispatch does not construct clients

r[agent_harness_runtime.dispatch.injected_services.provider]
- GIVEN a runtime turn requires model or provider execution
- WHEN dispatch handles the request
- THEN it calls only the injected service with a validated provider-neutral or adapter request
- AND it does not read credentials, discover provider configuration, open network connections directly, or construct concrete provider clients internally.

#### Scenario: Tool dispatch rejects unavailable services

r[agent_harness_runtime.dispatch.injected_services.tool_unavailable]
- GIVEN a loop decision requests a tool that has no injected executor or is denied by capability policy
- WHEN dispatch evaluates that decision
- THEN runtime emits a typed missing-tool, unavailable-service, or capability-denied event
- AND later model, provider, or tool calls for that turn are not executed.

### Requirement: Focused adaptation verification rails

r[agent_harness_runtime.verification.focused_rails] Animus MUST provide focused verification rails for Nickel policy drift, Steel profile validity, receipt redaction invariants, session-ledger replay behavior, and provider/tool dispatch boundary failures before claiming the Clankers pattern adaptation complete.

#### Scenario: Focused rails gate adaptation closeout

r[agent_harness_runtime.verification.focused_rails.closeout]
- GIVEN the Clankers pattern adaptation is ready for review
- WHEN maintainers run closeout validation
- THEN evidence includes successful focused policy, Steel, receipt, ledger, dispatch, workspace test, clippy, flake, Cairn validation, and Cairn gate checks
- AND any skipped rail is documented with a concrete blocker and next-best validation.
