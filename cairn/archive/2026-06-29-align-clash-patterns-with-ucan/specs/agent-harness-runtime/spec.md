# agent-harness-runtime Change Spec: UCAN-aligned Clash safety patterns

## Requirements

### Requirement: Bounded Clash adaptation scope

r[agent_harness_runtime.clash_ucan.scope] Animus MUST treat Clash as a bounded reference for selected safety-harness patterns, including capability taxonomy, decision explanations, sandbox descriptors, and scenario tests, and MUST NOT adopt Clash's Starlark policy language, full policy engine, brush shell, agent plugin machinery, editor/LSP surfaces, installer flows, audit schema, or precedence-as-authority model as part of this adaptation.

#### Scenario: Scope review rejects wholesale adoption

r[agent_harness_runtime.clash_ucan.scope.review]
- GIVEN a proposed change imports Clash's Starlark DSL, brush shell, plugin infrastructure, or audit schema into Animus runtime crates
- WHEN maintainers compare it with the bounded adaptation scope
- THEN the change is rejected or split until only the approved safety-harness patterns remain
- AND the rejection names the Animus-owned alternative boundary.

### Requirement: UCAN-first capability requests

r[agent_harness_runtime.clash_ucan.capability_request] Guarded runtime actions MUST be represented as typed capability requests with contract, resource, and ability fields before policy or dispatch decisions are trusted.

#### Scenario: Runtime action is normalized before authorization

r[agent_harness_runtime.clash_ucan.capability_request.normalized]
- GIVEN a runtime decision requests a model call, tool invocation, filesystem access, process execution, network access, or Steel evaluation
- WHEN the runtime prepares to authorize the action
- THEN it constructs a typed capability request naming the selected contract, resource, and ability
- AND malformed, missing, or empty request fields fail before adapter or host dispatch.

### Requirement: Nickel contracts and UCAN authority gate local allows

r[agent_harness_runtime.clash_ucan.nickel_contracts] Animus capability policy MUST be authored as Nickel-reviewed contracts or fixtures that define stable resource/ability names for guarded model, tool, filesystem, exec, network, and Steel actions.

#### Scenario: Nickel contract export is the policy source

r[agent_harness_runtime.clash_ucan.nickel_contracts.export]
- GIVEN a maintained capability policy contract source
- WHEN the policy export or drift check runs
- THEN generated runtime policy data matches the reviewed Nickel source
- AND unknown abilities, malformed resource prefixes, unsupported action classes, or missing contracts fail before runtime use.

r[agent_harness_runtime.clash_ucan.ucan_authority] A local policy `allow` MUST NOT authorize a guarded action unless the Nickel contract permits the requested resource and ability and the supplied UCAN grant set authorizes the same resource and ability.

#### Scenario: Local allow without UCAN grant denies

r[agent_harness_runtime.clash_ucan.ucan_authority.missing_grant]
- GIVEN local policy contains an allow caveat for a requested tool action
- AND the request lacks a UCAN grant authorizing the same resource and ability
- WHEN authorization is evaluated
- THEN the action is denied with a typed missing-authority reason
- AND no model, provider, tool, process, network, filesystem, or Steel host dispatch occurs.

### Requirement: Local policy layers are caveats

r[agent_harness_runtime.clash_ucan.local_caveats] User, project, session, and harness policy layers MUST be interpreted as local caveats that can narrow, deny, or prompt for a guarded action, and MUST NOT expand effective authority unless the session/request carries a verified UCAN grant for the expanded resource and ability.

#### Scenario: Session override cannot bypass authority

r[agent_harness_runtime.clash_ucan.local_caveats.session_override]
- GIVEN a session policy layer asks to allow a broader filesystem, network, exec, tool, model, or Steel action than lower layers allow
- WHEN no verified UCAN grant authorizes the broader resource and ability
- THEN the broader action is denied despite the session allow
- AND the receipt records local caveat status separately from UCAN authority status.

#### Scenario: Human approval attaches bounded authority

r[agent_harness_runtime.clash_ucan.local_caveats.ask_grant]
- GIVEN a local caveat returns ask for a guarded action
- WHEN the user approval path produces a bounded session UCAN grant for the requested resource and ability
- THEN the runtime may reevaluate the same capability request with that grant
- AND the resulting receipt identifies the session grant without recording raw prompt or tool input data.

### Requirement: Sandbox descriptors are execution constraints

r[agent_harness_runtime.clash_ucan.sandbox_constraints] Sandbox descriptors MAY be attached to authorized actions as execution constraints, but MUST NOT be treated as authorization proof, and runtime dispatch MUST fail closed when a required sandbox descriptor is malformed, denied by policy, or cannot be enforced by the selected shell/platform adapter.

#### Scenario: Required sandbox is unavailable

r[agent_harness_runtime.clash_ucan.sandbox_constraints.unavailable]
- GIVEN a capability request is authorized by Nickel contracts and UCAN grants
- AND the selected local caveat requires a sandbox descriptor
- WHEN the shell/platform adapter reports that the descriptor is malformed or unenforceable
- THEN runtime dispatch denies the action before host execution
- AND the receipt records the sandbox issue without claiming that UCAN authorization failed.

### Requirement: Safe authorization receipts

r[agent_harness_runtime.clash_ucan.receipts] Authorization receipts MUST carry safe metadata only, including status, issue code, contract id, ability label, redacted resource identifier or BLAKE3 resource hash, policy hash, checked-capability evidence, local caveat results, sandbox descriptor hash when present, and BLAKE3 receipt hash, and MUST NOT include raw prompts, provider payloads, credentials, full paths, raw tool bodies, raw model outputs, or unbounded outputs.

#### Scenario: Authorized receipt carries authority evidence

r[agent_harness_runtime.clash_ucan.receipts.authorized]
- GIVEN a guarded action is allowed after Nickel contract checks, UCAN grant checks, local caveat checks, and required sandbox checks
- WHEN the authorization receipt is emitted
- THEN it records the allowed status, contract id, ability, checked UCAN grants, policy hash, caveat summary, optional sandbox descriptor hash, and receipt hash
- AND sensitive resource, prompt, payload, and output material is omitted or represented only by approved hashes/redaction markers.

#### Scenario: Receipt validation rejects leaks

r[agent_harness_runtime.clash_ucan.receipts.rejects_leaks]
- GIVEN a receipt fixture contains a raw prompt, credential-looking field, absolute path dump, provider payload, raw tool body, raw model output, or unbounded output
- WHEN receipt validation runs
- THEN validation fails with a typed redaction error
- AND the receipt cannot be used as review or release evidence.

### Requirement: Focused verification for UCAN-aligned adaptation

r[agent_harness_runtime.clash_ucan.verification] The adaptation MUST include focused positive and negative tests for capability request normalization, Nickel contract validation, UCAN authority checks, local caveat narrowing, session grant behavior, sandbox descriptor enforcement, and receipt redaction.

#### Scenario: Positive and negative fixtures gate closeout

r[agent_harness_runtime.clash_ucan.verification.fixtures]
- GIVEN the UCAN-aligned Clash pattern adaptation is ready for review
- WHEN focused verification runs
- THEN positive fixtures show authorized model/tool actions, local caveat narrowing, bounded session grant authorization, sandbox-constrained dispatch, and redacted receipts
- AND negative fixtures show missing UCAN grants, contract/resource/ability mismatches, local policy denials, malformed or unenforceable sandbox descriptors, and receipt leak attempts fail closed.

r[agent_harness_runtime.clash_ucan.closeout] Closeout evidence MUST include successful focused capability/caveat/sandbox/receipt tests, workspace checks, Cairn validation, and Cairn proposal/design/tasks gates before this adaptation is archived.

#### Scenario: Closeout evidence is reviewable

r[agent_harness_runtime.clash_ucan.closeout.evidence]
- GIVEN maintainers intend to archive the UCAN-aligned Clash pattern adaptation
- WHEN they collect closeout evidence
- THEN the evidence names every successful focused rail, workspace check, and Cairn gate
- AND any skipped rail includes a concrete blocker and next-best validation.
