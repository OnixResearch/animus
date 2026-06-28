# agent-harness-runtime Change Spec: Clankers injected effect services

## Requirements

### Requirement: Runtime dispatch uses injected services

r[agent_harness_runtime.clankers_injected_services.contracts] Animus runtime dispatch MUST consume injected model/provider, tool, host-effect, capability, cancellation, confirmation, and runtime-policy services through typed contracts rather than constructing concrete clients or reading ambient host state.

#### Scenario: Service contracts are explicit

r[agent_harness_runtime.clankers_injected_services.contracts.explicit]
- GIVEN runtime code needs model, provider, tool, host-effect, confirmation, cancellation, or policy behavior
- WHEN the boundary is reviewed
- THEN the behavior is represented by an injected trait, adapter, or typed service handle
- AND reusable runtime code does not depend on concrete provider clients, credential stores, shell tools, plugin runtimes, or host persistence.

### Requirement: Missing or denied services fail closed

r[agent_harness_runtime.clankers_injected_services.fail_closed] Missing, unavailable, denied, cancelled, timed-out, malformed-output, and completed service outcomes MUST be typed and MUST prevent false success receipts or later side effects when the outcome is not completed.

#### Scenario: Non-completed services block effects

r[agent_harness_runtime.clankers_injected_services.fail_closed.blocks]
- GIVEN a runtime action requires an injected service
- WHEN the service is absent, unavailable, denied, cancelled, timed out, or returns malformed output
- THEN runtime returns the corresponding typed outcome
- AND no receipt or provenance fact claims the effect completed.

### Requirement: Confirmation denies by default

r[agent_harness_runtime.clankers_injected_services.confirmation] Protected actions that require confirmation MUST deny by default when no broker is injected or when the broker reports unavailable, timed-out, or cancelled.

#### Scenario: Missing confirmation broker prevents protected action

r[agent_harness_runtime.clankers_injected_services.confirmation.default_deny]
- GIVEN a protected action requires human or host confirmation
- WHEN no confirmation broker is available
- THEN runtime records a denied decision
- AND the protected side effect is not executed.

### Requirement: Provider boundary forbids ambient clients

r[agent_harness_runtime.clankers_injected_services.provider_boundary] Provider dispatch tests or rails MUST prove reusable runtime code does not construct provider clients, read credentials, discover provider config, or open network connections directly.

#### Scenario: Provider dispatch goes through the injected service

r[agent_harness_runtime.clankers_injected_services.provider_boundary.injected]
- GIVEN a model/provider request is dispatched
- WHEN focused boundary tests run
- THEN the runtime calls only the injected service with a validated adapter-neutral request
- AND ambient credential lookup, provider discovery, network opening, and concrete client construction are absent from reusable runtime code.

### Requirement: Tool and effect boundaries fail before unsafe execution

r[agent_harness_runtime.clankers_injected_services.tool_boundary] Tool/effect dispatch tests MUST cover missing executor, unavailable service, capability denial, cancellation, timeout, and malformed output.

#### Scenario: Tool denial stops the turn safely

r[agent_harness_runtime.clankers_injected_services.tool_boundary.denied]
- GIVEN a loop decision requests a tool or effect denied by capability policy
- WHEN dispatch evaluates the decision
- THEN a typed capability-denied outcome is returned
- AND later model, provider, or tool calls for that denied effect are not executed.

### Requirement: Effect receipts are redacted

r[agent_harness_runtime.clankers_injected_services.receipts] Service and effect receipts MUST record safe outcome classes and MUST omit raw provider bodies, credentials, environment values, full paths, raw tool arguments, and raw tool output.

#### Scenario: Injected-service closeout is evidenced

r[agent_harness_runtime.clankers_injected_services.closeout]
- GIVEN the injected service change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes provider-boundary tests, tool/effect negative tests, receipt redaction tests, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.
