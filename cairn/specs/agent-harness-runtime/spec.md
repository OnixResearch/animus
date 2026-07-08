# Agent Harness Runtime Specification

## Purpose

Defines the accepted baseline for Animus as a modular Rust agent-harness framework with explicit crate barriers, typed adapter contracts, a Steel decision fabric, and standard-library host shells.

## Requirements

### Requirement: Crate boundary baseline

r[agent_harness_runtime.baseline.crate_boundaries] The workspace MUST preserve explicit crate boundaries between pure core state, implementation-free adapter contracts, optional Preserves boundary serialization, Steel turn decisions, standard-library host orchestration, optional terminal UI hosting, and the root compatibility facade.

#### Scenario: Embedders choose the smallest surface

r[agent_harness_runtime.baseline.crate_boundaries.smallest_dependency]
- GIVEN an embedder needs only deterministic state logic, adapter DTOs, serialization, Steel scripting, std shell orchestration, or TUI hosting
- WHEN they select workspace crates
- THEN each reusable crate exposes only its owned boundary
- AND concrete host, provider, filesystem, process, network, and UI behavior stays outside unrelated reusable crates.

### Requirement: Functional core and imperative shell split

r[agent_harness_runtime.baseline.functional_core] Non-trivial Animus behavior MUST be decomposed into pure deterministic core logic over explicit inputs and thin imperative shells for filesystem, process, environment, network, terminal, credential, clock, and persistence effects.

#### Scenario: Core logic is testable without standing up the world

r[agent_harness_runtime.baseline.functional_core.testable]
- GIVEN runtime, policy, adapter, prompt, provenance, or serialization logic contains branch decisions
- WHEN maintainers add or change that behavior
- THEN the branch decisions are represented as deterministic functions over in-memory values
- AND tests can exercise positive and negative cases without real providers, shells, terminals, credentials, or storage backends.

### Requirement: Adapter-first integration boundary

r[agent_harness_runtime.baseline.adapters] Model, tool, memory, ledger, queue, policy, confirmation, provider, workspace, and host integrations MUST cross typed adapter or service contracts rather than being constructed by reusable runtime core code.

#### Scenario: Missing host services fail closed

r[agent_harness_runtime.baseline.adapters.missing_service]
- GIVEN reusable runtime logic requires a host capability
- WHEN the corresponding adapter or service is absent, denied, unavailable, malformed, or cancelled
- THEN runtime returns a typed failure before claiming completed effects
- AND it does not discover ambient config, read credentials, spawn commands, or contact providers as a fallback.

### Requirement: Nickel-authored process-job profiles

r[agent_harness_runtime.clankers_process_job_profiles.nickel] Animus process-job profile schemas and policy MUST be authored as Nickel contracts or checked fixtures covering profile schema, backend choices, command/program shape, args, cwd, writable paths, environment allowlist, timeout/resource bounds, notification policy, and safe metadata.

#### Scenario: Generated profile policy stays in sync

r[agent_harness_runtime.clankers_process_job_profiles.nickel.drift]
- GIVEN a maintained process-job profile Nickel source and generated runtime fixture
- WHEN drift validation runs
- THEN generated data matches the Nickel source
- AND schema-invalid profiles fail before runtime dispatch.

### Requirement: Pure profile resolution

r[agent_harness_runtime.clankers_process_job_profiles.resolution] Profile resolution MUST be pure over loaded profile sources and policy, producing backend-neutral queue start requests plus safe identity evidence without spawning processes, reading the filesystem, contacting queue backends, or writing storage.

#### Scenario: Valid profile resolves without side effects

r[agent_harness_runtime.clankers_process_job_profiles.resolution.valid]
- GIVEN loaded profile sources and policy contain a valid profile
- WHEN the resolver selects that profile
- THEN it returns a backend-neutral start request and safe identity envelope
- AND no shell, pueue, systemd, filesystem, or storage operation is invoked.

### Requirement: Deterministic source precedence

r[agent_harness_runtime.clankers_process_job_profiles.precedence] Profile source precedence MUST be deterministic and MUST reject duplicate profile names at the same precedence level.

#### Scenario: Duplicate workspace profiles fail closed

r[agent_harness_runtime.clankers_process_job_profiles.precedence.duplicate]
- GIVEN two workspace-level sources define the same profile name
- WHEN profile resolution runs
- THEN resolution fails with a duplicate-profile diagnostic
- AND no arbitrary source order is used to choose a winner.

### Requirement: Profile policy validation

r[agent_harness_runtime.clankers_process_job_profiles.policy] Profile validation MUST reject secret-like environment keys, env keys outside allowed prefixes, cwd or writable-path escapes, unsupported backends, conflicting command/program fields, invalid args, and timeout/resource-limit violations before dispatch.

#### Scenario: Secret environment key rejected

r[agent_harness_runtime.clankers_process_job_profiles.policy.secret_env]
- GIVEN a profile declares an environment key that looks like a token, password, key, or secret
- WHEN profile validation runs
- THEN validation rejects the profile with a secret-environment diagnostic
- AND no backend-neutral start request is produced.

### Requirement: Safe profile identity receipts

r[agent_harness_runtime.clankers_process_job_profiles.identity] Resolved profile identity metadata MUST include schema version, source label, policy hash, profile name, selected backend, and BLAKE3 profile/content identities, and MUST NOT include raw environment values, credentials, full path dumps, or backend-specific secret locators.

#### Scenario: Identity metadata rejects raw env value

r[agent_harness_runtime.clankers_process_job_profiles.identity.rejects_env]
- GIVEN a profile identity fixture includes a raw environment value or credential-looking backend locator
- WHEN identity validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Process-job profile verification

r[agent_harness_runtime.clankers_process_job_profiles.verification] The adaptation MUST include focused positive and negative tests for Nickel policy drift, valid profile resolution, duplicate names, invalid command shapes, secret env keys, path escapes, unsupported backends, resource-limit violations, identity envelopes, and leak rejection.

#### Scenario: Closeout evidence covers profile fixtures

r[agent_harness_runtime.clankers_process_job_profiles.verification.closeout]
- GIVEN maintainers intend to archive the process-job profile adaptation
- WHEN closeout validation runs
- THEN evidence names focused Nickel, resolver, precedence, policy, identity, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

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

### Requirement: Bounded agent-team reference

r[agent_harness_runtime.claude_code_agents.scope] Animus MUST treat Claude Code subagents and background agents as a bounded reference for scoped delegation, and MUST NOT require cloud sessions, inherit full parent context by default, spawn OS processes from reusable core crates, or bypass permission and tool policy.

#### Scenario: Worker cannot bypass parent policy

r[agent_harness_runtime.claude_code_agents.scope.no_bypass]
- GIVEN a worker task requests a tool or mutation outside its allowed capabilities
- WHEN runtime evaluates the worker task or report
- THEN the request is denied or returned as an unresolved mutation intent
- AND no dispatch occurs without normal permission, capability, and mutation-coordination checks.

### Requirement: Scoped worker task contracts

r[agent_harness_runtime.claude_code_agents.task_contract] Worker tasks MUST declare objective, scope, input context packet, allowed tools or capabilities, budgets, expected output schema, parent trace metadata, and authority/grant metadata before worker execution.

#### Scenario: Overbroad worker task is rejected

r[agent_harness_runtime.claude_code_agents.task_contract.overbroad]
- GIVEN a worker task omits scope, allowed capabilities, budget, output schema, or parent trace metadata
- WHEN worker task validation runs
- THEN validation returns typed diagnostics
- AND the worker task is not enqueued.

### Requirement: Minimized worker context packets

r[agent_harness_runtime.claude_code_agents.context_packet] Worker context packets MUST be explicitly assembled from selected parent messages, summaries, file observations, memory descriptors, and tool results, and MUST NOT include full parent transcripts, raw hidden memory, credentials, provider payloads, or unbounded tool outputs by default.

#### Scenario: Parent context leak is rejected

r[agent_harness_runtime.claude_code_agents.context_packet.leak]
- GIVEN a worker context packet attempts to include a full parent transcript or raw credential-looking memory body without policy admission
- WHEN context packet validation runs
- THEN validation rejects the packet with a leak diagnostic
- AND the worker task cannot start with that packet.

### Requirement: Host-injected worker service

r[agent_harness_runtime.claude_code_agents.worker_service] Worker execution MUST use an injected host service for enqueue, status, cancel, and collect operations with typed unavailable-service, cancellation, timeout, and malformed-report failures.

#### Scenario: Missing worker service fails before delegation

r[agent_harness_runtime.claude_code_agents.worker_service.missing]
- GIVEN a coordinator requests worker delegation
- WHEN no worker service is injected
- THEN runtime returns a worker-service-unavailable failure
- AND no fallback thread, process, shell command, model call, or network request is attempted.

### Requirement: Evidence-bearing worker report merge

r[agent_harness_runtime.claude_code_agents.report_merge] Coordinator merge logic MUST validate worker reports against their output schemas, preserve evidence references, surface conflicts, and require policy or operator resolution before treating conflicting or malformed reports as final.

#### Scenario: Conflicting worker reports require arbitration

r[agent_harness_runtime.claude_code_agents.report_merge.conflict]
- GIVEN two worker reports make incompatible claims or propose conflicting mutations
- WHEN coordinator merge evaluates the reports
- THEN the merge result records a structured conflict with evidence references
- AND no conflicting mutation is applied until policy or operator arbitration resolves it.

### Requirement: Bounded workflow command reference

r[agent_harness_runtime.claude_code_commands.scope] Animus MUST treat Claude Code slash commands as a bounded reference for named workflow invocation, and MUST NOT execute command content as shell scripts by default, open pull requests, or treat command prompt text as a substitute for runtime policy enforcement.

#### Scenario: Command text cannot bypass policy

r[agent_harness_runtime.claude_code_commands.scope.no_policy_bypass]
- GIVEN a workflow command asks for a mutating action in its prompt text
- WHEN runtime expands and evaluates the command
- THEN the resulting guarded action still passes tool, permission, and capability policy checks
- AND no shell, VCS, network, or provider side effect occurs solely because command text requested it.

### Requirement: Typed workflow command manifests

r[agent_harness_runtime.claude_code_commands.manifest] Workflow commands MUST be represented as typed manifests with validated name, summary, input schema, capability requirements, side-effect posture, retrieval budget reference, validation gates, output contract, and source/provenance metadata.

#### Scenario: Unsafe manifest is rejected

r[agent_harness_runtime.claude_code_commands.manifest.invalid]
- GIVEN a command manifest has an invalid name, missing input schema, missing side-effect posture, undeclared capability, or absent output contract
- WHEN command manifest validation runs
- THEN validation returns typed diagnostics
- AND the command is not advertised or invokable.

### Requirement: Deterministic command registry

r[agent_harness_runtime.claude_code_commands.registry] Command registry construction MUST deterministically resolve or reject duplicate command names across host, user, and trusted project sources according to explicit policy.

#### Scenario: Duplicate command name reports diagnostic

r[agent_harness_runtime.claude_code_commands.registry.duplicate]
- GIVEN two trusted command manifests advertise the same invocation name
- WHEN registry construction runs
- THEN policy selects a deterministic winner or disables the duplicate name
- AND diagnostics include source IDs without exposing unsafe paths or raw command bodies.

### Requirement: Pure argument binding before expansion

r[agent_harness_runtime.claude_code_commands.binding] Workflow invocation arguments MUST be bound and validated against the command input schema before prompt expansion, tool dispatch, or workflow state mutation.

#### Scenario: Malformed command arguments stop expansion

r[agent_harness_runtime.claude_code_commands.binding.invalid]
- GIVEN a user invokes a workflow command with missing, unknown, or malformed arguments
- WHEN argument binding runs
- THEN runtime returns a typed binding error
- AND no workflow prompt, tool call, shell action, network call, or VCS mutation is started.

### Requirement: Contract-bound workflow outputs

r[agent_harness_runtime.claude_code_commands.outputs] Workflow commands MUST declare expected output contracts and validation evidence so hosts can distinguish explanation, review, patch summary, validation report, commit-message proposal, and structured DTO outputs.

#### Scenario: Output contract mismatch is reported

r[agent_harness_runtime.claude_code_commands.outputs.mismatch]
- GIVEN a workflow command declares a structured validation-report output
- WHEN the model or tool result cannot be validated against that output contract
- THEN runtime reports an output-contract mismatch
- AND downstream automation does not treat the malformed result as a successful workflow completion.

### Requirement: Confirmation denies by default

r[agent_harness_runtime.clankers_injected_services.confirmation] Protected actions that require confirmation MUST deny by default when no broker is injected or when the broker reports unavailable, timed-out, or cancelled.

#### Scenario: Missing confirmation broker prevents protected action

r[agent_harness_runtime.clankers_injected_services.confirmation.default_deny]
- GIVEN a protected action requires human or host confirmation
- WHEN no confirmation broker is available
- THEN runtime records a denied decision
- AND the protected side effect is not executed.

### Requirement: Typed confirmation requests

r[agent_harness_runtime.clankers_confirmation_broker.requests] Animus confirmation requests MUST be typed protected-action envelopes with request ID, action kind, resource, ability, safe summary, policy hash, session or run identity, required confirmation level, optional expiry, and redaction class.

#### Scenario: Protected mutation requests confirmation

r[agent_harness_runtime.clankers_confirmation_broker.requests.mutation]
- GIVEN a protected mutating tool action requires confirmation by policy
- WHEN runtime prepares the action
- THEN it emits a typed confirmation request with action/resource/ability identity
- AND the raw tool payload is not embedded in the request receipt.

### Requirement: Default deny confirmation behavior

r[agent_harness_runtime.clankers_confirmation_broker.default_deny] Confirmation evaluation MUST fail closed when the broker is absent, unavailable, timed out, cancelled, stale, malformed, or returns no decision.

#### Scenario: Missing broker denies protected action

r[agent_harness_runtime.clankers_confirmation_broker.default_deny.missing]
- GIVEN a protected action requires confirmation
- AND no confirmation broker is injected
- WHEN confirmation evaluation runs
- THEN the protected action is denied
- AND no fallback approval or action dispatch occurs.

### Requirement: Decision binding and freshness

r[agent_harness_runtime.clankers_confirmation_broker.binding] Confirmation decisions MUST bind to the pending request ID, action kind, resource, ability, policy hash, and expiry, and MUST be rejected when stale or mismatched.

#### Scenario: Mismatched approval rejected

r[agent_harness_runtime.clankers_confirmation_broker.binding.mismatch]
- GIVEN a broker returns an approval for a different request ID or resource
- WHEN runtime validates the decision
- THEN the decision is rejected with a mismatched-confirmation failure
- AND the protected action remains unexecuted.

### Requirement: Host-owned confirmation interaction

r[agent_harness_runtime.clankers_confirmation_broker.host_owned] UI prompts, human approval collection, notification channels, external approval services, and durable approval storage MUST remain host-owned adapter behavior outside reusable runtime crates.

#### Scenario: Runtime does not prompt directly

r[agent_harness_runtime.clankers_confirmation_broker.host_owned.no_ui]
- GIVEN a confirmation request is required
- WHEN reusable runtime evaluates the protected action
- THEN it calls the injected broker contract
- AND it does not display UI, read terminal input, send notifications, or write approval storage directly.

### Requirement: Safe confirmation receipts

r[agent_harness_runtime.clankers_confirmation_broker.receipts] Confirmation receipts MUST record safe metadata such as request ID hash, action kind, resource and ability labels or hashes, decision status, broker class, policy hash, timing class, expiry class, and BLAKE3 receipt hash, and MUST NOT include raw prompts, credentials, full paths, provider payloads, raw tool payloads, or secret summaries.

#### Scenario: Confirmation receipt leak rejected

r[agent_harness_runtime.clankers_confirmation_broker.receipts.rejects_leaks]
- GIVEN a confirmation receipt fixture includes raw prompt text, credentials, a full path dump, provider JSON, raw tool payload, or secret-like summary
- WHEN receipt validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Confirmation broker verification

r[agent_harness_runtime.clankers_confirmation_broker.verification] The adaptation MUST include focused positive and negative tests for typed requests, approved and denied decisions, missing brokers, timeouts, cancellations, stale and mismatched decisions, host-owned interaction boundaries, and receipt redaction.

#### Scenario: Closeout evidence covers confirmation fixtures

r[agent_harness_runtime.clankers_confirmation_broker.verification.closeout]
- GIVEN maintainers intend to archive the confirmation broker kit adaptation
- WHEN closeout validation runs
- THEN evidence names focused request, default-deny, binding, host-boundary, receipt, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Typed remote/headless admission requests

r[agent_harness_runtime.clankers_remote_admission.requests] Animus MUST represent remote or headless entrypoint admission as typed requests with audience, actor, entrypoint, session or workspace scope, requested action, resource, ability, credential evidence, local caveats, policy hash, and expiry or window metadata.

#### Scenario: Prompt submission admission normalizes

r[agent_harness_runtime.clankers_remote_admission.requests.prompt]
- GIVEN a headless or remote client requests prompt submission for a session
- WHEN admission normalization runs
- THEN Animus produces a typed admission request with audience, actor, entrypoint, session scope, and prompt-submit ability
- AND the raw prompt body is not embedded in the admission receipt.

### Requirement: Admission before side effects

r[agent_harness_runtime.clankers_remote_admission.before_effects] Admission checks MUST complete before session create, attach or resume, prompt submission, tool execution, memory mutation, artifact delivery, or workflow command side effects occur.

#### Scenario: Missing credential blocks session mutation

r[agent_harness_runtime.clankers_remote_admission.before_effects.missing_credential]
- GIVEN an entrypoint requires a credential or session grant
- AND no credential evidence is supplied
- WHEN admission evaluates the request
- THEN admission denies the request before session state changes
- AND no prompt, tool, memory, delivery, or workflow side effect is invoked.

### Requirement: UCAN-aligned authority and local caveats

r[agent_harness_runtime.clankers_remote_admission.authority] Admission MUST evaluate credential audience, expiry, revocation or verifier status, requested resource/ability, and local caveats, and local caveats MUST NOT expand authority beyond verified grants.

#### Scenario: Wrong audience denied

r[agent_harness_runtime.clankers_remote_admission.authority.wrong_audience]
- GIVEN credential evidence targets a different audience than the current runtime or session entrypoint
- WHEN admission authority validation runs
- THEN admission denies the request with a wrong-audience reason
- AND the local caveat layer cannot override the denial into an allow.

### Requirement: Transport and credential stores are host-owned

r[agent_harness_runtime.clankers_remote_admission.host_owned] Network transports, daemon sockets, iroh, Matrix, MCP, HTTP servers, OAuth stores, credential persistence, revocation databases, and verifier backends MUST remain host-owned adapter behavior outside reusable admission core logic.

#### Scenario: Admission core does not read credential store

r[agent_harness_runtime.clankers_remote_admission.host_owned.no_store]
- GIVEN admission validation receives typed credential evidence
- WHEN reusable admission core evaluates the request
- THEN it validates the supplied evidence and policy inputs
- AND it does not read credential files, open sockets, contact OAuth endpoints, or query revocation storage directly.

### Requirement: Safe admission receipts

r[agent_harness_runtime.clankers_remote_admission.receipts] Admission receipts MUST record safe metadata such as verdict, reason class, entrypoint label, resource and ability labels, audience hash, actor hash, credential hash, caveat summary, policy hash, expiry class, verifier status, and BLAKE3 receipt hash, and MUST NOT include raw tokens, private keys, raw prompts, provider payloads, transport headers, Matrix room IDs, full paths, or credentials.

#### Scenario: Admission receipt leak rejected

r[agent_harness_runtime.clankers_remote_admission.receipts.rejects_leaks]
- GIVEN an admission receipt fixture contains a raw token, private key, prompt body, provider JSON, transport header, Matrix room ID, full path, or credential body
- WHEN receipt validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Remote admission verification

r[agent_harness_runtime.clankers_remote_admission.verification] The adaptation MUST include focused positive and negative tests for request normalization, before-effect gating, authorized admission, missing/expired/revoked/wrong-audience credentials, verifier-unavailable status, local caveat denial, host-owned store/transport boundaries, unsupported entrypoints, and receipt redaction.

#### Scenario: Closeout evidence covers admission fixtures

r[agent_harness_runtime.clankers_remote_admission.verification.closeout]
- GIVEN maintainers intend to archive the remote admission receipt adaptation
- WHEN closeout validation runs
- THEN evidence names focused request, before-effect, authority, host-boundary, receipt, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

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

### Requirement: Typed agent definition manifests

r[agent_harness_runtime.clankers_agent_definitions.manifest] Animus agent definitions MUST be represented as typed manifests with name, description, model preference, prompt fragments, capability ceilings, toolset limits, source scope, trust status, and manifest identity before activation.

#### Scenario: Valid user manifest activates as typed data

r[agent_harness_runtime.clankers_agent_definitions.manifest.valid]
- GIVEN a user-scoped agent definition satisfies the typed manifest contract
- WHEN activation validates the definition
- THEN Animus produces a typed activation candidate with safe manifest identity
- AND raw manifest parsing does not grant tool, model, memory, or filesystem authority.

### Requirement: Deterministic discovery and precedence

r[agent_harness_runtime.clankers_agent_definitions.discovery] Agent-definition discovery MUST be deterministic over loaded sources and MUST reject duplicate definition names at the same precedence instead of selecting by filesystem order.

#### Scenario: Duplicate project definitions fail closed

r[agent_harness_runtime.clankers_agent_definitions.discovery.duplicate]
- GIVEN two project-scoped sources define the same agent name
- WHEN discovery resolves available definitions
- THEN discovery returns a duplicate-definition diagnostic
- AND no arbitrary source is selected.

### Requirement: Project definitions require admission

r[agent_harness_runtime.clankers_agent_definitions.project_trust] Project or workspace agent definitions MUST be untrusted by default and MUST require host confirmation or policy admission before they influence prompt assembly, tool access, model selection, memory, or workflow behavior.

#### Scenario: Unadmitted project agent cannot influence prompt

r[agent_harness_runtime.clankers_agent_definitions.project_trust.denied]
- GIVEN a project-scoped agent definition is discovered
- AND no host admission decision trusts that manifest identity
- WHEN a session requests that agent
- THEN Animus denies activation or activates a safe no-influence failure state
- AND the prompt and tool/model permissions are unchanged.

### Requirement: Manifest capabilities are ceilings

r[agent_harness_runtime.clankers_agent_definitions.capability_ceilings] Agent-definition toolsets, model preferences, memory scopes, context-reference scopes, and workflow abilities MUST be interpreted as ceilings or requests and MUST NOT expand effective authority beyond active capability policy and verified grants.

#### Scenario: Manifest escalation rejected

r[agent_harness_runtime.clankers_agent_definitions.capability_ceilings.escalation]
- GIVEN an agent manifest requests a tool, model role, network context reference, or workflow ability that active policy does not authorize
- WHEN activation evaluates capability ceilings
- THEN the unauthorized ability is rejected or omitted according to policy
- AND the receipt records the denied escalation without applying it.

### Requirement: Safe activation receipts

r[agent_harness_runtime.clankers_agent_definitions.receipts] Agent-definition discovery and activation receipts MUST record safe metadata such as definition name, source scope, trust/admission status, capability ceilings, selected model role label, manifest hash, policy hash, and redaction markers, and MUST NOT include raw prompt bodies, full paths, credentials, provider payloads, or hidden memory bodies.

#### Scenario: Activation receipt leak rejected

r[agent_harness_runtime.clankers_agent_definitions.receipts.rejects_leaks]
- GIVEN an activation receipt fixture includes raw prompt text, a full project path, credentials, provider JSON, or hidden memory body
- WHEN receipt validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Agent definition trust verification

r[agent_harness_runtime.clankers_agent_definitions.verification] The adaptation MUST include focused positive and negative tests for typed manifests, deterministic discovery, duplicate names, project admission, capability ceiling enforcement, malformed manifests, compatibility-import boundaries, and receipt redaction.

#### Scenario: Closeout evidence covers agent-definition fixtures

r[agent_harness_runtime.clankers_agent_definitions.verification.closeout]
- GIVEN maintainers intend to archive the agent definition trust adaptation
- WHEN closeout validation runs
- THEN evidence names focused manifest, discovery, trust, capability, receipt, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

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

### Requirement: Permission modes are bounded UX caveats

r[agent_harness_runtime.claude_code_permissions.scope] Animus MUST treat Claude Code permission modes as a bounded UX reference for allow, ask, and deny outcomes, and MUST NOT treat local permission allows as authority beyond Nickel contract checks and verified UCAN grants.

#### Scenario: Local allow lacks authority

r[agent_harness_runtime.claude_code_permissions.scope.local_allow_denied]
- GIVEN local permission policy says to allow a guarded filesystem, exec, network, model, Steel, or tool action
- AND the requested action lacks matching contract and UCAN authority
- WHEN permission resolution runs
- THEN the action is denied before dispatch
- AND the receipt separates missing authority from local permission status.

### Requirement: Typed permission policy descriptors

r[agent_harness_runtime.claude_code_permissions.policy] Permission policy descriptors MUST map guarded action classes and resource labels to typed allow, ask, or deny outcomes with source identity, trust scope, precedence, and prompt-summary policy.

#### Scenario: Malformed permission policy fails closed

r[agent_harness_runtime.claude_code_permissions.policy.invalid]
- GIVEN a permission descriptor has an unknown action class, missing resource label, unsupported outcome, or missing source identity
- WHEN permission policy validation runs
- THEN validation returns typed diagnostics
- AND the malformed descriptor cannot authorize or prompt for an action.

### Requirement: Pure permission resolution

r[agent_harness_runtime.claude_code_permissions.resolution] Permission resolution MUST be a pure deterministic decision over in-memory action, authority, policy, and prompt-decision inputs that returns execute, prompt, or deny with typed reasons.

#### Scenario: Ask mode pauses before execution

r[agent_harness_runtime.claude_code_permissions.resolution.ask]
- GIVEN authority checks pass and local policy maps the action to ask
- WHEN permission resolution runs without a matching prompt-decision token
- THEN the resolver returns a prompt-required decision
- AND no model, tool, shell, filesystem, or network dispatch occurs.

### Requirement: Host-mediated prompt decisions

r[agent_harness_runtime.claude_code_permissions.prompt] Ask decisions MUST be completed only through an injected host prompt service that receives redacted action summaries and returns scoped prompt-decision tokens before dispatch can continue.

#### Scenario: Stale prompt token is rejected

r[agent_harness_runtime.claude_code_permissions.prompt.stale]
- GIVEN a prompt-decision token was issued for a different action identity, policy hash, session, or resource label
- WHEN permission resolution evaluates the token
- THEN the token is rejected as stale or mismatched
- AND the guarded action remains unexecuted.

### Requirement: Safe permission receipts

r[agent_harness_runtime.claude_code_permissions.receipts] Permission receipts MUST record safe metadata for action class, redacted resource identity or BLAKE3 resource hash, authority status, local outcome, prompt status, policy hash, and decision reasons, and MUST NOT include raw prompts, credentials, provider payloads, raw tool bodies, full paths, or unbounded outputs.

#### Scenario: Denied action emits redacted receipt

r[agent_harness_runtime.claude_code_permissions.receipts.denied]
- GIVEN permission resolution denies a guarded action
- WHEN the runtime emits a permission receipt
- THEN the receipt records denial reason, action class, policy hash, and redacted resource metadata
- AND sensitive request material is omitted or represented only by approved hashes and redaction markers.

### Requirement: Bounded headless automation reference

r[agent_harness_runtime.claude_code_headless.scope] Animus MUST treat Claude Code CLI, CI, and SDK automation as a bounded reference for noninteractive agent runs, and MUST NOT copy Claude Code flags, require cloud sessions, create pull requests by default, or make networked providers mandatory.

#### Scenario: CI run does not open a pull request by default

r[agent_harness_runtime.claude_code_headless.scope.no_default_pr]
- GIVEN a headless run completes with a patch summary or commit-message proposal
- WHEN final automation policy is evaluated
- THEN Animus does not open, merge, ready, or reopen a pull request by default
- AND any VCS or hosting side effect requires an explicit host adapter and policy grant.

### Requirement: Typed headless run request

r[agent_harness_runtime.claude_code_headless.request] Headless run requests MUST declare objective, session or workspace identity, input message or stdin payload references, policy profile, allowed capabilities, output contract, validation plan, and noninteractive behavior before runtime execution.

#### Scenario: Malformed headless request fails before execution

r[agent_harness_runtime.claude_code_headless.request.invalid]
- GIVEN a headless request is missing objective, identity, policy profile, output contract, or noninteractive behavior
- WHEN request validation runs
- THEN validation returns typed diagnostics
- AND no model, tool, shell, network, filesystem, or worker service is called.

### Requirement: Stable headless event DTOs

r[agent_harness_runtime.claude_code_headless.events] Headless runs MUST emit stable event DTOs for lifecycle, prompt assembly, model requests and responses, tool dispatch, permission decisions, background work, validation, receipts, and final response using bounded safe payloads.

#### Scenario: Oversized event payload is bounded

r[agent_harness_runtime.claude_code_headless.events.bounded]
- GIVEN a model, tool, validation, or background event contains large output
- WHEN the headless event is prepared
- THEN model-visible and event-visible payloads are bounded according to policy
- AND safe metadata records truncation state, sizes, and BLAKE3 hashes without embedding raw full bodies.

### Requirement: JSONL and Preserves projections

r[agent_harness_runtime.claude_code_headless.serialization] Headless event DTOs SHOULD provide JSONL and Preserves boundary projections that preserve event identity, ordering metadata, status classes, redaction markers, and BLAKE3 hashes without making either format the core state model.

#### Scenario: Unsupported projection fails safely

r[agent_harness_runtime.claude_code_headless.serialization.unsupported]
- GIVEN a host requests an unsupported headless event projection
- WHEN serialization is selected
- THEN runtime returns a typed unsupported-projection error
- AND the run does not fall back to lossy or unbounded debug text.

### Requirement: Noninteractive completion statuses

r[agent_harness_runtime.claude_code_headless.completion] Headless completion MUST use typed noninteractive status classes for success, denied, prompt-required, input-required, validation-failed, service-unavailable, cancelled, and execution-failed outcomes with safe evidence references.

#### Scenario: Permission ask becomes prompt-required

r[agent_harness_runtime.claude_code_headless.completion.prompt_required]
- GIVEN a noninteractive headless run reaches a permission decision that requires user confirmation
- WHEN policy does not provide a valid preauthorized prompt-decision token
- THEN the run completes with prompt-required status
- AND no guarded action is dispatched after the unresolved prompt.

### Requirement: Rich tool definition metadata

r[agent_harness_runtime.pi_tool_contracts.definition_metadata] Animus tool definitions MUST expose validated name, label, description, parameter schema identity, prompt snippet, prompt guidelines, source/provenance metadata, capability labels, and result metadata shape without embedding concrete tool execution code in the reusable runtime.

#### Scenario: Tool metadata enters prompt construction safely

r[agent_harness_runtime.pi_tool_contracts.definition_metadata.prompt]
- GIVEN a registered tool with prompt snippet and tool-specific guidelines
- WHEN runtime builds model-facing tool context
- THEN only active tool metadata is included
- AND each guideline remains attributable to the tool name or capability it describes.

### Requirement: Argument preparation and strict validation

r[agent_harness_runtime.pi_tool_contracts.argument_validation] Tool-call arguments MUST pass optional pure compatibility preparation before strict validation against the current public parameter contract.

#### Scenario: Legacy arguments normalize before validation

r[agent_harness_runtime.pi_tool_contracts.argument_validation.compat]
- GIVEN a resumed ledger contains an older accepted tool-call argument shape
- WHEN the tool definition provides a compatibility preparer
- THEN runtime validates the prepared current-shape arguments
- AND unknown or malformed fields after preparation fail before tool execution.

### Requirement: Bounded tool output contract

r[agent_harness_runtime.pi_tool_contracts.output_bounds] Tool results MUST provide bounded model-visible output and safe metadata for truncated or full outputs, including sizes, truncation status, and BLAKE3 content hashes when full output identity is needed.

#### Scenario: Oversized tool output is truncated

r[agent_harness_runtime.pi_tool_contracts.output_bounds.truncated]
- GIVEN a tool executor returns output larger than the configured visible-output budget
- WHEN runtime records the result
- THEN model-visible content is truncated according to policy
- AND the result metadata records bounded sizes, truncation status, and a BLAKE3 hash without embedding the full raw body.

### Requirement: Resource mutation serialization

r[agent_harness_runtime.pi_tool_contracts.mutation_queue] Tools that mutate shared resources MUST declare mutation targets so a host-owned mutation coordinator can serialize conflicting mutation windows and avoid lost updates.

#### Scenario: Conflicting mutations serialize

r[agent_harness_runtime.pi_tool_contracts.mutation_queue.conflict]
- GIVEN two parallel tool calls declare the same mutation target
- WHEN runtime dispatches the calls through a mutation coordinator
- THEN the mutation windows execute in a deterministic serialized order
- AND neither call observes a stale pre-mutation state caused by the other call racing.

### Requirement: Typed tool failure classes

r[agent_harness_runtime.pi_tool_contracts.failure_classes] Tool dispatch MUST report typed failures for invalid input, missing executor, unavailable service, capability denial, cancellation, malformed output, oversized unbounded output, and execution failure.

#### Scenario: Missing executor fails before execution

r[agent_harness_runtime.pi_tool_contracts.failure_classes.missing_executor]
- GIVEN a loop decision requests a known tool with no injected executor
- WHEN runtime evaluates the tool dispatch
- THEN runtime emits a missing-executor failure class
- AND no fallback process, shell command, or provider call is attempted.

### Requirement: BLAKE3 snapshot identity

r[agent_harness_runtime.zerostack_hashed_edit.snapshot] Animus hashed edit snapshots MUST identify the edited resource with a full-content BLAKE3 snapshot hash, text normalization mode, line count, and line tag generation metadata before edits can be validated.

#### Scenario: Stale snapshot is rejected

r[agent_harness_runtime.zerostack_hashed_edit.snapshot.stale]
- GIVEN an edit request references an earlier snapshot hash
- AND the host reports a different current snapshot hash for the resource
- WHEN edit validation runs
- THEN validation fails with a stale-snapshot diagnostic
- AND no write or partial mutation is performed.

### Requirement: Tagged span edit requests

r[agent_harness_runtime.zerostack_hashed_edit.tagged_spans] Hashed edit requests MUST reference validated tagged lines or contiguous tagged spans plus bounded replacement text rather than relying on fuzzy old-text matching.

#### Scenario: Unknown tag cannot select text

r[agent_harness_runtime.zerostack_hashed_edit.tagged_spans.unknown]
- GIVEN an edit request contains a tag not present in the supplied snapshot
- WHEN edit-plan validation runs
- THEN validation fails with an unknown-tag diagnostic
- AND the request cannot select or mutate any text.

### Requirement: Pure edit-plan validation

r[agent_harness_runtime.zerostack_hashed_edit.plan_validation] Edit-plan validation MUST be pure deterministic logic over in-memory snapshots and requests, detecting invalid spans, duplicate span selectors, overlapping edits, out-of-order edits, unsupported newline policy, and overlong replacement text.

#### Scenario: Overlapping edits fail closed

r[agent_harness_runtime.zerostack_hashed_edit.plan_validation.overlap]
- GIVEN two requested edits select overlapping tagged spans
- WHEN edit-plan validation runs
- THEN validation fails with an overlap diagnostic
- AND the host receives no executable edit plan.

### Requirement: Host-owned file mutation

r[agent_harness_runtime.zerostack_hashed_edit.host_mutation] Filesystem reads, writes, locks, atomic replacement, permission checks, and path resolution MUST remain host/tool-adapter responsibilities behind injected tool contracts.

#### Scenario: Core validation performs no ambient I/O

r[agent_harness_runtime.zerostack_hashed_edit.host_mutation.no_io]
- GIVEN edit-plan validation receives a snapshot and edit request in memory
- WHEN validation runs
- THEN it does not read files, inspect environment variables, open locks, spawn processes, or write output files
- AND mutation remains unavailable until a host adapter applies the validated plan.

### Requirement: Safe edit receipts

r[agent_harness_runtime.zerostack_hashed_edit.receipts] Hashed edit receipts MUST record resource class, redacted path or BLAKE3 resource hash, before and after snapshot hashes, selected span metadata, replacement size metadata, and diagnostics without embedding raw private file bodies by default.

#### Scenario: Receipt omits full private path

r[agent_harness_runtime.zerostack_hashed_edit.receipts.redacted_path]
- GIVEN an edit targets a private host path
- WHEN an edit receipt is emitted
- THEN the path is omitted, relativized, or represented by a BLAKE3 resource hash according to policy
- AND before/after snapshot hashes remain available for audit.

### Requirement: Safe receipt DTOs

r[agent_harness_runtime.clankers_safe_receipts.dto] Animus runtime and provenance receipts MUST expose safe metadata only, including schema, status, issue code, labels, bounded counts, BLAKE3 hashes, effect or mutation kind, backend/interchange labels, and explicit redaction markers.

#### Scenario: Positive receipts expose only safe evidence

r[agent_harness_runtime.clankers_safe_receipts.positive]
- GIVEN model, tool, authorization, runtime block, final response, adapter error, or provenance observations contain raw sensitive inputs and outputs
- WHEN safe receipts are constructed
- THEN receipts expose only safe labels, bounded counts, status, issue class, redaction markers, and BLAKE3 hashes
- AND the raw sensitive material is omitted.

### Requirement: Receipt leak rejection

r[agent_harness_runtime.clankers_safe_receipts.rejects_leaks] Receipt validation MUST reject or redact raw prompt text, provider JSON, credential-looking values, absolute path dumps, raw tool input, raw tool output, raw model output, and unbounded output before evidence is accepted.

#### Scenario: Leak fixtures fail closed

r[agent_harness_runtime.clankers_safe_receipts.rejects_leaks.fixture]
- GIVEN a receipt fixture includes raw prompt text, provider JSON, token-like values, absolute paths, raw tool/model output, or an oversized body
- WHEN receipt validation runs
- THEN validation returns a typed redaction error or redacted output
- AND the unsafe fixture is not accepted as review or release evidence.

### Requirement: BLAKE3 canonical artifact envelopes

r[agent_harness_runtime.clankers_artifact_identity.envelope] Animus-owned artifact identity MUST use BLAKE3 over versioned canonical envelopes containing artifact kind, envelope version, redaction class, normalized payload, and sorted/deduplicated dependency hashes.

#### Scenario: Artifact identity is deterministic

r[agent_harness_runtime.clankers_artifact_identity.deterministic]
- GIVEN equivalent artifact envelopes with dependencies provided in different orders and with duplicates
- WHEN artifact identity is computed
- THEN dependencies are sorted and deduplicated before hashing
- AND the resulting BLAKE3 hash is stable and round-trips through display/parse validation.

### Requirement: Redaction classes govern review output

r[agent_harness_runtime.clankers_artifact_identity.redaction] Artifact redaction classes MUST determine whether payload bodies may appear in receipts, CLI, or review output.

#### Scenario: Non-public artifacts hide payloads

r[agent_harness_runtime.clankers_artifact_identity.redaction.display]
- GIVEN an artifact is metadata-only, redacted-payload, or secret
- WHEN a receipt or review output is rendered
- THEN payload bodies are omitted or replaced with explicit redaction markers
- AND only public artifacts may expose payload bodies by default.

### Requirement: Focused receipt and artifact rails

r[agent_harness_runtime.clankers_safe_receipts.rails] Animus MUST provide focused verification rails for safe receipt validation, leak rejection, artifact hash determinism, and redaction-class display behavior before claiming the Clankers receipt/artifact pattern adopted.

#### Scenario: Receipt and artifact closeout is evidenced

r[agent_harness_runtime.clankers_safe_receipts.closeout]
- GIVEN the safe receipt and artifact identity change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes focused receipt/artifact rails, positive and negative tests, dependency checks, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Animus-owned neutral ledger DTOs

r[agent_harness_runtime.clankers_neutral_ledger.dto] Animus MUST define its own neutral session-ledger DTOs for model-visible messages, summaries, usage observations, safe receipts, unsupported entries, replay metadata, and typed replay errors without depending on Clankers session, daemon, TUI, provider, Automerge, or concrete storage types.

#### Scenario: DTOs stay storage-neutral

r[agent_harness_runtime.clankers_neutral_ledger.dto.storage_neutral]
- GIVEN reusable Animus ledger DTOs are reviewed
- WHEN dependency and field boundaries are inspected
- THEN the DTOs do not require concrete storage, filesystem paths, wall-clock construction, daemon events, TUI types, provider-native payloads, or Clankers transcript IDs
- AND host persistence can wrap them without changing pure replay semantics.

### Requirement: Pure deterministic replay

r[agent_harness_runtime.clankers_neutral_ledger.replay] Ledger replay MUST be a pure deterministic function over in-memory entries that returns model-visible context plus safe metadata counts or a typed error.

#### Scenario: Supported entries replay in order

r[agent_harness_runtime.clankers_neutral_ledger.replay_positive]
- GIVEN an in-memory ledger containing user, assistant, tool, summary, usage, and safe receipt entries
- WHEN replay runs
- THEN model-visible user, assistant, tool, and summary context is returned in deterministic order
- AND usage and receipt entries contribute safe metadata counts without leaking raw storage details.

### Requirement: Unsupported entries fail closed

r[agent_harness_runtime.clankers_neutral_ledger.unsupported] Replay MUST fail closed on unsupported or adapter-owned ledger entries rather than silently dropping or exposing them.

#### Scenario: Unsupported payloads do not leak

r[agent_harness_runtime.clankers_neutral_ledger.unsupported.redacted]
- GIVEN replay encounters an unsupported or adapter-owned entry
- WHEN replay is required before a model request
- THEN replay returns a typed unsupported-entry error
- AND the error includes only a safe reason class, not the raw unsupported payload.

### Requirement: Resume requires an injected ledger service

r[agent_harness_runtime.clankers_neutral_ledger.missing_service] Runtime paths that require session resume MUST consume an injected ledger service and MUST fail before adapter dispatch when the service or requested session is unavailable.

#### Scenario: Missing ledger service blocks dispatch

r[agent_harness_runtime.clankers_neutral_ledger.missing_service.blocks]
- GIVEN a runtime request requires resume semantics
- WHEN no ledger service is injected or the requested session is missing
- THEN runtime returns a typed unavailable-service or missing-session failure
- AND no model or tool adapter call is made.

### Requirement: Pure branch query helpers when parent links exist

r[agent_harness_runtime.clankers_neutral_ledger.branch_queries] If Animus ledger entries include parent/head relationships, branch query helpers MUST be pure and MUST detect duplicate IDs, missing parents, cycles, active branch ancestry, and abandoned/dead-end attempts.

#### Scenario: Malformed graphs fail without panic

r[agent_harness_runtime.clankers_neutral_ledger.branch_queries.invalid]
- GIVEN a ledger graph has duplicate IDs, a missing parent, a cycle, or reversed-order assumptions
- WHEN query preparation runs
- THEN it returns a typed graph validation error
- AND no panic, infinite traversal, or silent ancestry corruption occurs.

#### Scenario: Ledger closeout is evidenced

r[agent_harness_runtime.clankers_neutral_ledger.closeout]
- GIVEN the neutral ledger change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes focused replay tests, unsupported-entry negative tests, missing-service tests, dependency-isolation checks, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Neutral session tree ledger contract

r[agent_harness_runtime.pi_session_tree.ledger_contract] Animus MUST define a neutral append-only session tree ledger contract for runtime-visible entries, parent links, active leaf selection, labels, summaries, replay metadata, and typed ledger errors without depending on Pi session files, a TUI, a database, provider crates, filesystem traversal, or process-global state.

#### Scenario: Ledger snapshot validates tree shape

r[agent_harness_runtime.pi_session_tree.ledger_contract.validate]
- GIVEN an in-memory ledger snapshot with entry IDs, parent links, and an active leaf
- WHEN the runtime validates the snapshot before replay
- THEN duplicate IDs, missing parents, parent cycles, invalid active leaves, and unsupported entry kinds fail with typed ledger errors
- AND no model or tool adapter is called.

### Requirement: Active-leaf replay context

r[agent_harness_runtime.pi_session_tree.replay_context] Animus MUST rebuild model-visible context by walking from the selected active leaf to the root, preserving root-to-leaf ordering and supported content semantics.

#### Scenario: Replay follows the active branch only

r[agent_harness_runtime.pi_session_tree.replay_context.branch]
- GIVEN a ledger with two branches from a shared ancestor
- WHEN runtime rebuilds context for one active leaf
- THEN only entries on that active branch contribute model-visible messages
- AND sibling branch messages remain available as ledger history but are not sent to the model.

### Requirement: Summary entries preserve bounded context

r[agent_harness_runtime.pi_session_tree.summary_entries] Animus MUST represent compaction summaries and branch handoff summaries as first-class ledger entries with bounded model-visible summary text and safe replay metadata.

#### Scenario: Summary entry replaces older model-visible span

r[agent_harness_runtime.pi_session_tree.summary_entries.replay]
- GIVEN a branch contains earlier messages, a compaction summary, and later kept messages
- WHEN replay context is rebuilt
- THEN the model-visible context includes the summary and the later kept messages in order
- AND it does not include the raw older span replaced by the summary.

### Requirement: Labels remain replay-neutral metadata

r[agent_harness_runtime.pi_session_tree.labels] Animus labels MUST attach operator-facing names to ledger entries without changing model-visible replay by default.

#### Scenario: Label does not steer model context

r[agent_harness_runtime.pi_session_tree.labels.replay_neutral]
- GIVEN a ledger entry has a label used for navigation
- WHEN runtime rebuilds model-visible context
- THEN the label is preserved in metadata queries
- AND the label text is not included in the model request unless an explicit host policy injects it.

### Requirement: Ledger service absence fails closed

r[agent_harness_runtime.pi_session_tree.missing_service] Runtime requests that require resume or replay semantics MUST fail closed with typed missing-ledger, missing-session, or unavailable-ledger-service errors when no suitable ledger service is injected.

#### Scenario: Resume without ledger service halts

r[agent_harness_runtime.pi_session_tree.missing_service.halts]
- GIVEN a runtime input requests resume from a prior session
- WHEN no ledger service is available
- THEN runtime emits a typed missing-ledger-service failure
- AND no Steel fabric, model adapter, or tool adapter is invoked.

### Requirement: Candidate inventory

r[agent_harness_runtime.trellis_verified_core.candidate_inventory] Animus MUST inventory pure safety-critical decision cores that are candidates for Trellis-style verification, including policy/effect gates, ledger graph validation, branch replay, receipt redaction, schema compatibility, compaction cut points, and roadmap dependency checks when those areas are in scope.

#### Scenario: Only pure cores are candidates

r[agent_harness_runtime.trellis_verified_core.candidate_inventory.pure_only]
- GIVEN maintainers nominate a verification candidate
- WHEN the candidate inventory is reviewed
- THEN the candidate identifies explicit inputs and outputs
- AND filesystem, process, provider, network, credential, terminal, clock, and ambient global effects are excluded from the verified core.

### Requirement: Proof obligations

r[agent_harness_runtime.trellis_verified_core.obligations] Each selected verified-core candidate MUST declare invariants, preconditions, postconditions, rejected-state classes, and equivalence expectations before proof or model code is treated as release evidence.

#### Scenario: Safety property is explicit

r[agent_harness_runtime.trellis_verified_core.obligations.explicit]
- GIVEN a candidate such as graph validation, effect authorization, redaction, or schema compatibility is selected
- WHEN maintainers inspect its verification plan
- THEN the plan states what cannot happen, what must happen, and which invalid inputs are rejected
- AND the proof/model artifact links back to requirement IDs.

### Requirement: Dependency isolation

r[agent_harness_runtime.trellis_verified_core.dependency_isolation] Trellis or verified-helper dependencies MUST be optional, test-only, or isolated in dedicated verified modules until a stable verified-core dependency boundary is accepted.

#### Scenario: Proof dependency does not leak into default runtime

r[agent_harness_runtime.trellis_verified_core.dependency_isolation.default_clean]
- GIVEN default Animus crates are built without verification features
- WHEN dependency checks run
- THEN default reusable runtime/core crates do not require Trellis or proof-tool dependencies
- AND verification-enabled builds remain explicit.

### Requirement: Model or verified logic

r[agent_harness_runtime.trellis_verified_core.model_logic] Animus SHOULD provide model or verified-core implementations for selected pure decision boundaries where proof-oriented hardening materially improves safety claims.

#### Scenario: Model logic handles deterministic input

r[agent_harness_runtime.trellis_verified_core.model_logic.deterministic]
- GIVEN a selected pure decision boundary has modeled inputs
- WHEN model or verified logic evaluates those inputs
- THEN it returns deterministic outputs or typed rejection classes
- AND it does not perform I/O, mutation of external state, logging, or adapter dispatch.

### Requirement: Equivalence tests

r[agent_harness_runtime.trellis_verified_core.equivalence] When model or verified logic and production Rust logic both exist for a decision boundary, Animus MUST include equivalence tests for valid and invalid inputs.

#### Scenario: Divergence is reported

r[agent_harness_runtime.trellis_verified_core.equivalence.divergence]
- GIVEN model/verified logic and production Rust evaluate the same fixture
- WHEN their accepted/rejected status or normalized outputs differ
- THEN the test fails with a diagnostic naming the fixture and boundary
- AND production release evidence cannot claim verified coverage for that boundary.

### Requirement: Negative invariant fixtures

r[agent_harness_runtime.trellis_verified_core.negative_fixtures] Verified-core work MUST include negative fixtures for malformed graphs, authorization bypass attempts, unsafe redaction inputs, unsupported compatibility cases, and invariant-violating counterexamples relevant to the selected candidates.

#### Scenario: Counterexample remains durable

r[agent_harness_runtime.trellis_verified_core.negative_fixtures.counterexample]
- GIVEN a rejected state could break safety claims if accepted
- WHEN negative fixtures run
- THEN the state is rejected with a typed issue code
- AND the fixture can be replayed without live host services.

### Requirement: Verified-core closeout evidence

r[agent_harness_runtime.trellis_verified_core.closeout] Trellis verified-core work MUST document proof scope and run verification/model checks, equivalence tests, negative fixtures, dependency-isolation checks, workspace checks, and Cairn gates before archive.

#### Scenario: Verification evidence is reviewable

r[agent_harness_runtime.trellis_verified_core.closeout.evidence]
- GIVEN verified-core work is ready to archive
- WHEN reviewers inspect evidence
- THEN proof scope, model/equivalence checks, negative fixtures, dependency isolation, workspace checks, and Cairn gates are named with status
- AND unsupported proof tooling is recorded as unavailable rather than silently skipped.

### Requirement: Optional dataspace adapter contract

r[agent_harness_runtime.syndicate_dataspace.adapter_contract] Animus MAY provide optional dataspace adapter contracts for publishing bounded runtime assertions, subscribing to safe query patterns, and receiving host orchestration events, but reusable runtime/core crates MUST NOT depend on Syndicate crates or live dataspace transports by default.

#### Scenario: Runtime uses injected dataspace service

r[agent_harness_runtime.syndicate_dataspace.adapter_contract.injected]
- GIVEN runtime wants to publish or observe dataspace state
- WHEN a dataspace backend is configured
- THEN runtime communicates through an injected typed adapter contract
- AND no default runtime/core crate constructs a Syndicate backend or network transport.

### Requirement: Redacted assertion projection

r[agent_harness_runtime.syndicate_dataspace.assertion_projection] Dataspace projections MUST map runtime events, tool states, background work, session/provenance summaries, and agent/subagent lifecycle states into bounded redacted assertion shapes.

#### Scenario: Raw sensitive material is not asserted

r[agent_harness_runtime.syndicate_dataspace.assertion_projection.redacted]
- GIVEN an event or state contains prompt text, provider payloads, credentials, full paths, file bodies, tool input/output, model output, or unbounded logs
- WHEN it is projected to a dataspace assertion
- THEN the assertion includes only approved redaction markers, hashes, counts, stable IDs, and safe status classes
- AND raw sensitive material is omitted by default.

### Requirement: Explicit scoping and lifecycle

r[agent_harness_runtime.syndicate_dataspace.scoping] Dataspace adapter DTOs MUST represent facet, session, tenant, workspace, branch, agent/subagent, cancellation, lease/expiry, stale assertion, and ownership-transfer semantics explicitly before being used for concurrent orchestration.

#### Scenario: Scoped cancellation removes assertions

r[agent_harness_runtime.syndicate_dataspace.scoping.cancellation]
- GIVEN a scoped agent, subagent, tool, or background task is cancelled
- WHEN the dataspace adapter processes the cancellation
- THEN it emits or updates bounded cancellation assertions for the affected scope
- AND unrelated tenant, workspace, branch, or session assertions are not modified.

### Requirement: Fake dataspace backend

r[agent_harness_runtime.syndicate_dataspace.fake_backend] Animus MUST provide deterministic fake-dataspace fixtures before live dataspace transports are treated as release evidence.

#### Scenario: Fake backend exercises ordering and stale cleanup

r[agent_harness_runtime.syndicate_dataspace.fake_backend.deterministic]
- GIVEN publish, subscribe, expiry, cancellation, and malformed-subscription fixtures
- WHEN fake-dataspace tests run
- THEN ordering and cleanup outcomes are deterministic
- AND no live network or daemon is required.

### Requirement: Dataspace tests

r[agent_harness_runtime.syndicate_dataspace.tests] Dataspace adapter work MUST include positive and negative tests for bounded projections, scoped publish/subscribe, cancellation, stale assertions, malformed subscriptions, denied queries, dependency isolation, and safe failure evidence.

#### Scenario: Malformed subscription fails safely

r[agent_harness_runtime.syndicate_dataspace.tests.malformed_subscription]
- GIVEN a subscription requests raw private payloads, crosses tenant/workspace scope, or uses an unsupported query shape
- WHEN the dataspace adapter validates it
- THEN validation fails with a typed issue code
- AND no raw private material is returned.

### Requirement: Dependency isolation

r[agent_harness_runtime.syndicate_dataspace.dependency_isolation] Animus MUST check that Syndicate crates, daemon clients, and live transport dependencies remain out of default reusable runtime/core dependency graphs unless a future accepted change makes them default.

#### Scenario: Syndicate dependency leaks into core

r[agent_harness_runtime.syndicate_dataspace.dependency_isolation.rejects_leak]
- GIVEN a Syndicate or live transport dependency is added to a default reusable runtime/core crate
- WHEN dependency isolation checks run
- THEN the check fails with an actionable diagnostic
- AND maintainers are directed to use the optional dataspace adapter boundary.

### Requirement: Dataspace closeout evidence

r[agent_harness_runtime.syndicate_dataspace.closeout] Syndicate dataspace adapter work MUST document optional live integration configuration and run fake-dataspace tests, dependency-isolation checks, workspace checks, and Cairn gates before archive.

#### Scenario: Dataspace evidence is reviewable

r[agent_harness_runtime.syndicate_dataspace.closeout.evidence]
- GIVEN dataspace adapter work is ready to archive
- WHEN reviewers inspect evidence
- THEN fake-dataspace tests, dependency isolation, workspace checks, and Cairn gates are named with status
- AND live integration is either explicitly configured or marked unavailable with a blocker.

### Requirement: Steel decision fabric baseline

r[agent_harness_runtime.baseline.steel_fabric] Animus SHOULD use Steel Scheme as the default scriptable turn-decision fabric while Rust boundary code MUST revalidate every selected model, tool, memory, response, or host action before dispatch.

#### Scenario: Script decisions are revalidated

r[agent_harness_runtime.baseline.steel_fabric.revalidated]
- GIVEN a Steel script returns a loop decision
- WHEN Rust runtime code receives that decision
- THEN the decision is decoded through typed boundary DTOs
- AND malformed payloads, unsupported actions, unauthorized effects, or invalid text fail closed before adapter dispatch.

### Requirement: Nickel-owned repository configuration

r[agent_harness_runtime.baseline.nickel_config] Repo-owned tool and policy configuration MUST use Nickel as the source of truth when Animus owns the config shape, while generated tool-native artifacts are kept fresh for consumers that require JSON, TOML, YAML, or other native formats.

#### Scenario: Generated config drift is detectable

r[agent_harness_runtime.baseline.nickel_config.drift]
- GIVEN maintainers change repo-owned Nickel configuration
- WHEN generated native config files are checked
- THEN drift is reported deterministically
- AND stale generated config is not treated as authoritative evidence.

### Requirement: Verification safety rails

r[agent_harness_runtime.baseline.verification] The workspace MUST maintain focused positive and negative tests, formatting, linting, Tiger Style, generated-config drift checks, feature checks, and lifecycle validation evidence for behavior that crosses public API, adapter, policy, serialization, or host boundaries.

#### Scenario: Boundary changes include reviewable evidence

r[agent_harness_runtime.baseline.verification.evidence]
- GIVEN a boundary-affecting change is ready for review
- WHEN maintainers inspect its evidence
- THEN the evidence names the focused checks that cover the changed boundary
- AND unavailable checks are recorded with a blocker and next best validation path.

### Requirement: Focused rail convention

r[agent_harness_runtime.clankers_focused_rails.convention] Animus MUST define a focused verification rail convention where each rail validates one boundary, uses deterministic fixtures, emits safe versioned evidence, and avoids live credentials or host-specific state by default.

#### Scenario: Rails have bounded evidence

r[agent_harness_runtime.clankers_focused_rails.convention.evidence]
- GIVEN a focused rail validates an Animus boundary
- WHEN the rail writes evidence
- THEN evidence includes schema, status, fixture names, hashed artifacts, and safe metadata
- AND it omits raw prompts, credentials, provider payloads, full paths, raw tool output, and live host state.

### Requirement: Lifecycle documentation rail

r[agent_harness_runtime.clankers_focused_rails.lifecycle] Animus MUST provide a lifecycle-doc marker rail that checks boundary names, owner markers, forbidden dependency sections, and validation command references.

#### Scenario: Lifecycle rail catches drift

r[agent_harness_runtime.clankers_focused_rails.lifecycle.drift]
- GIVEN the lifecycle document loses a required boundary, owner, forbidden dependency section, or validation marker
- WHEN the lifecycle rail runs
- THEN it fails with an actionable diagnostic
- AND it does not require provider credentials or live services.

### Requirement: Steel runtime policy rail

r[agent_harness_runtime.clankers_focused_rails.steel_policy] Animus MUST provide a focused Steel runtime policy rail that covers Nickel-authored policy export drift, generated-policy activation, current Steel activation, Rust adapter-boundary revalidation, invalid Steel program rejection before adapter dispatch, and an explicit policy-drift hook when a canonical policy source is supplied.

#### Scenario: Steel rail covers positive and negative activation

r[agent_harness_runtime.clankers_focused_rails.steel_policy.fixtures]
- GIVEN current positive and negative Steel runtime fixtures exist
- WHEN the Steel runtime policy rail runs
- THEN valid Steel programs activate safely through Rust DTO validation
- AND invalid Steel programs fail before model or tool adapter dispatch
- AND policy-drift checking is either executed against an explicit canonical source or reported as skipped with a next-best validation path.

### Requirement: Nickel-authored Steel runtime policy

r[agent_harness_runtime.clankers_steel_policy.nickel] Animus Steel runtime profiles MUST be authored in Nickel with typed contracts for profile identity, script bindings, BLAKE3 script hashes, runtime budgets, allowed host actions, fallback mode, and receipt redaction.

#### Scenario: Policy contract accepts valid profiles

r[agent_harness_runtime.clankers_steel_policy.nickel.valid]
- GIVEN a maintained Steel runtime profile under `policy/steel-runtime/`
- WHEN Nickel validation runs
- THEN valid profile identity, script binding, hash, budget, host-action, fallback, and receipt-redaction fields pass
- AND the policy remains reviewable before runtime use.

### Requirement: Checked export and invalid fixtures

r[agent_harness_runtime.clankers_steel_policy.export] Rust runtime tests MUST consume checked policy data or typed fixtures and MUST NOT evaluate Nickel during turn execution.

#### Scenario: Policy export drift fails

r[agent_harness_runtime.clankers_steel_policy.export.drift]
- GIVEN generated policy data exists for Rust consumption
- WHEN the drift check runs
- THEN it fails if generated data differs from Nickel source
- AND it reports a deterministic update path.

#### Scenario: Invalid policies fail before activation

r[agent_harness_runtime.clankers_steel_policy.invalid_fixtures]
- GIVEN a policy fixture has a missing script, bad name, non-positive budget, unknown host action, hash mismatch, path escape, or unsupported fallback mode
- WHEN policy validation runs
- THEN validation returns a typed policy error
- AND Steel is not loaded or evaluated.

### Requirement: Policy enforcement before Steel evaluation

r[agent_harness_runtime.clankers_steel_policy.enforced] Animus MUST validate selected profile, script hash, runtime budget, action allowlist, fallback mode, and receipt redaction before Steel loop evaluation can influence model or tool dispatch.

#### Scenario: Policy denial blocks adapter calls

r[agent_harness_runtime.clankers_steel_policy.enforced.blocks]
- GIVEN a runtime turn selects a Steel script that fails policy activation
- WHEN the runtime prepares the turn
- THEN it returns a typed policy-denied failure
- AND no model or tool adapter call is made.

### Requirement: Redacted policy receipts and rails

r[agent_harness_runtime.clankers_steel_policy.receipts] Steel policy activation receipts MUST expose safe status, profile labels, script hash, policy hash, denied reason class, fallback decision, and receipt hash only.

#### Scenario: Receipts omit raw sensitive material

r[agent_harness_runtime.clankers_steel_policy.receipts.redacted]
- GIVEN activation observes prompt text, script source, provider payloads, credentials, or tool bodies
- WHEN a receipt is rendered
- THEN those values are omitted or represented by redaction markers and BLAKE3 hashes
- AND the receipt remains usable as review evidence.

#### Scenario: Focused rails gate closeout

r[agent_harness_runtime.clankers_steel_policy.rails]
- GIVEN maintainers claim Steel runtime policy is adopted
- WHEN focused policy drift and activation rails run
- THEN they cover valid policy, invalid fixtures, enforcement-before-evaluation, and receipt redaction
- AND closeout evidence names the successful rails or explicit blockers.

#### Scenario: Steel policy closeout is evidenced

r[agent_harness_runtime.clankers_steel_policy.closeout]
- GIVEN the Steel policy change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes focused policy rails, Steel tests, Nickel/config drift checks, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Receipt, artifact, ledger, and service rails

r[agent_harness_runtime.clankers_focused_rails.receipts_artifacts] Animus MUST provide focused rails for safe receipt validation, leak rejection, BLAKE3 artifact determinism, and redaction-class display behavior.

r[agent_harness_runtime.clankers_focused_rails.ledger] Animus MUST provide focused rails for neutral ledger replay, unsupported-entry failure, safe metadata counts, and missing ledger service behavior.

r[agent_harness_runtime.clankers_focused_rails.injected_services] Animus MUST provide focused rails or tests for injected provider/tool service boundaries, absence of ambient provider behavior, and fail-closed missing/denied/cancelled/malformed outcomes.

#### Scenario: Boundary rails cover negative cases

r[agent_harness_runtime.clankers_focused_rails.boundary_negative]
- GIVEN receipt, artifact, ledger, or injected-service boundary fixtures include unsafe or unavailable cases
- WHEN focused rails run
- THEN each unsafe or unavailable case returns a typed failure or redacted result
- AND no rail accepts leaked sensitive material or false success evidence.

### Requirement: Stable rails wire into Nix checks

r[agent_harness_runtime.clankers_focused_rails.nix_wiring] Stable credential-free focused rails SHOULD be wired into named Nix checks or the default flake check once deterministic; rails not wired into Nix MUST document the blocker.

#### Scenario: Closeout separates evidence from non-claims

r[agent_harness_runtime.clankers_focused_rails.closeout]
- GIVEN a Clankers-pattern adoption is ready for archive
- WHEN closeout evidence is prepared
- THEN it names successful focused rails, workspace tests, clippy, Nickel/config drift checks when applicable, Cairn validation, and Cairn gates
- AND it explicitly identifies skipped rails and non-claims such as live providers, TUI dogfood, daemon tests, VM checks, or production readiness when not covered.

### Requirement: Roadmap taxonomy

r[agent_harness_runtime.roadmap_sequence.taxonomy] Animus MUST classify active Cairn changes by roadmap role, including foundational lifecycle, core-runtime contract, host-adapter, optional-reference, product-surface, and evaluation-only roles.

#### Scenario: Active changes have roadmap roles

r[agent_harness_runtime.roadmap_sequence.taxonomy.classified]
- GIVEN the Cairn tree contains active changes
- WHEN the roadmap classification is checked
- THEN each active change has exactly one primary role
- AND optional secondary relationships are represented as explicit prerequisite or related-change links.

### Requirement: Dependency order

r[agent_harness_runtime.roadmap_sequence.dependency_order] The roadmap MUST define prerequisite tiers that land accepted baseline specs and local validation wiring before runtime behavior, reusable runtime contracts before concrete host adapters, and core safety contracts before product surfaces.

#### Scenario: Dependent change waits for prerequisite contracts

r[agent_harness_runtime.roadmap_sequence.dependency_order.blocks_dependents]
- GIVEN a change depends on runtime loop, injected services, receipts, ledger, event, policy, or host-adapter contracts
- WHEN the prerequisite requirement IDs are not accepted, not in the same archive batch, and not explicitly waived
- THEN roadmap validation reports the dependency as blocked
- AND the dependent change is not marked archive-ready.

### Requirement: Overlap resolution

r[agent_harness_runtime.roadmap_sequence.overlap_resolution] The roadmap MUST assign ownership for overlapping ledger, receipt, provider, hook, manifest, catalog, and tool-contract concerns so duplicate changes converge on shared requirement IDs instead of creating incompatible contracts.

#### Scenario: Overlapping changes share owners

r[agent_harness_runtime.roadmap_sequence.overlap_resolution.shared_owner]
- GIVEN two active changes describe similar ledger, receipt, provider, hook, manifest, catalog, or tool behavior
- WHEN maintainers classify the changes
- THEN one owning concept and prerequisite path is recorded
- AND the non-owning change links to that owner or narrows itself to evaluation-only scope.

### Requirement: Roadmap validation

r[agent_harness_runtime.roadmap_sequence.validation] Animus SHOULD provide a deterministic roadmap fixture or static check that detects unknown change IDs, unknown prerequisite requirement IDs, dependency cycles, unclassified active changes, and product-surface changes that bypass foundational safety contracts.

#### Scenario: Invalid roadmap fails closed

r[agent_harness_runtime.roadmap_sequence.validation.negative]
- GIVEN roadmap fixtures include an unknown change ID, unknown prerequisite ID, dependency cycle, unclassified change, or unsafe product-surface shortcut
- WHEN roadmap validation runs
- THEN validation fails with deterministic issue codes
- AND the diagnostics do not depend on filesystem ordering or ambient state.

### Requirement: Roadmap closeout evidence

r[agent_harness_runtime.roadmap_sequence.closeout] Roadmap sequencing work MUST produce reviewable evidence that lifecycle validation, roadmap validation, and Cairn gates pass before the change is archived.

#### Scenario: Sequencing evidence is reviewable

r[agent_harness_runtime.roadmap_sequence.closeout.evidence]
- GIVEN the roadmap sequencing change is ready to archive
- WHEN reviewers inspect the evidence
- THEN the evidence names successful lifecycle validation, focused roadmap checks, and Cairn proposal/design/tasks gates
- AND any unavailable check has a blocker and a next best validation path.

### Requirement: Generated policy artifact

r[agent_harness_runtime.cairn_policy.generated_artifact] Animus MUST provide repo-local Cairn generated policy data at the path expected by default Cairn runtime validation, or an equivalent repo-owned wrapper that resolves the policy without hidden session-specific absolute paths.

#### Scenario: Default validation finds policy

r[agent_harness_runtime.cairn_policy.generated_artifact.default_path]
- GIVEN a developer runs Cairn validation from the Animus repository root
- WHEN no explicit external policy path is supplied
- THEN validation loads the repo-owned policy artifact or wrapper-resolved policy
- AND it does not depend on a private checkout path.

### Requirement: Policy refresh documentation

r[agent_harness_runtime.cairn_policy.refresh_docs] Animus MUST document the policy source, refresh command, check command, and expected default validation command for Cairn lifecycle artifacts.

#### Scenario: Maintainer refreshes policy intentionally

r[agent_harness_runtime.cairn_policy.refresh_docs.reproducible]
- GIVEN the canonical Cairn policy source changes
- WHEN a maintainer refreshes Animus policy wiring
- THEN the documentation names the source identity and commands used
- AND the generated artifact can be reviewed as an intentional lifecycle dependency update.

### Requirement: Policy drift check

r[agent_harness_runtime.cairn_policy.drift_check] Animus SHOULD provide a deterministic policy drift check or documented fixture that compares the checked-in generated policy to the canonical exported policy when the source is available.

#### Scenario: Stale policy is detected

r[agent_harness_runtime.cairn_policy.drift_check.stale]
- GIVEN the checked-in generated policy differs from the canonical exported policy
- WHEN the drift check runs with the source available
- THEN the check fails with a deterministic diagnostic
- AND it reports the source identity without embedding secrets or local-only full paths in durable evidence.

### Requirement: Default Cairn validation

r[agent_harness_runtime.cairn_policy.default_validation] Animus lifecycle validation MUST be runnable through the default Cairn validation command from the repository root without requiring a developer to remember an external policy argument.

#### Scenario: Validation is local and reproducible

r[agent_harness_runtime.cairn_policy.default_validation.local]
- GIVEN the repository contains lifecycle artifacts and policy wiring
- WHEN `cairn validate --root .` or the pinned equivalent runs
- THEN validation succeeds or fails only on lifecycle/policy content
- AND missing external checkout paths are not reported as validation blockers.

### Requirement: Policy evidence

r[agent_harness_runtime.cairn_policy.evidence] Cairn policy validation evidence SHOULD record policy source identity, policy hash when available, validation command, and validation result.

#### Scenario: Reviewers can audit policy used for validation

r[agent_harness_runtime.cairn_policy.evidence.reviewable]
- GIVEN lifecycle validation evidence is attached to a change
- WHEN reviewers inspect it
- THEN the evidence identifies which policy artifact was used
- AND it includes enough stable identity to reproduce or challenge the validation result.

### Requirement: Policy wiring closeout

r[agent_harness_runtime.cairn_policy.closeout] Policy wiring work MUST run default validation, explicit-policy validation when available, and Cairn proposal/design/tasks gates before archive.

#### Scenario: Policy wiring closeout is reviewable

r[agent_harness_runtime.cairn_policy.closeout.evidence]
- GIVEN the policy wiring change is ready to archive
- WHEN reviewers inspect closeout evidence
- THEN default validation, explicit-policy validation, and relevant Cairn gates are named with their status
- AND unavailable checks include a blocker and next best validation path.

### Requirement: Lifecycle layout

r[agent_harness_runtime.cairn_dogfood.lifecycle_layout] Animus MUST treat `cairn/specs/` as the accepted requirements baseline, `cairn/changes/` as active change deltas, and `cairn/archive/` as completed lifecycle history once changes are accepted and archived.

#### Scenario: Accepted spec is source of current requirements

r[agent_harness_runtime.cairn_dogfood.lifecycle_layout.accepted_baseline]
- GIVEN a requirement has been accepted and synced
- WHEN maintainers inspect current lifecycle requirements
- THEN the requirement appears under `cairn/specs/`
- AND the completed change package is archived rather than left as active planning state.

### Requirement: Change threshold guidance

r[agent_harness_runtime.cairn_dogfood.change_threshold] Animus MUST document when work requires a Cairn change package, including public behavior, compatibility semantics, mutation behavior, architecture, receipt/schema contracts, release/evidence contracts, roadmap scope, and safety-boundary changes.

#### Scenario: Small safe edits can stay direct

r[agent_harness_runtime.cairn_dogfood.change_threshold.direct_safe_edit]
- GIVEN a change is limited to typo repair, fixture hardening, or narrow documentation cleanup
- WHEN it does not affect public behavior, architecture, compatibility, mutation, schema, receipt, or roadmap scope
- THEN maintainers may apply it directly
- AND ordinary repository checks still provide evidence.

### Requirement: Gate workflow

r[agent_harness_runtime.cairn_dogfood.gate_workflow] Animus MUST define a lifecycle workflow for proposal, design, tasks, validation, sync dry-run, archive dry-run, and executed archive closeout using Cairn commands or pinned equivalents.

#### Scenario: Change cannot archive without gates

r[agent_harness_runtime.cairn_dogfood.gate_workflow.blocks_incomplete]
- GIVEN an active Cairn change lacks passing proposal, design, tasks, validation, or sync/archive dry-run evidence
- WHEN maintainers attempt closeout
- THEN the change is not considered archive-ready
- AND blockers or waivers are recorded explicitly.

### Requirement: Bounded evidence contract

r[agent_harness_runtime.cairn_dogfood.evidence_contract] Cairn lifecycle evidence SHOULD record command identity, status, issue codes, requirement/change IDs, policy source identity, policy hash when available, output artifact hashes, and unavailable-check blockers while keeping evidence bounded and reviewable.

#### Scenario: Policy identity is visible in evidence

r[agent_harness_runtime.cairn_dogfood.evidence_contract.policy]
- GIVEN Cairn validation or gates run under a policy artifact
- WHEN evidence is recorded
- THEN the evidence identifies the policy source or artifact and hash when available
- AND reviewers can tell which policy accepted the lifecycle content.

### Requirement: Safe evidence summaries

r[agent_harness_runtime.cairn_dogfood.safe_evidence] Durable Cairn evidence summaries MUST omit raw prompts, credentials, provider payloads, full private paths, raw private file bodies, and unbounded command output by default.

#### Scenario: Evidence summary redacts private material

r[agent_harness_runtime.cairn_dogfood.safe_evidence.redacted]
- GIVEN validation, gate, sync, archive, release-readiness, or workspace-check output includes private or unbounded material
- WHEN durable evidence is summarized
- THEN it includes bounded status, counts, issue codes, hashes, and safe relative labels
- AND raw sensitive material is omitted or replaced by approved redaction markers.

### Requirement: Cairn dogfood closeout

r[agent_harness_runtime.cairn_dogfood.closeout] Cairn dogfood work MUST run validation, proposal/design/tasks gates, and sync/archive dry runs for the change itself before archive.

#### Scenario: Dogfood evidence is reviewable

r[agent_harness_runtime.cairn_dogfood.closeout.evidence]
- GIVEN Cairn dogfood work is ready to archive
- WHEN reviewers inspect evidence
- THEN validation, gates, sync dry-run, archive dry-run, and evidence-safety checks are named with status
- AND unavailable checks include a blocker and next best validation path.

### Requirement: Clankers product surfaces are rejected as defaults

r[agent_harness_runtime.clankers_product_rejection.docs] Animus MUST document that it adapts bounded Clankers patterns while rejecting Clankers product surfaces as default reusable Animus behavior.

#### Scenario: Rejected surfaces are explicit

r[agent_harness_runtime.clankers_product_rejection.docs.explicit]
- GIVEN maintainers review Clankers references in Animus
- WHEN they inspect the architecture note
- THEN it explicitly rejects default adoption of Clankers TUI, daemon/attach protocol, Matrix/P2P, global `.clankers` state, provider router/OAuth, plugin subprocess runtime, Wasm/Extism loading, Automerge desktop storage, self-evolution apply/rollback, live dogfood harnesses, product tools, and release tagging workflow
- AND it distinguishes those surfaces from adopted boundary patterns.

### Requirement: Reusable crates remain product-surface free

r[agent_harness_runtime.clankers_product_rejection.dependency_isolation] Reusable Animus crates and default std-shell contracts MUST NOT require Clankers product crates, daemon/TUI concepts, global state directories, Matrix/P2P, plugin runtimes, Automerge storage, OAuth/provider router, self-evolution workflows, product tools, or dogfood harnesses.

#### Scenario: Dependency isolation rail rejects product leakage

r[agent_harness_runtime.clankers_product_rejection.dependency_isolation.rail]
- GIVEN reusable Animus crate dependencies and source markers are checked
- WHEN a rejected Clankers product surface appears as a runtime dependency or assumption
- THEN the rail fails with an actionable diagnostic
- AND bounded-reference documentation mentions remain allowed only in documented reference sections.

### Requirement: Future exceptions require explicit Cairn scope

r[agent_harness_runtime.clankers_product_rejection.future_exception] Any future change that adopts a previously rejected Clankers product surface MUST create a focused Cairn change that names host ownership, injected service boundaries, safety policy, redacted receipts, non-default feature gating, and focused validation rails.

#### Scenario: Future exception is gated

r[agent_harness_runtime.clankers_product_rejection.future_exception.gated]
- GIVEN a maintainer wants to add a daemon, TUI, plugin runtime, provider router, Matrix/P2P, global state, or self-evolution surface
- WHEN the proposal is reviewed
- THEN it includes a focused Cairn change with host ownership, injected services, safety policy, redacted receipts, feature gating, and validation rails
- AND the surface remains non-default unless the change explicitly justifies default behavior.

### Requirement: Marker rail guards non-adoption

r[agent_harness_runtime.clankers_product_rejection.marker_rail] Animus MUST provide a focused marker or dependency rail that allows bounded-reference docs while rejecting product-surface dependencies or runtime assumptions in reusable crates.

#### Scenario: Product rejection closeout is evidenced

r[agent_harness_runtime.clankers_product_rejection.closeout]
- GIVEN the product-surface rejection change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes docs checks, dependency/marker rail results, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Bounded tool catalog evaluation

r[agent_harness_runtime.clankers_tool_catalog.scope] Animus MUST evaluate Clankers tool-catalog and capability-pack patterns as bounded references and MUST define any adopted catalog schema in Animus-owned terms.

#### Scenario: Catalog adoption rejects product tools

r[agent_harness_runtime.clankers_tool_catalog.scope.rejects_product]
- GIVEN maintainers evaluate Clankers tool catalogs
- WHEN the adoption decision is documented
- THEN Animus distinguishes descriptor/capability patterns from Clankers product tools and plugin runtimes
- AND it does not import Clankers built-in tools as Animus runtime dependencies.

### Requirement: Optional catalog descriptor schema

r[agent_harness_runtime.clankers_tool_catalog.schema] An Animus tool catalog MAY define descriptors for tool name, description, runtime kind, capability classes, side-effect level, approval policy, input/output schema references, and redaction policy.

#### Scenario: Descriptor schema is explicit

r[agent_harness_runtime.clankers_tool_catalog.schema.explicit]
- GIVEN a tool descriptor is maintained by a host or repo
- WHEN catalog validation reads it
- THEN the descriptor names capabilities, side-effect level, approval behavior, schema references, and redaction behavior
- AND missing required fields fail validation.

### Requirement: Catalog validation fails unsafe descriptors

r[agent_harness_runtime.clankers_tool_catalog.validation] Catalog validation MUST reject duplicate names, built-in collisions, unknown runtime kinds, invalid schemas, dangerous capabilities without approval, and secret-adjacent capabilities without redaction.

#### Scenario: Unsafe catalog fixtures fail

r[agent_harness_runtime.clankers_tool_catalog.fixtures]
- GIVEN catalog fixtures include duplicate names, invalid schemas, unknown runtime kinds, unsafe capabilities, missing approval, or missing redaction
- WHEN validation runs
- THEN each invalid fixture fails with an actionable diagnostic
- AND safe descriptors still normalize deterministically.

### Requirement: Catalog parsing is runtime-neutral

r[agent_harness_runtime.clankers_tool_catalog.runtime_neutral] Catalog parsing and validation MUST NOT execute product tools, start plugins, load providers, read secrets, inherit environment, or open network connections.

#### Scenario: Runtime-neutral rail catches side effects

r[agent_harness_runtime.clankers_tool_catalog.runtime_neutral.rail]
- GIVEN a catalog manifest is parsed for validation
- WHEN the runtime-neutrality rail runs
- THEN it proves validation is data-only
- AND no tool execution, plugin launch, provider startup, secret read, environment inheritance, or network access is required.

#### Scenario: Tool catalog closeout is evidenced

r[agent_harness_runtime.clankers_tool_catalog.closeout]
- GIVEN the tool catalog evaluation is ready to archive
- WHEN closeout validation runs
- THEN evidence includes adoption-scope docs, manifest validation fixtures, runtime-neutrality checks, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Wasm-supported surface

r[agent_harness_runtime.wasm_compile.surface] Animus MUST define a Wasm-supported crate and feature surface for deterministic embedders, initially targeting `wasm32-unknown-unknown` and including only crates that do not require ambient std shell services.

#### Scenario: Embedders select a portable surface

r[agent_harness_runtime.wasm_compile.surface.portable]
- GIVEN an embedder wants Animus logic in a Wasm module
- WHEN they select the documented Wasm-supported crate and feature profile
- THEN the selected surface includes deterministic reusable contracts such as core state and adapter DTOs
- AND it excludes std shell orchestration, terminal UI, provider clients, native persistence, process execution, and filesystem authority unless a later target-specific contract accepts them.

### Requirement: Root facade supports a no-std Wasm profile

r[agent_harness_runtime.wasm_compile.root_facade] The root `animus` facade MUST be able to expose a no-std-compatible profile for the Wasm-supported surface without requiring `animus-std`, terminal, process, filesystem, provider, or native persistence dependencies.

#### Scenario: Root no-std profile avoids shell dependencies

r[agent_harness_runtime.wasm_compile.root_facade.no_shell]
- GIVEN the root facade is built for the documented Wasm target without std shell features
- WHEN dependency resolution and compile checks run
- THEN shell-only crates are not required by that build
- AND std-backed convenience APIs remain available only under documented host-shell features or host-target builds.

### Requirement: Wasm compile rail

r[agent_harness_runtime.wasm_compile.rail] Animus MUST provide a credential-free Wasm compile rail that checks the documented supported surface against the accepted Wasm target and reports deterministic evidence for review.

#### Scenario: Compile rail catches portability drift

r[agent_harness_runtime.wasm_compile.rail.drift]
- GIVEN a change introduces an incompatible dependency or target assumption into the Wasm-supported surface
- WHEN the Wasm compile rail runs
- THEN the rail fails before review claims Wasm support
- AND the diagnostic identifies the crate or feature profile that no longer compiles.

### Requirement: Shell isolation for Wasm support

r[agent_harness_runtime.wasm_compile.shell_isolation] Animus MUST keep std shell, terminal, Steel runtime, provider, native persistence, process, filesystem, network, credential, clock, and host execution dependencies outside the Wasm-supported surface unless guarded by target or feature gates that the Wasm rail does not enable.

#### Scenario: Host behavior remains explicit

r[agent_harness_runtime.wasm_compile.shell_isolation.explicit]
- GIVEN a host wants process, filesystem, provider, terminal, browser, WASI, plugin, or persistence behavior
- WHEN it composes Animus with host services
- THEN that behavior crosses explicit adapter or shell contracts
- AND Wasm compile support alone does not imply those host behaviors are present or authorized.

### Requirement: Optional bridge participation is explicit

r[agent_harness_runtime.wasm_compile.optional_bridges] Optional bridge crates such as Preserves MAY join the Wasm rail only after their target compatibility, dependency behavior, and adapter-DTO revalidation path are documented and tested.

#### Scenario: Optional bridge support is not assumed

r[agent_harness_runtime.wasm_compile.optional_bridges.evidence]
- GIVEN an optional bridge is considered for Wasm support
- WHEN closeout evidence is prepared
- THEN it either includes successful Wasm compile evidence and positive/negative boundary tests for that bridge
- OR records the blocker and excludes the bridge from the supported Wasm surface.

### Requirement: Wasm closeout evidence

r[agent_harness_runtime.wasm_compile.closeout] Boundary-affecting Wasm support changes MUST include reviewable evidence for the focused Wasm rail, relevant crate tests, workspace checks, Cairn validation, and Cairn gates, with unavailable checks recorded alongside blockers and next-best validation.

#### Scenario: Reviewers can distinguish support from non-claims

r[agent_harness_runtime.wasm_compile.closeout.non_claims]
- GIVEN a Wasm support change is ready to archive
- WHEN reviewers inspect its evidence
- THEN the evidence names the exact crate and feature surface that compiles for Wasm
- AND it explicitly lists excluded runtime claims such as browser APIs, WASI host behavior, plugin loading, providers, terminal UI, process execution, filesystem access, credentials, and native persistence.

### Requirement: Animus lifecycle map

r[agent_harness_runtime.clankers_lifecycle_doc.map] Animus MUST document the runtime request lifecycle from typed runtime input through deterministic core state, Steel loop decisions, adapter dispatch, event/report projection, safe receipt construction, and host ledger or persistence seams.

#### Scenario: Golden path is reviewable

r[agent_harness_runtime.clankers_lifecycle_doc.map.reviewable]
- GIVEN a maintainer needs to change runtime request behavior
- WHEN they inspect `docs/request-lifecycle.md`
- THEN the document describes the ordered Animus runtime path
- AND it distinguishes reusable core/adapters/Steel behavior from std-shell or host-owned behavior.

### Requirement: Boundary owners and source-of-truth types

r[agent_harness_runtime.clankers_lifecycle_doc.owners] Each lifecycle boundary MUST name the owning crate and source-of-truth type or trait for the state transition.

#### Scenario: Owner names prevent duplicate state machines

r[agent_harness_runtime.clankers_lifecycle_doc.owners.no_duplicate_state]
- GIVEN a change touches runtime input, Steel decisions, model/tool dispatch, reports, events, receipts, or ledger replay
- WHEN reviewers inspect the lifecycle document
- THEN the owning crate and source-of-truth type or trait are named
- AND the change does not introduce an undocumented duplicate owner for the same transition.

### Requirement: Forbidden dependency rules

r[agent_harness_runtime.clankers_lifecycle_doc.forbidden_dependencies] The lifecycle document MUST identify forbidden dependencies and side effects for reusable core, adapter, Steel, std-shell, receipt, and future ledger boundaries.

#### Scenario: Forbidden dependencies stay visible

r[agent_harness_runtime.clankers_lifecycle_doc.forbidden_dependencies.visible]
- GIVEN a reusable Animus crate boundary is reviewed
- WHEN the lifecycle document is consulted
- THEN it identifies which boundaries must not read environment, filesystem, credentials, provider config, network, clocks, or concrete host persistence
- AND it points to the validation rail that checks those constraints when one exists.

### Requirement: Lifecycle links and marker rail

r[agent_harness_runtime.clankers_lifecycle_doc.links] Animus README or architecture docs MUST link to the lifecycle document so maintainers can find it before changing runtime behavior.

#### Scenario: Focused rail guards lifecycle drift

r[agent_harness_runtime.clankers_lifecycle_doc.rail]
- GIVEN the lifecycle document is part of the maintained architecture contract
- WHEN the focused marker rail runs
- THEN it fails if required boundary names, owner markers, forbidden dependency sections, or validation command references are missing
- AND it emits reviewable evidence without requiring live providers or host credentials.

#### Scenario: Lifecycle closeout is evidenced

r[agent_harness_runtime.clankers_lifecycle_doc.closeout]
- GIVEN the lifecycle change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes the focused lifecycle rail, relevant docs checks, Cairn validation, and Cairn proposal/design/tasks gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Bounded runtime facade inventory adoption

r[agent_harness_runtime.clankers_runtime_facade_inventory.scope] Animus MUST adopt Clankers' runtime-facade inventory pattern only as a bounded architecture and verification rail, not as a dependency on Clankers runtime crates or product surfaces.

#### Scenario: Inventory adoption rejects product surfaces

r[agent_harness_runtime.clankers_runtime_facade_inventory.scope.rejects_product]
- GIVEN maintainers adapt Clankers runtime-facade inventory ideas
- WHEN the adoption scope is documented
- THEN Animus distinguishes export-classification discipline from Clankers daemon, TUI, provider router, plugin runtime, auth flow, process manager, desktop storage, and product shell behavior
- AND reusable Animus crates do not import Clankers runtime crates for the inventory.

### Requirement: Public export inventory schema

r[agent_harness_runtime.clankers_runtime_facade_inventory.schema] Animus MUST define an inventory record schema for runtime-facing public exports that records export path, owning crate, source module or marker, class, side-effect allowance, stability intent, review note, and safe evidence metadata.

#### Scenario: Export records name owners and authority class

r[agent_harness_runtime.clankers_runtime_facade_inventory.schema.owner_class]
- GIVEN a runtime-facing public export is included in the inventory
- WHEN reviewers inspect the record
- THEN the record names the owning crate and source location or marker
- AND it classifies the export as a reusable contract, host-injection surface, or shell adapter.

### Requirement: Export classes preserve reusable boundaries

r[agent_harness_runtime.clankers_runtime_facade_inventory.classification] Runtime-facing public exports MUST be classified as reusable contracts, host-injection surfaces, or shell adapters, and each class MUST define forbidden side effects.

#### Scenario: Reusable contract remains no-ambient

r[agent_harness_runtime.clankers_runtime_facade_inventory.classification.reusable_no_ambient]
- GIVEN an export is classified as a reusable contract
- WHEN the inventory rail checks it
- THEN the export does not require ambient filesystem, environment, credentials, clocks, randomness, network, provider discovery, TUI state, daemon frames, plugin launch, process execution, or concrete persistence
- AND any needed authority is represented through a host-injection surface or shell adapter instead.

#### Scenario: Host injection fails closed by default

r[agent_harness_runtime.clankers_runtime_facade_inventory.classification.fail_closed]
- GIVEN an export is classified as a host-injection surface
- WHEN the default service or adapter is absent, unavailable, denied, cancelled, or unsupported
- THEN it fails closed with typed status or denial metadata
- AND it does not silently create a shell adapter, read ambient config, discover providers, or contact backends.

### Requirement: Focused inventory rail

r[agent_harness_runtime.clankers_runtime_facade_inventory.rail] Animus MUST provide a focused static rail or equivalent deterministic test that fails on unclassified runtime-facing public exports, missing owner metadata, class and side-effect contradictions, and unsafe evidence fields.

r[agent_harness_runtime.clankers_runtime_facade_inventory.positive] The inventory rail MUST accept fixtures or real exports for pure DTOs, validation helpers, adapter traits, and deny-by-default host service seams.

r[agent_harness_runtime.clankers_runtime_facade_inventory.negative] The inventory rail MUST reject fixtures or real exports that classify ambient filesystem, environment, credential, clock, network, provider, TUI, daemon, plugin, process, or persistence behavior as reusable contracts.

#### Scenario: Positive and negative fixtures guard classification

r[agent_harness_runtime.clankers_runtime_facade_inventory.rail.fixtures]
- GIVEN inventory fixtures or real exports include reusable contracts, host-injection seams, and unsafe shell-authority examples
- WHEN the focused inventory rail runs
- THEN valid reusable and host-injection surfaces are accepted
- AND unsafe ambient-authority examples are rejected as reusable contracts.

#### Scenario: Unclassified exports fail review

r[agent_harness_runtime.clankers_runtime_facade_inventory.rail.unclassified]
- GIVEN a runtime-facing crate adds a public export without inventory classification
- WHEN the focused inventory rail runs
- THEN it fails with an actionable diagnostic naming the missing classification
- AND it does not require live providers, host credentials, plugin runtimes, daemon sessions, or TUI state.

### Requirement: Safe inventory evidence and docs

r[agent_harness_runtime.clankers_runtime_facade_inventory.evidence] Inventory evidence MUST include schema/status, checked crates, class counts, fixture names, safe diagnostics, and content hashes when needed, and MUST omit raw prompts, provider payloads, credentials, environment values, full host paths, and tool output.

r[agent_harness_runtime.clankers_runtime_facade_inventory.docs] Animus README or architecture docs MUST link to an inventory summary that tells embedders which surfaces are default-safe reusable contracts, host-owned injection points, or shell-only adapters.

#### Scenario: Closeout evidence separates claims from non-claims

r[agent_harness_runtime.clankers_runtime_facade_inventory.closeout]
- GIVEN the runtime facade inventory change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes the focused inventory rail, positive and negative fixtures, dependency checks, docs links, Cairn validation, and Cairn gates
- AND skipped checks or non-claims such as live providers, TUI dogfood, daemon tests, plugin runtime tests, VM checks, or production readiness are identified explicitly.

### Requirement: Plugin manifests are bounded descriptor references

r[agent_harness_runtime.clankers_plugin_manifest.scope] Animus MUST treat Clankers plugin manifests as bounded descriptor references and MUST NOT adopt plugin execution, stdio supervision, Wasm loading, Zellij, MCP, or sandbox enforcement in this change.

#### Scenario: Descriptor scope rejects runtime adoption

r[agent_harness_runtime.clankers_plugin_manifest.scope.rejects_runtime]
- GIVEN maintainers evaluate Clankers plugin manifests
- WHEN the adoption scope is reviewed
- THEN it documents descriptor fields separately from runtime execution
- AND it does not add plugin process launch, Wasm loading, MCP/Zellij integration, or sandbox enforcement as default Animus behavior.

### Requirement: Optional manifest schema

r[agent_harness_runtime.clankers_plugin_manifest.schema] Animus MAY define plugin descriptor fields for identity, runtime kind, declared tools, capabilities, launch policy metadata, sandbox requirements, environment allowlist names, writable roots, network policy, and redaction.

#### Scenario: Manifest descriptors declare authority needs

r[agent_harness_runtime.clankers_plugin_manifest.schema.authority]
- GIVEN a plugin descriptor is parsed
- WHEN validation inspects it
- THEN declared tools, capabilities, runtime kind, launch policy metadata, sandbox requirements, environment allowlist names, writable roots, network policy, and redaction are explicit
- AND missing required descriptor fields fail validation.

### Requirement: Manifest validation rejects unsafe declarations

r[agent_harness_runtime.clankers_plugin_manifest.validation] Manifest validation MUST reject unknown runtime kinds, missing required fields, duplicate or built-in tool collisions, path escapes, absolute writable roots, environment value leakage, and unsafe network/sandbox combinations.

#### Scenario: Invalid manifest fixtures fail

r[agent_harness_runtime.clankers_plugin_manifest.fixtures]
- GIVEN manifest fixtures include path escapes, absolute writable roots, env values, missing commands, unknown runtime kinds, tool collisions, or unsafe network/sandbox combinations
- WHEN validation runs
- THEN each invalid fixture fails with an actionable diagnostic
- AND safe descriptors normalize deterministically.

### Requirement: Manifest parsing is runtime-neutral

r[agent_harness_runtime.clankers_plugin_manifest.runtime_neutral] Manifest parsing MUST NOT launch stdio processes, load Wasm, inherit environment values, read secrets, connect to network, or execute tools.

#### Scenario: Runtime-neutrality rail catches plugin side effects

r[agent_harness_runtime.clankers_plugin_manifest.runtime_neutral.rail]
- GIVEN plugin descriptor parsing is validated
- WHEN the runtime-neutrality rail runs
- THEN it proves parsing is data-only
- AND no plugin execution, Wasm load, secret read, environment inheritance, or network access is required.

#### Scenario: Plugin manifest closeout is evidenced

r[agent_harness_runtime.clankers_plugin_manifest.closeout]
- GIVEN the plugin manifest boundary evaluation is ready to archive
- WHEN closeout validation runs
- THEN evidence includes scope docs, manifest fixtures, runtime-neutrality checks, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Release evidence index is a bounded future pattern

r[agent_harness_runtime.clankers_evidence_index.scope] Animus MUST treat Clankers release evidence indexing as a bounded future pattern and MUST NOT adopt the full Clankers harness, dogfood tests, live model checks, VM gates, or release tagging workflow in this evaluation change.

#### Scenario: Evidence index scope avoids harness adoption

r[agent_harness_runtime.clankers_evidence_index.scope.bounded]
- GIVEN maintainers evaluate Clankers release evidence
- WHEN the adoption note is reviewed
- THEN it documents evidence-index concepts separately from Clankers' full harness and dogfood suites
- AND it keeps live model checks, VM gates, TUI/daemon dogfood, and release tagging out of scope.

### Requirement: Evidence index schema composes receipts

r[agent_harness_runtime.clankers_evidence_index.schema] A future Animus evidence index SHOULD compose focused rail receipts, workspace tests, clippy, flake checks, Nickel/config drift checks, Cairn validation, and Cairn gates without rerunning missing modes by default.

#### Scenario: Index composes selected evidence

r[agent_harness_runtime.clankers_evidence_index.schema.compose]
- GIVEN multiple local receipts and check summaries exist
- WHEN the evidence index is generated
- THEN it selects and summarizes existing evidence by mode
- AND it does not imply missing modes ran.

### Requirement: Current-head evidence is explicit

r[agent_harness_runtime.clankers_evidence_index.current_head] An evidence index that makes current-head claims MUST record commit, branch, dirty status, receipt/head match, stale receipt status, and missing modes, with VCS collection kept in the imperative shell.

#### Scenario: Stale or dirty evidence is not current-head proof

r[agent_harness_runtime.clankers_evidence_index.current_head.stale]
- GIVEN selected receipts do not match the current head or the worktree is dirty
- WHEN the evidence index is reviewed
- THEN those receipts are labeled stale or dirty
- AND they are not treated as current-head proof.

### Requirement: Non-claims are explicit

r[agent_harness_runtime.clankers_evidence_index.non_claims] Evidence indexes MUST separate pure credential-free checks from optional host-dependent checks and MUST state non-claims for live providers, TUI/daemon dogfood, VM checks, remote services, plugin runtime, and production readiness unless those checks explicitly ran.

#### Scenario: Overclaim fixtures fail

r[agent_harness_runtime.clankers_evidence_index.fixtures]
- GIVEN fixture evidence omits live provider, daemon/TUI, VM, plugin, or production-readiness checks
- WHEN index validation runs
- THEN claims for those omitted surfaces are rejected or labeled as non-claims
- AND pure evidence remains usable for the contracts it actually covers.

#### Scenario: Evidence index closeout is documented

r[agent_harness_runtime.clankers_evidence_index.closeout]
- GIVEN the evidence index evaluation is ready to archive
- WHEN closeout validation runs
- THEN evidence includes scope docs, schema fixtures, current-head fixtures, non-claim fixtures, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Runtime crate boundary

r[agent_harness_runtime.loop.contract] The workspace MUST provide an `animus-runtime` crate that owns reusable agent-loop orchestration without depending on concrete provider, tool, filesystem, process, network, UI, or Syndicate implementations.

#### Scenario: Runtime crate is embeddable

r[agent_harness_runtime.loop.contract.embeddable]
- GIVEN an embedder depends on `animus-runtime`, `animus-adapters`, and a chosen loop fabric
- WHEN they construct a runtime with model and tool adapters
- THEN they can execute the agent loop without pulling in concrete provider or tool crates
- AND the runtime public API exposes typed inputs, configuration, events, reports, and errors.

### Requirement: Pure runtime loop core

r[agent_harness_runtime.loop.core] Runtime decision handling MUST be decomposed into a pure deterministic core over explicit turn state, decisions, adapter outcomes, and configuration, plus a thin shell that calls fabrics and adapters.

#### Scenario: Runtime core is directly testable

r[agent_harness_runtime.loop.core.pure]
- GIVEN a modeled turn state, runtime configuration, and next decision
- WHEN the runtime core evaluates the transition
- THEN it returns typed next state, events, and completion status deterministically
- AND it does not read files, inspect environment, spawn processes, use clocks, log, mutate global state, or call adapters.

### Requirement: Steel default loop fabric

r[agent_harness_runtime.steel_fabric.default] The runtime SHOULD provide a default constructor or helper that uses `animus-steel` as the scriptable agent-loop fabric while preserving a swappable fabric boundary.

#### Scenario: Steel decision drives runtime turn

r[agent_harness_runtime.steel_fabric.default.turn]
- GIVEN a Steel program defines a valid `(animus-decide)` function
- WHEN the runtime evaluates a turn through the Steel fabric
- THEN the returned decision is validated by Rust boundary DTOs before dispatch
- AND invalid Steel load, evaluation, shape, or payload errors stop the turn with typed runtime failure.

### Requirement: Adapter decision dispatch

r[agent_harness_runtime.adapter_dispatch] Runtime dispatch MUST route validated model and tool decisions through injected `ModelAdapter` and `ToolAdapter` implementations and MUST NOT construct concrete provider or tool clients internally.

#### Scenario: Model decision uses injected adapter

r[agent_harness_runtime.adapter_dispatch.model]
- GIVEN the loop fabric returns a validated model request decision
- WHEN runtime dispatches the turn
- THEN it calls only the injected model adapter with the validated request
- AND records a typed model-requested and model-responded event or a typed adapter-error event.

#### Scenario: Tool decision uses injected adapter

r[agent_harness_runtime.adapter_dispatch.tool]
- GIVEN the loop fabric returns a validated tool request decision
- WHEN runtime dispatches the turn
- THEN it calls only the injected tool adapter with the validated request
- AND records a typed tool-requested and tool-responded event or a typed adapter-error event.

### Requirement: Runtime stop and max-turn behavior

r[agent_harness_runtime.loop.max_turns] Runtime configuration MUST include an explicit max-turn limit and MUST halt with typed status when the fabric returns stop, final response, adapter failure, invalid decision, or max-turn exhaustion.

#### Scenario: Max turns fail closed

r[agent_harness_runtime.loop.max_turns.exhausted]
- GIVEN runtime configuration allows a finite number of turns
- WHEN the loop reaches that limit without final response or stop
- THEN runtime halts with a max-turns-exceeded status
- AND no additional model or tool adapter calls occur.

### Requirement: Runtime positive tests

r[agent_harness_runtime.tests.positive] Runtime work MUST include positive tests for one-turn model calls, tool calls, final responses, explicit stop behavior, and multi-turn propagation of previous model/tool outputs into the next fabric turn.

#### Scenario: Multi-turn state is propagated

r[agent_harness_runtime.tests.positive.multi_turn]
- GIVEN a model adapter response or tool adapter output is produced in one turn
- WHEN the next turn is evaluated
- THEN the loop fabric receives the previous output through typed turn input
- AND the final report records the ordered event sequence.

### Requirement: Runtime negative tests

r[agent_harness_runtime.tests.negative] Runtime work MUST include negative tests for invalid Steel decisions, malformed inputs, model adapter failure, tool adapter failure, max-turn exhaustion, missing final response, and stop-without-response behavior.

#### Scenario: Adapter failure stops safely

r[agent_harness_runtime.tests.negative.adapter_failure]
- GIVEN an injected model or tool adapter returns an error
- WHEN runtime dispatch handles that adapter call
- THEN runtime records a typed adapter-error event
- AND halts without invoking later adapters or fabric turns.

### Requirement: Preserves runtime schema

r[agent_harness_runtime.preserves_schema.contract] Runtime boundary messages SHOULD have Preserves schema coverage for stable event, request, response, final-report, and error payloads before those payloads are treated as external wire contracts.

#### Scenario: Schema validates runtime payloads

r[agent_harness_runtime.preserves_schema.contract.round_trip]
- GIVEN a runtime event or final report is encoded for a boundary
- WHEN Preserves schema validation and round-trip tests run
- THEN valid payloads decode into equivalent typed Rust values
- AND malformed, wrong-kind, missing-field, and invalid-text payloads fail closed.

### Requirement: Preserves schema coverage tiers

r[agent_harness_runtime.preserves_boundary.coverage_tiers] Animus MUST classify Preserves payload coverage as internal DTO round trip, stable external wire contract, compatibility import/export adapter, or review-only fixture before publishing schema compatibility expectations.

#### Scenario: Boundary declares compatibility level

r[agent_harness_runtime.preserves_boundary.coverage_tiers.declared]
- GIVEN a payload family crosses an adapter, runtime, session, provenance, policy, dataset, or external protocol boundary
- WHEN maintainers add Preserves coverage
- THEN the payload family declares its coverage tier
- AND stable external wire contracts have stricter versioning and fixture requirements than internal DTO round trips.

### Requirement: Versioned Preserves schema identities

r[agent_harness_runtime.preserves_boundary.versioning] Stable Preserves boundary payloads MUST carry or bind a schema family and version identity with explicit additive, breaking, deprecated, and unsupported-version behavior.

#### Scenario: Unsupported version fails closed

r[agent_harness_runtime.preserves_boundary.versioning.unsupported]
- GIVEN a Preserves payload has an unsupported schema version
- WHEN boundary decoding runs
- THEN decoding returns a typed unsupported-version error
- AND the runtime does not reinterpret the payload as a different family.

### Requirement: Preserves positive round-trip fixtures

r[agent_harness_runtime.preserves_boundary.round_trip] Stable Preserves boundary work MUST include positive text and binary round-trip fixtures for valid adapter messages, runtime events, final reports, errors, session/provenance records, policy-visible DTOs, and dataset/export rows when those families are in scope.

#### Scenario: Valid payload round trips

r[agent_harness_runtime.preserves_boundary.round_trip.valid]
- GIVEN a valid boundary payload fixture
- WHEN it is encoded and decoded through supported Preserves forms
- THEN the decoded typed Rust value is equivalent to the source value
- AND canonical fixture output is deterministic.

### Requirement: Preserves malformed payload rejection

r[agent_harness_runtime.preserves_boundary.malformed] Stable Preserves boundary work MUST include negative fixtures for wrong kind, missing required fields, invalid text, unsupported versions, unbounded payloads, unsafe raw material, and compatibility leak attempts.

#### Scenario: Malformed payload is rejected safely

r[agent_harness_runtime.preserves_boundary.malformed.rejects]
- GIVEN a malformed, wrong-kind, wrong-version, unbounded, or unsafe Preserves payload
- WHEN boundary decoding runs
- THEN decoding fails with a typed boundary error
- AND diagnostics omit raw private payload bodies.

### Requirement: Preserves unknown and future fields

r[agent_harness_runtime.preserves_boundary.unknown_fields] Animus MAY preserve unknown or future fields only when they are bounded, structurally safe metadata; otherwise boundary code MUST fail with typed compatibility errors.

#### Scenario: Unsafe unknown field is rejected

r[agent_harness_runtime.preserves_boundary.unknown_fields.rejects_unsafe]
- GIVEN a payload includes an unknown field carrying raw prompt, credential, provider JSON, full path, file body, tool input/output, or unbounded content
- WHEN compatibility handling evaluates the field
- THEN the field is rejected or redacted according to policy
- AND it is not silently preserved into model-visible context, receipts, or review output.

### Requirement: Preserves core isolation

r[agent_harness_runtime.preserves_boundary.core_isolation] Core business logic MUST consume typed Rust values after Preserves boundary validation and MUST NOT make policy, runtime, or provenance decisions by inspecting raw Preserves syntax trees directly.

#### Scenario: Raw AST stays at boundary

r[agent_harness_runtime.preserves_boundary.core_isolation.no_ast_logic]
- GIVEN a behavior decision depends on adapter, runtime, policy, session, provenance, or dataset fields
- WHEN maintainers inspect the implementation
- THEN raw Preserves parsing and validation is isolated to boundary modules
- AND the decision logic receives typed validated values.

### Requirement: Preserves boundary closeout evidence

r[agent_harness_runtime.preserves_boundary.closeout] Preserves schema work MUST run positive round-trip fixtures, negative malformed fixtures, unknown-field tests, core-isolation checks, workspace checks, and Cairn gates before archive.

#### Scenario: Schema evidence is reviewable

r[agent_harness_runtime.preserves_boundary.closeout.evidence]
- GIVEN Preserves boundary work is ready to archive
- WHEN reviewers inspect evidence
- THEN fixture, compatibility, isolation, workspace, and Cairn gate checks are named with status
- AND unavailable checks include a blocker and next best validation path.

### Requirement: Runtime docs and validation

r[agent_harness_runtime.docs.validation] Runtime work MUST update architecture docs and safety contracts to describe `animus-runtime`, Steel loop-fabric use, adapter dispatch boundaries, and validation commands.

#### Scenario: Runtime closeout is reviewable

r[agent_harness_runtime.validation]
- GIVEN runtime work is ready to archive
- WHEN reviewers inspect the change evidence
- THEN successful logs show focused crate tests, positive and negative runtime tests, Preserves schema tests when added, workspace check/clippy/tests, pre-commit, Octet/Tigerstyle, flake checks, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Semantic agent status DTOs

r[agent_harness_runtime.agent_status_authority.semantic_state] Animus MUST define semantic agent status DTOs for `idle`, `working`, `blocked`, `done`, and `unknown` with source identity, agent label, optional native session reference, bounded message text, presentation metadata, sequence, and optional TTL fields.

#### Scenario: Lifecycle report is validated

r[agent_harness_runtime.agent_status_authority.semantic_state.validation]
- GIVEN a trusted host integration reports an agent label, source identifier, semantic state, target, and sequence
- WHEN Animus validates the status report
- THEN the report is accepted as a typed lifecycle observation
- AND malformed states, malformed source identifiers, missing targets, and unsupported session references are rejected with typed diagnostics.

### Requirement: Authority precedence is deterministic

r[agent_harness_runtime.agent_status_authority.authority_precedence] Status selection MUST choose one semantic authority for a target at a time, and a current trusted lifecycle authority MUST suppress heuristic fallback observations for that target.

#### Scenario: Lifecycle authority wins over heuristic fallback

r[agent_harness_runtime.agent_status_authority.authority_precedence.lifecycle_wins]
- GIVEN a current trusted lifecycle report marks an agent `working`
- AND a host heuristic observation would otherwise classify the same target as `idle`
- WHEN status selection runs
- THEN the selected semantic state is `working`
- AND the heuristic observation is recorded only as non-authoritative fallback evidence.

### Requirement: Presentation metadata is visual-only

r[agent_harness_runtime.agent_status_authority.presentation_only] Titles, display agent names, compact activity text, custom status labels, and state label overrides MUST be presentation-only metadata and MUST NOT change semantic lifecycle state, waits, notifications, rollups, or policy decisions.

#### Scenario: Custom label cannot escalate state

r[agent_harness_runtime.agent_status_authority.presentation_only.no_semantic_escalation]
- GIVEN a metadata report sets a custom visual label that says `needs input`
- AND the selected semantic state remains `working`
- WHEN waits, notifications, rollups, and policy decisions consume agent status
- THEN they use `working`
- AND the visual label is available only to presentation hosts.

### Requirement: Explainable status decisions are bounded

r[agent_harness_runtime.agent_status_authority.explain] Animus SHOULD expose bounded status-decision explanations that identify the selected authority, rejected candidates, freshness class, fallback reason, safe hashes, and redacted evidence summaries without embedding raw prompts, terminal snapshots, provider payloads, credentials, or unbounded detector output.

#### Scenario: Explanation rejects raw evidence leaks

r[agent_harness_runtime.agent_status_authority.explain.redacted]
- GIVEN a status explanation fixture contains raw terminal text, prompt text, provider JSON, a credential-looking value, or an unbounded detector transcript
- WHEN explanation validation runs
- THEN validation rejects the fixture with a typed leak diagnostic
- AND the explanation is not accepted as durable evidence.

### Requirement: Detection is host-owned

r[agent_harness_runtime.agent_status_authority.host_owned_detection] Reusable Animus runtime crates MUST NOT inspect processes, read terminals, parse screen buffers, or load detector manifests; they MUST consume typed host observations supplied through adapter contracts.

#### Scenario: Pure status selection has no ambient I/O

r[agent_harness_runtime.agent_status_authority.host_owned_detection.no_ambient_io]
- GIVEN status selection receives typed lifecycle and heuristic observations in memory
- WHEN it selects the current semantic state
- THEN the selection result is deterministic over those inputs
- AND the pure status core does not read terminals, processes, files, clocks, environment variables, or network state.

### Requirement: Status authority verification

r[agent_harness_runtime.agent_status_authority.verification] The adaptation MUST include focused positive and negative tests for report validation, authority precedence, stale sequence and TTL handling, presentation-only metadata, bounded explanations, leak rejection, host-owned detection, and pure status selection.

#### Scenario: Closeout evidence covers status authority rails

r[agent_harness_runtime.agent_status_authority.verification.closeout]
- GIVEN maintainers intend to archive the agent status authority adaptation
- WHEN closeout validation runs
- THEN evidence names focused status DTO, authority precedence, presentation metadata, explanation redaction, host-owned detection, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a blocker and next-best validation.

### Requirement: Adapter families

r[agent_harness_runtime.std_runtime_adapters.adapter_families] Animus MUST define concrete std-shell adapter families for model providers, filesystem tools, process execution, HTTP/network helpers, and deterministic fixture backends before those effects are exposed through runtime workflows.

#### Scenario: Concrete effects stay in shell crates

r[agent_harness_runtime.std_runtime_adapters.adapter_families.shell_owned]
- GIVEN a concrete adapter opens a provider connection, reads or writes host files, starts a process, accesses HTTP/network resources, reads credentials, or depends on platform APIs
- WHEN maintainers inspect crate dependencies
- THEN that adapter is owned by `animus-std` or an optional host crate
- AND reusable runtime/core crates consume only injected traits and DTOs.

### Requirement: Effect gates

r[agent_harness_runtime.std_runtime_adapters.effect_gates] Concrete adapters MUST check accepted provider registry, tool contract, policy, sandbox, receipt, and event lifecycle contracts before performing host effects, and MUST fail closed when a required contract or approval is absent.

#### Scenario: Adapter refuses ungated effect

r[agent_harness_runtime.std_runtime_adapters.effect_gates.rejects_ungated]
- GIVEN an adapter request lacks required policy authorization, sandbox compatibility, tool schema validation, provider capability support, or host approval
- WHEN the adapter receives the request
- THEN it returns a typed denial before performing the effect
- AND it emits only safe status evidence.

### Requirement: Shell-owned provider registry

r[agent_harness_runtime.pi_provider_registry.shell_registry] Animus MUST keep provider and model registry discovery, config loading, credential lookup, and concrete client construction in `animus-std` or host shells rather than reusable runtime core crates.

#### Scenario: Runtime receives injected model service

r[agent_harness_runtime.pi_provider_registry.shell_registry.injected]
- GIVEN the shell selects a provider/model from registry config
- WHEN runtime dispatches a model call
- THEN runtime calls only the injected model service with a validated request
- AND it does not read provider config, inspect credentials, or construct a network client.

### Requirement: Typed provider and model metadata

r[agent_harness_runtime.pi_provider_registry.metadata] Provider/model registry entries MUST expose typed provider IDs, model IDs, API kind, endpoint identity, input modalities, context budget, output budget, reasoning support, allowed thinking levels, and optional cost metadata.

#### Scenario: Capability metadata clamps thinking level

r[agent_harness_runtime.pi_provider_registry.metadata.thinking]
- GIVEN a selected model does not support a requested thinking level
- WHEN the shell resolves runtime model settings
- THEN the setting is clamped or rejected according to policy
- AND the runtime receives a valid selected model service and effective capability metadata.

### Requirement: Nickel-authored owned registry config

r[agent_harness_runtime.pi_provider_registry.nickel_config] Animus-owned provider registry configuration MUST be authored in Nickel with typed contracts and exported or checked before shell runtime consumption.

#### Scenario: Invalid provider config fails before use

r[agent_harness_runtime.pi_provider_registry.nickel_config.invalid]
- GIVEN a provider config has an invalid provider ID, unsupported API kind, non-positive budget, or missing required endpoint identity
- WHEN the config export or check runs
- THEN validation fails before the shell can select that provider
- AND no runtime model service is constructed from the invalid entry.

### Requirement: Redacted credential resolution

r[agent_harness_runtime.pi_provider_registry.credentials] Credential resolution MUST be an imperative-shell service that returns usable provider credentials or typed redacted failures without exposing raw tokens, environment values, OAuth secrets, or credential file contents.

#### Scenario: Missing credential reports safe diagnostic

r[agent_harness_runtime.pi_provider_registry.credentials.missing]
- GIVEN a selected provider requires a credential that is absent
- WHEN the shell resolves the provider service
- THEN resolution fails with a typed missing-credential diagnostic
- AND the diagnostic names only the credential class or source label, not any secret value.

### Requirement: Dynamic provider validation

r[agent_harness_runtime.pi_provider_registry.dynamic_validation] Host-provided or dynamically registered provider entries MUST pass the same provider/model metadata validation as file-backed registry entries before they can be selected.

#### Scenario: Dynamic invalid model is rejected

r[agent_harness_runtime.pi_provider_registry.dynamic_validation.reject]
- GIVEN a host registers a provider entry at runtime with malformed model metadata
- WHEN the shell validates the dynamic entry
- THEN the entry is rejected with typed diagnostics
- AND active runtime model services remain unchanged.

### Requirement: Provider parity remains a future bounded pattern

r[agent_harness_runtime.clankers_provider_parity.scope] Animus MUST treat Clankers provider/router contract parity as a future bounded pattern and MUST NOT add provider clients, router daemons, OAuth, credential lookup, network transport, or live provider checks in this evaluation change.

#### Scenario: Provider parity scope is explicit

r[agent_harness_runtime.clankers_provider_parity.scope.future]
- GIVEN maintainers evaluate Clankers provider parity patterns
- WHEN the adoption note is reviewed
- THEN it documents parity criteria for future provider abstractions
- AND it keeps concrete provider clients, router daemons, OAuth, credentials, network transport, and live checks out of scope.

### Requirement: Shared provider request fields are defined when needed

r[agent_harness_runtime.clankers_provider_parity.request_fields] If Animus introduces multiple provider or router request DTOs, it MUST define shared request-field expectations and parity checks for fields such as model, messages, system prompt, output limits, temperature, tools, metadata, cache policy, and extra parameters as applicable.

#### Scenario: Shared fields stay aligned

r[agent_harness_runtime.clankers_provider_parity.request_fields.aligned]
- GIVEN multiple provider request DTOs coexist
- WHEN parity tests run
- THEN shared fields serialize and project consistently
- AND missing cross-cutting metadata is detected before provider dispatch.

### Requirement: Constructor inventory catches request drift

r[agent_harness_runtime.clankers_provider_parity.constructor_inventory] Concrete provider request structs with multiple construction sites SHOULD have a constructor-inventory rail that requires each real constructor to set all required shared fields.

#### Scenario: Constructor inventory fails on omitted fields

r[agent_harness_runtime.clankers_provider_parity.constructor_inventory.omission]
- GIVEN a new provider request constructor omits a required shared field
- WHEN the constructor-inventory rail runs
- THEN it fails with an actionable diagnostic
- AND maintainers update either the constructor or the accepted inventory count.

### Requirement: Provider-native message conversion

r[agent_harness_runtime.clankers_provider_parity.message_conversion] Provider routing adapters MUST use provider-native message conversion fixtures when provider-specific message shapes are required and MUST NOT rely on lossy generic JSON conversion.

#### Scenario: Native conversion preserves content

r[agent_harness_runtime.clankers_provider_parity.message_conversion.fixture]
- GIVEN a provider backend requires a native message shape
- WHEN conversion tests compare against an explicit fixture
- THEN user, assistant, tool, summary, and metadata content required by the backend are preserved
- AND lossy generic conversion that drops content fails.

### Requirement: Streaming and provider-prefix failures are tested

r[agent_harness_runtime.clankers_provider_parity.streaming] Future streaming provider claims MUST include parser-entrypoint tests that feed raw stream bytes through the real parser.

r[agent_harness_runtime.clankers_provider_parity.fail_closed_prefix] Future provider-qualified model IDs MUST fail closed for known-but-unavailable providers rather than falling back silently.

#### Scenario: Provider parity closeout is evidenced

r[agent_harness_runtime.clankers_provider_parity.closeout]
- GIVEN provider parity evaluation is ready to archive
- WHEN closeout validation runs
- THEN evidence includes scope docs, request-field criteria, constructor-inventory pattern, message-conversion fixtures, streaming parser-entrypoint criteria, fail-closed prefix criteria, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Deterministic fixture adapters

r[agent_harness_runtime.std_runtime_adapters.fixtures] Animus MUST provide deterministic fixture adapters for default tests of model, filesystem, process, network, clock, and credential behavior without live providers, real host mutation, or ambient secrets.

#### Scenario: Default tests avoid live effects

r[agent_harness_runtime.std_runtime_adapters.fixtures.no_live_effects]
- GIVEN default workspace checks run on a developer or CI machine
- WHEN concrete adapter tests execute
- THEN they use in-memory or fixture-backed adapters
- AND they do not require network access, credentials, process side effects, or host filesystem mutation beyond test-owned temporary state.

### Requirement: Positive adapter tests

r[agent_harness_runtime.std_runtime_adapters.tests_positive] Concrete adapter work MUST include positive tests for successful injected model, tool, filesystem, process, and HTTP helper flows using deterministic fixtures.

#### Scenario: Fixture-backed adapters produce runtime events

r[agent_harness_runtime.std_runtime_adapters.tests_positive.events]
- GIVEN fixture-backed adapters are injected into runtime or shell orchestration
- WHEN a supported request is dispatched
- THEN the adapter returns a typed success
- AND runtime events and receipts record safe identities, status classes, counts, and hashes.

### Requirement: Negative adapter tests

r[agent_harness_runtime.std_runtime_adapters.tests_negative] Concrete adapter work MUST include negative tests for missing credentials, denied capabilities, sandbox mismatch, malformed outputs, timeout or cancellation, unsupported provider/tool features, and receipt leak attempts.

#### Scenario: Adapter failures do not leak sensitive payloads

r[agent_harness_runtime.std_runtime_adapters.tests_negative.safe_failures]
- GIVEN an adapter fails because credentials are missing, capability is denied, sandbox cannot be enforced, output is malformed, or operation times out
- WHEN failure evidence is emitted
- THEN the evidence contains typed issue codes and safe metadata
- AND raw credentials, prompts, provider payloads, full paths, private file bodies, and unbounded command output are omitted.

### Requirement: Dependency isolation

r[agent_harness_runtime.std_runtime_adapters.dependency_isolation] Animus MUST provide dependency-isolation checks proving reusable runtime/core crates do not depend on concrete provider, filesystem, process, network, credential, or platform-enforcement crates.

#### Scenario: Concrete dependency enters reusable crate

r[agent_harness_runtime.std_runtime_adapters.dependency_isolation.rejects_leak]
- GIVEN a concrete adapter dependency is added to a reusable runtime/core crate
- WHEN dependency isolation checks run
- THEN the check fails with an actionable diagnostic
- AND maintainers are directed to move the dependency behind a shell adapter.

### Requirement: Adapter closeout evidence

r[agent_harness_runtime.std_runtime_adapters.closeout] Concrete adapter work MUST document opt-in live smoke-test configuration and run deterministic adapter tests, dependency-isolation checks, workspace checks, and Cairn gates before archive.

#### Scenario: Adapter evidence is reviewable

r[agent_harness_runtime.std_runtime_adapters.closeout.evidence]
- GIVEN concrete adapter work is ready to archive
- WHEN reviewers inspect the evidence
- THEN deterministic tests and dependency-isolation checks are named with status
- AND live smoke tests are either explicitly run with safe configuration or marked unavailable with a blocker.

### Requirement: Typed runtime event stream

r[agent_harness_runtime.pi_event_lifecycle.typed_events] Animus MUST expose a typed runtime event stream for stable agent lifecycle transitions including runtime start, context replay, turn start, loop decision, model dispatch, tool dispatch, tool result, final response, cancellation, compaction, and shutdown.

#### Scenario: Events preserve turn order

r[agent_harness_runtime.pi_event_lifecycle.typed_events.order]
- GIVEN a runtime turn that calls a model and then a tool before producing a final response
- WHEN events are collected from the run
- THEN the events appear in deterministic transition order
- AND each event contains typed correlation IDs sufficient to associate decisions, dispatches, and results.

### Requirement: Host-injected middleware hooks

r[agent_harness_runtime.pi_event_lifecycle.middleware] Animus MUST support host-injected middleware hooks for observe-only, mutate, block, and replace-style boundary decisions without runtime code discovering plugins or loading code from disk.

#### Scenario: Middleware blocks a tool call

r[agent_harness_runtime.pi_event_lifecycle.middleware.block_tool]
- GIVEN a host injects a tool-call middleware that denies a requested capability
- WHEN the loop decision requests that tool
- THEN runtime emits a typed tool-denied event and report status
- AND the tool adapter is not called.

### Requirement: Deterministic hook chaining

r[agent_harness_runtime.pi_event_lifecycle.hook_order] Middleware hooks that can modify a boundary payload MUST run in configured deterministic order, and each later hook MUST observe the accepted output of earlier hooks.

#### Scenario: Hook mutation order is reproducible

r[agent_harness_runtime.pi_event_lifecycle.hook_order.chained]
- GIVEN two middleware hooks modify safe metadata on a model dispatch request
- WHEN runtime prepares the dispatch
- THEN the second hook observes the first hook's accepted mutation
- AND repeated runs with the same hook order produce identical event and report metadata.

### Requirement: Hook failures fail safe at guarded boundaries

r[agent_harness_runtime.pi_event_lifecycle.fail_safe_hooks] Errors returned by middleware guarding model dispatch, tool dispatch, compaction, replay, or session replacement MUST fail safe with typed runtime failures rather than allowing the guarded action to continue.

#### Scenario: Tool guard error denies execution

r[agent_harness_runtime.pi_event_lifecycle.fail_safe_hooks.tool]
- GIVEN a middleware hook errors while evaluating a tool-call boundary
- WHEN runtime handles the hook error
- THEN runtime emits a typed hook-failure event and marks the tool call denied or aborted
- AND it does not execute the tool adapter.

### Requirement: Event stream is mode-neutral

r[agent_harness_runtime.pi_event_lifecycle.mode_neutral] Runtime events MUST be reusable by direct SDK-style embedding, headless protocol streaming, receipts, and future UI shells without embedding TUI-specific component state in the core event DTOs.

#### Scenario: Headless consumer receives same lifecycle facts

r[agent_harness_runtime.pi_event_lifecycle.mode_neutral.headless]
- GIVEN a headless host subscribes to runtime events
- WHEN a run completes with model and tool activity
- THEN the host receives the same stable lifecycle facts as an in-process SDK subscriber
- AND no event requires terminal UI state to interpret.

### Requirement: Typed lifecycle hook registry

r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.event_registry] Animus MUST expose typed lifecycle hook events for session, prompt, tool, permission, compaction, subagent, and stop boundaries without requiring concrete hook execution in the runtime core.

#### Scenario: Hook event routes to matching definitions

r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.event_registry.routing]
- GIVEN a trusted hook definition matches a pre-tool-use event
- WHEN runtime emits that lifecycle event
- THEN the shell receives a typed hook request for the matching definition
- AND definitions for unrelated event kinds are not selected.

### Requirement: Content-addressed hook trust

r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.trust_identity] Hook definitions MUST carry stable identities and BLAKE3 content hashes so trust decisions apply to the reviewed definition content.

#### Scenario: Changed hook definition loses prior trust

r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.trust_identity.changed]
- GIVEN a user-trusted hook definition is edited after review
- WHEN Animus computes the hook definition hash for a later run
- THEN the old trust record does not authorize the changed definition
- AND the hook is skipped or blocked according to policy until reviewed again.

### Requirement: Event-scoped hook input and output contracts

r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.io_contract] Hook inputs and outputs MUST be validated against the event-specific contract before hook effects are applied.

#### Scenario: Unsupported hook effect is rejected

r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.io_contract.unsupported]
- GIVEN a prompt-submit hook returns an output field reserved for tool-input rewriting
- WHEN Animus validates the hook output
- THEN the unsupported effect is rejected for that event
- AND the runtime records a malformed hook-output failure without applying the effect.

### Requirement: Hooks cannot bypass policy boundaries

r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.policy_binding] Hook decisions MUST flow through runtime policy and MUST NOT directly invoke adapters, weaken sandbox policy, or suppress required approvals.

#### Scenario: Hook cannot override a denied action

r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.policy_binding.bypass]
- GIVEN exec policy forbids a command outside the active sandbox
- WHEN a hook attempts to allow or rewrite that denied command
- THEN the command remains denied
- AND the runtime records the hook attempt as policy-bounded context rather than executing the command.

### Requirement: Hook outcome reporting

r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.outcome_reporting] Runtime reports MUST include hook outcome events for skipped, untrusted, failed, denied, malformed, unsupported, and applied hook results with redacted inputs and auditable hook identities.

#### Scenario: Hook report redacts sensitive input

r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.outcome_reporting.redacted]
- GIVEN a hook input contains prompt or tool data marked sensitive by redaction policy
- WHEN runtime records the hook outcome
- THEN the report includes hook identity, event kind, trust status, and outcome class
- AND the sensitive input body is omitted or replaced by approved redaction metadata.

### Requirement: Bounded lifecycle hook reference

r[agent_harness_runtime.claude_code_hooks.scope] Animus MUST treat Claude Code hooks as a bounded reference for lifecycle extension points, and MUST NOT adopt arbitrary shell hook execution, hook-file authority, or unvalidated hook-driven state mutation in reusable runtime crates.

#### Scenario: Hook cannot bypass action policy

r[agent_harness_runtime.claude_code_hooks.scope.no_bypass]
- GIVEN a pre-action hook requests a broader filesystem, exec, network, model, Steel, or tool action
- WHEN runtime evaluates the hook outcome
- THEN the proposed action must pass normal DTO validation, permission checks, and capability authorization
- AND dispatch is denied if those checks fail.

### Requirement: Stable hook point vocabulary

r[agent_harness_runtime.claude_code_hooks.points] Animus hook descriptors MUST use a stable hook-point vocabulary for session lifecycle, prompt assembly, model request/response, tool pre/post, mutation pre/post, validation, receipt emission, and workflow completion events.

#### Scenario: Unknown hook point fails validation

r[agent_harness_runtime.claude_code_hooks.points.unknown]
- GIVEN a hook descriptor names an unsupported hook point
- WHEN hook registry validation runs
- THEN validation returns a typed unknown-hook-point diagnostic
- AND the hook is not registered.

### Requirement: Safe hook event payloads

r[agent_harness_runtime.claude_code_hooks.payloads] Hook event payloads MUST expose bounded safe metadata such as IDs, classes, redacted resource labels, sizes, hashes, status, and summaries, and MUST NOT include raw prompts, credentials, provider payloads, raw tool bodies, full paths, or unbounded outputs by default.

#### Scenario: Tool post-hook sees bounded result metadata

r[agent_harness_runtime.claude_code_hooks.payloads.bounded]
- GIVEN a tool call produced a large result
- WHEN a tool post-hook request is prepared
- THEN the hook payload includes truncation status, sizes, safe labels, and BLAKE3 hashes where needed
- AND the raw full result body is absent unless a host-specific policy explicitly grants a separate safe handle.

### Requirement: Host-owned hook execution

r[agent_harness_runtime.claude_code_hooks.execution] Hook execution MUST be performed through injected host adapters with declared capabilities, policy-gated execution, typed unavailable-service failures, and no process, filesystem, network, clock, or shell behavior in reusable core crates.

#### Scenario: Missing hook service fails safely

r[agent_harness_runtime.claude_code_hooks.execution.missing_service]
- GIVEN policy requires a pre-tool hook before dispatch
- WHEN no hook service is injected
- THEN runtime reports a required-hook-unavailable failure
- AND the guarded action is not dispatched.

### Requirement: Hook outcomes require revalidation

r[agent_harness_runtime.claude_code_hooks.outcomes] Hook outcomes MAY observe, annotate, deny, or request revalidation, but any action or payload modification proposed by a hook MUST pass normal runtime validation and authorization before use.

#### Scenario: Malformed hook modification is rejected

r[agent_harness_runtime.claude_code_hooks.outcomes.invalid_modification]
- GIVEN a hook returns a malformed or unauthorized modified tool request
- WHEN runtime processes the hook outcome
- THEN runtime rejects the modification with typed diagnostics
- AND the original guarded action is denied or revalidated according to policy without using malformed data.

### Requirement: Typed model-router recipes

r[agent_harness_runtime.vllm_semantic_router_recipes.contracts] Animus model-router recipes MUST be typed contracts for single-route, confidence escalation, ratings ensemble, ReMoM-style quorum synthesis, fusion judge/finalizer, and bounded role workflow route families, including allowed roles, output contract, budget, concurrency, quorum, synthesis, repair, and fallback fields.

#### Scenario: Malformed recipe rejected

r[agent_harness_runtime.vllm_semantic_router_recipes.contracts.invalid]
- GIVEN a router recipe has duplicate roles, unknown model roles, a missing output contract, or an unsupported route family
- WHEN recipe validation runs
- THEN validation rejects the recipe before provider dispatch.

### Requirement: Pure request-signal projection

r[agent_harness_runtime.vllm_semantic_router_recipes.signals] Request-signal projection MUST be a pure deterministic function over bounded metadata for task shape, risk, contract pressure, context pressure, latency band, cost band, privacy/locality, and fallback eligibility.

#### Scenario: Signal projection omits unsafe payloads

r[agent_harness_runtime.vllm_semantic_router_recipes.signals.safe]
- GIVEN request metadata is available with sensitive prompt bodies, provider payloads, credentials, or unbounded text omitted
- WHEN signal projection runs
- THEN projection returns bounded labels and score buckets
- AND it does not require or emit raw prompt bodies, provider payloads, credentials, or unbounded text.

### Requirement: Deterministic recipe selection

r[agent_harness_runtime.vllm_semantic_router_recipes.selection] Recipe selection MUST be deterministic over projected signals, validated recipe policy, budget state, allowed backends, and safe model catalog projections, returning a selected recipe plan or typed denial before any provider call.

#### Scenario: Hard task selects collaborative recipe

r[agent_harness_runtime.vllm_semantic_router_recipes.selection.valid]
- GIVEN projected signals indicate high reasoning variance, strict output contract pressure, and eligible model roles within budget
- WHEN recipe selection runs
- THEN the selected plan uses the configured quorum synthesis or fusion recipe
- AND no provider request is sent during selection.

### Requirement: Bounded execution plans

r[agent_harness_runtime.vllm_semantic_router_recipes.bounds] Collaborative router execution plans MUST declare hard limits for max candidates, max parallelism, min-success quorum, timeout class, cancellation behavior, synthesis attempts, repair attempts, and fallback policy before host execution begins.

#### Scenario: Unbounded fan-out denied

r[agent_harness_runtime.vllm_semantic_router_recipes.bounds.invalid]
- GIVEN a recipe plan omits max parallelism, has zero quorum, requires more successes than candidates, allows unlimited repair, or falls back to a disallowed model
- WHEN execution-plan validation runs
- THEN validation returns a typed denial
- AND no model/provider service is called.

### Requirement: Materialized router execution plans

r[agent_harness_runtime.model_router.execution_plan] A `RouterExecutionPlan` MUST be derived as a pure deterministic function of a `SelectedPlan`, model catalog, and budget, encoding ordered role steps, per-step candidate labels, parallelism caps, per-step quorum thresholds, remaining synthesis and repair attempt counters, and an ordered fallback chain, before any provider or host execution begins.

#### Scenario: Collaborative recipe materializes ordered steps

r[agent_harness_runtime.model_router.execution_plan.ordered_steps]
- GIVEN a `SelectedPlan` for a ReMoM or Fusion recipe with worker, synthesizer, judge, and finalizer roles
- WHEN execution-plan materialization runs
- THEN the plan emits ordered role steps with worker steps before synthesizer, synthesizer before judge, and judge before finalizer
- AND each step records its candidate labels, parallelism cap, and quorum threshold
- AND no provider request is sent during materialization.

#### Scenario: Single recipe materializes trivially

r[agent_harness_runtime.model_router.execution_plan.single]
- GIVEN a `SelectedPlan` for a Single route family with one primary model
- WHEN materialization runs
- THEN the plan has one role step with a quorum of one and no fallback chain
- AND synthesis and repair counters are zero.

### Requirement: Execution plan invariants fail closed

r[agent_harness_runtime.model_router.execution_plan.invariants] Plan materialization MUST return a typed denial when quorum exceeds candidate count, quorum is zero, repair is unbounded, a recipe role has no mapped step, the fallback chain is cyclic, or a fallback model is not eligible under backend or privacy constraints, and MUST NOT return a plan in those cases.

#### Scenario: Impossible quorum denied

r[agent_harness_runtime.model_router.execution_plan.invariants.impossible_quorum]
- GIVEN a selected plan requires a min-success quorum greater than its candidate count
- WHEN materialization runs
- THEN it returns an impossible-quorum denial
- AND no execution plan is produced.

#### Scenario: Cyclic fallback denied

r[agent_harness_runtime.model_router.execution_plan.invariants.cyclic_fallback]
- GIVEN a fallback chain references the same model label more than once in a cycle
- WHEN materialization runs
- THEN it returns a cyclic-fallback denial
- AND no execution plan is produced.

### Requirement: Execution plan verification

r[agent_harness_runtime.model_router.execution_plan.verification] The change MUST include focused positive and negative tests for each recipe family's plan, invariant denials, budget counters, fallback ordering, workspace checks, Cairn validation, and Cairn gates before archive.

#### Scenario: Closeout evidence covers execution plans

r[agent_harness_runtime.model_router.execution_plan.verification.closeout]
- GIVEN maintainers intend to archive the execution-plan change
- WHEN closeout validation runs
- THEN evidence names focused materialization, invariant, fail-closed, facade-inventory, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Pure router budget tracking

r[agent_harness_runtime.model_router.budget_fallback] Animus MUST track router budget as an immutable `BudgetLedger` initialized from `Bounds` with pure decrement helpers for candidate, parallelism, synthesis, and repair consumption, and MUST report exhaustion as typed `BudgetExhausted` denials rather than allowing caps to be exceeded.

#### Scenario: Repair budget exhaustion stops retry

r[agent_harness_runtime.model_router.budget_fallback.repair_exhausted]
- GIVEN a route has consumed all `max_repair_attempts`
- WHEN a step fails and `evaluate_fallback` runs
- THEN it returns `Deny` or `FailRoute`, never `Repair`
- AND no further repair attempt is permitted.

### Requirement: Fallback evaluation is pure and deny-by-default

r[agent_harness_runtime.model_router.budget_fallback.evaluation] Fallback MUST be resolved by a pure `evaluate_fallback` function over a step failure, ledger, fallback chain, and `FallbackPolicy` that returns `Deny`, `Repair(attempt)`, `FallbackTo(model)`, or `FailRoute(reason)`, and the default `FallbackPolicy` MUST deny fallback unless policy explicitly authorizes the allowed fallback role/backend and exhaustion class.

#### Scenario: Unauthorized fallback denied

r[agent_harness_runtime.model_router.budget_fallback.evaluation.unauthorized]
- GIVEN a step fails and the `FallbackPolicy` does not name an allowed fallback for its class
- WHEN `evaluate_fallback` runs
- THEN it returns `Deny`
- AND no rerouting to another model occurs.

### Requirement: Fallback chain is acyclic and budget-bound

r[agent_harness_runtime.model_router.budget_fallback.chain] Fallback MUST consume the chain in order, MUST NOT revisit a model already tried in the route, MUST stay within remaining candidate and parallelism budget, and MUST check backend and privacy eligibility before authorizing `FallbackTo`.

#### Scenario: Previously tried model not revisited

r[agent_harness_runtime.model_router.budget_fallback.chain.no_revisit]
- GIVEN a fallback model has already been tried and failed in the route
- WHEN `evaluate_fallback` runs for a later failure
- THEN it does not return `FallbackTo` for that model
- AND if no unconsumed eligible entry remains, it returns `FailRoute`.

### Requirement: Budget and fallback verification

r[agent_harness_runtime.model_router.budget_fallback.verification] The change MUST include focused positive and negative tests for budget tracking, authorized repair, authorized fallback, fail-route, exhaustion, policy deny, cyclic fallback, ineligible fallback, cap-exceeding retry, workspace checks, Cairn validation, and Cairn gates before archive.

#### Scenario: Closeout evidence covers budget and fallback

r[agent_harness_runtime.model_router.budget_fallback.verification.closeout]
- GIVEN maintainers intend to archive the budget-and-fallback change
- WHEN closeout validation runs
- THEN evidence names focused ledger, evaluation, chain, default-deny, fail-closed, facade-inventory, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Output contracts gate collaborative success

r[agent_harness_runtime.vllm_semantic_router_recipes.output_contract] Collaborative routes MUST validate the final response contract after synthesis, finalization, repair, or fallback before reporting success to callers or automation.

#### Scenario: Malformed final output fails closed

r[agent_harness_runtime.vllm_semantic_router_recipes.output_contract.invalid]
- GIVEN worker responses produce evidence but synthesis or repair returns malformed final output
- WHEN response validation runs
- THEN the route returns a typed malformed-output failure or a policy-declared best-valid-evidence fallback
- AND success is not claimed for invalid output.

### Requirement: Safe router-recipe receipts

r[agent_harness_runtime.vllm_semantic_router_recipes.receipts] Router-recipe receipts MUST record safe metadata such as recipe ID, signal buckets, selected model labels, bounded counts, quorum class, fallback class, policy hash, duration buckets, redaction markers, and BLAKE3 identity, and MUST NOT include raw prompts, provider payloads, worker outputs, judge rationales, hidden reasoning, credentials, full paths, or unbounded traces.

#### Scenario: Router receipt leak rejected

r[agent_harness_runtime.vllm_semantic_router_recipes.receipts.rejects_leaks]
- GIVEN a router-recipe receipt fixture contains raw prompt text, provider JSON, worker output, judge rationale, hidden reasoning text, credentials, full paths, or an unbounded trace
- WHEN receipt validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Router-recipe verification

r[agent_harness_runtime.vllm_semantic_router_recipes.verification] The adaptation MUST include focused positive and negative tests for recipe validation, signal projection, deterministic selection, execution bounds, output-contract validation, fallback behavior, and receipt redaction before closeout.

#### Scenario: Closeout evidence covers router recipes

r[agent_harness_runtime.vllm_semantic_router_recipes.verification.closeout]
- GIVEN maintainers intend to archive the router-recipe adaptation
- WHEN closeout validation runs
- THEN evidence names focused recipe, signal, selection, bounds, output, receipt, roadmap, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Mode-neutral task graph DTOs

r[agent_harness_runtime.hermes_kanban.task_graph] Animus MUST define mode-neutral task graph DTOs for durable orchestration with task ID, title, body, assignee label, lifecycle state, parent dependencies, child links, priority metadata, workspace scope, tenant label, comments, heartbeats, block records, completion summary, and bounded completion metadata.

#### Scenario: Valid task graph snapshot is accepted

r[agent_harness_runtime.hermes_kanban.task_graph.valid]
- GIVEN a host supplies a task graph snapshot with unique IDs, valid parent links, workspace scopes, tenant labels, and bounded metadata
- WHEN task graph validation runs
- THEN the graph is accepted for orchestration decisions
- AND no board database, process supervisor, VCS command, or dispatcher service is required by the pure validation logic.

### Requirement: Dependency promotion is deterministic

r[agent_harness_runtime.hermes_kanban.dependencies] Animus MUST provide deterministic parent/child dependency evaluation for fan-out/fan-in orchestration and MUST reject cycles, missing parents, duplicate IDs, and ambiguous dependency state.

#### Scenario: Child becomes ready after all parents complete

r[agent_harness_runtime.hermes_kanban.dependencies.promote]
- GIVEN a child task depends on two parent tasks
- WHEN both parents transition to completed with accepted handoff metadata
- THEN dependency evaluation marks the child eligible for ready state
- AND repeated evaluation over the same graph snapshot returns the same result.

#### Scenario: Cyclic dependencies fail closed

r[agent_harness_runtime.hermes_kanban.dependencies.cycle]
- GIVEN task A depends on task B and task B depends on task A
- WHEN graph validation runs
- THEN validation returns a typed cycle diagnostic
- AND neither task is promoted or scheduled.

### Requirement: Workspace scopes are explicit

r[agent_harness_runtime.hermes_kanban.workspace_scope] Task orchestration MUST represent workspace scopes explicitly and MUST NOT infer filesystem mutation authority from ambient process state.

#### Scenario: Worker is constrained to declared workspace

r[agent_harness_runtime.hermes_kanban.workspace_scope.constrained]
- GIVEN a task declares a scratch workspace with a host-supplied root capability
- WHEN a worker attempts to record a mutation outside that workspace capability
- THEN orchestration validation reports an out-of-scope mutation or requires a separate host approval
- AND the task completion cannot claim unrestricted workspace authority.

### Requirement: Tenant labels prevent context leakage

r[agent_harness_runtime.hermes_kanban.tenant_scope] Task handoffs, comments, memory references, and downstream context summaries MUST preserve tenant labels when supplied and MUST support host enforcement that prevents cross-tenant leakage.

#### Scenario: Cross-tenant handoff is rejected

r[agent_harness_runtime.hermes_kanban.tenant_scope.reject_cross_tenant]
- GIVEN a child task belongs to tenant `alpha` and a parent handoff belongs to tenant `beta`
- WHEN dependency promotion or context assembly evaluates the handoff
- THEN validation rejects or quarantines the cross-tenant dependency according to host policy
- AND the beta handoff body is not inserted into alpha model context.

### Requirement: Lifecycle handoffs are structured and bounded

r[agent_harness_runtime.hermes_kanban.handoffs] Task lifecycle events MUST support structured heartbeat, block, comment, and completion records with bounded text, safe metadata, and clear actor/source attribution.

#### Scenario: Block reason carries actionable context

r[agent_harness_runtime.hermes_kanban.handoffs.block]
- GIVEN a worker cannot continue without a human or upstream decision
- WHEN it emits a block record
- THEN the record includes a short reason, optional bounded context comment, actor attribution, timestamp or sequence metadata supplied by the shell, and next-action hint
- AND the task is not marked completed.

### Requirement: Kanban adaptation verification

r[agent_harness_runtime.hermes_kanban.verification] The Kanban adaptation MUST include focused positive and negative tests for graph validation, dependency promotion, cycle rejection, workspace scope checks, tenant isolation, handoff bounds, stale completion rejection, and optional adapter isolation.

#### Scenario: Closeout evidence covers orchestration fixtures

r[agent_harness_runtime.hermes_kanban.verification.closeout]
- GIVEN maintainers intend to archive the Hermes Kanban adaptation
- WHEN closeout validation runs
- THEN evidence names focused graph tests, dependency tests, workspace/tenant tests, handoff tests, workspace checks, Cairn validation, and Cairn gates
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Hermes-style skill manifest descriptors

r[agent_harness_runtime.hermes_skill_manifest.descriptor] Animus MUST define typed skill manifest descriptors for host-provided workflow guidance with validated name, description, version, source category, tags, related skill names, provenance, trust status, and safe content identity.

#### Scenario: Valid skill manifest is accepted

r[agent_harness_runtime.hermes_skill_manifest.descriptor.valid]
- GIVEN a host supplies a Hermes-style skill manifest with a valid name, description, version, category, tags, related skill names, body identity, provenance, and trust status
- WHEN Animus validates descriptors for resource advertisement
- THEN the manifest is accepted as a skill resource descriptor
- AND the default model-visible summary contains only the validated name, description, category, tags, and safe provenance label.

### Requirement: Manifest validation fails closed

r[agent_harness_runtime.hermes_skill_manifest.validation] Animus MUST reject malformed skill manifests before prompt construction, command lookup, or full-content loading.

#### Scenario: Invalid manifest fields are rejected

r[agent_harness_runtime.hermes_skill_manifest.validation.invalid]
- GIVEN a skill manifest has an empty name, missing description, overlong description, unsupported version shape, duplicate tags, missing body identity, malformed related skill name, or unsafe content handle
- WHEN manifest validation runs
- THEN validation returns typed diagnostics naming every failed invariant
- AND the invalid manifest is not advertised or loaded.

### Requirement: Skill manifests map into progressive disclosure

r[agent_harness_runtime.hermes_skill_manifest.progressive_disclosure] Hermes-style skill manifests MUST integrate with Animus progressive disclosure so summary metadata can be advertised independently from the full skill body.

#### Scenario: Explicit load includes only selected body

r[agent_harness_runtime.hermes_skill_manifest.progressive_disclosure.selected]
- GIVEN multiple trusted skill manifests are advertised by summary
- WHEN a host-approved full-content load selects one skill by name
- THEN only that skill body is made available to prompt construction
- AND unrelated skill bodies, unresolved related skills, and untrusted project-local bodies remain out of model context.

### Requirement: Hermes runtime layout remains host policy

r[agent_harness_runtime.hermes_skill_manifest.layout_boundary] Animus MUST NOT require Hermes local directories, symlink conventions, runtime caches, user-local categories, or plugin installation paths in reusable runtime crates.

#### Scenario: Host supplies manifests from a non-Hermes store

r[agent_harness_runtime.hermes_skill_manifest.layout_boundary.host_store]
- GIVEN a host extracts equivalent skill manifest data from an application database, embedded bundle, or non-Hermes directory
- WHEN it passes validated manifest descriptors to Animus
- THEN runtime behavior matches descriptors parsed from Hermes-style `SKILL.md` files
- AND no reusable Animus crate attempts to inspect `~/.hermes`, `~/.pi`, or repository-local skill directories.

### Requirement: Manifest adaptation verification

r[agent_harness_runtime.hermes_skill_manifest.verification] The Hermes skill manifest adaptation MUST include positive and negative tests for valid manifests, malformed manifest rejection, progressive disclosure, untrusted-body omission, related-skill diagnostics, duplicate-name handling, and host-layout isolation.

#### Scenario: Closeout evidence covers manifest fixtures

r[agent_harness_runtime.hermes_skill_manifest.verification.closeout]
- GIVEN maintainers intend to archive the Hermes skill manifest adaptation
- WHEN closeout validation runs
- THEN evidence names focused manifest fixture tests, prompt disclosure tests, workspace checks, Cairn validation, and Cairn gates
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Structured review report DTO

r[agent_harness_runtime.hermes_review_reports.report] Animus MUST define a structured review report contract with a required `VERDICT: PASS | WARN | FAIL` line, parsed sections, metric-bearing findings, diagnostics, source, stage, subject, and safe metadata.

#### Scenario: Valid report parses deterministically

r[agent_harness_runtime.hermes_review_reports.report.valid]
- GIVEN a markdown review report with `VERDICT: WARN`, a `## Findings` section, tagged finding bullets, and optional `## Gaps` or `## Next Steps` sections
- WHEN report parsing runs over the in-memory text
- THEN the parsed report contains verdict `WARN`, normalized sections, parsed findings, and no diagnostics
- AND parsing does not read files, clocks, environment, process state, or network resources.

### Requirement: Findings are parsed only from Findings section

r[agent_harness_runtime.hermes_review_reports.findings_scope] Animus MUST parse metric-bearing finding bullets only from an explicit `## Findings` section.

#### Scenario: Checklist bullets outside Findings are ignored

r[agent_harness_runtime.hermes_review_reports.findings_scope.ignore_checklists]
- GIVEN a report contains bracketed checklist bullets in `## OpenSpec Check`, `## Gaps`, or `## Next Steps`
- WHEN metric findings are extracted
- THEN only valid bullets inside `## Findings` become findings
- AND checklist bullets outside that section remain human-readable prose.

### Requirement: Finding tags are validated

r[agent_harness_runtime.hermes_review_reports.finding_tags] Metric-bearing finding bullets MUST include `class`, `scope`, `route`, and `promoted` tags, and `promoted` MUST be either `yes` or `no`.

#### Scenario: Malformed finding tags reject metrics record

r[agent_harness_runtime.hermes_review_reports.finding_tags.invalid]
- GIVEN a review report has a finding bullet missing `route` or using `promoted=maybe`
- WHEN a metrics record action validates the report
- THEN validation fails with typed diagnostics naming the malformed tags
- AND no durable metrics record is appended.

### Requirement: Metrics read/write behavior is explicit

r[agent_harness_runtime.hermes_review_reports.metrics_boundary] Animus MUST separate read-only report parsing, summary, and promotion-candidate analysis from explicit append-only metrics recording.

#### Scenario: Summary does not write metrics

r[agent_harness_runtime.hermes_review_reports.metrics_boundary.summary_readonly]
- GIVEN durable review metric records already exist
- WHEN a host requests a summary or promotion-candidate report
- THEN the operation reads supplied records and returns bounded aggregate output
- AND it does not append, rewrite, compact, delete, or promote records.

#### Scenario: Explicit record action appends valid report

r[agent_harness_runtime.hermes_review_reports.metrics_boundary.record_append]
- GIVEN a valid parsed report, source, stage, subject, and host-supplied timestamp or sequence metadata
- WHEN an explicit metrics record action runs
- THEN exactly one append-only record is produced through the host storage adapter
- AND the record excludes raw prompts, provider payloads, credentials, full host paths, and unbounded tool/model output.

### Requirement: Repeated finding promotion summaries

r[agent_harness_runtime.hermes_review_reports.promotions] Animus SHOULD support read-only repeated-finding grouping that suggests durable prevention routes such as tests, deterministic checks, prompt rails, spec rules, or human checkpoints without mutating tasks or specs automatically.

#### Scenario: Repeated finding becomes promotion candidate

r[agent_harness_runtime.hermes_review_reports.promotions.candidate]
- GIVEN multiple metric records contain unpromoted findings with the same class, scope, and route tags
- WHEN promotion-candidate analysis runs with a configured minimum count
- THEN the output groups those findings, includes representative examples, and suggests a prevention route
- AND the analysis does not mark anything promoted or edit lifecycle artifacts.

### Requirement: Review report adaptation verification

r[agent_harness_runtime.hermes_review_reports.verification] The review report adaptation MUST include positive and negative tests for verdict parsing, section parsing, finding extraction, malformed tag rejection, metrics append validation, read-only summaries, promotion candidate grouping, and safe metadata redaction.

#### Scenario: Closeout evidence covers report fixtures

r[agent_harness_runtime.hermes_review_reports.verification.closeout]
- GIVEN maintainers intend to archive the Hermes review report adaptation
- WHEN closeout validation runs
- THEN evidence names focused report parser tests, metrics boundary tests, promotion grouping tests, workspace checks, Cairn validation, and Cairn gates
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Typed policy/profile descriptors

r[agent_harness_runtime.hermes_policy_profiles.descriptor] Animus MUST define typed policy/profile descriptors for host-supplied operating guidance with policy kind, source scope, trust status, priority, goal text, success criteria, constraints, validation rules, stop rules, provenance, and optional prompt snippets.

#### Scenario: Valid policy profile composes into prompt input

r[agent_harness_runtime.hermes_policy_profiles.descriptor.valid]
- GIVEN a trusted host supplies a policy profile with goal, success criteria, constraints, validation rules, stop rules, provenance, and priority
- WHEN prompt assembly prepares policy context
- THEN the profile contributes an attributed bounded prompt segment
- AND the descriptor remains available as structured metadata for runtime/report evidence.

### Requirement: Deterministic policy precedence and conflict diagnostics

r[agent_harness_runtime.hermes_policy_profiles.precedence] Animus MUST compose multiple policy descriptors in deterministic precedence order and report conflicts between side-effect limits, validation requirements, or stop rules.

#### Scenario: Global safety rule defeats project override

r[agent_harness_runtime.hermes_policy_profiles.precedence.safety_wins]
- GIVEN a trusted global policy forbids a destructive action without explicit approval and a project-local policy attempts to allow that action implicitly
- WHEN policy composition runs
- THEN the global safety constraint remains active
- AND diagnostics identify the overridden project-local rule without embedding unsafe project prompt text.

### Requirement: Policy prompts are outcome-first and bounded

r[agent_harness_runtime.hermes_policy_profiles.prompt_hygiene] Policy-derived prompt text MUST prioritize goal, success criteria, constraints, required output shape, validation, and stop rules, and MUST be bounded to prevent context bloat.

#### Scenario: Process-heavy policy is rejected or warned

r[agent_harness_runtime.hermes_policy_profiles.prompt_hygiene.process_heavy]
- GIVEN a policy descriptor contains a long process script with no observable success criteria or stop rule
- WHEN policy validation runs
- THEN validation rejects it or emits a prompt-hygiene diagnostic according to policy severity
- AND prompt assembly does not silently include unbounded process-heavy text.

### Requirement: Project-local policy is trust-gated

r[agent_harness_runtime.hermes_policy_profiles.project_trust] Project-local policy descriptors MUST NOT influence prompts, tool activation, command expansion, or runtime guardrails until host trust permits them.

#### Scenario: Untrusted project policy is omitted

r[agent_harness_runtime.hermes_policy_profiles.project_trust.omit]
- GIVEN a project-local policy descriptor is marked untrusted
- WHEN policy context is assembled
- THEN the descriptor is omitted or marked unavailable according to host policy
- AND its constraints and prompt snippets do not override trusted global policy.

### Requirement: Policy descriptors do not replace enforcement

r[agent_harness_runtime.hermes_policy_profiles.enforcement_boundary] Animus MUST treat policy descriptors as guidance and configuration input, not as the sole enforcement mechanism for capability denial, read-only mode, mutation serialization, or credential redaction.

#### Scenario: Read-only policy also has runtime guard

r[agent_harness_runtime.hermes_policy_profiles.enforcement_boundary.readonly]
- GIVEN policy context says the session is read-only
- WHEN a tool call requests filesystem mutation
- THEN runtime enforcement denies or blocks the mutation through a typed capability boundary
- AND the denial does not rely only on model obedience to prompt text.

### Requirement: Policy profile adaptation verification

r[agent_harness_runtime.hermes_policy_profiles.verification] The policy profile adaptation MUST include focused tests for valid composition, conflict precedence, prompt hygiene, trust gating, enforcement-boundary integration, prompt budget bounds, stale model guidance rejection, and safe diagnostics.

#### Scenario: Closeout evidence covers policy composition

r[agent_harness_runtime.hermes_policy_profiles.verification.closeout]
- GIVEN maintainers intend to archive the Hermes policy profile adaptation
- WHEN closeout validation runs
- THEN evidence names focused policy composition tests, prompt-hygiene tests, trust-boundary tests, enforcement-boundary tests, workspace checks, Cairn validation, and Cairn gates
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: MCP Apps evaluation matrix is explicit

r[agent_harness_runtime.goose_mcp_app_surfaces.evaluation_matrix] Animus MUST evaluate Goose MCP Apps patterns with an explicit matrix covering UI resource identity, MIME/profile tags, CSP domains, permission requests, host-context sync, callbacks, sizing, lifecycle bounds, and adoption risk.

#### Scenario: Unsafe UI metadata is classified

r[agent_harness_runtime.goose_mcp_app_surfaces.evaluation_matrix.fixtures]
- GIVEN evaluation fixtures include safe UI metadata and unsafe broad-domain, broad-permission, unbounded-callback, or ambient-host-control descriptors
- WHEN the MCP Apps evaluation matrix is applied
- THEN each fixture is classified with an adoption decision and risk reason
- AND no fixture claims runtime UI execution behavior.

### Requirement: UI metadata descriptors remain data-only

r[agent_harness_runtime.goose_mcp_app_surfaces.metadata_descriptor] Animus MAY define a data-only UI metadata descriptor sketch for future tool-result UI resources, but reusable validation MUST NOT render HTML, run JavaScript, open remote resources, request browser permissions, or send chat callbacks.

#### Scenario: Descriptor validation has no UI execution

r[agent_harness_runtime.goose_mcp_app_surfaces.metadata_descriptor.no_execution]
- GIVEN a metadata descriptor includes a UI resource URI, MIME/profile tag, CSP domains, permissions, and callback names
- WHEN reusable validation runs
- THEN validation inspects only descriptor data
- AND HTML rendering, JavaScript execution, remote resource loading, browser permission requests, and chat callbacks are absent.

### Requirement: Future MCP App surfaces are deny-by-default

r[agent_harness_runtime.goose_mcp_app_surfaces.security_constraints] Any future MCP App-style surface SHOULD default to no remote domains, no nested frames, no device permissions, no clipboard write, no direct model-visible message injection, and bounded host-context callbacks unless explicitly admitted by policy.

#### Scenario: Broad UI permission request is denied in evaluation

r[agent_harness_runtime.goose_mcp_app_surfaces.security_constraints.denied]
- GIVEN a candidate UI metadata descriptor requests wildcard resource domains, camera access, clipboard write, and direct chat-message callbacks without policy admission
- WHEN evaluation applies the security constraints
- THEN the descriptor is classified as rejected for future default behavior
- AND the evaluation records the required host-policy gates before reconsideration.

### Requirement: MCP Apps evaluation closeout is evidenced

r[agent_harness_runtime.goose_mcp_app_surfaces.verification] MCP Apps surface evaluation MUST include the adoption matrix, positive and negative metadata fixtures, runtime-neutrality checks, security-constraint documentation, Cairn validation, and Cairn gates before archive.

#### Scenario: MCP Apps evaluation evidence is complete

r[agent_harness_runtime.goose_mcp_app_surfaces.verification.closeout]
- GIVEN the Goose MCP Apps surface evaluation change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes the evaluation matrix, safe and unsafe fixtures, runtime-neutrality checks, security-constraint docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Security review inputs are bounded and redacted

r[agent_harness_runtime.goose_security_review_stack.review_input] Animus MUST define bounded security review inputs for original task labels, recent context summaries, tool call descriptor, action class, active policy labels, and redacted evidence identities.

#### Scenario: Review input omits sensitive raw material

r[agent_harness_runtime.goose_security_review_stack.review_input.redacted]
- GIVEN a tool call includes raw arguments, paths, credentials, provider text, or prompt content
- WHEN security review input is constructed
- THEN the input exposes only safe labels, bounded summaries, and BLAKE3 evidence hashes
- AND raw prompts, credentials, full paths, and unbounded tool arguments are omitted.

### Requirement: Deterministic pattern findings are available

r[agent_harness_runtime.goose_security_review_stack.pattern_findings] Animus MUST provide deterministic scanner findings for prompt injection, destructive actions, exfiltration, privilege escalation, obfuscated remote execution, and out-of-scope mutation attempts.

#### Scenario: Dangerous payloads produce findings

r[agent_harness_runtime.goose_security_review_stack.pattern_findings.detects]
- GIVEN a requested tool call attempts hidden instruction following, credential access, unknown-domain upload, recursive deletion, privilege escalation, or download-and-execute behavior
- WHEN deterministic security scanning runs
- THEN scanning returns one or more findings with category, confidence, finding ID, and safe explanation
- AND the finding evidence does not include the raw secret or full sensitive payload.

### Requirement: Classifier and adversary reviews are advisory

r[agent_harness_runtime.goose_security_review_stack.advisory_review] Optional classifier and adversary reviewer outputs MUST be modeled as advisory allow, warn, block, unavailable, timeout, or malformed outcomes that cannot grant missing authority or weaken active policy.

#### Scenario: Reviewer allow cannot bypass authority

r[agent_harness_runtime.goose_security_review_stack.advisory_review.no_authority]
- GIVEN active capability or sandbox policy denies a tool call
- WHEN an adversary reviewer recommends allowing the call
- THEN permission resolution preserves the denial
- AND no provider, tool, host-effect, or confirmation bypass is executed.

### Requirement: Security review failure behavior is policy controlled

r[agent_harness_runtime.goose_security_review_stack.fail_behavior] Security review outages, timeouts, and malformed results MUST resolve through explicit policy-controlled fail behavior by action risk class.

#### Scenario: High-risk review outage fails closed

r[agent_harness_runtime.goose_security_review_stack.fail_behavior.closed]
- GIVEN policy marks network exfiltration or destructive filesystem mutation as high risk
- WHEN the configured adversary reviewer times out before evaluating that action
- THEN runtime records a review-unavailable denial or required confirmation according to policy
- AND the risky action is not silently allowed by default.

### Requirement: Findings bind through permission and receipt policy

r[agent_harness_runtime.goose_security_review_stack.policy_binding] Security findings MUST flow through permission resolution, confirmation prompts, sandbox descriptors, and safe receipts before any reviewed effect is dispatched.

#### Scenario: Blocked finding stops dispatch

r[agent_harness_runtime.goose_security_review_stack.policy_binding.blocks]
- GIVEN a deterministic scanner or admitted advisory reviewer produces a blocking finding for a tool call
- WHEN runtime evaluates the tool call
- THEN dispatch stops before provider, tool, or host-effect execution
- AND the denial receipt contains only safe finding metadata and redacted evidence identities.

### Requirement: Security review closeout is evidenced

r[agent_harness_runtime.goose_security_review_stack.verification] Security review stack adoption MUST include positive benign-review tests, negative prompt-injection and dangerous-action fixtures, advisory-authority bypass tests, fail-behavior tests, receipt redaction tests, documentation, Cairn validation, and Cairn gates before archive.

#### Scenario: Security evidence is complete

- GIVEN the Goose security review stack change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes focused scanner tests, advisory-review tests, fail-behavior tests, policy-binding tests, docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Typed recipe workflow manifests

r[agent_harness_runtime.goose_recipe_workflows.manifest] Animus MUST define typed recipe/workflow manifests for identity, instructions, initial prompt, activities, parameters, model profile, capability ceilings, toolset references, validation gates, retry policy, subrecipes, response schema, and provenance.

#### Scenario: Valid recipe manifest loads deterministically

r[agent_harness_runtime.goose_recipe_workflows.manifest.valid]
- GIVEN a recipe manifest declares instructions, prompt, parameters, allowed toolsets, validation gates, response schema, and provenance
- WHEN pure manifest validation runs
- THEN validation returns a deterministic accepted manifest DTO
- AND no provider, filesystem, process, network, credential, or tool adapter is invoked.

### Requirement: Nickel is the authoritative recipe source

r[agent_harness_runtime.goose_recipe_workflows.nickel_source] Animus recipe authoring MUST use typed Nickel contracts as source of truth, and YAML or JSON recipe files MUST be derived interchange artifacts unless an external adapter explicitly imports them under validation.

#### Scenario: Generated recipe drift is rejected

r[agent_harness_runtime.goose_recipe_workflows.nickel_source.drift]
- GIVEN a YAML export differs from the Nickel recipe source for the same recipe identity
- WHEN generated-config drift validation runs
- THEN validation reports recipe drift
- AND runtime does not prefer the edited export over the checked Nickel source.

### Requirement: Parameter binding is pure and declared

r[agent_harness_runtime.goose_recipe_workflows.parameters] Recipe parameter binding MUST validate declared template variables, safe built-in variables, optional defaults, select options, and file-parameter admission before prompt assembly.

#### Scenario: Unsafe parameter template fails closed

r[agent_harness_runtime.goose_recipe_workflows.parameters.invalid]
- GIVEN a recipe template references an undeclared variable, omits an optional default, includes an invalid select option, or assigns a default to a file parameter
- WHEN parameter binding runs
- THEN binding returns a typed validation error
- AND no recipe prompt, activity, or subrecipe input is assembled from the invalid template.

### Requirement: Structured automation output is validated

r[agent_harness_runtime.goose_recipe_workflows.structured_output] Recipes that declare structured output MUST validate final responses against the declared schema before reporting automation success.

#### Scenario: Malformed final output blocks success

r[agent_harness_runtime.goose_recipe_workflows.structured_output.invalid]
- GIVEN a headless recipe declares a required JSON output field
- WHEN the final assistant output omits that field or uses the wrong type
- THEN output validation records a structured-output failure
- AND automation does not consume the malformed output as a successful result.

### Requirement: Retry policy remains host-mediated

r[agent_harness_runtime.goose_recipe_workflows.retry_policy] Recipe retry checks and cleanup actions MUST be represented as host-effect requests with capability labels, sandbox requirements, timeouts, redaction, and receipts rather than direct reusable-core shell execution.

#### Scenario: Retry cannot bypass policy denial

r[agent_harness_runtime.goose_recipe_workflows.retry_policy.bypass]
- GIVEN a recipe retry check requests a shell command denied by active capability or sandbox policy
- WHEN the retry planner evaluates the check
- THEN the retry attempt records a denied host-effect request
- AND reusable core does not execute the command or report validation success.

### Requirement: Recipe trust and provenance are explicit

r[agent_harness_runtime.goose_recipe_workflows.trust] Imported, changed, first-run, or project-provided recipes MUST have explicit trust and provenance receipts before they can influence prompt assembly, tool availability, model selection, memory use, or workflow permissions.

#### Scenario: Untrusted recipe has no influence

r[agent_harness_runtime.goose_recipe_workflows.trust.denied]
- GIVEN a project recipe has not been admitted by host policy or user confirmation
- WHEN runtime prepares a session using that recipe
- THEN the recipe is rejected or activated in a no-influence failure state
- AND prompt, tools, model, memory, and workflow permissions remain unchanged.

### Requirement: Recipe workflow closeout is evidenced

r[agent_harness_runtime.goose_recipe_workflows.verification] Recipe workflow adoption MUST include positive and negative tests for manifest validation, Nickel source authority, parameter binding, structured output, retry policy, trust receipts, documentation, Cairn validation, and Cairn gates before archive.

#### Scenario: Recipe workflow evidence is complete

- GIVEN the Goose recipe workflow change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes focused recipe workflow tests, Nickel drift checks, structured-output failures, retry-bypass failures, trust-denial failures, docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Recipe-backed subagent delegation is typed

r[agent_harness_runtime.goose_recipe_backed_subagents.delegation] Animus MUST support explicit subagent delegation DTOs that may reference an admitted recipe or profile and include parameter bindings, tool ceilings, workspace scope, model profile, timeout and turn ceilings, return mode, and summary schema.

#### Scenario: Valid recipe-backed subagent request is accepted

r[agent_harness_runtime.goose_recipe_backed_subagents.delegation.valid]
- GIVEN a trusted delegation request references an admitted review recipe with valid parameter bindings and a summary schema
- WHEN subagent delegation validation runs
- THEN validation returns an accepted child-run request
- AND no subagent model or tool dispatch occurs during validation.

### Requirement: Subagent starts require explicit admission

r[agent_harness_runtime.goose_recipe_backed_subagents.explicit] Recipe-backed subagents MUST start only from explicit user delegation, trusted skill delegation, or admitted automation policy delegation, and MUST NOT auto-spawn or recursively spawn by default.

#### Scenario: Implicit or recursive subagent start is denied

r[agent_harness_runtime.goose_recipe_backed_subagents.explicit.denied]
- GIVEN no explicit admitted delegation exists, or a subagent attempts to create another subagent
- WHEN runtime evaluates the child-run request
- THEN the request is rejected
- AND no model, tool, external agent, or host-effect dispatch is started for that child run.

### Requirement: Subagent capability inheritance is intersected

r[agent_harness_runtime.goose_recipe_backed_subagents.capability_intersection] Effective subagent capabilities MUST be the intersection of parent policy, recipe or profile ceilings, request ceilings, workspace scope, and host admission.

#### Scenario: Inherited tool widening is denied

r[agent_harness_runtime.goose_recipe_backed_subagents.capability_intersection.denied]
- GIVEN a parent session has write-capable tools but a recipe-backed review subagent is read-only
- WHEN the subagent requests a write-capable tool or out-of-scope workspace access
- THEN runtime denies the request
- AND the denial is recorded in the subagent ledger and parent handoff status.

### Requirement: Parent handoff follows return mode bounds

r[agent_harness_runtime.goose_recipe_backed_subagents.return_modes] Recipe-backed subagents MUST hand results to the parent using explicit return modes for summary-only, status-plus-summary, or full-detail-with-bounds, while keeping subagent transcripts isolated by default.

#### Scenario: Raw logs cannot flood parent context

r[agent_harness_runtime.goose_recipe_backed_subagents.return_modes.bounded]
- GIVEN a subagent produces long exploratory logs and test output
- WHEN the subagent completes with summary-only return mode
- THEN the parent receives only the bounded summary and evidence references
- AND raw subagent logs remain outside parent context unless explicitly imported under configured bounds.

### Requirement: External subagents use injected adapters

r[agent_harness_runtime.goose_recipe_backed_subagents.external] External subagents backed by ACP or MCP agents MUST use explicit descriptors and injected host adapters rather than ambient CLI discovery, inherited credentials, or undeclared tools.

#### Scenario: Ambient external agent discovery is rejected

r[agent_harness_runtime.goose_recipe_backed_subagents.external.no_ambient]
- GIVEN a subagent recipe names an external agent without an admitted ACP or MCP descriptor and injected host service
- WHEN runtime prepares the child run
- THEN preparation returns a missing-external-agent-service failure
- AND runtime does not search PATH, read credentials, or start a fallback CLI.

### Requirement: Subagent aggregation records partial outcomes

r[agent_harness_runtime.goose_recipe_backed_subagents.aggregation] Subagent aggregation MUST distinguish success, failure, timeout, cancellation, malformed summary, denied tool, partial parallel success, and write conflict outcomes.

#### Scenario: Failed subagent is not reported as success

r[agent_harness_runtime.goose_recipe_backed_subagents.aggregation.failure]
- GIVEN parallel recipe-backed subagents include one successful summary and one timed-out child run
- WHEN parent aggregation completes
- THEN the aggregate report names the successful and failed outcomes separately
- AND the timed-out child run is not counted as a successful finding.

### Requirement: Recipe-backed subagent closeout is evidenced

r[agent_harness_runtime.goose_recipe_backed_subagents.verification] Recipe-backed subagent adoption MUST include positive recipe-backed delegation tests, negative implicit-spawn tests, capability-denial tests, return-mode bound tests, external-adapter tests, aggregation failure tests, documentation, Cairn validation, and Cairn gates before archive.

#### Scenario: Subagent evidence is complete

- GIVEN the Goose recipe-backed subagent change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes focused subagent tests, interop descriptor tests, ledger isolation tests, docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: MCP descriptors are explicit capability claims

r[agent_harness_runtime.goose_acp_mcp_interop.mcp_descriptors] Animus MUST define MCP descriptors for transport kind, command or endpoint labels, tool/resource/prompt identities, schema identities, result contracts, timeout, environment key names, available-tool ceilings, and provenance.

#### Scenario: MCP descriptor validation rejects unsafe claims

r[agent_harness_runtime.goose_acp_mcp_interop.mcp_descriptors.validation]
- GIVEN an MCP descriptor contains an unknown transport, duplicate tool identity, missing timeout, unsafe network claim, or environment value instead of an environment key name
- WHEN descriptor validation runs
- THEN validation returns a typed descriptor error
- AND no MCP server process or network connection is started.

### Requirement: ACP provider descriptors are explicit

r[agent_harness_runtime.goose_acp_mcp_interop.acp_descriptors] Animus MUST define ACP provider or agent descriptors for external agent identity, model/profile preference, permission-mode mapping, session correlation, known limitations, and tool pass-through policy.

#### Scenario: Unsafe ACP permission mapping is rejected

r[agent_harness_runtime.goose_acp_mcp_interop.acp_descriptors.validation]
- GIVEN an ACP descriptor maps a guarded Animus mode to an unrestricted external agent mode without explicit host policy admission
- WHEN ACP descriptor validation runs
- THEN validation rejects the mapping
- AND no external agent session is started from that descriptor.

### Requirement: Interop parsing is data-only

r[agent_harness_runtime.goose_acp_mcp_interop.data_only] MCP and ACP descriptor parsing MUST be data-only and MUST NOT launch processes, discover ambient binaries, open network connections, read environment values, contact external agents, or execute tools.

#### Scenario: Descriptor parsing remains runtime-neutral

r[agent_harness_runtime.goose_acp_mcp_interop.data_only.neutral]
- GIVEN an MCP descriptor names a stdio command label and an ACP descriptor names an external agent adapter
- WHEN reusable descriptor validation runs
- THEN validation inspects only in-memory descriptor data
- AND process launch, HTTP probing, ambient binary discovery, credential lookup, and provider contact are absent.

### Requirement: Tool pass-through is capability intersection

r[agent_harness_runtime.goose_acp_mcp_interop.tool_passthrough] Tools passed through MCP or ACP bridges MUST be limited to the intersection of Animus capability ceilings, active permission mode, host admission, external agent mode, and available-tool declarations.

#### Scenario: Bridge cannot widen tool access

r[agent_harness_runtime.goose_acp_mcp_interop.tool_passthrough.denied]
- GIVEN an ACP bridge requests access to a write-capable tool excluded by active Animus policy
- WHEN tool pass-through resolution runs
- THEN the tool is omitted or denied for that bridge
- AND the external agent cannot invoke it through a fallback tool path.

### Requirement: Interop receipts are safe and bounded

r[agent_harness_runtime.goose_acp_mcp_interop.receipts] MCP and ACP discovery, session start, pass-through, denial, timeout, unavailable-binary, malformed-schema, and network-policy outcomes MUST produce safe receipts with redacted or hashed command, endpoint, argument, environment, and session identities.

#### Scenario: Interop receipt redacts sensitive identities

r[agent_harness_runtime.goose_acp_mcp_interop.receipts.redacted]
- GIVEN an MCP or ACP operation includes command arguments, endpoint URLs, environment key names, external session IDs, or tool arguments
- WHEN an interop receipt is rendered
- THEN the receipt exposes only safe labels, bounded counts, redaction markers, and BLAKE3 identities
- AND raw secret values, full arguments, and unbounded external payloads are omitted.

### Requirement: ACP/MCP interop closeout is evidenced

r[agent_harness_runtime.goose_acp_mcp_interop.verification] ACP/MCP interop adoption MUST include descriptor validation tests, data-only runtime-neutrality tests, tool pass-through denial tests, receipt redaction tests, documentation, Cairn validation, and Cairn gates before archive.

#### Scenario: Interop evidence is complete

- GIVEN the Goose ACP/MCP interop change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes focused MCP descriptor tests, ACP descriptor tests, runtime-neutrality checks, pass-through denial tests, receipt redaction tests, docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Scheduled workflow descriptors are explicit

r[agent_harness_runtime.goose_scheduled_recipes.schedule_descriptor] Animus MUST define scheduled workflow descriptors for schedule identity, recipe identity and version, parameter values, cadence, timezone or calendar policy, execution mode, concurrency policy, workspace scope, retry bounds, retention policy, and provenance.

#### Scenario: Schedule descriptor validation rejects invalid plans

r[agent_harness_runtime.goose_scheduled_recipes.schedule_descriptor.validation]
- GIVEN a scheduled workflow descriptor has an invalid cadence, missing recipe identity, unsupported execution mode, or malformed retention policy
- WHEN schedule validation runs
- THEN validation returns a typed schedule descriptor error
- AND no timer, queue task, process, or workflow run is started.

### Requirement: Schedule planning is data-only in reusable core

r[agent_harness_runtime.goose_scheduled_recipes.data_only] Reusable schedule validation and next-run planning MUST operate over explicit in-memory clock and calendar inputs and MUST NOT start timers, daemons, cron jobs, processes, filesystem mutations, or environment reads.

#### Scenario: Next-run planning has no ambient effects

r[agent_harness_runtime.goose_scheduled_recipes.data_only.neutral]
- GIVEN a schedule descriptor and explicit current time input
- WHEN next-run planning runs in reusable core
- THEN it returns a deterministic plan or typed validation error
- AND ambient cron edits, daemon startup, process launch, filesystem mutation, and environment reads are absent.

### Requirement: Every scheduled run requires admission

r[agent_harness_runtime.goose_scheduled_recipes.run_admission] Scheduled recipe execution MUST require recipe trust, capability policy, sandbox policy, queue admission, and cancellation handles before dispatch.

#### Scenario: Missing queue service denies scheduled run

r[agent_harness_runtime.goose_scheduled_recipes.run_admission.denied]
- GIVEN a schedule is due but no queue service is injected or the recipe is not trusted
- WHEN runtime evaluates run admission
- THEN the scheduled run is denied with a typed unavailable-service or untrusted-recipe outcome
- AND no model, tool, or host-effect dispatch begins.

### Requirement: Scheduled runs handle concurrency explicitly

r[agent_harness_runtime.goose_scheduled_recipes.concurrency] Scheduled workflow concurrency policy MUST prevent overlapping writes from merging silently by requiring serialization, isolated workspace execution, explicit handoff, or denial.

#### Scenario: Overlapping writes are not silent

r[agent_harness_runtime.goose_scheduled_recipes.concurrency.conflict]
- GIVEN a scheduled recipe run is still modifying a workspace when the next run becomes due
- WHEN concurrency policy evaluates the new run
- THEN the new run is serialized, isolated, or denied according to policy
- AND no shared-resource write conflict is merged silently.

### Requirement: Scheduled run records are safe

r[agent_harness_runtime.goose_scheduled_recipes.run_records] Scheduled run records MUST link schedule ID, recipe identity, parameter hashes, queue or job IDs, ledger references, retry counts, outcomes, and redacted evidence without raw prompts, secrets, full paths, or raw tool output.

#### Scenario: Secret parameters are redacted from run record

r[agent_harness_runtime.goose_scheduled_recipes.run_records.redacted]
- GIVEN a scheduled recipe uses parameter values that include sensitive strings or file-derived content
- WHEN a scheduled run record is rendered
- THEN the record contains hashes, safe labels, counts, and redaction markers
- AND raw prompts, secret parameters, full paths, and raw tool output are omitted.

### Requirement: Schedule controls are host-mediated

r[agent_harness_runtime.goose_scheduled_recipes.control] Pause, resume, edit, run-now, and cancellation controls for scheduled recipes MUST be host-mediated control requests with admission decisions and safe receipts.

#### Scenario: Unauthorized schedule mutation is denied

r[agent_harness_runtime.goose_scheduled_recipes.control.denied]
- GIVEN a session lacks authority to edit or run a schedule
- WHEN it requests pause, resume, edit, run-now, or cancellation
- THEN host-control admission denies the request
- AND the schedule state and queued runs remain unchanged.

### Requirement: Scheduled recipe closeout is evidenced

r[agent_harness_runtime.goose_scheduled_recipes.verification] Scheduled recipe adoption MUST include positive schedule validation tests, negative data-only runtime-neutrality tests, run-admission tests, concurrency conflict tests, run-record redaction tests, control-denial tests, documentation, Cairn validation, and Cairn gates before archive.

#### Scenario: Scheduled recipe evidence is complete

- GIVEN the Goose scheduled recipe change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes focused schedule tests, queue-admission tests, concurrency tests, receipt redaction tests, control-denial tests, docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Bounded BranchFS adaptation scope

r[agent_harness_runtime.branchfs_workspace.scope] Animus MUST treat BranchFS as a bounded reference for speculative workspace patterns, including copy-on-write branch creation, nested parentage, leaf-only commit and abort discipline, commit-to-parent promotion, zero-cost discard intent, virtual branch addressing, and mmap or cache invalidation awareness, and MUST NOT vendor BranchFS, require FUSE or macFUSE for all users, copy the BranchFS CLI or storage layout as the Animus contract, or expose BranchFS-specific ioctls, tombstones, daemon paths, or mount paths from `animus-core`.

#### Scenario: Scope review rejects backend coupling

r[agent_harness_runtime.branchfs_workspace.scope.review]
- GIVEN a proposed implementation imports BranchFS into reusable core crates, shells out to `branchfs` from pure logic, stores BranchFS mount paths in core DTOs, or requires FUSE for non-workspace runtime paths
- WHEN maintainers compare it with the bounded adaptation scope
- THEN the change is rejected or split until BranchFS behavior remains adapter-local reference evidence
- AND portable Animus workspace descriptors remain independent of BranchFS implementation details.

### Requirement: VCS-neutral speculative workspace descriptors

r[agent_harness_runtime.branchfs_workspace.descriptor] Animus speculative workspace descriptors MUST be typed portable runtime intent covering source identity, workspace identity, parent workspace identity, writable roots, ignored or local-file inclusion policy, commit target, abort policy, conflict policy, cleanup policy, and required adapter capability classes.

#### Scenario: Descriptor captures workspace intent without FUSE details

r[agent_harness_runtime.branchfs_workspace.descriptor.intent]
- GIVEN a runtime action requests an isolated speculative workspace
- WHEN Animus validates the workspace descriptor
- THEN the descriptor names portable workspace intent rather than BranchFS CLI flags, FUSE mount paths, daemon socket paths, storage directories, ioctls, or tombstone files
- AND missing, duplicate, malformed, or unknown required fields fail validation before any workspace backend is invoked.

### Requirement: Pure workspace planning

r[agent_harness_runtime.branchfs_workspace.planning] Workspace descriptor parsing, normalization, branch graph validation, conflict policy validation, unsupported-capability diagnostics, and receipt-safe projection MUST be pure deterministic logic over typed inputs and MUST NOT read the filesystem, inspect the environment, spawn processes, consult clocks, probe kernels, inspect VCS state, start daemons, mount filesystems, or mutate host state.

#### Scenario: Invalid branch graph fails before platform setup

r[agent_harness_runtime.branchfs_workspace.planning.invalid_graph]
- GIVEN a workspace descriptor graph contains a parent cycle, duplicate workspace identity, missing parent identity, non-leaf commit request, or incompatible commit and abort policy
- WHEN pure workspace planning runs
- THEN it returns typed diagnostics naming the invalid graph or policy condition
- AND no FUSE operation, VCS operation, process executor, filesystem operation, daemon request, or platform probe is invoked.

### Requirement: Workspace adapter capability model

r[agent_harness_runtime.branchfs_workspace.adapter] Animus workspace adapters SHOULD expose an explicit capability model for translating normalized workspace plans into enforceable backend behavior, including labels for BranchFS-like copy-on-write branches, Jujutsu or Git worktrees, temporary snapshot directories, nested workspaces, commit-to-parent support, abort support, conflict detection, cleanup guarantees, and unsupported platform states.

#### Scenario: Backend reports unsupported nested commit

r[agent_harness_runtime.branchfs_workspace.adapter.unsupported]
- GIVEN a normalized workspace plan requires nested workspaces and commit-to-parent behavior
- AND the selected adapter can create a temporary snapshot but cannot represent nested parentage or commit-to-parent semantics
- WHEN adapter capability matching runs
- THEN it reports unsupported capability diagnostics for the missing nested and commit behavior
- AND the runtime does not silently downgrade the plan to a weaker workspace mode.

### Requirement: Required workspace isolation fails closed

r[agent_harness_runtime.branchfs_workspace.fail_closed] Runtime dispatch MUST fail closed before model, tool, or process execution when required workspace isolation is malformed, denied by policy, unsupported by the selected adapter, unavailable on the current platform, or only partially enforceable, and MUST NOT treat workspace isolation as authorization proof.

#### Scenario: Required workspace does not fall back to host checkout

r[agent_harness_runtime.branchfs_workspace.fail_closed.no_host_fallback]
- GIVEN a capability request is authorized by its normal contract and grant checks
- AND its local caveat requires speculative workspace isolation
- AND the selected adapter cannot start, mount, snapshot, clean up, or enforce the requested writable-root and commit/abort semantics
- WHEN dispatch evaluates the action
- THEN dispatch is denied before host execution with a typed workspace-unavailable or partial-enforcement issue
- AND the receipt records workspace isolation failure separately from authorization success.

### Requirement: Explicit commit and abort outcomes

r[agent_harness_runtime.branchfs_workspace.commit_abort] Workspace adapters MUST report explicit outcomes for created, committed, aborted, conflict, unavailable, failed-cleanup, and ambiguous states, and nested branch backends MUST preserve leaf-scoped commit and abort behavior so commit promotes only to the immediate parent and abort discards only the selected leaf workspace while preserving parent and sibling workspaces.

#### Scenario: Leaf commit promotes without affecting siblings

r[agent_harness_runtime.branchfs_workspace.commit_abort.leaf_commit]
- GIVEN a parent workspace has two child workspaces and one child is selected for commit
- WHEN the adapter commits the selected leaf workspace
- THEN the committed changes are promoted only to the selected parent target
- AND sibling workspaces remain isolated unless conflict policy explicitly blocks or invalidates them.

#### Scenario: Abort discards selected leaf only

r[agent_harness_runtime.branchfs_workspace.commit_abort.leaf_abort]
- GIVEN a nested workspace tree has a leaf workspace with uncommitted changes
- WHEN the adapter aborts that leaf workspace
- THEN only the selected leaf workspace is discarded
- AND the parent workspace, sibling workspaces, and base source identity remain available or are reported with explicit adapter diagnostics.

### Requirement: Redacted workspace receipts

r[agent_harness_runtime.branchfs_workspace.receipts] Workspace receipts MUST carry safe metadata only, including workspace status, issue code, normalized descriptor hash, source identity hash, adapter capability labels, parent and workspace identity labels, commit or abort outcome, conflict status, cleanup status, bounded manifest counts or BLAKE3 hashes, and BLAKE3 receipt hash, and MUST NOT include raw prompts, credentials, absolute path dumps, raw file contents, raw process output, raw branch manifests, raw network payloads, or unbounded event streams.

#### Scenario: Successful commit receipt is reviewable

r[agent_harness_runtime.branchfs_workspace.receipts.success]
- GIVEN an isolated workspace is created, used, committed to its parent, and cleaned up successfully
- WHEN the workspace receipt is emitted
- THEN it records descriptor, source, manifest, commit, cleanup, and receipt hashes plus bounded adapter capability labels and outcome status
- AND sensitive path, prompt, credential, file-content, process-output, branch-manifest, and payload material is omitted or represented only by approved hashes and redaction markers.

#### Scenario: Receipt validation rejects workspace leaks

r[agent_harness_runtime.branchfs_workspace.receipts.rejects_leaks]
- GIVEN a workspace receipt fixture contains raw prompts, credential-looking values, absolute path dumps, raw file contents, raw process output, raw branch manifests, raw network payloads, or an unbounded event stream
- WHEN receipt validation runs
- THEN validation fails with a typed redaction error
- AND the receipt cannot be used as review or release evidence.

### Requirement: Focused verification for speculative workspaces

r[agent_harness_runtime.branchfs_workspace.verification] The BranchFS-inspired adaptation MUST include focused positive and negative tests for descriptor normalization, branch graph validation, adapter capability translation, fail-closed dispatch, commit and abort outcomes, cleanup handling, and receipt redaction.

#### Scenario: Positive and negative workspace fixtures gate closeout

r[agent_harness_runtime.branchfs_workspace.verification.fixtures]
- GIVEN the BranchFS-inspired speculative workspace adaptation is ready for review
- WHEN focused verification runs
- THEN positive fixtures show valid descriptor normalization, nested workspace planning, successful create, commit, abort, cleanup, and redacted receipts
- AND negative fixtures show malformed descriptors, invalid branch graphs, unsupported FUSE or backend environments, partial enforcement, commit conflicts, failed cleanup, ambiguous state, and receipt leak attempts fail closed.

r[agent_harness_runtime.branchfs_workspace.closeout] Closeout evidence MUST include successful focused workspace descriptor, adapter, commit/abort, cleanup, receipt, workspace checks, Cairn validation, and Cairn proposal/design/tasks gates before this adaptation is archived.

#### Scenario: Closeout evidence is reviewable

r[agent_harness_runtime.branchfs_workspace.closeout.evidence]
- GIVEN maintainers intend to archive the BranchFS-inspired speculative workspace adaptation
- WHEN they collect closeout evidence
- THEN the evidence names every successful focused rail, workspace check, and Cairn gate
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Typed exec policy contract

r[agent_harness_runtime.openai_codex_execpolicy_rules.policy_contract] Animus exec policy MUST represent command intents, sandbox context, rule identities, rationales, decisions, and validation examples as typed data before runtime evaluation.

#### Scenario: Policy rule carries validation examples

r[agent_harness_runtime.openai_codex_execpolicy_rules.nickel_source]
- GIVEN a policy rule with a command prefix, rationale, positive examples, and negative examples
- WHEN Animus validates the authored policy
- THEN every positive example matches the rule
- AND every negative example does not match the rule.

### Requirement: Deterministic prefix-rule evaluation

r[agent_harness_runtime.openai_codex_execpolicy_rules.rule_evaluation] Exec policy evaluation MUST be a deterministic pure function over parsed command intent and sandbox context, returning the most restrictive matching decision.

#### Scenario: Most restrictive matching decision wins

r[agent_harness_runtime.openai_codex_execpolicy_rules.rule_evaluation.examples]
- GIVEN one matching allow rule and one matching forbid rule for the same command intent
- WHEN the policy evaluator computes the decision
- THEN the result is forbid
- AND the matched-rule evidence includes both contributing rules.

### Requirement: Conservative shell-wrapper handling

r[agent_harness_runtime.openai_codex_execpolicy_rules.shell_splitting] Shell-wrapper analysis MUST split only simple command chains made from plain command words and safe chain operators, and MUST treat unsupported shell constructs as opaque commands requiring conservative policy handling.

#### Scenario: Destructive command cannot hide behind allowed prefix

r[agent_harness_runtime.openai_codex_execpolicy_rules.shell_splitting.smuggling]
- GIVEN a shell wrapper contains an allowed command followed by a forbidden destructive command in a simple chain
- WHEN Animus evaluates the wrapper
- THEN each command in the simple chain is evaluated separately
- AND the wrapper receives the forbidden decision.

### Requirement: Sandbox escalation binding

r[agent_harness_runtime.openai_codex_execpolicy_rules.sandbox_binding] Runtime sandbox escalation handling MUST use exec policy decisions to allow, prompt, or deny command execution outside the active sandbox boundary.

#### Scenario: Policy decision is recorded without leaking command bodies

r[agent_harness_runtime.openai_codex_execpolicy_rules.sandbox_binding.audit]
- GIVEN a command requests capability outside the active sandbox
- WHEN policy evaluation returns a prompt decision
- THEN runtime emits an escalation event with rule identity, decision, sandbox context, and redacted command evidence
- AND unbounded raw command output is not embedded in the event.

### Requirement: VCS-neutral review run requests

r[agent_harness_runtime.openai_codex_diff_review_mode.review_request] Animus review mode MUST accept VCS-neutral change-set selectors for working copy changes, commits, branch comparisons, and externally supplied patches.

#### Scenario: Unsupported selector fails before review starts

r[agent_harness_runtime.openai_codex_diff_review_mode.review_request.selectors]
- GIVEN a review request references a change-set selector unsupported by the active VCS adapter
- WHEN Animus validates the review request
- THEN validation fails before model or tool calls begin
- AND the failure identifies the unsupported selector capability.

### Requirement: Review mode is read-only by default

r[agent_harness_runtime.openai_codex_diff_review_mode.read_only_policy] Review mode MUST use a read-only policy profile that forbids file mutation, write-capable tools, and approval escalation by default.

#### Scenario: Review cannot mutate files

r[agent_harness_runtime.openai_codex_diff_review_mode.read_only_policy.no_mutation]
- GIVEN a review-mode run is active
- WHEN the loop fabric requests a file edit or write-capable tool
- THEN runtime denies the request
- AND the review report records a read-only policy denial without changing workspace files.

### Requirement: Structured review findings

r[agent_harness_runtime.openai_codex_diff_review_mode.finding_schema] Review findings MUST include severity, category, affected span, evidence, rationale, confidence, and remediation guidance.

#### Scenario: Finding with invalid span is rejected

r[agent_harness_runtime.openai_codex_diff_review_mode.finding_schema.validation]
- GIVEN a review finding references a span outside the reviewed change set
- WHEN Animus validates the finding
- THEN the finding is rejected as invalid
- AND the report records a finding-validation failure.

### Requirement: Bounded review evidence

r[agent_harness_runtime.openai_codex_diff_review_mode.evidence_bounds] Review evidence MUST be source-attributed and bounded to relevant file spans, patch hunks, or external review inputs.

#### Scenario: Full file is not embedded as evidence

r[agent_harness_runtime.openai_codex_diff_review_mode.evidence_bounds.no_unbounded]
- GIVEN a finding only needs one patch hunk as evidence
- WHEN Animus renders the review report
- THEN the report includes the bounded hunk reference or snippet
- AND it does not embed the full file or unrelated logs.

### Requirement: Rerun finding-state tracking

r[agent_harness_runtime.openai_codex_diff_review_mode.rerun_tracking] Review mode SHOULD compare rerun results against prior review findings to classify findings as open, fixed, dismissed, or newly introduced when prior review state is available.

#### Scenario: Fixed finding is distinguished from new finding

r[agent_harness_runtime.openai_codex_diff_review_mode.rerun_tracking.state]
- GIVEN a prior review found one issue that has since been fixed and the rerun finds a different issue
- WHEN Animus compares the review reports
- THEN the old issue is marked fixed
- AND the different issue is marked newly introduced.

### Requirement: Typed resource descriptors

r[agent_harness_runtime.pi_resources.descriptors] Animus MUST define typed descriptors for skills, prompt templates, context guidance, and host-provided resources with name, description, kind, source scope, provenance metadata, trust status, and safe content identity.

#### Scenario: Descriptor validation rejects missing descriptions

r[agent_harness_runtime.pi_resources.descriptors.validate]
- GIVEN a skill descriptor lacks a description
- WHEN resource descriptors are validated for prompt availability
- THEN validation fails with a typed descriptor error
- AND the resource is not advertised to the model.

### Requirement: Host-owned discovery boundary

r[agent_harness_runtime.pi_resources.host_discovery] Animus runtime MUST consume host-provided resource descriptors and MUST NOT scan project directories, install packages, execute package managers, or load extension code as part of core resource discovery.

#### Scenario: Runtime receives descriptors only

r[agent_harness_runtime.pi_resources.host_discovery.no_scan]
- GIVEN a host supplies an explicit list of resource descriptors
- WHEN runtime builds prompt resource context
- THEN it uses only those descriptors
- AND it performs no filesystem traversal, package installation, or dynamic code loading.

### Requirement: Project trust gating

r[agent_harness_runtime.pi_resources.project_trust] Project-local dynamic resources MUST be unavailable for prompt influence, command expansion, or tool activation until a host-supplied project trust decision permits them.

#### Scenario: Untrusted project resource is ignored

r[agent_harness_runtime.pi_resources.project_trust.denied]
- GIVEN a project-local skill descriptor is marked untrusted
- WHEN prompt construction enumerates available skills
- THEN the skill is omitted or marked unavailable according to policy
- AND its content is not loaded into model context.

### Requirement: Progressive skill disclosure

r[agent_harness_runtime.pi_resources.skill_disclosure] Skills SHOULD use progressive disclosure where only validated name and description are advertised by default, and full skill content is loaded only by explicit command, host policy, or approved matching decision.

#### Scenario: Full skill loads on explicit command

r[agent_harness_runtime.pi_resources.skill_disclosure.command]
- GIVEN a trusted skill descriptor is advertised by name and description
- WHEN an explicit skill-load command selects that skill
- THEN the host-provided content for that skill is added to the next prompt context
- AND unrelated skill bodies remain out of context.

### Requirement: Deterministic resource command and collision handling

r[agent_harness_runtime.pi_resources.commands] Resource-backed commands and names MUST resolve deterministically and report duplicate names, unsupported resource kinds, invalid names, and unsafe content handles through typed diagnostics.

#### Scenario: Duplicate skill names produce diagnostic

r[agent_harness_runtime.pi_resources.commands.duplicate]
- GIVEN two trusted skill descriptors have the same command name
- WHEN command/resource lookup is built
- THEN policy selects a deterministic winner or disables the duplicate name
- AND a diagnostic identifies the collision without exposing unsafe host paths.

### Requirement: Typed plugin toolset manifests

r[agent_harness_runtime.hermes_plugin_manifests.toolset] Animus MUST define typed plugin/toolset manifest descriptors with toolset name, version, kind, provided tool names, descriptions, parameter schema identities, result contract identities, capability labels, provenance, and trust status.

#### Scenario: Valid plugin manifest advertises tools

r[agent_harness_runtime.hermes_plugin_manifests.toolset.valid]
- GIVEN a host supplies a valid plugin manifest for a review-gate-style toolset
- WHEN manifest validation and tool definition construction run
- THEN every provided tool becomes an inactive or active typed tool definition according to trust and executor availability
- AND prompt metadata includes only validated names, descriptions, capability labels, and safe provenance labels.

### Requirement: Manifest validation rejects ambiguous tools

r[agent_harness_runtime.hermes_plugin_manifests.validation] Animus MUST reject plugin manifests with missing tool descriptions, invalid names, unsupported kinds, missing capability labels, duplicate provided tools, unsafe provenance, malformed schema identities, or malformed result contract identities.

#### Scenario: Duplicate tool names fail closed

r[agent_harness_runtime.hermes_plugin_manifests.validation.duplicate]
- GIVEN two plugin manifests provide the same public tool name without an explicit host policy override
- WHEN tool registry construction runs
- THEN registry construction returns a typed duplicate-tool diagnostic
- AND neither duplicate is silently selected by map insertion order.

### Requirement: Plugin execution requires host-injected executor

r[agent_harness_runtime.hermes_plugin_manifests.executor_boundary] Plugin manifests MUST NOT cause reusable Animus runtime code to import plugin modules, run scripts, inspect installation directories, or shell out. Tool execution MUST require an injected executor matching the validated tool definition.

#### Scenario: Manifest tool without executor is unavailable

r[agent_harness_runtime.hermes_plugin_manifests.executor_boundary.missing]
- GIVEN a trusted plugin manifest advertises a tool but the host injects no executor for that tool
- WHEN the loop requests the tool
- THEN runtime reports a missing-executor failure class
- AND no script, shell command, module import, or provider fallback is attempted.

### Requirement: Capability labels separate read-only and mutating actions

r[agent_harness_runtime.hermes_plugin_manifests.capabilities] Every manifest-provided tool MUST declare side-effect capability labels sufficient to distinguish read-only inspection, dry-run planning, explicit metrics writes, filesystem mutation, process execution, and network access when applicable.

#### Scenario: Metrics write is explicit

r[agent_harness_runtime.hermes_plugin_manifests.capabilities.metrics_write]
- GIVEN a review metrics toolset includes both summary and record actions
- WHEN tool metadata is built
- THEN summary actions are labeled read-only
- AND record actions carry an explicit metrics-write or append-only mutation capability label.

### Requirement: Plugin manifest adaptation verification

r[agent_harness_runtime.hermes_plugin_manifests.verification] The plugin manifest adaptation MUST include focused tests for valid manifests, malformed manifest rejection, duplicate tool diagnostics, capability-label prompt construction, missing executor failure, untrusted plugin omission, and host-injected executor dispatch.

#### Scenario: Closeout evidence covers manifest and executor rails

r[agent_harness_runtime.hermes_plugin_manifests.verification.closeout]
- GIVEN maintainers intend to archive the Hermes plugin manifest adaptation
- WHEN closeout validation runs
- THEN evidence names focused manifest tests, executor-boundary tests, prompt metadata tests, workspace checks, Cairn validation, and Cairn gates
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Manifest role declarations

r[agent_harness_runtime.plugin_action_manifests.manifest_roles] Animus MUST define data-only plugin descriptor roles for actions, event hooks, host-pane entrypoints, and link handlers without executing commands, opening panes, registering runtime actions dynamically, or requiring a specific manifest file format.

#### Scenario: Manifest roles validate as data

r[agent_harness_runtime.plugin_action_manifests.manifest_roles.validation]
- GIVEN a plugin descriptor declares actions, event hooks, pane entrypoints, and link handlers with stable IDs, titles, context labels, and command or artifact labels
- WHEN descriptor validation runs
- THEN the roles are accepted as data-only declarations
- AND duplicate IDs, missing required labels, unsafe command labels, and unsupported roles are rejected with typed diagnostics.

### Requirement: Compatibility gating

r[agent_harness_runtime.plugin_action_manifests.compatibility] Plugin manifests and entrypoints MUST declare or inherit minimum host capability/version and effective platform compatibility before host link or invocation planning treats them as available.

#### Scenario: Unsupported platform blocks invocation

r[agent_harness_runtime.plugin_action_manifests.compatibility.unsupported]
- GIVEN a plugin action is valid data but its effective platform set excludes the current host platform
- WHEN invocation planning evaluates that action
- THEN planning returns a typed platform-unsupported diagnostic
- AND no command descriptor is dispatched to a host executor.

### Requirement: Safe invocation context

r[agent_harness_runtime.plugin_action_manifests.invocation_context] Action, event, pane, and link-handler planning MUST use typed invocation context DTOs containing safe workspace, tab, pane, agent status, worktree/session provenance, request identity, plugin root/config/state labels, and source metadata while excluding raw prompts, transcripts, credentials, full private paths, unbounded logs, and provider payloads by default.

#### Scenario: Context injection rejects unsafe payloads

r[agent_harness_runtime.plugin_action_manifests.invocation_context.redacted]
- GIVEN an invocation context fixture contains a raw prompt, terminal transcript, credential-looking value, full private path, unbounded log, or provider payload
- WHEN context validation runs
- THEN validation rejects the fixture with a typed leak diagnostic
- AND the unsafe context is not passed to invocation planning.

### Requirement: Registry operations are host-owned

r[agent_harness_runtime.plugin_action_manifests.host_registry] Link, unlink, enable, disable, list, action-list, invocation planning, event planning, pane-entrypoint planning, registry persistence, plugin logs, install source tracking, command execution, and pane opening MUST be host-owned operations outside reusable runtime core.

#### Scenario: Planning does not execute plugin code

r[agent_harness_runtime.plugin_action_manifests.host_registry.no_execution]
- GIVEN a validated plugin action descriptor and safe invocation context
- WHEN reusable invocation planning runs
- THEN it returns a typed plan for a host adapter to evaluate
- AND it does not persist registry state, install plugins, execute commands, open panes, read plugin storage, or weaken runtime policy.

### Requirement: Diagnostics distinguish warnings from blockers

r[agent_harness_runtime.plugin_action_manifests.diagnostics] Plugin manifest handling MUST distinguish link-time warnings from invocation-blocking failures for unknown events, disabled plugins, unknown entrypoints, unsupported platforms, newer host requirements, policy denials, malformed contexts, and malformed manifests.

#### Scenario: Disabled plugin fails closed

r[agent_harness_runtime.plugin_action_manifests.diagnostics.fail_closed]
- GIVEN a linked plugin is disabled by host registry state
- WHEN a caller requests one of its actions
- THEN invocation planning returns a typed disabled-plugin failure
- AND no host executor command, pane entrypoint, event hook, or link handler is dispatched.

### Requirement: Plugin manifest verification

r[agent_harness_runtime.plugin_action_manifests.verification] The adaptation MUST include focused positive and negative tests for manifest roles, compatibility gating, safe invocation contexts, host-owned registry operations, warnings, invocation blockers, disabled plugins, unknown events, unsafe command labels, and no-execution validation.

#### Scenario: Closeout evidence covers plugin manifest rails

r[agent_harness_runtime.plugin_action_manifests.verification.closeout]
- GIVEN maintainers intend to archive the plugin manifest adaptation
- WHEN closeout validation runs
- THEN evidence names focused manifest-role, compatibility, context-redaction, host-registry, diagnostic, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a blocker and next-best validation.

### Requirement: Explicit subagent delegation

r[agent_harness_runtime.openai_codex_subagent_threads.delegation_request] Animus MUST start subagent threads only from explicit delegation requests that define role, task, allowed tools, workspace scope, model profile, and expected summary shape.

#### Scenario: Implicit subagent spawn is rejected

r[agent_harness_runtime.openai_codex_subagent_threads.delegation_request.explicit]
- GIVEN a runtime turn has no explicit user, trusted skill, or approved automation delegation request
- WHEN the loop fabric attempts to start a subagent
- THEN runtime rejects the subagent start
- AND no model or tool calls are dispatched for that subagent.

### Requirement: Parent thread is protected from raw subagent logs

r[agent_harness_runtime.openai_codex_subagent_threads.thread_isolation] Subagent intermediate output MUST remain in an isolated subagent thread ledger by default, with only bounded status and summary references imported into the parent thread.

#### Scenario: Raw subagent logs stay out of parent context

r[agent_harness_runtime.openai_codex_subagent_threads.thread_isolation.no_raw_logs]
- GIVEN a subagent produces long test logs and exploratory notes
- WHEN the subagent completes successfully
- THEN the parent thread receives a bounded summary reference
- AND the raw logs are not inserted into parent model context unless explicitly imported under output bounds.

### Requirement: Typed subagent summary contract

r[agent_harness_runtime.openai_codex_subagent_threads.summary_contract] Subagent completion MUST produce a typed summary containing findings, evidence references, confidence, unresolved questions, and recommended next actions.

#### Scenario: Malformed subagent summary fails aggregation

r[agent_harness_runtime.openai_codex_subagent_threads.summary_contract.validation]
- GIVEN a subagent returns a summary missing required evidence references for findings
- WHEN the parent aggregates subagent results
- THEN aggregation records a malformed-summary failure
- AND the parent does not treat the incomplete finding as validated evidence.

### Requirement: Subagent role profiles constrain capabilities

r[agent_harness_runtime.openai_codex_subagent_threads.role_profiles] Subagent role profiles MUST constrain model preference, reasoning effort, tool permissions, workspace scope, and output schema before execution.

#### Scenario: Unauthorized subagent tool use is denied

r[agent_harness_runtime.openai_codex_subagent_threads.role_profiles.permissions]
- GIVEN a review-only subagent profile excludes write-capable tools
- WHEN the subagent requests a write-capable tool
- THEN runtime denies the request
- AND the denial is recorded in the subagent ledger and parent summary status.

### Requirement: Parallel write conflict policy

r[agent_harness_runtime.openai_codex_subagent_threads.write_conflict_policy] Write-capable subagents MUST use isolated workspaces, mutation serialization, or parent-approved handoff before changes to shared resources are merged.

#### Scenario: Parallel writes do not merge silently

r[agent_harness_runtime.openai_codex_subagent_threads.write_conflict_policy.conflict]
- GIVEN two subagents modify the same shared resource in parallel
- WHEN their results are aggregated
- THEN runtime reports a write conflict unless a mutation coordinator or explicit handoff resolved the conflict
- AND neither change is silently preferred over the other.

### Requirement: Subagent runtime event reporting

r[agent_harness_runtime.openai_codex_subagent_threads.event_reporting] Runtime reports MUST include subagent start, stop, continuation, failure, and aggregation events with parent/subagent correlation identifiers.

#### Scenario: Failed subagent is visible to parent

r[agent_harness_runtime.openai_codex_subagent_threads.event_reporting.failure]
- GIVEN a subagent fails before producing a valid summary
- WHEN the parent turn aggregates subagent outcomes
- THEN the parent report includes the subagent identifier, failure class, and bounded failure evidence
- AND the parent does not report the subagent task as successful.

### Requirement: Non-interactive run request contract

r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.run_request] Animus MUST represent non-interactive execution as a typed run request containing prompt input, optional external context handles, sandbox profile, output preferences, and optional resume identity.

#### Scenario: Invalid run request fails before runtime start

r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.run_request.validation]
- GIVEN a non-interactive run request with an invalid sandbox profile
- WHEN Animus validates the request
- THEN validation fails before any model, tool, or hook adapter is invoked
- AND the failure identifies the invalid run-request field.

### Requirement: Versioned machine-readable event stream

r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.event_stream] Runtime automation MUST expose a versioned append-only event stream for thread, turn, item, model, tool, policy, hook, usage, completion, and failure events.

#### Scenario: Events preserve runtime ordering

r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.event_stream.ordering]
- GIVEN a runtime turn dispatches a model request followed by a tool request and final response
- WHEN the automation event stream is serialized
- THEN event order reflects the runtime order
- AND each event body remains within the configured event-size budget.

### Requirement: Final output derives from terminal response item

r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.final_output] Non-interactive shells MUST derive final output from the terminal runtime response item instead of scraping logs or progress messages.

#### Scenario: Human-readable progress is not final output

r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.final_output.selection]
- GIVEN a run emits progress messages and a terminal final-response item
- WHEN the shell writes the final output sink
- THEN the sink contains the final-response item content
- AND progress messages are not included unless they are part of that item.

### Requirement: Structured final-output validation

r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.structured_output] Structured final outputs MUST be validated against a schema-identified contract before the run is reported successful.

#### Scenario: Malformed structured output fails the run

r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.structured_output.validation]
- GIVEN a run requests structured final output with required fields
- WHEN the model produces a final response missing a required field
- THEN validation fails before success is reported
- AND the event stream records a structured-output validation failure.

### Requirement: Thread controller API

r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.thread_controller] Animus MUST provide controller-facing operations to start, resume, run, inspect, and cancel runtime threads through typed thread identities.

#### Scenario: Resume checks thread identity before continuing

r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.thread_controller.resume]
- GIVEN a resume request references a thread identity with a mismatched ledger hash
- WHEN the controller validates the resume request
- THEN the thread is not resumed
- AND no new model or tool calls are dispatched for that request.

### Requirement: Streaming model adapter contract

r[agent_harness_runtime.model_streaming.adapter_contract] Animus MUST define a streaming model-adapter contract that reports ordered frames for visible content token or delta updates, provider-exposed thought/reasoning updates, tool-call deltas, usage, finish, cancellation, and failure with model-call correlation IDs and per-call ordering.

#### Scenario: Stream frames preserve adapter order

r[agent_harness_runtime.model_streaming.adapter_contract.order]
- GIVEN an injected model adapter emits visible content frames, thought/reasoning frames, usage metadata, and a finish frame for one model call
- WHEN runtime consumes the model stream
- THEN emitted runtime facts preserve the adapter frame order and model-call correlation ID
- AND final assistant-visible content is assembled from visible content frames unless an explicit policy selects another visible channel.

### Requirement: Streaming capabilities are explicit

r[agent_harness_runtime.model_streaming.capabilities] Effective model settings MUST identify whether the selected model supports visible content streaming, thought/reasoning streaming, tool-call delta streaming, and usage streaming before runtime dispatches a streaming model request.

#### Scenario: Unsupported thought streaming fails explicitly

r[agent_harness_runtime.model_streaming.capabilities.unsupported_thought]
- GIVEN the selected model does not support thought/reasoning streaming
- WHEN a host requests thought/reasoning streaming for that model
- THEN settings resolution returns a typed unsupported-capability failure or an explicit policy-approved content-only downgrade
- AND runtime does not fabricate thought/reasoning frames.

### Requirement: Provider streaming uses parser-entrypoint fixtures

r[agent_harness_runtime.model_streaming.parser_entrypoint] Any provider or shell adapter that claims streaming support MUST include fixtures that feed raw provider stream bytes or chunks through the same parser entrypoint used by production code.

#### Scenario: Raw stream fixture becomes canonical frames

r[agent_harness_runtime.model_streaming.parser_entrypoint.fixture]
- GIVEN a raw provider stream fixture contains fragmented content deltas, thought/reasoning deltas, tool-call deltas, usage records, and a finish record
- WHEN the production parser entrypoint consumes the fixture chunks
- THEN it emits the expected canonical stream frames in order
- AND no helper-only parser path is sufficient to satisfy the streaming claim.

#### Scenario: Malformed stream fails closed

r[agent_harness_runtime.model_streaming.parser_entrypoint.malformed]
- GIVEN raw provider stream input contains invalid records, a missing finish record, out-of-order terminal events, or truncated chunks
- WHEN the production parser entrypoint consumes the input
- THEN parsing returns a typed stream failure
- AND runtime does not report a successful final response from the malformed stream.

### Requirement: Runtime projects streaming events

r[agent_harness_runtime.model_streaming.runtime_events] Runtime MUST project model stream frames into typed runtime events and protocol/UI-neutral DTOs before final response completion, preserving correlation IDs, channel kind, ordering, bounded body policy, and terminal status.

#### Scenario: Subscribers observe incremental output before completion

r[agent_harness_runtime.model_streaming.runtime_events.end_to_end]
- GIVEN a model call streams visible content frames and authorized thought/reasoning frames before producing a final response
- WHEN a headless or SDK-style subscriber observes runtime events
- THEN the subscriber receives ordered stream-start, stream-frame, and stream-end or stream-failure events before final response completion
- AND each event carries stable correlation metadata without requiring terminal UI state.

### Requirement: Thought and reasoning visibility is policy-gated

r[agent_harness_runtime.model_streaming.thought_visibility] Thought/reasoning stream frames MUST carry visibility and sensitivity metadata, and default runtime sinks MUST NOT treat them as assistant-visible final answer text, receipt bodies, dataset rows, review report bodies, or logs unless policy explicitly authorizes a safe projection.

#### Scenario: Default policy prevents thought leakage

r[agent_harness_runtime.model_streaming.thought_visibility.default_redaction]
- GIVEN a model stream contains provider-exposed thought/reasoning frames and visible answer frames
- WHEN runtime builds final output, receipts, dataset export rows, review reports, or logs under the default policy
- THEN visible answer frames may appear in assistant-visible output according to normal content policy
- AND thought/reasoning frames are omitted, redacted, or safely summarized without exposing raw private reasoning.

### Requirement: Streaming closeout is evidenced

r[agent_harness_runtime.model_streaming.verification] Model streaming work MUST include positive and negative tests for stream frame ordering, content finalization, thought/reasoning policy, parser-entrypoint fixtures, unsupported capabilities, malformed streams, cancellation or failure, and protocol/UI-neutral event projection before the change is archived.

#### Scenario: Closeout evidence covers streaming behavior

r[agent_harness_runtime.model_streaming.verification.closeout]
- GIVEN token and thought streaming work is ready to archive
- WHEN closeout validation runs
- THEN evidence includes focused streaming tests, parser fixture tests, leak-rejection tests, capability-negotiation tests, runtime event projection tests, workspace checks, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Typed retrieval budget descriptors

r[agent_harness_runtime.hermes_retrieval_budgets.descriptor] Animus MUST define typed retrieval budget descriptors with retrieval purpose, first action guidance, repeat conditions, stop conditions, missing-evidence behavior, citation or source-naming expectations, progress guidance, provenance, and optional host-configured budget limits.

#### Scenario: Valid retrieval budget is advertised with resource

r[agent_harness_runtime.hermes_retrieval_budgets.descriptor.valid]
- GIVEN a retrieval-capable skill or tool has a validated retrieval budget descriptor
- WHEN prompt resource context is built
- THEN the model-visible guidance includes the bounded first/repeat/stop behavior
- AND the descriptor remains attributable to the resource that introduced it.

### Requirement: Retrieval-capable resources declare budget policy

r[agent_harness_runtime.hermes_retrieval_budgets.resource_policy] Resources that perform web search, browser automation, documentation lookup, repository browsing, multi-lens analysis, or property discovery SHOULD declare whether a retrieval budget is required before they are advertised.

#### Scenario: Required budget is missing

r[agent_harness_runtime.hermes_retrieval_budgets.resource_policy.missing]
- GIVEN host policy requires retrieval budgets for retrieval-capable resources
- WHEN a web-search tool or browsing skill lacks a valid budget descriptor
- THEN resource validation fails or marks the resource unavailable
- AND prompt construction reports a deterministic missing-budget diagnostic.

### Requirement: Repeat retrieval is justified by missing evidence

r[agent_harness_runtime.hermes_retrieval_budgets.repeat_rules] Retrieval budget descriptors MUST distinguish initial retrieval, permitted repeat retrieval, and stopping behavior, and repeat retrieval MUST be justified by missing required facts, missing artifacts, contradictions, or insufficient evidence for the requested output.

#### Scenario: Evidence is sufficient after first retrieval

r[agent_harness_runtime.hermes_retrieval_budgets.repeat_rules.stop_after_sufficient]
- GIVEN the first retrieval result supplies the required fact, owner, date, parameter, source, or code artifact
- WHEN retrieval planning evaluates the next step
- THEN the budget decision recommends answering from existing evidence
- AND it does not authorize additional retrieval for nonessential polish.

#### Scenario: Missing required fact authorizes targeted repeat

r[agent_harness_runtime.hermes_retrieval_budgets.repeat_rules.retrieve_missing]
- GIVEN the first retrieval result lacks a required fact needed for the user's requested output
- WHEN retrieval planning evaluates the next step
- THEN the budget decision authorizes one targeted retrieval or asks for the smallest missing input if retrieval cannot supply it
- AND the missing evidence is named in the report.

### Requirement: Retrieval evidence metadata is bounded

r[agent_harness_runtime.hermes_retrieval_budgets.evidence_metadata] Runtime reports and receipts MAY record retrieval budget decisions, source labels, safe references, counts, truncation status, and BLAKE3 content hashes, but MUST NOT embed raw fetched bodies, credentials, full host paths, or unbounded retrieved content by default.

#### Scenario: Receipt records retrieval decision safely

r[agent_harness_runtime.hermes_retrieval_budgets.evidence_metadata.safe]
- GIVEN a retrieval-capable tool returns a large document
- WHEN the runtime records retrieval evidence
- THEN the report contains source labels, bounded counts, truncation status, and optional BLAKE3 hashes
- AND it does not store the full raw document body unless an explicit host storage policy permits it outside reusable core receipts.

### Requirement: Retrieval budget adaptation verification

r[agent_harness_runtime.hermes_retrieval_budgets.verification] The retrieval budget adaptation MUST include focused tests for valid descriptors, missing budget diagnostics, repeat/stop decisions, missing-evidence naming, bounded evidence metadata, prompt attribution, and prompt-hygiene checker behavior.

#### Scenario: Closeout evidence covers retrieval fixtures

r[agent_harness_runtime.hermes_retrieval_budgets.verification.closeout]
- GIVEN maintainers intend to archive the Hermes retrieval budget adaptation
- WHEN closeout validation runs
- THEN evidence names focused budget descriptor tests, repeat/stop decision tests, evidence metadata tests, prompt-hygiene checks, workspace checks, Cairn validation, and Cairn gates
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Typed context-reference inputs

r[agent_harness_runtime.clankers_context_references.typed_inputs] Animus MUST represent prompt context references as typed DTOs with raw label, normalized target, reference kind, optional range, requested render mode, and source trust metadata before policy or resolver decisions are trusted.

#### Scenario: Local file reference normalizes

r[agent_harness_runtime.clankers_context_references.typed_inputs.file]
- GIVEN a prompt includes a local file reference with an optional line range
- WHEN reference parsing and normalization run
- THEN Animus produces a typed file-reference DTO with the requested range and a safe target label
- AND no filesystem read is performed by the parser.

### Requirement: Expansion uses host-injected resolvers

r[agent_harness_runtime.clankers_context_references.resolvers] Context-reference expansion MUST call host-injected resolver services for filesystem, directory, diff, image, and URL references and MUST fail closed when a required resolver or capability is absent, denied, or unavailable.

#### Scenario: URL reference denied by policy

r[agent_harness_runtime.clankers_context_references.resolvers.url_denied]
- GIVEN a prompt includes an HTTPS reference
- AND URL retrieval is disabled by the active context-reference policy
- WHEN expansion is requested
- THEN Animus returns a typed denied-reference outcome
- AND no network resolver is invoked.

### Requirement: Context expansion bounds

r[agent_harness_runtime.clankers_context_references.bounds] Expanded context fragments MUST be included, truncated, or rejected according to deterministic budgets before they become model-visible prompt content.

#### Scenario: Oversized file reference truncates safely

r[agent_harness_runtime.clankers_context_references.bounds.oversized]
- GIVEN a resolver returns text larger than the configured context-reference visible budget
- WHEN budget evaluation runs
- THEN the model-visible fragment is truncated or rejected according to policy
- AND metadata records byte counts, line counts when available, truncation status, and a BLAKE3 hash without embedding the full raw body.

### Requirement: Safe provenance metadata

r[agent_harness_runtime.clankers_context_references.provenance] Context-reference receipts and ledger metadata MUST record safe provenance only, including kind, status, safe target label or hash, range, counts, truncation state, resolver class, policy decision, and optional BLAKE3 content hash.

#### Scenario: Metadata rejects leaks

r[agent_harness_runtime.clankers_context_references.provenance.rejects_leaks]
- GIVEN a context-reference metadata fixture contains raw expanded content, URL credentials, a full absolute path dump, a credential-looking value, or unbounded output
- WHEN metadata validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Context-reference verification

r[agent_harness_runtime.clankers_context_references.verification] The adaptation MUST include focused positive and negative tests for parsing, resolver injection, policy denial, malformed targets, path escapes, oversized references, unsupported kinds, missing resolver services, and safe metadata validation.

#### Scenario: Closeout evidence covers reference fixtures

r[agent_harness_runtime.clankers_context_references.verification.closeout]
- GIVEN maintainers intend to archive the context-reference adaptation
- WHEN closeout validation runs
- THEN evidence names focused parser, resolver, policy, budget, metadata, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Local batch run planning

r[agent_harness_runtime.clankers_batch_runner.planning] Animus MUST validate local batch inputs into a deterministic batch plan with run identity, ordered jobs, output format, concurrency policy, execution mode, and safe output labels before execution begins.

#### Scenario: Remote input fails closed

r[agent_harness_runtime.clankers_batch_runner.planning.remote_rejected]
- GIVEN a batch request names an HTTP, HTTPS, S3, or otherwise remote input or output location
- WHEN batch planning validates the request
- THEN planning returns a typed unsupported-location failure
- AND no execution service or renderer is invoked.

### Requirement: Injected batch execution

r[agent_harness_runtime.clankers_batch_runner.execution] Batch jobs MUST execute through an injected headless or session execution service and MUST NOT require reusable runtime code to construct providers, spawn daemons, write session stores, or call shell commands directly.

#### Scenario: Execution service unavailable

r[agent_harness_runtime.clankers_batch_runner.execution.unavailable]
- GIVEN a valid batch plan requires job execution
- AND no compatible execution service is injected
- WHEN the runner starts
- THEN Animus records an unavailable-execution failure
- AND no provider, daemon, or shell fallback is attempted.

### Requirement: Deterministic resume manifests

r[agent_harness_runtime.clankers_batch_runner.resume] Batch resume MUST use sidecar manifest evidence to skip only completed jobs whose run ID, job ID, output format, redaction policy, and output evidence match the current plan.

#### Scenario: Stale manifest does not skip job

r[agent_harness_runtime.clankers_batch_runner.resume.stale]
- GIVEN a resume manifest contains a completed job with a mismatched plan hash or output format
- WHEN resume planning evaluates the manifest
- THEN the job is not skipped
- AND the resume report records the stale evidence reason.

### Requirement: Stable trajectory output

r[agent_harness_runtime.clankers_batch_runner.output] Batch renderers MUST produce stable job ordering and safe trajectory metadata even when jobs complete concurrently or are retried.

#### Scenario: Concurrent completions remain ordered

r[agent_harness_runtime.clankers_batch_runner.output.stable_order]
- GIVEN multiple batch jobs execute with different completion times
- WHEN results are rendered
- THEN output records appear in deterministic plan order
- AND each record carries safe job status and redaction metadata.

### Requirement: Batch metadata redaction

r[agent_harness_runtime.clankers_batch_runner.metadata] Batch run and job receipts MUST record safe metadata such as counts, statuses, prompt sizes, model/session handles, objective summaries, output labels, redaction status, and BLAKE3 hashes, and MUST NOT include raw prompts, provider payloads, credentials, or remote destinations.

#### Scenario: Metadata rejects raw prompt

r[agent_harness_runtime.clankers_batch_runner.metadata.rejects_prompt]
- GIVEN a batch receipt fixture includes a raw prompt body or provider payload
- WHEN metadata validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Batch runner verification

r[agent_harness_runtime.clankers_batch_runner.verification] The adaptation MUST include focused positive and negative tests for local planning, remote-path rejection, injected execution, unavailable services, resume manifests, stable ordering, renderer metadata, objective receipts, and redaction.

#### Scenario: Closeout evidence covers batch fixtures

r[agent_harness_runtime.clankers_batch_runner.verification.closeout]
- GIVEN maintainers intend to archive the batch runner adaptation
- WHEN closeout validation runs
- THEN evidence names focused batch planning, execution, resume, output, metadata, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Headless command-response protocol

r[agent_harness_runtime.pi_headless_protocol.command_response] Animus MUST define a headless command-response protocol for driving runtime turns, queueing steering and follow-up messages, aborting work, inspecting state, selecting models, operating on session ledger state, requesting compaction, and shutting down without requiring a terminal UI.

#### Scenario: Prompt command acknowledges acceptance

r[agent_harness_runtime.pi_headless_protocol.command_response.prompt]
- GIVEN a headless client sends a valid prompt command with a request ID while runtime is idle
- WHEN the command is accepted
- THEN the protocol emits a response carrying the same request ID and an accepted status
- AND runtime lifecycle events stream separately from that acknowledgement.

### Requirement: Strict record framing

r[agent_harness_runtime.pi_headless_protocol.framing] Text protocol transports MUST use strict LF-delimited records, MUST tolerate a trailing CR before LF, and MUST NOT split records on Unicode line separators inside encoded payloads.

#### Scenario: Unicode separator remains payload data

r[agent_harness_runtime.pi_headless_protocol.framing.unicode_separator]
- GIVEN a valid encoded command contains a Unicode line separator inside a string payload
- WHEN the protocol reader consumes bytes
- THEN it treats the separator as payload data rather than a record boundary
- AND the command is parsed only when an LF record delimiter is received.

### Requirement: Explicit active-turn queueing

r[agent_harness_runtime.pi_headless_protocol.queueing] Headless prompt-like commands sent while runtime is active MUST declare steering or follow-up semantics, and runtime MUST reject ambiguous active-turn prompts before they enter the ledger or model context.

#### Scenario: Ambiguous mid-turn prompt is rejected

r[agent_harness_runtime.pi_headless_protocol.queueing.reject_ambiguous]
- GIVEN the runtime is processing a turn
- WHEN a headless client sends a prompt command without steering or follow-up behavior
- THEN the command response reports a preflight rejection
- AND no message is appended to the active session ledger.

### Requirement: Runtime events stream through protocol DTOs

r[agent_harness_runtime.pi_headless_protocol.event_stream] Headless mode MUST stream typed runtime events through protocol DTOs with correlation IDs where applicable and without requiring TUI-specific component state.

#### Scenario: Tool execution events correlate

r[agent_harness_runtime.pi_headless_protocol.event_stream.tool]
- GIVEN a turn dispatches a tool call
- WHEN the headless event stream is observed
- THEN tool-start, tool-update if any, and tool-end events carry a shared tool-call correlation ID
- AND a client can render progress without inspecting internal runtime state.

### Requirement: Safe bounded protocol errors

r[agent_harness_runtime.pi_headless_protocol.safe_errors] Protocol parse errors, invalid commands, invalid streaming behavior, unavailable services, cancelled operations, and runtime failures MUST produce bounded safe error responses that do not expose credentials, provider payloads, raw ledger dumps, full host paths, or unbounded tool output.

#### Scenario: Malformed record returns bounded error

r[agent_harness_runtime.pi_headless_protocol.safe_errors.malformed]
- GIVEN a client sends a malformed encoded record
- WHEN the protocol parser rejects it
- THEN the response reports a bounded parse error class and optional request ID if recoverable
- AND the response does not echo the full malformed payload.

### Requirement: VCS-neutral checkpoint DTOs

r[agent_harness_runtime.clankers_checkpoint_rollback.dto] Animus MUST represent checkpoints and rollback targets with VCS-neutral DTOs that include backend kind, opaque checkpoint ID, safe workspace label or hash, content identities, changed-file counts, session/tool attribution when available, and status metadata.

#### Scenario: Checkpoint identity is backend-neutral

r[agent_harness_runtime.clankers_checkpoint_rollback.dto.backend_neutral]
- GIVEN a host reports a checkpoint created by a jj, git, in-memory, or workspace-isolation backend
- WHEN Animus records checkpoint metadata
- THEN the DTO preserves backend kind and opaque checkpoint ID
- AND reusable runtime logic does not require backend-specific command syntax.

### Requirement: Injected checkpoint service

r[agent_harness_runtime.clankers_checkpoint_rollback.service] Checkpoint create, list, inspect, rollback, and cleanup operations MUST call an injected checkpoint service and MUST fail closed when the service is missing, unavailable, denied, or reports an unsupported operation.

#### Scenario: Missing checkpoint service blocks strict protected mutation

r[agent_harness_runtime.clankers_checkpoint_rollback.service.missing_strict]
- GIVEN checkpoint policy requires strict pre-mutation checkpoints
- AND no checkpoint service is injected
- WHEN a protected mutating tool action is requested
- THEN Animus returns a missing-checkpoint-service failure
- AND the mutation is not dispatched.

### Requirement: Explicit rollback confirmation

r[agent_harness_runtime.clankers_checkpoint_rollback.confirmation] Rollback execution MUST require an explicit host/operator confirmation decision before workspace mutation occurs.

#### Scenario: Unconfirmed rollback is rejected

r[agent_harness_runtime.clankers_checkpoint_rollback.confirmation.required]
- GIVEN a user or agent requests rollback to a checkpoint
- AND no approval decision authorizes that rollback target
- WHEN rollback validation runs
- THEN Animus rejects the rollback before invoking the checkpoint service
- AND records an unconfirmed rollback status.

### Requirement: Rollback freshness verification

r[agent_harness_runtime.clankers_checkpoint_rollback.freshness] Rollback execution MUST verify that the current workspace identity matches the expected rollback precondition before restoring checkpoint state.

#### Scenario: Stale workspace blocks rollback

r[agent_harness_runtime.clankers_checkpoint_rollback.freshness.stale]
- GIVEN the workspace changed after rollback preflight selected a checkpoint
- WHEN rollback execution validates current state
- THEN Animus rejects the rollback with a stale-workspace failure
- AND no restore operation is invoked.

### Requirement: Safe checkpoint receipts

r[agent_harness_runtime.clankers_checkpoint_rollback.receipts] Checkpoint and rollback receipts MUST include safe metadata such as operation, status, backend kind, safe workspace label or hash, checkpoint ID, changed-file counts, confirmation status, before/after BLAKE3 hashes, and error class, and MUST NOT include raw diffs, file contents, credentials, environment variables, or full path dumps.

#### Scenario: Receipt leak rejected

r[agent_harness_runtime.clankers_checkpoint_rollback.receipts.rejects_leaks]
- GIVEN a checkpoint receipt fixture contains a raw diff, file body, credential-looking value, environment dump, or full absolute path list
- WHEN receipt validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Checkpoint rollback verification

r[agent_harness_runtime.clankers_checkpoint_rollback.verification] The adaptation MUST include focused positive and negative tests for checkpoint DTOs, strict and best-effort policy behavior, missing services, confirmation, freshness checks, unsupported backends, rollback receipts, and leak rejection.

#### Scenario: Closeout evidence covers checkpoint fixtures

r[agent_harness_runtime.clankers_checkpoint_rollback.verification.closeout]
- GIVEN maintainers intend to archive the checkpoint rollback adaptation
- WHEN closeout validation runs
- THEN evidence names focused DTO, service, confirmation, freshness, receipt, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Typed continuation state

r[agent_harness_runtime.clankers_continuation_policy.state] Animus MUST represent continuation state with typed queued prompt, active loop work, validation follow-up, busy prompt, cancellation, pending dispatch, and completion acknowledgement fields.

#### Scenario: Queued prompt state is explicit

r[agent_harness_runtime.clankers_continuation_policy.state.queued_prompt]
- GIVEN user input arrives while a prompt is busy
- WHEN continuation state is updated
- THEN the queued prompt is represented as typed pending work with an opaque ID
- AND no shell-local hidden flag is required to decide the next action.

### Requirement: Deterministic continuation priority

r[agent_harness_runtime.clankers_continuation_policy.priority] Continuation decisions MUST apply deterministic policy priority among queued user prompt replay, active loop continuation, validation or auto-test follow-up, and completion.

#### Scenario: Queued user prompt wins over automation

r[agent_harness_runtime.clankers_continuation_policy.priority.queued_wins]
- GIVEN a queued user prompt and an eligible validation follow-up both exist after a model turn
- WHEN continuation policy evaluates next action
- THEN it selects queued prompt replay first
- AND the validation follow-up remains pending.

### Requirement: Exact dispatch and completion acknowledgements

r[agent_harness_runtime.clankers_continuation_policy.acknowledgements] Shell adapters MUST acknowledge dispatch and completion using the exact pending work ID, and stale, duplicate, missing, or mismatched IDs MUST fail without clearing unrelated continuation state.

#### Scenario: Stale completion is rejected

r[agent_harness_runtime.clankers_continuation_policy.acknowledgements.stale]
- GIVEN a completion acknowledgement references a stale pending work ID
- WHEN continuation state validates the acknowledgement
- THEN it rejects the acknowledgement with a stale-work failure
- AND current pending work remains intact.

### Requirement: Continuation effects are shell-owned

r[agent_harness_runtime.clankers_continuation_policy.shell_owned] Provider calls, timers, queue waits, session persistence, UI events, and validation command execution MUST remain shell-owned behavior outside the pure continuation decision core.

#### Scenario: Decision core does not execute provider call

r[agent_harness_runtime.clankers_continuation_policy.shell_owned.no_provider]
- GIVEN continuation policy selects a loop continuation
- WHEN the pure decision core returns the action
- THEN it returns a typed decision only
- AND it does not call providers, spawn commands, read storage, or emit UI events directly.

### Requirement: Safe continuation receipts

r[agent_harness_runtime.clankers_continuation_policy.receipts] Continuation receipts MUST record safe metadata such as prior state class, selected action, pending work ID hash, reason, rejection class, policy hash, and BLAKE3 receipt hash, and MUST NOT include raw prompts, model outputs, tool outputs, provider payloads, full transcripts, or unbounded logs.

#### Scenario: Continuation receipt leak rejected

r[agent_harness_runtime.clankers_continuation_policy.receipts.rejects_leaks]
- GIVEN a continuation receipt fixture contains raw prompt text, model output, tool output, provider JSON, full transcript, or unbounded log text
- WHEN receipt validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Continuation policy verification

r[agent_harness_runtime.clankers_continuation_policy.verification] The adaptation MUST include focused positive and negative tests for typed state, priority order, queued prompts, loop continuation, validation follow-ups, dispatch/completion acknowledgements, stale and duplicate IDs, shell-owned effects, cancellation, and receipt redaction.

#### Scenario: Closeout evidence covers continuation fixtures

r[agent_harness_runtime.clankers_continuation_policy.verification.closeout]
- GIVEN maintainers intend to archive the controller continuation policy adaptation
- WHEN closeout validation runs
- THEN evidence names focused state, priority, acknowledgement, shell-boundary, receipt, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Layered guidance source attribution

r[agent_harness_runtime.openai_codex_guidance_skill_budget.guidance_sources] Animus guidance assembly MUST represent each guidance source with scope, precedence, override semantics, origin, and content hash.

#### Scenario: Nested guidance overrides broader guidance

r[agent_harness_runtime.openai_codex_guidance_skill_budget.guidance_sources.precedence]
- GIVEN a root guidance source and a nested guidance source both define behavior for the current working scope
- WHEN Animus assembles guidance for that scope
- THEN the nested guidance is applied after the root guidance
- AND the assembly ledger records both sources and their precedence order.

### Requirement: Prompt assembly budget ledger

r[agent_harness_runtime.openai_codex_guidance_skill_budget.budget_ledger] Prompt assembly MUST record included, skipped, truncated, and redacted guidance sources with budget consumption and reasons.

#### Scenario: Over-budget guidance is explicit

r[agent_harness_runtime.openai_codex_guidance_skill_budget.budget_ledger.truncation]
- GIVEN available guidance exceeds the configured prompt budget
- WHEN Animus assembles model context
- THEN lower-priority or over-budget guidance is skipped or truncated according to policy
- AND the assembly ledger records source identity, budget impact, and truncation reason.

### Requirement: Progressive skill metadata discovery

r[agent_harness_runtime.openai_codex_guidance_skill_budget.skill_metadata] Skill discovery MUST expose bounded metadata for available skills without loading full instructions, references, assets, or scripts into initial model context.

#### Scenario: Initial skill context is metadata-only

r[agent_harness_runtime.openai_codex_guidance_skill_budget.skill_metadata.progressive]
- GIVEN a repository has multiple skill packages with large references
- WHEN Animus builds initial model context
- THEN only bounded skill metadata is included
- AND full skill instructions and references are not loaded until a skill is selected.

### Requirement: Deferred skill loading respects invocation policy

r[agent_harness_runtime.openai_codex_guidance_skill_budget.deferred_loading] Full skill loading MUST require explicit invocation or a policy-approved implicit invocation decision.

#### Scenario: Implicit invocation is denied for disabled skill

r[agent_harness_runtime.openai_codex_guidance_skill_budget.deferred_loading.denied]
- GIVEN a skill is disabled or disallows implicit invocation
- WHEN a task appears to match that skill's description without explicit invocation
- THEN Animus does not load the full skill
- AND the selection ledger records the denied invocation reason.

### Requirement: Skill packages are trust-gated resources

r[agent_harness_runtime.openai_codex_guidance_skill_budget.skill_package] Skill package descriptors MUST treat instructions, optional scripts, references, assets, dependencies, and trust metadata as resources subject to runtime policy before use.

#### Scenario: Untrusted skill script cannot execute

r[agent_harness_runtime.openai_codex_guidance_skill_budget.skill_package.trust]
- GIVEN a selected skill references an untrusted script
- WHEN the runtime reaches a step that would execute the script
- THEN execution is denied unless shell policy and trust records authorize it
- AND the denial is recorded without bypassing hook or tool policy.

### Requirement: Typed prompt sources

r[agent_harness_runtime.clankers_prompt_assembly.sources] Animus prompt assembly MUST consume typed prompt-source DTOs with source kind, scope, trust/admission status, priority, budget class, redaction class, and safe identity before any source can influence model-visible prompt content.

#### Scenario: Host context source assembles without discovery

r[agent_harness_runtime.clankers_prompt_assembly.sources.host_context]
- GIVEN a host supplies an admitted host-context prompt source
- WHEN prompt assembly runs in host-context-only mode
- THEN the source can contribute an attributed bounded section
- AND no filesystem, memory, skill, URL, or project discovery is attempted.

### Requirement: Deterministic precedence policy

r[agent_harness_runtime.clankers_prompt_assembly.precedence] Prompt assembly MUST apply deterministic policy-defined section ordering, source precedence, conflict behavior, and required/optional source rules.

#### Scenario: Conflicting sources resolve deterministically

r[agent_harness_runtime.clankers_prompt_assembly.precedence.conflict]
- GIVEN two admitted sources conflict for the same prompt section
- WHEN assembly applies precedence policy
- THEN the selected source order and conflict diagnostic are deterministic
- AND the ledger identifies the skipped or overridden source without leaking raw body text.

### Requirement: Assembly budget ledger

r[agent_harness_runtime.clankers_prompt_assembly.ledger] Prompt assembly MUST produce a ledger recording included, skipped, truncated, redacted, denied, conflicting, and unsupported sources with safe metadata and budget consumption.

#### Scenario: Over-budget source is truncated with evidence

r[agent_harness_runtime.clankers_prompt_assembly.ledger.truncated]
- GIVEN an admitted prompt source exceeds its configured section budget
- WHEN assembly evaluates the source
- THEN model-visible content is truncated or skipped according to policy
- AND the ledger records budget use, truncation status, and BLAKE3 body identity without embedding the full raw source.

### Requirement: Ambient discovery remains host-owned

r[agent_harness_runtime.clankers_prompt_assembly.discovery] Reusable prompt assembly MUST NOT perform ambient filesystem, project-file, memory-store, skill-store, URL, or tool-discovery side effects; hosts MUST inject admitted sources or swappable discovery adapters, including memory-source adapters for any memory-backed prompt source.

#### Scenario: Missing discovery adapter is explicit

r[agent_harness_runtime.clankers_prompt_assembly.discovery.missing]
- GIVEN assembly policy references a source kind that requires host discovery, such as a memory-backed source
- AND no swappable discovery adapter or admitted source exists for that kind
- WHEN assembly runs
- THEN the ledger records an unsupported or unavailable source outcome
- AND no fallback ambient discovery, memory-store lookup, or provider extraction occurs.

### Requirement: Safe prompt assembly receipts

r[agent_harness_runtime.clankers_prompt_assembly.receipts] Prompt assembly receipts MUST record source kind, safe label or hash, trust/admission status, section, budget use, truncation status, conflict reason, policy hash, and BLAKE3 source hashes, and MUST NOT include raw prompt bodies, credentials, full paths, provider payloads, or hidden memory bodies.

#### Scenario: Prompt assembly receipt leak rejected

r[agent_harness_runtime.clankers_prompt_assembly.receipts.rejects_leaks]
- GIVEN a prompt assembly receipt fixture contains raw prompt body text, a credential-looking value, a full path dump, provider JSON, or hidden memory body
- WHEN receipt validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Prompt assembly verification

r[agent_harness_runtime.clankers_prompt_assembly.verification] The adaptation MUST include focused positive and negative tests for typed sources, deterministic precedence, ledger states, budget truncation, missing discovery adapters, untrusted sources, unsupported source kinds, and receipt redaction.

#### Scenario: Closeout evidence covers assembly fixtures

r[agent_harness_runtime.clankers_prompt_assembly.verification.closeout]
- GIVEN maintainers intend to archive the prompt assembly kit adaptation
- WHEN closeout validation runs
- THEN evidence names focused source, precedence, ledger, discovery, receipt, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Bounded audit snapshots

r[agent_harness_runtime.clankers_audit_receipts.snapshots] Animus audit snapshots MUST project runtime state into schema-versioned bounded counts, status classes, timing buckets, over-limit flags, event window identity, and error classes without exposing private pending maps.

#### Scenario: Empty audit snapshot is valid

r[agent_harness_runtime.clankers_audit_receipts.snapshots.empty]
- GIVEN no model calls, tool calls, confirmations, background work, or delivery attempts are pending or completed in the window
- WHEN audit projection runs
- THEN it returns a valid empty snapshot with zero counts and safe schema metadata.

### Requirement: Pending state clamping

r[agent_harness_runtime.clankers_audit_receipts.clamping] Audit projection MUST clamp public pending counts to configured limits and record over-limit diagnostics instead of serializing full pending state.

#### Scenario: Pending tool calls exceed public limit

r[agent_harness_runtime.clankers_audit_receipts.clamping.pending_tools]
- GIVEN internal pending tool-call state exceeds the configured public pending limit
- WHEN audit projection runs
- THEN the receipt reports the configured limit, clamped public count, and over-limit status
- AND it does not serialize raw pending call IDs, arguments, outputs, or private metadata.

### Requirement: Host-owned audit sinks

r[agent_harness_runtime.clankers_audit_receipts.host_owned] Metrics sinks, telemetry exporters, clocks, persistent storage, and UI rendering MUST remain host-owned behavior outside reusable audit projection logic.

#### Scenario: Sink unavailable does not alter projection

r[agent_harness_runtime.clankers_audit_receipts.host_owned.sink_unavailable]
- GIVEN audit projection produces a valid snapshot
- AND a host telemetry sink is unavailable
- WHEN the host attempts export
- THEN the reusable projection remains unchanged
- AND sink failure is recorded as a host adapter outcome rather than a projection mutation.

### Requirement: Audit receipt composition

r[agent_harness_runtime.clankers_audit_receipts.composition] Audit receipts MUST reference safe runtime event, tool dispatch, confirmation, background work, delivery, and receipt status categories rather than raw source payloads.

#### Scenario: Completed failure classes summarize safely

r[agent_harness_runtime.clankers_audit_receipts.composition.failures]
- GIVEN completed work includes invalid input, denied capability, timeout, cancellation, malformed output, and unavailable service failures
- WHEN audit projection runs
- THEN the receipt summarizes counts by stable failure class
- AND no raw tool arguments, provider payloads, prompts, or output bodies are included.

### Requirement: Safe audit receipts

r[agent_harness_runtime.clankers_audit_receipts.receipts] Audit receipts MUST record safe metadata such as schema version, policy hash, event window identity, bounded counts, status classes, timing buckets, over-limit diagnostics, failure class counts, and BLAKE3 receipt hash, and MUST NOT include raw prompts, tool arguments, tool outputs, provider payloads, credentials, authorization headers, environment values, full paths, or unbounded logs.

#### Scenario: Audit receipt leak rejected

r[agent_harness_runtime.clankers_audit_receipts.receipts.rejects_leaks]
- GIVEN an audit receipt fixture contains raw prompt text, tool arguments, tool output, provider JSON, credentials, headers, environment values, full paths, or unbounded logs
- WHEN receipt validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Audit receipt verification

r[agent_harness_runtime.clankers_audit_receipts.verification] The adaptation MUST include focused positive and negative tests for empty snapshots, pending-state clamping, completed summaries, host-owned sink boundaries, failure class composition, over-limit diagnostics, and receipt redaction.

#### Scenario: Closeout evidence covers audit fixtures

r[agent_harness_runtime.clankers_audit_receipts.verification.closeout]
- GIVEN maintainers intend to archive the observability audit receipt adaptation
- WHEN closeout validation runs
- THEN evidence names focused snapshot, clamping, host-boundary, composition, receipt, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

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

### Requirement: Nickel-authored routing policy

r[agent_harness_runtime.clankers_cost_model_routing.policy] Animus model-routing policy MUST be authored as Nickel contracts or checked fixtures defining role names, thresholds, complexity weights, user-hint mappings, budget behavior, downgrade rules, provider constraints, and missing-price behavior.

#### Scenario: Malformed routing policy rejected

r[agent_harness_runtime.clankers_cost_model_routing.policy.invalid]
- GIVEN a routing policy has duplicate role names, negative weights, unordered thresholds, unknown downgrade targets, or unspecified missing-price behavior
- WHEN policy validation runs
- THEN validation rejects the policy before any routing decision is used.

### Requirement: Pure model routing decisions

r[agent_harness_runtime.clankers_cost_model_routing.decision] Model routing MUST be a pure decision over task signals, current usage/cost summaries, model catalog projections, and validated policy, returning selected role/model metadata or typed failure without contacting providers.

#### Scenario: Complex task selects stronger eligible role

r[agent_harness_runtime.clankers_cost_model_routing.decision.complex]
- GIVEN task signals exceed the configured high-complexity threshold
- AND an eligible stronger model role is available within policy
- WHEN routing runs
- THEN the decision selects that role with a complexity-score reason
- AND no provider request is sent by routing logic.

### Requirement: Budget-aware routing

r[agent_harness_runtime.clankers_cost_model_routing.budget] Routing MUST account for current usage and configured budget behavior, including soft-budget pressure, hard-budget downgrade, and hard-budget denial when no compliant model is available.

#### Scenario: Hard budget blocks upgrade

r[agent_harness_runtime.clankers_cost_model_routing.budget.hard_limit]
- GIVEN current cost meets or exceeds the configured hard budget
- AND the requested or scored model would be a cost upgrade
- WHEN routing evaluates budget policy
- THEN the decision downgrades to an allowed cheaper role or denies routing if no cheaper role is available
- AND the receipt records the hard-budget reason.

### Requirement: Injected safe model catalog projections

r[agent_harness_runtime.clankers_cost_model_routing.catalog] Routing MUST consume injected safe model catalog projections and MUST fail closed when required model capability, availability, or pricing metadata is absent under the active policy.

#### Scenario: Missing price denied by policy

r[agent_harness_runtime.clankers_cost_model_routing.catalog.missing_price]
- GIVEN policy requires price metadata for budgeted routing
- AND a candidate model lacks pricing metadata
- WHEN routing evaluates candidates
- THEN the candidate is rejected or routing fails according to policy
- AND no provider fallback is attempted.

### Requirement: Safe routing receipts

r[agent_harness_runtime.clankers_cost_model_routing.receipts] Routing receipts MUST record safe metadata such as decision status, selected role/model label, score bucket, budget status, candidate counts, policy hash, fallback class, and BLAKE3 receipt hash, and MUST NOT include raw prompts, provider payloads, credentials, full pricing-store dumps, or hidden reasoning text.

#### Scenario: Routing receipt leak rejected

r[agent_harness_runtime.clankers_cost_model_routing.receipts.rejects_leaks]
- GIVEN a routing receipt fixture contains raw prompt text, provider JSON, credentials, full pricing file contents, or hidden reasoning text
- WHEN receipt validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Cost-aware routing verification

r[agent_harness_runtime.clankers_cost_model_routing.verification] The adaptation MUST include focused positive and negative tests for policy validation, simple/complex task selection, explicit user hints, soft and hard budget behavior, missing prices, unavailable models, malformed policies, and receipt redaction.

#### Scenario: Closeout evidence covers routing fixtures

r[agent_harness_runtime.clankers_cost_model_routing.verification.closeout]
- GIVEN maintainers intend to archive the cost-aware routing adaptation
- WHEN closeout validation runs
- THEN evidence names focused policy, decision, budget, catalog, receipt, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Bounded Clankers and Toolpath provenance references

r[agent_harness_runtime.provenance_ledger.scope] Animus MUST define its own internal provenance/session-ledger model, MUST treat Clankers as a bounded reference for neutral replay, safe typed facts, BLAKE3 artifact identity, tool-host/effect receipts, and redaction patterns, and MUST treat Toolpath as an optional interchange reference rather than the internal source of truth.

#### Scenario: Reference scope rejects wholesale imports

r[agent_harness_runtime.provenance_ledger.scope.rejects_wholesale]
- GIVEN maintainers adapt provenance, session-ledger, or receipt behavior from Clankers or Toolpath
- WHEN the reusable Animus core or adapter crates are reviewed
- THEN the implementation uses Animus-owned DTOs and traits
- AND it does not require `toolpath`, `toolpath-convo`, `clankers-session`, `clanker-message`, Automerge, daemon, TUI, Pathbase, GitHub, Git, Matrix, plugin runtime, or global session-directory dependencies in reusable core contracts.

### Requirement: Animus-owned neutral session ledger

r[agent_harness_runtime.provenance_ledger.session_ledger] Animus MUST provide neutral session-ledger DTOs for model-visible messages, summaries, usage observations, safe receipts, and unsupported entries without depending on a concrete storage backend, daemon, UI, provider, transcript compatibility type, or wall-clock construction.

#### Scenario: Ledger replay rebuilds model-visible context

r[agent_harness_runtime.provenance_ledger.session_ledger.replay]
- GIVEN an in-memory ledger containing user, assistant, tool, summary, usage, and receipt entries
- WHEN pure replay runs
- THEN it returns model-visible messages in deterministic order with safe replay metadata counts
- AND it does not read files, inspect environment, allocate timestamps, spawn processes, call providers, or depend on Clankers desktop transcript IDs.

#### Scenario: Unsupported ledger entries fail closed

r[agent_harness_runtime.provenance_ledger.session_ledger.unsupported]
- GIVEN replay encounters an unsupported or adapter-owned ledger entry
- WHEN replay is required for a model request
- THEN it returns a typed unsupported-entry error
- AND no raw unsupported payload is included in the error or receipt.

### Requirement: Append-only safe provenance facts

r[agent_harness_runtime.provenance_ledger.typed_facts] Animus MUST define append-only typed provenance facts for model activity, tool activity, runtime blocks, errors, artifact references, authorization decisions, provenance observations, and opaque unknown records with safe query fields only.

#### Scenario: Typed facts are queryable without raw payloads

r[agent_harness_runtime.provenance_ledger.typed_facts.query]
- GIVEN a ledger contains model, tool, artifact, authorization, and error facts
- WHEN safe query filtering runs by artifact hash, tool kind, error class, requirement id, request shape, authorization status, effect ability, or effect resource
- THEN matching facts are returned deterministically
- AND raw prompts, provider payloads, tool input/output, credentials, and full paths are not required for the query.

#### Scenario: Unknown facts become safe opaque records

r[agent_harness_runtime.provenance_ledger.typed_facts.opaque]
- GIVEN a future or unknown fact shape is loaded from a compatibility boundary
- WHEN the fact cannot be decoded as a current typed record
- THEN Animus preserves only sanitized safe metadata or returns a typed compatibility error
- AND it does not retain or emit the unknown raw payload by default.

### Requirement: BLAKE3 artifact identity and redaction classes

r[agent_harness_runtime.provenance_ledger.artifacts] Animus artifact identities MUST use versioned canonical envelopes with artifact kind, envelope version, redaction class, normalized payload, sorted dependencies, and BLAKE3 hashes for Animus-owned content identity.

#### Scenario: Canonical artifact hashes are deterministic

r[agent_harness_runtime.provenance_ledger.artifacts.deterministic]
- GIVEN equivalent artifact envelopes with differently ordered dependency inputs
- WHEN artifact identity is computed
- THEN dependencies are sorted and deduplicated before hashing
- AND the resulting BLAKE3 hash is deterministic.

#### Scenario: Redaction classes gate payload display

r[agent_harness_runtime.provenance_ledger.artifacts.redaction]
- GIVEN an artifact envelope is marked metadata-only, redacted-payload, or secret
- WHEN receipt, CLI, or review output is rendered
- THEN payload display is omitted or replaced by explicit redaction markers
- AND only public artifacts may expose payload bodies by default.

### Requirement: Pure parent/branch provenance queries

r[agent_harness_runtime.provenance_ledger.branch_queries] Animus MUST provide pure parent/branch query helpers for ancestry, active branch reconstruction, branch divergence, unique branch entries, and abandoned/dead-end attempts over Animus-owned ledger/provenance DTOs.

#### Scenario: Branch queries detect abandoned attempts

r[agent_harness_runtime.provenance_ledger.branch_queries.dead_ends]
- GIVEN a provenance graph has one active head and a sibling branch that is not an ancestor of that head
- WHEN dead-end query runs
- THEN the sibling branch entries are returned as abandoned attempts
- AND the active branch entries are not misclassified as dead ends.

#### Scenario: Malformed parent references fail without panic

r[agent_harness_runtime.provenance_ledger.branch_queries.invalid]
- GIVEN a graph contains a missing parent reference, cycle, duplicate entry id, or reversed ordering assumption
- WHEN validation or query preparation runs
- THEN it returns a typed graph validation error
- AND no panic, infinite traversal, or silently accepted invalid ancestry is produced.

### Requirement: Safe runtime and provenance receipts

r[agent_harness_runtime.provenance_ledger.receipts] Animus runtime and provenance receipts MUST expose safe metadata only, including status, issue code, actor/session/trace labels, bounded counts, artifact hashes, mutation or effect kind, backend/interchange labels, and redaction markers, and MUST NOT include raw prompt text, provider payloads, credentials, full paths, raw tool bodies, raw model outputs, or unbounded outputs.

#### Scenario: Safe receipts link evidence without leaking content

r[agent_harness_runtime.provenance_ledger.receipts.safe]
- GIVEN a model call, tool call, authorization decision, or provenance observation has raw sensitive inputs and outputs
- WHEN a receipt is constructed
- THEN the receipt contains only safe labels, bounded counts, redaction markers, and BLAKE3 hashes
- AND the sensitive raw material is omitted.

#### Scenario: Receipt leak fixture is rejected

r[agent_harness_runtime.provenance_ledger.receipts.rejects_leaks]
- GIVEN a receipt fixture contains raw prompt text, provider JSON, credential-looking values, an absolute path dump, raw tool input, raw model output, or unbounded output
- WHEN receipt validation runs
- THEN validation fails with a typed redaction error
- AND the invalid receipt is not accepted as review or release evidence.

### Requirement: Tool-host and effect receipts remain adapter-safe

r[agent_harness_runtime.provenance_ledger.tool_effects] Animus MUST model tool-host outcomes, service availability, capability decisions, hook phases, runtime policy decisions, cancellation, truncation metadata, effect requests, and effect results as adapter-safe DTOs before provenance facts or receipts claim tool execution evidence.

#### Scenario: Missing tool service fails before evidence is claimed

r[agent_harness_runtime.provenance_ledger.tool_effects.missing_service]
- GIVEN a tool action requires storage, search, hooks, process, progress, capability, cancellation, or runtime-policy service support
- WHEN the service is missing or unavailable
- THEN dispatch returns a typed unavailable-service or capability-denied outcome
- AND no provenance receipt claims the tool mutation or effect completed.

#### Scenario: Tool output truncation is recorded safely

r[agent_harness_runtime.provenance_ledger.tool_effects.truncation]
- GIVEN a tool emits output larger than configured byte or line limits
- WHEN the tool-host accumulator finishes
- THEN the model-visible output is bounded
- AND the receipt records original and truncated counts without storing the omitted output.

### Requirement: Optional Toolpath interchange adapter

r[agent_harness_runtime.provenance_ledger.toolpath_adapter] Animus MAY provide an optional std-only Toolpath adapter that exports redacted Animus provenance as Toolpath Graph/Path/Step documents and imports redacted Toolpath documents into review fixtures, MUST treat Toolpath as an interchange and review vocabulary rather than the internal ledger, and MUST keep Toolpath crates, `toolpath-convo`, Pathbase, provider importers, Git/GitHub derivation, and CLI cache/share/resume behavior out of default reusable contracts.

#### Scenario: Redacted Toolpath export maps Animus DAGs

r[agent_harness_runtime.provenance_ledger.toolpath_adapter.export]
- GIVEN an Animus provenance ledger contains actors, parent relationships, active heads, artifact hashes, safe structural changes, typed refs, correlation links, and redacted receipts
- WHEN Toolpath export is explicitly requested
- THEN the adapter emits a Toolpath graph with equivalent path head, step parents, actor labels, typed refs, artifact hash markers, and safe structural perspectives
- AND raw prompts, provider payloads, tool input/output, full paths, credentials, raw unified diffs, and unbounded before/after content are omitted or represented by hashes/redaction markers.

#### Scenario: Default Toolpath export remains redacted

r[agent_harness_runtime.provenance_ledger.toolpath_adapter.redacted_default]
- GIVEN a Toolpath export candidate includes raw prompt text, provider JSON, credential-looking values, absolute path dumps, raw tool input, raw tool output, raw model output, unbounded before/after content, or raw unified diffs
- WHEN default redacted export validation runs
- THEN the adapter rejects the unsafe fields or replaces them with explicit redaction markers and BLAKE3 artifact hashes
- AND full-fidelity raw export is unavailable unless a future explicit policy enables it.

#### Scenario: Toolpath adoption matrix is documented

r[agent_harness_runtime.provenance_ledger.toolpath_adapter.docs]
- GIVEN maintainers evaluate Toolpath features for Animus
- WHEN the adoption matrix is reviewed
- THEN Graph/Path/Step, actor taxonomy, parent/head semantics, dead-end queries, typed refs/correlation, JSONL streaming shape, and Markdown/DOT review surfaces are documented as optional projection or review concerns
- AND Toolpath is not documented as Animus' internal ledger, runtime conversation IR, hashing authority, provider importer, or default storage format.

#### Scenario: Toolpath dependency isolation is verified

r[agent_harness_runtime.provenance_ledger.toolpath_adapter.isolated]
- GIVEN default Animus features are built without Toolpath compatibility
- WHEN workspace dependency checks run
- THEN reusable core, adapter, Steel, and standard-shell contracts do not require Toolpath crates, `toolpath-convo`, Pathbase clients, provider session importers, GitHub/Git derivation crates, or Toolpath cache/share/resume CLI behavior.

### Requirement: Focused provenance-ledger verification

r[agent_harness_runtime.provenance_ledger.verification] The provenance-ledger adaptation MUST include focused positive and negative verification for bounded reference scope, neutral ledger replay, unsupported-entry failure, typed fact queries, opaque record handling, BLAKE3 artifact identity, redaction classes, branch/dead-end queries, malformed graph rejection, tool/effect outcomes, safe receipts, optional Toolpath export, redacted-by-default Toolpath behavior, Toolpath adoption documentation, and dependency isolation.

#### Scenario: Closeout evidence is reviewable

r[agent_harness_runtime.provenance_ledger.verification.closeout]
- GIVEN maintainers intend to archive the provenance-ledger adaptation
- WHEN closeout validation runs
- THEN evidence names every successful focused rail, workspace test, clippy check, Nickel/config drift check if applicable, Cairn validation, and Cairn gate
- AND any skipped rail includes a concrete blocker and next-best validation.

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

### Requirement: Host-injected background queue service

r[agent_harness_runtime.hermes_background_queue.service] Animus MUST define host-injected background queue service contracts and DTOs for enqueue, status, wait, log, send-input, pause, resume, restart, cancel, clean, group listing, and group parallelism operations without requiring reusable runtime crates to spawn processes directly.

#### Scenario: Queue service handles long-running task

r[agent_harness_runtime.hermes_background_queue.service.enqueue]
- GIVEN a host injects a background queue service and a tool requests a long-running non-interactive command through an approved queue tool
- WHEN runtime dispatches the request
- THEN the queue service receives a typed enqueue request with command identity, group, environment policy, timeout policy, and output bounds
- AND reusable runtime code does not call shell, process, or pueue APIs directly.

### Requirement: Destructive queue controls require explicit selectors

r[agent_harness_runtime.hermes_background_queue.explicit_targets] Queue operations that pause, kill, restart, clean, resume, or alter group-wide parallelism MUST require explicit task, group, default-group, or all-groups selectors and MUST reject ambiguous default behavior.

#### Scenario: Ambiguous kill request is rejected

r[agent_harness_runtime.hermes_background_queue.explicit_targets.reject_ambiguous]
- GIVEN a queue-control request asks to kill work but omits task IDs, group name, explicit default-group selector, and all-groups selector
- WHEN request validation runs
- THEN validation returns a typed ambiguous-target diagnostic
- AND no queue backend operation is invoked.

### Requirement: Queue groups and parallelism are typed

r[agent_harness_runtime.hermes_background_queue.groups] Background queue DTOs MUST represent group identity, configured parallelism, pending/running/paused/stashed/completed counts, and backend-specific status labels as safe metadata.

#### Scenario: Group status is inspectable

r[agent_harness_runtime.hermes_background_queue.groups.status]
- GIVEN a queue backend has multiple groups with different parallelism limits
- WHEN a host reports queue status to Animus
- THEN the normalized DTO includes each group, parallelism limit, counts by stable status class, and safe backend labels
- AND prompt/report rendering can distinguish CPU-heavy, IO-heavy, or default groups when hosts provide labels.

### Requirement: Logs and command output are bounded

r[agent_harness_runtime.hermes_background_queue.logs] Background task logs MUST be exposed through bounded viewports with visible text, total line/byte counts, truncation state, safe references, and BLAKE3 content hashes when full-output identity is needed.

#### Scenario: Large log tail is truncated safely

r[agent_harness_runtime.hermes_background_queue.logs.truncated]
- GIVEN a completed background task produced a large stdout/stderr log
- WHEN the model-visible log view is requested
- THEN the response includes only the configured visible window or tail, line/byte counts, truncation state, and optional BLAKE3 hash
- AND it does not embed the full raw log body in model-visible content or receipts.

### Requirement: Wait and send semantics are explicit capabilities

r[agent_harness_runtime.hermes_background_queue.wait_send] Background queue tooling MUST model wait, stream, poll, and send-input behavior as explicit capabilities with timeout, paused/stashed/queued handling, and unsupported-operation failures.

#### Scenario: Send input unsupported fails closed

r[agent_harness_runtime.hermes_background_queue.wait_send.unsupported_send]
- GIVEN a backend task does not support stdin/send-input operations
- WHEN a tool request attempts to send input to that task
- THEN the queue service returns an unsupported-operation failure
- AND runtime does not fall back to shelling out to a backend-specific CLI.

### Requirement: Background queue adaptation verification

r[agent_harness_runtime.hermes_background_queue.verification] The background queue adaptation MUST include focused tests for enqueue/status/log flows, explicit target validation, group/parallelism DTOs, bounded logs, wait behavior, unsupported send failures, missing queue service failures, and backend isolation.

#### Scenario: Closeout evidence covers queue fixtures

r[agent_harness_runtime.hermes_background_queue.verification.closeout]
- GIVEN maintainers intend to archive the Hermes background queue adaptation
- WHEN closeout validation runs
- THEN evidence names focused queue DTO tests, destructive-target tests, log-bound tests, wait/send tests, workspace checks, Cairn validation, and Cairn gates
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Bounded Git AI provenance reference

r[agent_harness_runtime.provenance_git_ai.scope] Animus MUST treat Git AI as a bounded reference for explicit checkpointing, line-level attribution, session/model/tool identity, accepted-line stats, and optional Git AI v3 authorship-log interoperability, and MUST NOT adopt Git AI's daemon, installer, updater, hosted telemetry, SQLite prompt store, Git operation watcher, or Git-only workflow assumptions as reusable runtime architecture.

#### Scenario: Git AI remains a compatibility boundary

r[agent_harness_runtime.provenance_git_ai.scope.boundary]
- GIVEN maintainers adapt a Git AI-inspired provenance behavior
- WHEN the behavior is added to Animus
- THEN reusable runtime and provenance APIs are expressed in VCS-neutral terms
- AND Git-specific formats, refs, commands, and history-operation names are isolated to optional backend or compatibility modules.

### Requirement: VCS-neutral provenance domain

r[agent_harness_runtime.provenance_git_ai.domain] Animus provenance types MUST model content observations, repository-relative file identities, content hashes, line ranges, session identities, trace IDs, mutating tool kinds, and safe receipt metadata without requiring Git, Git notes, commit SHAs, branch names, hooks, stash, rebase, or any other Git-specific concept.

#### Scenario: Provenance core is directly testable without a VCS

r[agent_harness_runtime.provenance_git_ai.domain.no_vcs]
- GIVEN a test constructs provenance observations, line ranges, sessions, trace IDs, and content hashes in memory
- WHEN pure provenance validation and range transformation functions run
- THEN they return deterministic typed results without reading files, executing commands, invoking Git, opening a repository, or depending on a concrete VCS backend.

#### Scenario: Git terms are rejected from core DTOs

r[agent_harness_runtime.provenance_git_ai.domain.rejects_git_terms]
- GIVEN a proposed core provenance DTO adds a required Git note ref, commit SHA, branch, stash, rebase, or Git command field
- WHEN provenance boundary review or focused validation runs
- THEN the change fails the VCS-neutrality check unless the field is moved behind an optional Git compatibility adapter.

### Requirement: Injected version-control provenance backend

r[agent_harness_runtime.provenance_git_ai.vcs_backend] Runtime provenance persistence MUST use an injected version-control provenance backend or explicitly disabled backend setting, and MUST fail closed with a typed unavailable-backend or unsupported-operation error when provenance recording is required but no suitable backend is injected.

#### Scenario: In-memory non-Git backend records checkpoints

r[agent_harness_runtime.provenance_git_ai.vcs_backend.in_memory]
- GIVEN runtime dispatch records mutating tool checkpoints through an injected in-memory backend that does not implement Git semantics
- WHEN the tool call succeeds
- THEN the backend receives before and after provenance observations with session, trace, file identity, mutation kind, and content-hash metadata
- AND no Git command, Git hook, or Git note API is called.

#### Scenario: Missing required backend fails before mutation evidence is claimed

r[agent_harness_runtime.provenance_git_ai.vcs_backend.missing]
- GIVEN provenance recording is required by policy for a mutating tool action
- WHEN no backend is injected or the selected backend reports unsupported mutation tracking
- THEN runtime returns a typed provenance-backend failure
- AND no receipt claims complete line-level attribution for that action.

### Requirement: Nickel-governed mutating-tool checkpoint policy

r[agent_harness_runtime.provenance_git_ai.tool_policy] Mutating-tool checkpoint classification MUST be governed by Nickel-authored policy that maps raw tool names to canonical mutation kinds, file-identity fields, backend requirements, and receipt-redaction behavior before runtime dispatch records provenance.

#### Scenario: Valid policy classifies mutating tools

r[agent_harness_runtime.provenance_git_ai.tool_policy.valid]
- GIVEN Nickel policy maps raw tools to canonical edit, write, replace, or rename mutation kinds with explicit file-identity fields
- WHEN runtime dispatch prepares a matching tool action
- THEN the provenance observer records a before observation, calls the injected tool executor, records an after observation, and emits safe receipt metadata for the selected mutation kind.

#### Scenario: Invalid policy fails closed

r[agent_harness_runtime.provenance_git_ai.tool_policy.invalid]
- GIVEN Nickel policy references an unknown mutation kind, missing file-identity field, unsupported backend requirement, or unsafe receipt field
- WHEN policy export, drift check, or runtime loading runs
- THEN validation fails before the tool action is treated as provenance-trackable
- AND the failure identifies the policy issue without recording raw tool input or full paths.

### Requirement: Pure line-range and hunk-transform core

r[agent_harness_runtime.provenance_git_ai.range_core] Animus MUST provide pure deterministic line-range, range-compression, overlap, removal, hunk-shift, and accepted-line counting logic for provenance attribution, with explicit validation errors for malformed ranges, unsorted ranges, zero line numbers, underflow, overflow, and impossible hunk shapes.

#### Scenario: Positive range transformations are deterministic

r[agent_harness_runtime.provenance_git_ai.range_core.positive]
- GIVEN valid line ranges and diff hunk descriptions for insertions, deletions, replacements, and multiple hunks
- WHEN range transformation functions run
- THEN preserved lines retain attribution, deleted or replaced lines lose attribution, later lines shift by the accumulated offset, and accepted-line counts are deterministic.

#### Scenario: Malformed ranges fail without panic

r[agent_harness_runtime.provenance_git_ai.range_core.negative]
- GIVEN malformed line ranges, zero line numbers, reversed ranges, unsorted entries, invalid hunk headers, or arithmetic overflow inputs
- WHEN range parsing or transformation functions run
- THEN they return typed validation errors
- AND no panic, wrapping line number, or silently accepted invalid attribution is produced.

### Requirement: Optional Git AI v3 compatibility adapter

r[agent_harness_runtime.provenance_git_ai.git_ai_adapter] Animus MAY provide an optional Git AI compatibility adapter that reads and writes `authorship/3.0.0` authorship logs for Git repositories, stores Git AI notes under `refs/notes/ai`, and confines Git AI-required SHA-256 key derivation to that adapter boundary while preserving BLAKE3 defaults for Animus-owned hashes and receipts.

#### Scenario: Git AI v3 authorship log round-trips at the adapter boundary

r[agent_harness_runtime.provenance_git_ai.git_ai_adapter.round_trip]
- GIVEN a valid Git AI v3 authorship log containing session, trace, known-human, legacy prompt, file attestation, and metadata sections
- WHEN the optional compatibility adapter decodes and re-encodes it
- THEN the adapter preserves the externally required schema version, key routing, line ranges, metadata maps, and `refs/notes/ai` storage target
- AND the shared provenance domain receives only VCS-neutral observation data and external compatibility identifiers.

#### Scenario: Git AI-specific data does not become core requirement

r[agent_harness_runtime.provenance_git_ai.git_ai_adapter.not_core]
- GIVEN an embedder uses a non-Git backend or disables Git AI compatibility
- WHEN provenance recording runs
- THEN runtime does not require `refs/notes/ai`, Git commit SHAs, Git CLI availability, or Git AI SHA-256 IDs
- AND receipts continue to use Animus-owned BLAKE3 content hashes where a hash algorithm is not externally mandated.

### Requirement: Safe provenance receipts and session links

r[agent_harness_runtime.provenance_git_ai.receipts] Provenance receipts MUST expose safe metadata only, including status, issue code, backend kind, mutation kind, labels, bounded counts, BLAKE3 content hashes, external compatibility IDs, session IDs, trace IDs, and explicit redaction markers, and MUST NOT include raw prompts, provider payloads, credentials, full paths, raw tool bodies, or unbounded outputs.

#### Scenario: Provenance receipt links to session without leaking prompt text

r[agent_harness_runtime.provenance_git_ai.receipts.session_link]
- GIVEN a mutating tool action is associated with an agent session, model, and trace ID
- WHEN provenance receipt construction runs
- THEN the receipt records safe session, model, tool, mutation, count, content-hash, and backend metadata
- AND raw prompt text, raw model output, raw tool input, raw tool output, credentials, and full paths are omitted or explicitly redacted.

#### Scenario: Receipt leak fixture is rejected

r[agent_harness_runtime.provenance_git_ai.receipts.rejects_leaks]
- GIVEN a provenance receipt fixture contains raw prompt text, credential-looking values, an absolute path dump, raw provider payload, raw tool input, or unbounded tool output
- WHEN receipt validation runs
- THEN validation fails with a typed redaction error
- AND no receipt with leaked content is accepted as release evidence.

### Requirement: Focused VCS-neutral provenance verification

r[agent_harness_runtime.provenance_git_ai.verification] The provenance adaptation MUST include focused positive and negative verification for bounded Git AI scope, VCS-neutral DTOs, Nickel tool policy, non-Git backend recording, missing backend failures, pure range/hunk transforms, optional Git AI v3 round trips, and provenance receipt redaction.

#### Scenario: Verification proves Git is optional

r[agent_harness_runtime.provenance_git_ai.verification.git_optional]
- GIVEN the provenance adaptation is ready for review
- WHEN focused verification runs
- THEN evidence includes a non-Git in-memory backend test that records mutating-tool provenance without Git
- AND Git AI compatibility tests are isolated behind the optional adapter boundary.

### Requirement: Optional TUI host dependency boundary

r[agent_harness_runtime.ratatui_tool_host.boundary] Animus MUST keep Rats, Ratatui, Crossterm, terminal raw-mode handling, redraw scheduling, and keybinding interpretation out of `animus-core`, `animus-adapters`, `animus-steel`, and reusable default tool contracts. A Ratatui-compatible shell MAY exist as an optional host crate or binary that depends on the reusable runtime contracts.

#### Scenario: Reusable crates remain terminal-free

r[agent_harness_runtime.ratatui_tool_host.boundary.terminal_free]
- GIVEN maintainers inspect reusable Animus crate dependencies and public DTOs
- WHEN a Ratatui-compatible host is added
- THEN terminal UI crates, Rats crates, and terminal-specific types appear only in the optional host shell
- AND ordinary model and tool adapters remain usable without linking a terminal backend.

### Requirement: Rats dependency is host-only and SSH-pinned

r[agent_harness_runtime.ratatui_tool_host.rats_dependency] The optional TUI host crate MUST depend on the Rats workspace's Subwayrat `rat-widgets` and `rat-keymap` packages through SSH-authenticated Cargo Git URLs pinned to explicit revisions, and MUST NOT expose Rats types through reusable core, adapter, Steel, std runtime, or tool contract APIs.

#### Scenario: Rats dependency stays inside TUI crate

r[agent_harness_runtime.ratatui_tool_host.rats_dependency.host_only]
- GIVEN the optional TUI host dependency graph is inspected
- WHEN the Rats dependency is resolved from `ssh://git@github.com/brittonr/rats.git`
- THEN only the TUI host crate depends on Rats/Subwayrat packages
- AND the dependencies are pinned to concrete revisions rather than floating branches
- AND keybinding interpretation goes through Subwayrat `rat-keymap` rather than a direct Crossterm dependency.

### Requirement: UI-neutral tool host bridge

r[agent_harness_runtime.ratatui_tool_host.bridge] Animus MUST define a UI-neutral tool host bridge for interactive tool execution with typed lifecycle events, progress updates, bounded output observations, approval requests, cancellation notices, final results, and failure classes.

#### Scenario: Tool progress renders without direct drawing

r[agent_harness_runtime.ratatui_tool_host.bridge.progress]
- GIVEN an interactive tool emits started, progress, output, and finished events through the host bridge
- WHEN a Ratatui host receives those events
- THEN it updates its render model from the typed events
- AND the tool never receives a Ratatui frame, terminal handle, or direct drawing callback.

### Requirement: Hosted execution remains adapter-compatible

r[agent_harness_runtime.ratatui_tool_host.adapter_compat] Hosted tools MUST execute around validated `ToolRequest` and `ToolResponse` values and MUST preserve the existing simple `ToolAdapter` path for noninteractive tools.

#### Scenario: Simple tool runs without TUI services

r[agent_harness_runtime.ratatui_tool_host.adapter_compat.simple]
- GIVEN a noninteractive tool implements the existing simple tool adapter contract
- WHEN the runtime dispatches the tool in a headless or test host
- THEN the tool runs without requiring any TUI host service
- AND the response still crosses the validated `ToolResponse` boundary.

### Requirement: Interactive commands are finite and correlated

r[agent_harness_runtime.ratatui_tool_host.commands] Interactive tool requests for host input MUST expose finite command choices with a correlation identity, and hosts MUST answer with typed decisions such as approve, deny, continue, or cancel rather than unbounded terminal-specific input.

#### Scenario: Approval denial fails safely

r[agent_harness_runtime.ratatui_tool_host.commands.denied]
- GIVEN a tool requests approval for a capability-bearing action through the host bridge
- WHEN the Ratatui host returns a denial decision for the matching correlation identity
- THEN runtime reports a typed denied or cancelled tool result
- AND the guarded tool action is not executed after the denial.

### Requirement: Render model is pure and replayable

r[agent_harness_runtime.ratatui_tool_host.render_model] The Ratatui-compatible shell MUST derive visible tool/session state by applying a pure reducer over typed runtime and tool-host events, with terminal I/O, polling, cleanup, channels, and redraw orchestration isolated in the shell.

#### Scenario: Test backend reproduces visible state

r[agent_harness_runtime.ratatui_tool_host.render_model.test_backend]
- GIVEN a recorded sequence of runtime and tool-host events
- WHEN the render reducer and a Ratatui test backend render the sequence
- THEN the visible panels show the expected session summary, active tool state, approval prompt, progress, final result, or failure state
- AND the test does not require a real terminal.

### Requirement: Terminal failures do not corrupt runtime state

r[agent_harness_runtime.ratatui_tool_host.failure_isolation] Terminal setup failures, render errors, dropped event consumers, closed command channels, and cancellation signals MUST surface as typed host errors or cancellation events without corrupting core state, bypassing tool policy, or leaking unbounded tool output.

#### Scenario: Dropped TUI consumer cancels interactive wait

r[agent_harness_runtime.ratatui_tool_host.failure_isolation.dropped_consumer]
- GIVEN an interactive tool is waiting for a host approval decision
- WHEN the TUI event consumer or command channel closes
- THEN the hosted execution path returns a typed cancellation or host-unavailable failure
- AND no pending guarded action is executed by default.

### Requirement: TUI host verification rails

r[agent_harness_runtime.ratatui_tool_host.verification] The TUI host surface MUST include focused positive and negative verification for dependency isolation, host bridge event handling, simple adapter compatibility, approval denial, cancellation, pure render reduction, terminal test-backend rendering, and safe failure isolation before closeout.

#### Scenario: Closeout evidence covers TUI and non-TUI hosts

r[agent_harness_runtime.ratatui_tool_host.verification.closeout]
- GIVEN maintainers intend to archive the Ratatui-compatible tool host change
- WHEN closeout validation runs
- THEN evidence names focused bridge tests, render reducer tests, Ratatui test-backend tests, dependency-isolation checks, workspace checks, Cairn validation, and Cairn gates
- AND any skipped TUI dogfood or live terminal check is recorded as a non-claim with a blocker and next-best validation.

### Requirement: UI-neutral host topology DTOs

r[agent_harness_runtime.host_control.topology_dtos] Animus MUST define UI-neutral host-control DTOs for workspaces, tabs, panes, agents, labels, cwd or provenance summaries, focus state, rename and close operations, revision metadata, and selected semantic status fields without embedding TUI, PTY, socket, or terminal rendering types.

#### Scenario: Topology command validates selectors

r[agent_harness_runtime.host_control.topology_dtos.validation]
- GIVEN a host-control command targets a workspace, tab, pane, or agent selector
- WHEN command validation runs
- THEN valid selectors and supported operations are admitted as typed host commands
- AND missing selectors, stale selectors, cross-host selectors, duplicate labels, and unsupported operations are rejected with typed diagnostics.

### Requirement: Selector lifetime is explicit

r[agent_harness_runtime.host_control.selector_lifetime] Host-control selectors MUST distinguish live-session selectors from host-declared durable identities, and callers MUST NOT treat compact live topology IDs as durable across topology changes unless the host marks them durable.

#### Scenario: Compact live selector is not durable

r[agent_harness_runtime.host_control.selector_lifetime.not_durable]
- GIVEN a pane selector was returned before a tab or pane topology change
- WHEN a caller tries to reuse that selector after the host reports a revision change
- THEN validation requires re-resolution or matching revision evidence
- AND the selector is not accepted as a durable identity by default.

### Requirement: Bounded host output reads

r[agent_harness_runtime.host_control.bounded_reads] Host output reads MUST return bounded view DTOs with source class, line and byte counts, truncation status, redaction status, safe references, and optional BLAKE3 content hashes rather than unbounded transcripts.

#### Scenario: Large output is truncated explicitly

r[agent_harness_runtime.host_control.bounded_reads.truncated]
- GIVEN a host pane has more output than the requested bounded view allows
- WHEN a caller requests recent output
- THEN the response contains only the bounded view
- AND line count, byte count, truncation status, redaction status, and optional safe hash metadata make the omission explicit.

### Requirement: Declarative layout contract

r[agent_harness_runtime.host_control.layout_contract] Animus SHOULD define declarative layout export and apply DTOs for pane and split trees, labels, cwd summaries, optional command descriptors, focus, and zoom state, while explicitly separating layout reconstruction from live process preservation.

#### Scenario: Layout apply does not claim live process preservation

r[agent_harness_runtime.host_control.layout_contract.apply]
- GIVEN a host applies a declarative layout tree to create or replace a tab
- WHEN the layout apply response is produced
- THEN it reports created topology and focus state
- AND it does not claim that previous live PTYs, scrollback, or running processes were preserved unless a separate host capability provides that evidence.

### Requirement: Event subscriptions and waits share semantics

r[agent_harness_runtime.host_control.events_waits] Event subscriptions and one-shot waits MUST use the same typed event and semantic status contracts for agent status changes, output matches, pane lifecycle, cancellation, timeout, and host errors.

#### Scenario: Wait timeout is typed

r[agent_harness_runtime.host_control.events_waits.timeout]
- GIVEN a caller waits for an agent to reach `done`
- AND the semantic state does not reach `done` before the requested timeout
- WHEN the wait completes
- THEN the response is a typed timeout outcome
- AND it does not fabricate a terminal command completion or final agent result.

### Requirement: Transport-neutral command envelopes

r[agent_harness_runtime.host_control.transport_neutral] Host-control commands, responses, errors, and events MUST be transport-neutral DTOs with correlation IDs, typed errors, feature negotiation, and unknown-field handling; sockets, named pipes, CLI wrappers, PTYs, and terminal rendering MUST remain host adapter concerns.

#### Scenario: Command validation needs no IPC server

r[agent_harness_runtime.host_control.transport_neutral.no_ipc_dependency]
- GIVEN a command envelope and host capability snapshot are available in memory
- WHEN reusable validation logic checks the command
- THEN validation succeeds or fails deterministically over those inputs
- AND no socket, named pipe, PTY, terminal renderer, or CLI wrapper is required.

### Requirement: Host control verification

r[agent_harness_runtime.host_control.verification] The adaptation MUST include focused positive and negative tests for topology DTOs, selector lifetime, bounded reads, layout export/apply, event subscriptions, waits, typed errors, feature negotiation, unknown fields, and transport-neutral validation.

#### Scenario: Closeout evidence covers host-control rails

r[agent_harness_runtime.host_control.verification.closeout]
- GIVEN maintainers intend to archive the Host control protocol adaptation
- WHEN closeout validation runs
- THEN evidence names focused topology, selector, bounded-read, layout, event/wait, transport-neutral, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a blocker and next-best validation.

### Requirement: Explicit restore matrix

r[agent_harness_runtime.session_restore.restore_matrix] Animus MUST define typed restore modes and capability claims for live process persistence, snapshot layout restore, bounded history replay, native agent session resume, handoff-style transfer, and safe fallback.

#### Scenario: Restore claim states what survived

r[agent_harness_runtime.session_restore.restore_matrix.claims]
- GIVEN a host reports a restored session plan
- WHEN Animus validates the plan
- THEN each restored target explicitly states whether process state survived, layout was reconstructed, display history was replayed, or native conversation resume was requested
- AND no mode implies stronger survival than the host capability evidence supports.

### Requirement: Safe native session references

r[agent_harness_runtime.session_restore.native_session_reference] Native agent session references MUST be represented as safe typed records with source, agent, kind, bounded value identity, freshness, uniqueness scope, and resume capability metadata before any host attempts resume.

#### Scenario: Invalid reference degrades to fallback

r[agent_harness_runtime.session_restore.native_session_reference.invalid]
- GIVEN a restore snapshot contains a missing, malformed, duplicated, stale, unsupported, or sensitive native session reference
- WHEN restore planning evaluates that reference
- THEN Animus rejects native resume for that target with a typed diagnostic
- AND the plan degrades to an explicit safe fallback instead of claiming conversation continuity.

### Requirement: History replay is opt-in and display-only

r[agent_harness_runtime.session_restore.history_replay] Pane or transcript history replay MUST be disabled by default, and when enabled MUST be bounded, redacted, classified as display replay, and excluded from model-visible context unless separately admitted by policy.

#### Scenario: Replayed history cannot masquerade as live context

r[agent_harness_runtime.session_restore.history_replay.safe]
- GIVEN a restore snapshot includes recent display history
- WHEN history replay planning runs
- THEN the replay metadata records redaction and truncation status
- AND the replay is not treated as a live process, native conversation resume, or model-visible context by default.

### Requirement: Restore execution is host-owned

r[agent_harness_runtime.session_restore.host_owned_execution] Reusable Animus runtime crates MUST validate snapshots and produce restore plans without spawning processes, reading files, opening PTYs, constructing shell commands, transferring terminals, or inspecting environment state.

#### Scenario: Pure restore planning has no ambient I/O

r[agent_harness_runtime.session_restore.host_owned_execution.no_ambient_io]
- GIVEN restore planning receives typed snapshots, host capability declarations, session references, and policy inputs in memory
- WHEN planning runs
- THEN the returned plan is deterministic over those inputs
- AND no reusable runtime code performs filesystem, terminal, process, network, clock, or environment I/O.

### Requirement: Degraded restore outcomes are explicit

r[agent_harness_runtime.session_restore.degraded_outcomes] Unsupported agents, unavailable resume services, failed handoff claims, invalid snapshots, and denied history replay MUST produce explicit degraded-restore outcomes and safe receipts rather than silent blank-shell fallbacks.

#### Scenario: Fallback cannot claim successful resume

r[agent_harness_runtime.session_restore.degraded_outcomes.no_false_claims]
- GIVEN native resume is unavailable for a restored target
- WHEN the host creates a fallback shell or replacement process
- THEN the outcome records fallback status and reason
- AND receipts do not claim process survival, handoff success, native resume, or conversation continuity.

### Requirement: Restore verification

r[agent_harness_runtime.session_restore.verification] The adaptation MUST include focused positive and negative tests for restore matrix claims, native session references, history replay safety, pure restore planning, host-owned execution, degraded outcomes, and false-survival-claim rejection.

#### Scenario: Closeout evidence covers restore rails

r[agent_harness_runtime.session_restore.verification.closeout]
- GIVEN maintainers intend to archive the session restore adaptation
- WHEN closeout validation runs
- THEN evidence names focused restore matrix, native reference, history replay, host-owned execution, degraded outcome, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a blocker and next-best validation.

### Requirement: Typed multimodal image parts

r[agent_harness_runtime.multimodal_image_payloads.typed_parts] Animus MUST represent image inputs as typed multimodal content parts with safe source labels, media type, byte count, BLAKE3 content identity, source trust metadata, and optional host-supplied image metadata, and MUST NOT place raw image bytes, base64 payloads, data URLs, credentials, or private absolute paths in reusable runtime content DTOs.

#### Scenario: Valid image descriptor is admitted

r[agent_harness_runtime.multimodal_image_payloads.typed_parts.validation]
- GIVEN a host-admitted image reference has a safe label, supported image media type, byte count, BLAKE3 content identity, and source trust metadata
- WHEN multimodal content-part validation runs
- THEN Animus accepts an image content part that can be ordered with text parts
- AND the reusable runtime DTO contains no raw image bytes, base64 payload, data URL, credential, or private absolute path.

#### Scenario: Malformed image descriptor is rejected

r[agent_harness_runtime.multimodal_image_payloads.typed_parts.invalid]
- GIVEN an image descriptor has an invalid media type, missing content identity, missing byte count, raw bytes, base64, a data URL, or a private absolute path
- WHEN multimodal content-part validation runs
- THEN Animus rejects the descriptor with a typed diagnostic
- AND no provider dispatch can be constructed from that descriptor.

### Requirement: Prompt assembly preserves image order and outcomes

r[agent_harness_runtime.multimodal_image_payloads.prompt_assembly] Prompt/model request assembly MUST preserve deterministic ordering of text and image content parts after policy admission and budget decisions, and MUST report explicit included, rejected, or unavailable outcomes for every requested image part before provider dispatch.

#### Scenario: Mixed text and image prompt keeps order

r[agent_harness_runtime.multimodal_image_payloads.prompt_assembly.order]
- GIVEN an admitted prompt contains text before and after an image part
- WHEN prompt assembly builds model-visible content
- THEN the assembled request preserves the relative text and image ordering
- AND any rejected or unavailable image part is represented by an explicit outcome instead of being silently dropped.

### Requirement: Image dispatch is capability gated

r[agent_harness_runtime.multimodal_image_payloads.capability_gate] Provider dispatch MUST require selected provider/model capability metadata that advertises image input support before sending image content parts, and MUST fail closed when image support is absent, disabled, or unavailable.

#### Scenario: Unsupported model rejects image request

r[agent_harness_runtime.multimodal_image_payloads.capability_gate.unsupported]
- GIVEN an assembled request contains an image content part
- AND the selected model metadata does not advertise image input support
- WHEN dispatch preparation runs
- THEN Animus returns a typed unsupported-modality diagnostic
- AND it does not silently switch providers, downgrade to text-only input, or call a provider adapter.

### Requirement: Host adapters own image bytes and native serialization

r[agent_harness_runtime.multimodal_image_payloads.adapter_serialization] Image byte loading, MIME sniffing, decoding, resizing, transcoding, remote handle resolution, and provider-specific URL/base64/file-handle serialization MUST live in host/provider adapters behind injected services, while reusable runtime crates validate descriptors and dispatch decisions only.

#### Scenario: Provider-native fixture serializes image request

r[agent_harness_runtime.multimodal_image_payloads.adapter_serialization.fixtures]
- GIVEN a provider adapter receives a validated image descriptor and an approved injected image payload service
- WHEN provider request serialization runs against a fixture
- THEN the adapter emits the provider-native image request shape expected by that fixture
- AND tests fail if serialization relies on lossy generic JSON conversion, requires a live provider call, or omits the image part.

### Requirement: Safe image receipts and ledger metadata

r[agent_harness_runtime.multimodal_image_payloads.safe_receipts] Multimodal receipts and session-ledger records MUST store safe image provenance only, including modality, status, safe label or hash, media type, byte count, optional dimensions, truncation or rejection status, adapter class, and BLAKE3 content identity, and MUST reject raw image bytes, base64/data URLs, full private paths, credentials, and unbounded provider payloads.

#### Scenario: Image metadata leak is rejected

r[agent_harness_runtime.multimodal_image_payloads.safe_receipts.rejects_leaks]
- GIVEN an image receipt fixture contains raw image bytes, base64, a data URL, a full private path, a credential-looking value, or an unbounded provider payload
- WHEN receipt metadata validation runs
- THEN validation rejects the fixture with a typed leak diagnostic
- AND the invalid fixture is not accepted as durable lifecycle evidence.

### Requirement: Multimodal image verification

r[agent_harness_runtime.multimodal_image_payloads.verification] The adaptation MUST include focused positive and negative tests for image content-part validation, prompt ordering, provider capability gating, host image-service injection, provider-native serialization fixtures, unsupported models, malformed descriptors, oversized inputs, missing services, lossy conversions, and safe receipt validation.

#### Scenario: Closeout evidence covers image payload rails

r[agent_harness_runtime.multimodal_image_payloads.verification.closeout]
- GIVEN maintainers intend to archive the multimodal image payload adaptation
- WHEN closeout validation runs
- THEN evidence names focused content-part, prompt assembly, capability gate, adapter serialization, receipt leak, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Permission modes are advisory defaults

r[agent_harness_runtime.zerostack_permission_kernel.modes] Animus MUST define typed permission modes for restrictive, read-only, guarded, standard, and yolo-style UX defaults, and those modes MUST remain advisory beneath authority, capability, sandbox, and redaction checks.

#### Scenario: Yolo mode cannot bypass authority

r[agent_harness_runtime.zerostack_permission_kernel.modes.authority]
- GIVEN a guarded action lacks required authority or sandbox allowance
- AND the active permission mode would locally allow the action
- WHEN permission resolution runs
- THEN the action is denied before adapter dispatch
- AND the receipt separates local mode outcome from missing authority.

### Requirement: Pattern rule descriptors

r[agent_harness_runtime.zerostack_permission_kernel.pattern_rules] Permission rule descriptors MUST support glob-like and regex-like match kinds over action class and resource label with explicit source identity, trust scope, precedence, and allow, ask, or deny outcome.

#### Scenario: Malformed pattern cannot authorize

r[agent_harness_runtime.zerostack_permission_kernel.pattern_rules.invalid]
- GIVEN a rule descriptor has an unknown match kind, unsupported outcome, missing action class, invalid regex, or absent source identity
- WHEN rule validation runs
- THEN validation returns typed diagnostics
- AND the rule cannot authorize, prompt for, or deny an action.

### Requirement: Scoped session allowlists

r[agent_harness_runtime.zerostack_permission_kernel.session_allowlist] Session allowlist entries MUST be scoped to action identity, redacted resource identity or hash, policy identity, session identity, and invalidation metadata before they can satisfy an ask decision.

#### Scenario: Stale allowlist entry is rejected

r[agent_harness_runtime.zerostack_permission_kernel.session_allowlist.stale]
- GIVEN an allowlist entry was created for a different action, resource, policy hash, or session
- WHEN permission resolution evaluates the entry
- THEN the entry is rejected as stale or mismatched
- AND the action remains unexecuted until another valid decision is provided.

### Requirement: Repeated action detection

r[agent_harness_runtime.zerostack_permission_kernel.repeated_action] Animus SHOULD detect repeated identical guarded action identities and return policy-selected outcomes for coach, ask, deny, or ignore without directly dispatching the repeated action first.

#### Scenario: Repeated destructive action is denied

r[agent_harness_runtime.zerostack_permission_kernel.repeated_action.deny]
- GIVEN policy marks an action class as destructive when repeated with the same resource identity
- AND the repeated action threshold has been reached
- WHEN permission resolution runs for the repeated action
- THEN the resolver returns a denied repeated-action outcome
- AND no filesystem, process, network, model, or tool adapter is called.

### Requirement: Safe permission receipts

r[agent_harness_runtime.zerostack_permission_kernel.receipts] Permission receipts MUST record bounded metadata for action class, redacted resource identity or BLAKE3 resource hash, matched rule identity, mode outcome, authority outcome, repeated-action status, prompt-decision status, and final decision.

#### Scenario: Receipt omits sensitive request material

r[agent_harness_runtime.zerostack_permission_kernel.receipts.redacted]
- GIVEN a permission request contains a prompt, credential-looking value, provider payload, full host path, or raw tool body
- WHEN a permission receipt is produced
- THEN the sensitive value is omitted or represented only by an approved redaction marker or BLAKE3 hash
- AND the receipt still includes enough typed reason codes to audit the decision.

### Requirement: Typed prompt profile descriptors

r[agent_harness_runtime.zerostack_prompt_profiles.descriptors] Animus MUST represent prompt profiles as typed descriptors with profile ID, purpose, action posture, required capabilities, source identity, trust scope, precedence, body text, and prompt-inclusion policy.

#### Scenario: Untrusted profile is not silently injected

r[agent_harness_runtime.zerostack_prompt_profiles.descriptors.untrusted]
- GIVEN a prompt profile comes from an untrusted source or lacks source identity
- WHEN prompt assembly evaluates the profile
- THEN the profile is rejected or withheld pending host admission according to policy
- AND its body text is not silently injected into model-visible context.

### Requirement: Prompt directives are parsed metadata

r[agent_harness_runtime.zerostack_prompt_profiles.directives] Prompt directives such as mode or chain hints MUST be parsed into typed metadata before prompt assembly and MUST NOT remain hidden control syntax in model-visible profile body text.

#### Scenario: Unknown mode directive fails closed

r[agent_harness_runtime.zerostack_prompt_profiles.directives.invalid]
- GIVEN a profile body contains a directive with an unknown mode, malformed syntax, unsupported chain target, or duplicate exclusive directive
- WHEN directive parsing runs
- THEN parsing returns typed diagnostics
- AND the directive cannot change runtime mode or continuation state.

### Requirement: Profile-requested modes are policy inputs

r[agent_harness_runtime.zerostack_prompt_profiles.mode_binding] Profile-requested permission modes MUST flow through permission policy and MUST NOT grant authority, suppress required approvals, or weaken sandbox constraints.

#### Scenario: Code profile cannot grant write authority

r[agent_harness_runtime.zerostack_prompt_profiles.mode_binding.no_grant]
- GIVEN a code profile requests a write-capable posture
- AND authority or policy denies the target write action
- WHEN the runtime resolves the write request
- THEN the write remains denied or ask-gated according to permission policy
- AND the profile request is recorded only as posture metadata.

### Requirement: Chain-of-prompts continuation plans

r[agent_harness_runtime.zerostack_prompt_profiles.chain] Animus SHOULD support chain-of-prompts continuation plans that declare source profile, target profile, transition condition, user or host decision, and bounded additional instruction text.

#### Scenario: Chain transition is declined

r[agent_harness_runtime.zerostack_prompt_profiles.chain.declined]
- GIVEN a prompt profile has a configured transition to another profile
- WHEN the user or host declines the transition
- THEN the active profile does not change
- AND the declined transition is recorded as safe metadata rather than model-visible instruction.

### Requirement: Prompt profile receipts

r[agent_harness_runtime.zerostack_prompt_profiles.receipts] Prompt profile assembly SHOULD emit bounded receipts naming profile IDs, source identities, directive outcomes, chain decisions, prompt section hashes, and omitted-profile reasons without exposing raw secrets or unbounded profile bodies.

#### Scenario: Receipt uses hashes for sensitive profile body

r[agent_harness_runtime.zerostack_prompt_profiles.receipts.hashes]
- GIVEN a profile body is not safe to include in durable evidence
- WHEN a prompt profile receipt is emitted
- THEN the body is represented by a BLAKE3 hash and redaction marker
- AND source identity, profile ID, and decision reason remain reviewable.

### Requirement: Bounded Claude Code memory reference

r[agent_harness_runtime.claude_code_memory.scope] Animus MUST treat Claude Code memory as a bounded reference for project instructions, session preferences, and learned facts, and MUST NOT adopt Claude Code prompt text, hidden memory writes, provider-specific semantics, or `CLAUDE.md` as the canonical Animus storage contract.

#### Scenario: Compatibility import remains non-authoritative

r[agent_harness_runtime.claude_code_memory.scope.compat_import]
- GIVEN a host imports project guidance from a Claude Code-style file
- WHEN Animus prepares prompt context
- THEN the imported content is represented as typed Animus descriptors
- AND the original file format does not bypass trust, policy, validation, or redaction checks.

### Requirement: Typed instruction and memory descriptors

r[agent_harness_runtime.claude_code_memory.descriptors] Instruction and learned-memory inputs MUST be represented as typed descriptors with stable ID, source identity, trust scope, precedence class, freshness state, prompt-inclusion policy, and redaction metadata.

#### Scenario: Malformed descriptor is rejected

r[agent_harness_runtime.claude_code_memory.descriptors.invalid]
- GIVEN a memory descriptor is missing source identity, trust scope, or prompt-inclusion policy
- WHEN descriptor validation runs
- THEN validation returns a typed malformed-descriptor error
- AND the descriptor cannot influence prompt assembly.

### Requirement: Deterministic prompt assembly with attribution

r[agent_harness_runtime.claude_code_memory.prompt_assembly] Prompt assembly MUST deterministically order admitted instruction and memory descriptors by policy-defined precedence and MUST preserve source attribution for every included prompt section.

#### Scenario: Conflicting layers produce diagnostics

r[agent_harness_runtime.claude_code_memory.prompt_assembly.conflict]
- GIVEN trusted user, project, and session descriptors contain conflicting guidance for the same prompt section
- WHEN prompt assembly evaluates the descriptors
- THEN the selected precedence order is deterministic
- AND the assembly result includes a conflict diagnostic that names descriptor IDs without exposing redacted bodies.

### Requirement: Learned memory admission gate

r[agent_harness_runtime.claude_code_memory.admission] Learned memory candidates MUST require an injected host admission decision before they persist or influence future model prompts.

#### Scenario: Unadmitted candidate is not reused

r[agent_harness_runtime.claude_code_memory.admission.denied]
- GIVEN runtime proposes a memory candidate from a prior turn
- WHEN the host denies or omits admission
- THEN the candidate is not stored for future prompt influence
- AND the receipt records the denied admission status without embedding the raw candidate body.

### Requirement: Swappable memory adapter boundary

r[agent_harness_runtime.claude_code_memory.adapter_boundary] Memory discovery, storage, deletion, compatibility import, provider-assisted extraction, and admission MUST be represented as swappable injected adapter contracts that exchange typed descriptor, candidate, and admission DTOs; reusable core/runtime logic MUST NOT depend on a concrete memory database, file format, provider client, or host path layout.

#### Scenario: Memory backend swap preserves reusable behavior

r[agent_harness_runtime.claude_code_memory.adapter_boundary.swappable]
- GIVEN two hosts provide different memory source, store, importer, or extraction adapters
- WHEN they supply the same validated descriptors and admission outcomes to prompt assembly
- THEN reusable Animus logic produces equivalent prompt sections, diagnostics, and receipts
- AND missing adapters produce explicit unavailable outcomes rather than fallback ambient storage or discovery.

### Requirement: Safe memory influence receipts

r[agent_harness_runtime.claude_code_memory.receipts] Memory receipts MUST record safe metadata for prompt influence, including descriptor IDs, source labels, trust status, freshness state, inclusion status, redaction classes, and BLAKE3 hashes for redacted bodies, and MUST NOT include raw credentials, raw provider payloads, absolute path dumps, or unbounded prompt text.

#### Scenario: Secret-looking memory body is redacted

r[agent_harness_runtime.claude_code_memory.receipts.redacted]
- GIVEN an admitted memory descriptor body matches host redaction policy
- WHEN prompt-influence receipts are emitted
- THEN the receipt includes redaction class and BLAKE3 body hash metadata
- AND the raw body is absent from model-visible receipts and audit metadata.

### Requirement: Separate opt-in controls for memory use and generation

r[agent_harness_runtime.openai_codex_opt_in_memories.policy_controls] Animus memories MUST have independent opt-in controls for using existing memories and generating new memories at user, project, and thread scopes.

#### Scenario: Missing consent blocks memory generation

r[agent_harness_runtime.openai_codex_opt_in_memories.policy_controls.consent]
- GIVEN a thread does not allow memory generation
- WHEN the thread ends with content that would otherwise be memorable
- THEN Animus does not create a memory-generation request
- AND the report records memory generation as disabled for that thread scope.

### Requirement: Evidence-backed memory records

r[agent_harness_runtime.openai_codex_opt_in_memories.record_contract] Memory records MUST include kind, summary, scope, freshness, source thread identity, supporting evidence handles, redaction status, and confidence.

#### Scenario: Malformed memory record is rejected

r[agent_harness_runtime.openai_codex_opt_in_memories.record_contract.validation]
- GIVEN a generated memory record has no source thread identity or evidence handle
- WHEN Animus validates the memory record
- THEN validation rejects the record
- AND the memory is not eligible for future prompt injection.

### Requirement: Swappable memory storage and extraction boundary

r[agent_harness_runtime.openai_codex_opt_in_memories.adapter_boundary] Memory storage, retrieval, search, update/delete, and provider-assisted extraction MUST be represented as swappable injected adapter contracts that exchange typed memory records, generation requests, extraction outcomes, and availability statuses; reusable core/runtime logic MUST NOT depend on a concrete memory database, vector index, provider extractor, or host path layout.

#### Scenario: Alternate backend preserves memory semantics

r[agent_harness_runtime.openai_codex_opt_in_memories.adapter_boundary.swappable]
- GIVEN two hosts back memories with different storage or extraction implementations
- WHEN the same validated memory records, extraction outcomes, and policy decisions reach reusable logic
- THEN generation eligibility, prompt injection, and ledgers use only typed adapter DTOs and produce equivalent reusable outcomes
- AND missing adapters produce explicit unavailable outcomes rather than ambient fallback storage, search, or extraction.

### Requirement: Conservative memory generation eligibility

r[agent_harness_runtime.openai_codex_opt_in_memories.generation_eligibility] Memory generation MUST be denied for threads without consent, disallowed external context, secret-bearing content, failed redaction, or exhausted memory-generation budget.

#### Scenario: External context prevents memory generation

r[agent_harness_runtime.openai_codex_opt_in_memories.generation_eligibility.exclusions]
- GIVEN a thread consumed external context that policy marks ineligible for persistence
- WHEN memory generation is considered
- THEN Animus skips memory generation for that thread
- AND the skip reason is recorded without storing the external context body.

### Requirement: Bounded memory injection ledger

r[agent_harness_runtime.openai_codex_opt_in_memories.injection_ledger] Prompt assembly MUST record included, skipped, expired, redacted, and over-budget memories with source attribution and bounded content.

#### Scenario: Memory injection is bounded

r[agent_harness_runtime.openai_codex_opt_in_memories.injection_ledger.bounded]
- GIVEN available memories exceed the configured memory context budget
- WHEN Animus assembles prompt context
- THEN only policy-selected bounded memory summaries are injected
- AND skipped memories are recorded by identity and reason.

### Requirement: Memories are non-authoritative context

r[agent_harness_runtime.openai_codex_opt_in_memories.non_authoritative] Animus MUST treat memories as optional context and MUST keep required rules in guidance, policy, or checked-in contracts.

#### Scenario: Disabled memories do not disable required rules

r[agent_harness_runtime.openai_codex_opt_in_memories.non_authoritative.guidance]
- GIVEN memories are disabled for a run
- WHEN prompt assembly loads required project guidance and policy
- THEN those required rules remain active
- AND no memory state is required for the rules to apply.

### Requirement: Bounded MCP bridge reference

r[agent_harness_runtime.claude_code_mcp.scope] Animus MUST treat Claude Code MCP support as a bounded interoperability reference, and MUST NOT make MCP a core dependency, auto-discover or auto-trust servers, execute network or process transports from reusable core crates, or replace Animus-owned tool contracts.

#### Scenario: Untrusted MCP server is not advertised

r[agent_harness_runtime.claude_code_mcp.scope.untrusted]
- GIVEN a host discovers or names an MCP server without a trust decision and explicit registration
- WHEN tool registry construction runs
- THEN the server's tools are not advertised to the model
- AND no server process, network connection, or transport probe is started by reusable runtime code.

### Requirement: MCP descriptors map into Animus tools

r[agent_harness_runtime.claude_code_mcp.mapping] MCP server and tool descriptors MUST be converted into Animus tool definitions with name, schema identity, capability labels, source metadata, result bounds, and resource/ability policy references before dispatch.

#### Scenario: Incomplete MCP mapping is rejected

r[agent_harness_runtime.claude_code_mcp.mapping.incomplete]
- GIVEN an MCP tool descriptor lacks strict schema identity, capability labels, result bounds, source metadata, or policy resource mapping
- WHEN bridge mapping validation runs
- THEN validation returns typed diagnostics
- AND the tool is not registered as an Animus tool.

### Requirement: MCP dispatch follows normal permission and capability checks

r[agent_harness_runtime.claude_code_mcp.dispatch] MCP-backed tool dispatch MUST pass Animus argument validation, permission resolution, capability authorization, mutation coordination, and result-bound checks before the response is accepted.

#### Scenario: Denied MCP capability prevents call

r[agent_harness_runtime.claude_code_mcp.dispatch.denied]
- GIVEN an MCP-backed tool maps to a network or filesystem capability denied by current policy
- WHEN the model requests that tool
- THEN dispatch returns a typed denied-capability failure
- AND the MCP server adapter is not called.

### Requirement: MCP transport lifecycle is host-owned

r[agent_harness_runtime.claude_code_mcp.transport] MCP process management, sockets, network access, authentication, server discovery, retries, clocks, and cancellation MUST remain host-owned adapter behavior with typed unavailable, unsupported, denied, timeout, and malformed-response failures.

#### Scenario: Unsupported MCP transport fails safely

r[agent_harness_runtime.claude_code_mcp.transport.unsupported]
- GIVEN a registered MCP tool requires a transport unsupported by the selected host adapter
- WHEN dispatch attempts to prepare the call
- THEN runtime reports an unsupported-transport failure
- AND no fallback shell command, network probe, or provider call is attempted.

### Requirement: Safe MCP bridge receipts

r[agent_harness_runtime.claude_code_mcp.receipts] MCP bridge receipts MUST include safe metadata such as server label, tool name, capability labels, schema identity, status class, redacted resource identity, truncation status, and BLAKE3 hashes, and MUST NOT include raw transport payloads, credentials, full paths, provider payloads, or unbounded external results by default.

#### Scenario: Malformed MCP response is redacted

r[agent_harness_runtime.claude_code_mcp.receipts.malformed]
- GIVEN an MCP server returns a malformed or oversized response
- WHEN the bridge records the failure
- THEN the receipt records status class, schema identity, truncation state, and safe hashes
- AND raw transport payloads and sensitive fields are omitted from receipts and model-visible context.

### Requirement: Bounded Sandlock adaptation scope

r[agent_harness_runtime.sandlock_sandbox.scope] Animus MUST treat Sandlock as a bounded reference for Linux sandbox-enforcement patterns, including Landlock-style filesystem and network constraints, seccomp-bpf syscall filtering, seccomp user-notification supervision, COW workdir staging, resource-limit supervision, deterministic execution controls, and network/HTTP ACL intent, and MUST NOT vendor Sandlock, require Sandlock as a runtime dependency, copy its CLI/profile format as the Animus descriptor contract, or expose Linux kernel APIs from `animus-core` as part of this adaptation.

#### Scenario: Scope review rejects dependency coupling

r[agent_harness_runtime.sandlock_sandbox.scope.review]
- GIVEN a proposed change vendors Sandlock, shells out to the Sandlock CLI from core crates, copies Sandlock profile TOML as the Animus descriptor format, or exposes Landlock/seccomp implementation details through `animus-core`
- WHEN maintainers compare it with the bounded adaptation scope
- THEN the change is rejected or split until the portable Animus descriptor boundary and std-shell enforcement boundary are restored
- AND the review names the Sandlock behavior only as reference evidence.

### Requirement: Portable sandbox descriptors

r[agent_harness_runtime.sandlock_sandbox.descriptor] Animus sandbox descriptors MUST be represented as typed portable execution constraints covering filesystem read/write intent, workdir and COW write disposition, process/resource limits, environment visibility, network egress and bind policy, optional HTTP method/host/path policy, deterministic time/randomness controls, and whether enforcement is required before host dispatch.

#### Scenario: Descriptor captures intent without kernel details

r[agent_harness_runtime.sandlock_sandbox.descriptor.intent]
- GIVEN a guarded process or tool action requires sandboxed execution
- WHEN the runtime receives the sandbox descriptor
- THEN the descriptor names portable constraint intent rather than raw seccomp programs, Landlock ABI values, kernel constants, or Sandlock CLI flags
- AND missing, empty, duplicate, or unknown required fields fail descriptor validation before dispatch.

### Requirement: Pure descriptor normalization

r[agent_harness_runtime.sandlock_sandbox.normalization] Descriptor parsing, normalization, conflict detection, unsupported-field diagnostics, and receipt-safe projection MUST be pure deterministic logic over typed inputs and MUST NOT read the filesystem, inspect the environment, spawn processes, consult clocks, probe kernels, or mutate host state.

#### Scenario: Conflicting descriptor fails in the core path

r[agent_harness_runtime.sandlock_sandbox.normalization.conflict]
- GIVEN a descriptor combines mutually exclusive network allowlist and denylist modes or combines incompatible COW commit and dry-run write dispositions
- WHEN descriptor normalization runs
- THEN it returns typed diagnostics naming the conflicting constraint classes
- AND no platform adapter, process executor, filesystem operation, network operation, or kernel probe is invoked.

### Requirement: Policy-reviewed sandbox constraints

r[agent_harness_runtime.sandlock_sandbox.policy] Sandbox descriptor fields MUST be checked against Nickel-authored policy fixtures or contracts before dispatch so unknown, denied, or out-of-profile constraint classes fail before platform enforcement begins.

#### Scenario: Policy denies unsupported constraint class

r[agent_harness_runtime.sandlock_sandbox.policy.denied]
- GIVEN a descriptor requests HTTP interception, COW commit, broad filesystem write access, or unrestricted network egress
- AND the active policy fixture does not allow that constraint class for the requested capability
- WHEN policy validation evaluates the normalized descriptor
- THEN validation returns a typed policy-denied sandbox reason
- AND host execution does not start.

### Requirement: Platform capability translation

r[agent_harness_runtime.sandlock_sandbox.platform] The std-shell sandbox adapter SHOULD expose an explicit platform capability model for translating normalized descriptors into enforceable constraints, including Linux capability labels inspired by Landlock, seccomp-bpf, seccomp user notification, COW staging, resource supervision, and HTTP/network ACL support when those mechanisms are available.

#### Scenario: Linux adapter reports enforceable capability set

r[agent_harness_runtime.sandlock_sandbox.platform.linux_matrix]
- GIVEN a normalized descriptor requests filesystem read/write limits, TCP egress allowlisting, process limits, and dry-run COW staging
- WHEN the Linux std-shell adapter evaluates platform support
- THEN it reports the capability labels it can enforce for the selected descriptor
- AND any descriptor field without a matching platform capability is returned as an unsupported-field diagnostic rather than being silently ignored.

### Requirement: Required sandbox enforcement fails closed

r[agent_harness_runtime.sandlock_sandbox.fail_closed] Runtime dispatch MUST fail closed before host execution when a required sandbox descriptor is malformed, denied by policy, unsupported by the selected platform adapter, or only partially enforceable, and MUST NOT treat sandbox enforcement as authorization proof.

#### Scenario: Required sandbox cannot be partially relaxed

r[agent_harness_runtime.sandlock_sandbox.fail_closed.partial]
- GIVEN a capability request is already authorized by the relevant contract and grant checks
- AND its local caveat marks a sandbox descriptor as required
- AND the selected platform adapter can enforce filesystem constraints but cannot enforce the requested network, HTTP, COW, process, environment, or determinism constraint
- WHEN dispatch is evaluated
- THEN the runtime denies dispatch before host execution with a typed sandbox-unavailable or partial-enforcement issue
- AND the receipt records sandbox failure separately from authorization success.

### Requirement: Redacted sandbox receipts

r[agent_harness_runtime.sandlock_sandbox.receipts] Sandbox dispatch receipts MUST carry safe metadata only, including sandbox status, issue code, normalized descriptor hash, platform capability labels, enforcement outcome, bounded COW summary counts or BLAKE3 hashes, bounded output hashes, and BLAKE3 receipt hash, and MUST NOT include raw prompts, credentials, absolute path dumps, raw network payloads, raw file contents, raw process output, or unbounded event streams.

#### Scenario: Successful sandbox receipt is reviewable

r[agent_harness_runtime.sandlock_sandbox.receipts.success]
- GIVEN a sandboxed dispatch completes with required filesystem, process, network, COW, and environment constraints enforced
- WHEN the dispatch receipt is emitted
- THEN it records the sandbox status, normalized descriptor hash, enforced capability labels, bounded COW/output evidence, and receipt hash
- AND sensitive path, payload, credential, prompt, file, and process-output material is omitted or represented only by approved hashes and redaction markers.

#### Scenario: Receipt validation rejects sandbox leaks

r[agent_harness_runtime.sandlock_sandbox.receipts.rejects_leaks]
- GIVEN a sandbox receipt fixture contains raw prompts, credential-looking values, absolute path dumps, raw network payloads, raw file contents, raw process output, or an unbounded event stream
- WHEN receipt validation runs
- THEN validation fails with a typed redaction error
- AND the receipt cannot be used as review or release evidence.

### Requirement: Focused verification for sandbox enforcement

r[agent_harness_runtime.sandlock_sandbox.verification] The Sandlock-inspired adaptation MUST include focused positive and negative tests for descriptor normalization, policy validation, platform capability translation, fail-closed dispatch, COW/dry-run evidence projection, and receipt redaction.

#### Scenario: Positive and negative fixtures gate closeout

r[agent_harness_runtime.sandlock_sandbox.verification.fixtures]
- GIVEN the Sandlock-inspired sandbox adaptation is ready for review
- WHEN focused verification runs
- THEN positive fixtures show normalized descriptors, allowed filesystem and network constraints, platform capability translation, dry-run or COW summary projection, and redacted successful receipts
- AND negative fixtures show malformed descriptors, conflicting network rules, policy-denied constraints, unsupported platform capabilities, partial enforcement attempts, and receipt leak attempts fail closed.

r[agent_harness_runtime.sandlock_sandbox.closeout] Closeout evidence MUST include successful focused descriptor/platform/sandbox/receipt tests, workspace checks, Cairn validation, and Cairn proposal/design/tasks gates before this adaptation is archived.

#### Scenario: Closeout evidence is reviewable

r[agent_harness_runtime.sandlock_sandbox.closeout.evidence]
- GIVEN maintainers intend to archive the Sandlock-inspired sandbox adaptation
- WHEN they collect closeout evidence
- THEN the evidence names every successful focused rail, workspace check, and Cairn gate
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: VCS-neutral isolated task workspaces

r[agent_harness_runtime.openai_codex_worktree_automations.isolated_workspace] Animus isolated task workspaces MUST be represented through VCS-neutral source identity, workspace identity, writable roots, snapshot identity, restore metadata, and adapter capability records.

#### Scenario: Workspace adapter is not Git-specific

r[agent_harness_runtime.openai_codex_worktree_automations.isolated_workspace.vcs_neutral]
- GIVEN a workspace adapter reports Jujutsu-oriented capabilities and no Git worktree capability
- WHEN Animus validates an isolated workspace run
- THEN validation uses the reported capabilities rather than requiring a Git worktree
- AND the run is rejected only if a required capability is missing.

### Requirement: Typed automation run contract

r[agent_harness_runtime.openai_codex_worktree_automations.automation_contract] Animus automations MUST define scheduled standalone runs and thread-bound wakeups with prompt, cadence, workspace policy, sandbox profile, and stop criteria.

#### Scenario: Unsafe automation configuration is rejected

r[agent_harness_runtime.openai_codex_worktree_automations.automation_contract.validation]
- GIVEN an unattended automation requests full-access sandboxing without an allowed policy profile
- WHEN Animus validates the automation definition
- THEN validation rejects the automation
- AND no scheduled run is registered.

### Requirement: Snapshot-based thread handoff

r[agent_harness_runtime.openai_codex_worktree_automations.handoff_snapshot] Moving a thread between foreground and isolated workspace contexts MUST create a snapshot record that identifies source state, target state, included files, and conflict status.

#### Scenario: Dirty-state conflict blocks handoff

r[agent_harness_runtime.openai_codex_worktree_automations.handoff_snapshot.conflict]
- GIVEN the foreground context and isolated workspace both changed the same tracked resource
- WHEN a handoff request attempts to merge the isolated context into the foreground
- THEN the handoff is blocked with a conflict outcome
- AND the snapshot record preserves enough evidence for manual review.

### Requirement: Explicit ignored-file inclusion

r[agent_harness_runtime.openai_codex_worktree_automations.ignored_file_include] Ignored or local-only files MUST be materialized into isolated workspaces only through explicit allowlist entries with provenance and redaction classification.

#### Scenario: Ignored secret is not copied implicitly

r[agent_harness_runtime.openai_codex_worktree_automations.ignored_file_include.secrets]
- GIVEN a local ignored file is classified as secret-bearing
- WHEN an isolated workspace is created without an allowlist entry and redaction approval for that file
- THEN the file is not copied
- AND the workspace manifest records a copy-denied entry without exposing the secret body.

### Requirement: Automation cleanup and restore evidence

r[agent_harness_runtime.openai_codex_worktree_automations.cleanup_reporting] Automation and isolated-workspace cleanup MUST record findings, no-op outcomes, failed runs, workspace cleanup, and restore-available states with bounded manifest evidence.

#### Scenario: Cleanup preserves restore manifest

r[agent_harness_runtime.openai_codex_worktree_automations.cleanup_reporting.restore]
- GIVEN an isolated workspace is eligible for cleanup
- WHEN cleanup removes the workspace materialization
- THEN Animus records manifest hashes, source identity, and restore metadata first
- AND later inspection can report that restore evidence is available.

### Requirement: Typed study contract

r[agent_harness_runtime.hermes_researchspec.study_contract] Animus MUST define a typed ResearchSpec-inspired study contract with study ID, objective, primary metric, metric direction, threshold or noise policy, workload command identity, correctness checks, files in scope, off-limits files, guarded measurement files, promotion thresholds, and provenance.

#### Scenario: Valid study contract is ready for baseline

r[agent_harness_runtime.hermes_researchspec.study_contract.valid]
- GIVEN a host supplies a complete typed study contract with objective, metric, workload, checks, scope, off-limits files, guardrails, and promotion policy
- WHEN study validation runs
- THEN the study is accepted as ready for baseline measurement
- AND a fresh agent can discover the required lifecycle state without relying on conversation context.

#### Scenario: Incomplete study contract is rejected

r[agent_harness_runtime.hermes_researchspec.study_contract.incomplete]
- GIVEN a study contract is missing a primary metric, metric direction, workload command identity, correctness checks declaration, scope, or off-limits declaration
- WHEN study validation runs
- THEN validation fails before any candidate run can be kept
- AND the diagnostic names the missing field.

### Requirement: Baseline evidence is immutable and fingerprinted

r[agent_harness_runtime.hermes_researchspec.baseline] Animus MUST require a baseline record before candidate runs can be evaluated, including primary metric value, secondary metrics when configured, check status, config snapshot, workload fingerprints, guarded input fingerprints, and BLAKE3 hashes unless an interoperability constraint requires another algorithm.

#### Scenario: Baseline records guarded fingerprints

r[agent_harness_runtime.hermes_researchspec.baseline.fingerprints]
- GIVEN a valid study and successful workload/check execution reported by the host
- WHEN baseline evidence is recorded
- THEN the baseline includes metric values, check status, config snapshot, guarded file/input fingerprints, and BLAKE3 hashes
- AND later candidate gates can detect guarded measurement drift.

#### Scenario: Missing primary metric blocks baseline

r[agent_harness_runtime.hermes_researchspec.baseline.missing_metric]
- GIVEN the workload completes but host-reported output lacks the configured primary metric
- WHEN baseline validation runs
- THEN the baseline fails closed
- AND no candidate run can be marked kept against that baseline.

### Requirement: Hypotheses are first-class artifacts

r[agent_harness_runtime.hermes_researchspec.hypotheses] Animus MUST track optimization hypotheses with stable IDs, claims, planned interventions, expected signals, falsification criteria, risk notes, linked runs, and final decisions.

#### Scenario: Vague hypothesis is rejected

r[agent_harness_runtime.hermes_researchspec.hypotheses.vague]
- GIVEN a hypothesis only says to try random tweaks or lacks expected signal and falsification criteria
- WHEN hypothesis validation runs
- THEN validation rejects it as not actionable
- AND the diagnostic requests a concrete claim, intervention, expected signal, and discard condition.

### Requirement: Run evidence is complete and resumable

r[agent_harness_runtime.hermes_researchspec.runs] Every run record MUST include study ID, hypothesis ID, run timestamp or sequence metadata, candidate revision or diff fingerprint, primary metric, secondary metrics when configured, status, check status, changed-file summary, side information, rollback reason for discarded or failed runs, and next-action hint.

#### Scenario: Failed run without rollback context is incomplete

r[agent_harness_runtime.hermes_researchspec.runs.failed_requires_context]
- GIVEN a candidate run crashes, fails checks, or is discarded
- WHEN the run record lacks rollback reason or next-action hint
- THEN run validation reports the record as incomplete
- AND the study cannot be archived as complete until the missing learning is added or explicitly waived.

### Requirement: Keep and promotion gates defend measurement integrity

r[agent_harness_runtime.hermes_researchspec.keep_promotion] A kept run MUST satisfy the configured metric comparison rule, required checks, threshold or noise policy, hypothesis traceability, guarded-file drift policy, and measurement-integrity guardrails before promotion eligibility is reported.

#### Scenario: Suspicious metric improvement is blocked

r[agent_harness_runtime.hermes_researchspec.keep_promotion.suspicious]
- GIVEN a candidate improves the primary metric but weakens checks, changes metric extraction, excludes benchmark inputs, hardcodes workload inputs, defers measured work, or edits guarded files without approved instrumentation-change metadata
- WHEN the keep gate evaluates the run
- THEN the run is rejected or blocked from promotion
- AND the report classifies the issue as a measurement-integrity risk with concrete evidence labels.

#### Scenario: Promotion groups detect overlap

r[agent_harness_runtime.hermes_researchspec.keep_promotion.overlap]
- GIVEN multiple kept runs are proposed as independent promotion groups
- WHEN two groups touch the same file or one group depends on an API introduced by another group
- THEN promotion evaluation reports the overlap or dependency
- AND the groups are merged or marked dependent before finalization.

### Requirement: Archive preserves study conclusions

r[agent_harness_runtime.hermes_researchspec.archive] Completed studies MUST archive objective, baseline, best result, kept changes, discarded hypotheses, final metric delta, unresolved ideas, downstream lifecycle links, and run evidence needed to audit keep/discard decisions.

#### Scenario: Unresolved decisions block archive

r[agent_harness_runtime.hermes_researchspec.archive.unresolved]
- GIVEN a study has linked runs without hypothesis decisions or in-scope ideas without keep, discard, supersede, defer, or waive decisions
- WHEN archive validation runs
- THEN archival is rejected
- AND the report lists unresolved hypotheses, runs, or ideas.

### Requirement: ResearchSpec adaptation verification

r[agent_harness_runtime.hermes_researchspec.verification] The ResearchSpec adaptation MUST include focused tests for study contract validation, baseline evidence, BLAKE3 fingerprints, missing metric rejection, hypothesis validation, run evidence completeness, keep/promotion integrity gates, promotion overlap detection, archive rejection, and optional host-adapter isolation.

#### Scenario: Closeout evidence covers lifecycle gates

r[agent_harness_runtime.hermes_researchspec.verification.closeout]
- GIVEN maintainers intend to archive the Hermes ResearchSpec adaptation
- WHEN closeout validation runs
- THEN evidence names focused study tests, baseline tests, hypothesis/run tests, keep/promotion tests, archive tests, workspace checks, Cairn validation, and Cairn gates
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Reviewed scaffold profile contract

r[agent_harness_runtime.ornith_scaffold.profile_contract] Animus MUST represent runtime scaffold strategies as typed reviewed data with stable profile identity, revision identity, task-class labels, supported strategy knobs, provenance, review status, route constraints, and BLAKE3 content hashes.

#### Scenario: Invalid scaffold profile fails closed

r[agent_harness_runtime.ornith_scaffold.profile_contract.invalid]
- GIVEN a scaffold profile fixture contains malformed IDs, duplicate active revisions, unsupported strategy knobs, missing provenance, an unreviewed active revision, or a mismatched content hash
- WHEN scaffold profile validation runs
- THEN validation returns a typed scaffold-profile error
- AND the profile is not eligible for prompt, Steel, runtime, or evaluation influence.

### Requirement: Deterministic scaffold selection

r[agent_harness_runtime.ornith_scaffold.selection] Animus MUST select scaffold profiles through pure deterministic planning over in-memory profiles, task-class input, host policy, and route capability labels before task execution begins.

#### Scenario: Route policy controls scaffold eligibility

r[agent_harness_runtime.ornith_scaffold.selection.policy]
- GIVEN accepted scaffold profiles have task-class labels and route capability constraints
- WHEN scaffold selection evaluates a task and model route
- THEN the selected profile is compatible with both the task class and route policy
- AND disabled, stale, or incompatible profiles are rejected with deterministic diagnostics.

### Requirement: Host-admitted scaffold revision proposals

r[agent_harness_runtime.ornith_scaffold.admission] Animus MAY allow runtime or evaluation flows to propose scaffold revisions, but proposed revisions MUST require an injected host admission outcome before persistence, activation, or future prompt influence.

#### Scenario: Unreviewed revision cannot influence future turns

r[agent_harness_runtime.ornith_scaffold.admission.unreviewed]
- GIVEN a model or evaluator proposes a new scaffold revision
- WHEN the host admission service is absent, denies the proposal, or reports a stale base revision
- THEN Animus records the unavailable, denied, or stale outcome
- AND the proposed revision is not persisted, activated, or included in future prompt/Steel inputs.

### Requirement: Scaffold influence receipts

r[agent_harness_runtime.ornith_scaffold.receipts] Runtime and evaluation receipts MUST record scaffold influence using safe metadata such as profile ID, revision ID, task class, review status, route labels, validation outcome, and BLAKE3 hashes, and MUST NOT include raw prompt bodies, scaffold bodies, provider payloads, credentials, full paths, raw tool bodies, or unbounded outputs.

#### Scenario: Scaffold receipt rejects sensitive bodies

r[agent_harness_runtime.ornith_scaffold.receipts.redacted]
- GIVEN a scaffold influence receipt fixture contains a raw prompt, scaffold body, provider payload, credential-looking value, absolute path dump, or unbounded tool output
- WHEN receipt validation runs
- THEN validation fails with a typed redaction diagnostic
- AND the receipt is not accepted as runtime or evaluation evidence.

### Requirement: Route-neutral scaffold scope

r[agent_harness_runtime.ornith_scaffold.scope] Animus scaffold profiles MUST remain route-neutral reviewed strategy data and MUST NOT require reusable runtime code to depend on a concrete provider, model family, training harness, or executable model-authored scaffold.

#### Scenario: Scaffold scope excludes executable harnesses

r[agent_harness_runtime.ornith_scaffold.scope.executable_rejected]
- GIVEN a scaffold profile attempts to include executable code, shell commands, provider-specific client construction, or verifier mutation instructions
- WHEN scaffold profile validation evaluates the profile
- THEN validation rejects the profile before runtime use
- AND no model, tool, provider, filesystem, process, or network adapter is called.

### Requirement: Scaffold adaptation verification

r[agent_harness_runtime.ornith_scaffold.verification] The Ornith scaffold adaptation MUST include focused positive and negative tests for profile validation, selection policy, admission outcomes, receipt redaction, route-neutral scope, and executable-harness rejection before archive.

#### Scenario: Closeout evidence covers scaffold profile behavior

r[agent_harness_runtime.ornith_scaffold.verification.closeout]
- GIVEN maintainers intend to archive the scaffold profile adaptation
- WHEN closeout validation runs
- THEN evidence names focused scaffold, admission, receipt, workspace, roadmap, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Immutable outer trust boundary

r[agent_harness_runtime.ornith_trust_boundary.outer_boundary] Animus MUST represent the outer trust boundary as immutable typed data covering sanctioned tools, withheld path labels, verifier artifact identity, sandbox profile identity, approval requirements, evidence policy, and BLAKE3 policy hashes before scaffold, Steel, model, or tool execution can claim success.

#### Scenario: Invalid boundary descriptor fails closed

r[agent_harness_runtime.ornith_trust_boundary.outer_boundary.invalid]
- GIVEN a trust-boundary descriptor has a missing verifier hash, duplicate path label, empty sanctioned-tool set, raw sensitive path serialization, or a request to mutate boundary authority from inside the runtime loop
- WHEN boundary validation runs
- THEN validation returns a typed trust-boundary error
- AND no scaffold, model, tool, verifier, or evidence acceptance path is invoked.

### Requirement: Deterministic boundary monitor

r[agent_harness_runtime.ornith_trust_boundary.monitor] Animus MUST provide a deterministic monitor that evaluates planned and observed action descriptors against the trust boundary and returns typed issue codes for exact violations before evidence is accepted.

#### Scenario: Boundary violations deny trajectory evidence

r[agent_harness_runtime.ornith_trust_boundary.monitor.violations]
- GIVEN an action descriptor attempts a withheld-path read, verifier modification, unsanctioned tool call, sandbox weakening, approval suppression, or evidence-policy bypass
- WHEN the deterministic monitor evaluates the action
- THEN the monitor returns the matching violation issue code
- AND the trajectory is not accepted as successful runtime, batch, or evaluation evidence.

### Requirement: Judge veto cannot grant authority

r[agent_harness_runtime.ornith_trust_boundary.judge_veto] Animus MAY accept a host-provided judge veto for suspicious intent, but judge output MUST be reject-only and MUST NOT override deterministic monitor denials, grant permissions, mutate boundary descriptors, suppress approvals, or mark denied evidence successful.

#### Scenario: Judge approval cannot override monitor denial

r[agent_harness_runtime.ornith_trust_boundary.judge_veto.no_override]
- GIVEN the deterministic monitor denies a trajectory for a trust-boundary violation
- AND a judge output labels the trajectory acceptable
- WHEN policy binding combines monitor and judge outcomes
- THEN the denial remains final
- AND the combined outcome records that judge output cannot grant authority.

### Requirement: Anti-gaming fixture coverage

r[agent_harness_runtime.ornith_trust_boundary.anti_gaming] Animus MUST maintain explicit anti-gaming fixture classes for oracle copying, hardcoded expected outputs or artifacts, visible-test overfitting, verifier edits, and allowed-tool intent gaming.

#### Scenario: Anti-gaming fixtures are reproducible

r[agent_harness_runtime.ornith_trust_boundary.anti_gaming.fixtures]
- GIVEN anti-gaming fixtures contain both allowed work patterns and gaming attempts
- WHEN focused monitor validation runs
- THEN allowed patterns pass with safe evidence
- AND gaming attempts fail with deterministic issue codes without depending on hidden provider calls.

### Requirement: Redacted boundary violation receipts

r[agent_harness_runtime.ornith_trust_boundary.receipts] Trust-boundary violation receipts MUST expose safe metadata such as policy hash, issue code, sanitized labels, counts, and redaction status, and MUST NOT include hidden paths, raw verifier bodies, prompts, tool inputs, tool outputs, provider payloads, credentials, or full path dumps.

#### Scenario: Boundary receipt rejects leaks

r[agent_harness_runtime.ornith_trust_boundary.receipts.redacted]
- GIVEN a boundary violation receipt fixture contains hidden paths, raw verifier bodies, prompt text, tool input/output bodies, provider payloads, credential-looking values, or full path dumps
- WHEN receipt validation runs
- THEN validation fails with a typed redaction diagnostic
- AND the receipt is not accepted as review or release evidence.

### Requirement: Trust-boundary monitor verification

r[agent_harness_runtime.ornith_trust_boundary.verification] The trust-boundary monitor adaptation MUST include focused positive and negative tests for boundary validation, monitor denials, judge-veto binding, anti-gaming fixtures, receipt redaction, workspace checks, roadmap validation, Cairn validation, and Cairn gates before archive.

#### Scenario: Closeout evidence covers boundary monitor behavior

r[agent_harness_runtime.ornith_trust_boundary.verification.closeout]
- GIVEN maintainers intend to archive the trust-boundary monitor adaptation
- WHEN closeout validation runs
- THEN evidence names focused monitor, judge-veto, anti-gaming, receipt, workspace, roadmap, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Offline scaffold evaluation plans

r[agent_harness_runtime.ornith_eval_feedback.plan] Animus MUST represent offline scaffold evaluation as deterministic plan data with task-set identity, scaffold revision ID and hash, model route, verifier identity and hash, trust-boundary policy hash, tool-surface hash, sampling profile, resource labels, run count, and output format before execution.

#### Scenario: Invalid evaluation plan fails closed

r[agent_harness_runtime.ornith_eval_feedback.plan.invalid]
- GIVEN an evaluation plan contains a remote task-set location, missing scaffold hash, missing verifier hash, unsupported output format, non-positive run count, or provider-specific public benchmark claim
- WHEN plan validation runs
- THEN validation returns a typed evaluation-plan error
- AND no batch, model, verifier, tool, filesystem, process, or network execution is started.

### Requirement: Redacted evaluation receipts

r[agent_harness_runtime.ornith_eval_feedback.receipts] Scaffold evaluation receipts MUST expose safe comparison metadata such as model ID, scaffold hash, trust-boundary hash, tool-surface hash, verifier hash, context-window class, sampling labels, timeout/resource labels, run count, aggregate status, and redaction state, and MUST NOT include raw prompts, hidden tests, provider payloads, credentials, full paths, verifier bodies, or unbounded tool outputs.

#### Scenario: Evaluation receipt rejects transcript leaks

r[agent_harness_runtime.ornith_eval_feedback.receipts.redacted]
- GIVEN an evaluation receipt fixture contains raw prompt text, hidden test content, provider payloads, credential-looking values, absolute path dumps, verifier bodies, or unbounded tool outputs
- WHEN receipt validation runs
- THEN validation fails with a typed redaction diagnostic
- AND the receipt is not accepted as scaffold comparison evidence.

### Requirement: Reviewed feedback candidates

r[agent_harness_runtime.ornith_eval_feedback.review] Evaluation feedback MUST be represented as non-authoritative candidates with explicit host review outcomes for promote, demote, revise, investigate, denied, unavailable, and stale-evidence states.

#### Scenario: Feedback cannot auto-activate a scaffold

r[agent_harness_runtime.ornith_eval_feedback.review.no_auto_accept]
- GIVEN a feedback candidate is unreviewed, denied, unavailable, or based on stale evidence
- WHEN scaffold admission is evaluated
- THEN the candidate cannot activate a scaffold revision
- AND it cannot change runtime selection policy without a separate host-owned admission decision.

### Requirement: Evaluation evidence freshness policy

r[agent_harness_runtime.ornith_eval_feedback.freshness] Animus MUST classify feedback evidence freshness by comparing model route, scaffold hash, verifier hash, trust-boundary hash, tool-surface hash, runtime version label, and sampling profile against current policy.

#### Scenario: Stale or mismatched feedback is downweighted

r[agent_harness_runtime.ornith_eval_feedback.freshness.stale]
- GIVEN feedback evidence was produced for an older scaffold hash, verifier hash, tool-surface hash, runtime label, or sampling profile
- WHEN freshness classification runs
- THEN the evidence is marked stale or reduced-weight according to policy
- AND it cannot independently justify scaffold promotion.

### Requirement: Offline-only evaluation feedback scope

r[agent_harness_runtime.ornith_eval_feedback.scope] The scaffold evaluation feedback adaptation MUST remain local and offline and MUST reject RL training, live self-mutation, cloud benchmark runners, hidden-test access, provider-specific evaluation APIs, and public benchmark claims.

#### Scenario: Live self-improvement workflow is rejected

r[agent_harness_runtime.ornith_eval_feedback.scope.live_mutation_rejected]
- GIVEN an evaluation workflow requests live model fine-tuning, automatic scaffold activation, cloud benchmark submission, hidden tests, or provider evaluation APIs
- WHEN runtime validation evaluates the workflow metadata
- THEN validation rejects the workflow as out of scope
- AND no runtime mutation, network dispatch, or scaffold activation occurs.

### Requirement: Evaluation feedback verification

r[agent_harness_runtime.ornith_eval_feedback.verification] The adaptation MUST include focused positive and negative tests for local plans, invalid plans, redacted receipts, receipt leaks, reviewed feedback candidates, no-auto-accept behavior, freshness classification, stale evidence, and offline-only scope rejection.

#### Scenario: Closeout evidence covers evaluation feedback fixtures

r[agent_harness_runtime.ornith_eval_feedback.verification.closeout]
- GIVEN maintainers intend to archive the offline evaluation feedback adaptation
- WHEN closeout validation runs
- THEN evidence names focused plan, receipt, review, freshness, scope, workspace, roadmap, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Configurable compaction policy

r[agent_harness_runtime.pi_compaction.policy] Animus compaction MUST be governed by typed policy inputs for context budget, response reserve budget, kept-recent budget, summary format, summarizer selection, and redaction behavior.

#### Scenario: Policy drives compaction decision

r[agent_harness_runtime.pi_compaction.policy.trigger]
- GIVEN a replay context estimate exceeds the configured usable budget
- WHEN runtime evaluates compaction readiness
- THEN it prepares a compaction plan according to policy
- AND it does not compact when policy disables automatic compaction.

### Requirement: Safe compaction cut points

r[agent_harness_runtime.pi_compaction.cut_points] Compaction preparation MUST choose cut points that preserve valid model-visible message ordering and MUST NOT orphan tool results from their corresponding tool calls.

#### Scenario: Tool result boundary is rejected

r[agent_harness_runtime.pi_compaction.cut_points.reject_tool_result]
- GIVEN a candidate cut point would keep a tool result while removing the tool call that produced it
- WHEN compaction preparation evaluates the candidate
- THEN the candidate is rejected or moved to a valid boundary
- AND the resulting context remains structurally valid.

### Requirement: Split-turn compaction handling

r[agent_harness_runtime.pi_compaction.split_turn] Animus MUST represent split-turn compaction explicitly when a single turn exceeds the kept-recent budget, summarizing the early turn prefix without corrupting the kept suffix.

#### Scenario: Oversized turn gets prefix summary

r[agent_harness_runtime.pi_compaction.split_turn.prefix]
- GIVEN one user turn contains more model-visible content than the kept-recent budget
- WHEN compaction preparation runs
- THEN it identifies the kept suffix and early turn prefix
- AND the generated summary entry records that a split-turn prefix was summarized.

### Requirement: Branch handoff summaries

r[agent_harness_runtime.pi_compaction.branch_summary] When navigating away from a ledger branch, Animus SHOULD support host-requested branch handoff summaries that preserve bounded context from the abandoned branch at the new navigation point.

#### Scenario: Abandoned branch is summarized at navigation

r[agent_harness_runtime.pi_compaction.branch_summary.navigation]
- GIVEN a ledger has an old active leaf and a target leaf with a common ancestor
- WHEN the host requests branch handoff summarization during navigation
- THEN runtime summarizes the abandoned branch span from old leaf back to the common ancestor
- AND records a bounded summary entry associated with the navigation.

### Requirement: Injected summarizer service

r[agent_harness_runtime.pi_compaction.injected_summarizer] Runtime compaction and branch summarization MUST call only injected summarizer services and MUST fail closed when summarization is required but no summarizer is available, denied, cancelled, or returns malformed output.

#### Scenario: Missing summarizer halts compaction

r[agent_harness_runtime.pi_compaction.injected_summarizer.missing]
- GIVEN policy requires summarization before continuing a turn
- WHEN no summarizer service is injected
- THEN runtime emits a typed missing-summarizer failure
- AND it does not construct a provider client or call model/tool adapters as a fallback.

### Requirement: Summary metadata is safe and cumulative

r[agent_harness_runtime.pi_compaction.safe_metadata] Compaction and branch-summary entries MUST store bounded summary text and safe cumulative metadata, including counts, redacted resource identifiers, and BLAKE3 hashes when sensitive content identity is needed.

#### Scenario: Summary metadata rejects raw tool bodies

r[agent_harness_runtime.pi_compaction.safe_metadata.reject_raw]
- GIVEN a summary result attempts to include raw tool output or unredacted host paths in metadata
- WHEN runtime validates the summary entry
- THEN validation fails with a typed redaction or metadata error
- AND the unsafe summary is not appended to the ledger.

### Requirement: Provider prompt pressure calibration

r[agent_harness_runtime.zerostack_midturn_compaction.prompt_pressure] Animus MUST represent provider-reported prompt pressure separately from estimated replay tokens and MUST account for provider route metadata before using cached-token fields.

#### Scenario: Cached tokens are not double-counted

r[agent_harness_runtime.zerostack_midturn_compaction.prompt_pressure.cache]
- GIVEN provider usage reports input tokens and cached-token fields
- AND route metadata says cached tokens are already included in input tokens
- WHEN prompt pressure is calculated
- THEN cached-token fields are not added a second time
- AND the calculation records the route rule used.

### Requirement: Mid-turn compaction planning

r[agent_harness_runtime.zerostack_midturn_compaction.plan] Runtime MUST produce an explicit mid-turn stop, compact, and resume plan when policy enables mid-turn compaction and prompt pressure crosses the configured threshold.

#### Scenario: Disabled mid-turn compaction does not trigger

r[agent_harness_runtime.zerostack_midturn_compaction.plan.disabled]
- GIVEN prompt pressure crosses the threshold
- AND mid-turn compaction is disabled by policy
- WHEN runtime evaluates continuation
- THEN it does not prepare a mid-turn compaction plan
- AND it returns the policy-selected continue, warn, or fail outcome.

### Requirement: Explicit ledger summary entries

r[agent_harness_runtime.zerostack_midturn_compaction.ledger_entries] Mid-turn compaction MUST append explicit bounded summary entries to the session ledger and MUST NOT hide compaction as an unrecorded in-memory transcript rewrite.

#### Scenario: Missing ledger service halts required compaction

r[agent_harness_runtime.zerostack_midturn_compaction.ledger_entries.missing]
- GIVEN policy requires mid-turn compaction before continuing
- AND no ledger service is injected
- WHEN runtime attempts to prepare the compaction
- THEN it returns a missing-ledger-service failure
- AND it does not call model or tool adapters as a fallback.

### Requirement: Injected summarizer for mid-turn compaction

r[agent_harness_runtime.zerostack_midturn_compaction.summarizer] Mid-turn compaction MUST use only injected summarizer services and MUST validate summary output for bounded text, safe metadata, and supported summary kind.

#### Scenario: Malformed summary fails closed

r[agent_harness_runtime.zerostack_midturn_compaction.summarizer.malformed]
- GIVEN the summarizer returns empty text, overlong text, unsupported metadata, or raw sensitive output in metadata
- WHEN runtime validates the summary
- THEN validation fails with a typed summary diagnostic
- AND the unsafe summary is not appended to the ledger.

### Requirement: Long tool-output artifactization

r[agent_harness_runtime.zerostack_midturn_compaction.tool_artifacts] Tool outputs that exceed model-visible policy limits SHOULD be represented by bounded head/tail or windowed text plus artifact descriptors, total size metadata, truncation status, and BLAKE3 artifact hashes.

#### Scenario: Unbounded tool output is not embedded

r[agent_harness_runtime.zerostack_midturn_compaction.tool_artifacts.bounded]
- GIVEN a tool output exceeds the configured model-visible limit
- WHEN it is recorded in ledger or receipt metadata
- THEN only the configured bounded window and artifact descriptor are model-visible by default
- AND the full raw output is not embedded in durable evidence.

### Requirement: Bounded Qualifier quality-record reference

r[agent_harness_runtime.qualifier_quality.scope] Animus MUST treat Qualifier as a bounded reference for ambient review annotations, Metabox-style envelopes, append-only JSONL sidecars, span freshness, unknown-record preservation, annotation lifecycle semantics, and AGENTS-CLI guidance, and MUST NOT adopt Qualifier as a core runtime dependency, session ledger, transcript store, Git-only diff engine, or numeric scoring authority.

#### Scenario: Qualifier remains a compatibility boundary

r[agent_harness_runtime.qualifier_quality.scope.boundary]
- GIVEN maintainers adapt Qualifier-inspired quality-record behavior
- WHEN reusable Animus runtime, core, or adapter APIs are reviewed
- THEN the APIs use Animus-owned DTOs and traits
- AND they do not require the `qualifier` crate, `.qual` files, Git commands, filesystem discovery, CLI execution, or scoring semantics in reusable core contracts.

#### Scenario: Raw payloads are outside quality records

r[agent_harness_runtime.qualifier_quality.scope.redaction]
- GIVEN a quality observation is derived from a prompt, provider payload, tool input, tool output, model output, credential-bearing configuration, or absolute host path
- WHEN an Animus quality/provenance record body is constructed
- THEN raw sensitive material is omitted or represented only by approved BLAKE3 hashes, bounded counts, labels, and explicit redaction markers
- AND validation rejects any record body that stores the raw material directly.

### Requirement: Animus-owned Metabox-style record envelope

r[agent_harness_runtime.qualifier_quality.envelope] Animus MUST define a safe record envelope with explicit version, record type, subject, issuer, optional issuer classification, shell-supplied creation metadata, content-addressed BLAKE3 id, and typed body, with deterministic canonicalization and validation implemented as pure functional-core logic.

#### Scenario: Canonical IDs are deterministic

r[agent_harness_runtime.qualifier_quality.envelope.deterministic]
- GIVEN equivalent safe record envelopes with body fields and nested fields in different source orders
- WHEN canonicalization and id generation run
- THEN field order is normalized, optional absent fields are omitted, and the same lowercase BLAKE3 id is produced
- AND the canonicalization function does not read files, clocks, environment, process state, or network resources.

#### Scenario: Malformed envelopes fail closed

r[agent_harness_runtime.qualifier_quality.envelope.invalid]
- GIVEN a record envelope has an unsupported version, empty subject, empty issuer, non-URI issuer, invalid id, mismatched id, missing required body field, or malformed issuer classification
- WHEN envelope validation runs
- THEN validation returns typed errors naming the failed invariant
- AND the invalid record is not accepted as runtime, review, or release evidence.

### Requirement: Optional Qualifier sidecar adapter

r[agent_harness_runtime.qualifier_quality.sidecar_adapter] Animus MAY provide an optional std-only Qualifier sidecar adapter that reads and writes `.qual`-compatible JSONL annotations for repository-local quality observations while keeping sidecar persistence, filesystem traversal, ignore handling, and compaction outside no-std core and adapter-contract crates.

#### Scenario: Sidecar export preserves safe annotation semantics

r[agent_harness_runtime.qualifier_quality.sidecar_adapter.export]
- GIVEN an Animus safe quality observation contains subject, kind, span, summary, suggested fix, references, supersedes, tags, issuer classification, and redaction-safe metadata
- WHEN Qualifier sidecar export is explicitly requested
- THEN the adapter writes a `.qual`-compatible annotation record with a deterministic id and safe body fields
- AND raw prompts, provider payloads, tool/model outputs, credentials, full paths, and unbounded content are not written.

#### Scenario: Unknown sidecar records round-trip without becoming core data

r[agent_harness_runtime.qualifier_quality.sidecar_adapter.unknown]
- GIVEN a `.qual` file contains unknown record types or future body shapes
- WHEN the optional sidecar adapter loads, filters, compacts, or rewrites the file
- THEN unknown records are preserved byte-for-byte or as an explicitly equivalent opaque sidecar record where the sidecar operation requires rewriting
- AND reusable Animus core receives only sanitized safe metadata or a typed compatibility error, not the raw unknown body.

#### Scenario: Default builds remain isolated

r[agent_harness_runtime.qualifier_quality.sidecar_adapter.isolated]
- GIVEN default Animus features are built without Qualifier compatibility
- WHEN dependency and feature-isolation checks run
- THEN `animus-core`, `animus-adapters`, `animus-preserves`, `animus-steel`, and default `animus-std` contracts do not require Qualifier crates, `.qual` discovery, Git diff libraries, or sidecar compaction code.

### Requirement: Annotation lifecycle state transitions

r[agent_harness_runtime.qualifier_quality.lifecycle] Animus MUST model quality annotation lifecycle behavior as pure state transitions over in-memory records, including built-in and custom kinds, typo warnings, references, supersession chains, resolve records, waiver rationale, suggested fixes, active-record filtering, same-subject supersession checks, and supersession cycle detection.

#### Scenario: References thread context without closing records

r[agent_harness_runtime.qualifier_quality.lifecycle.references]
- GIVEN one quality record references another record by id
- WHEN active quality records are computed
- THEN both records remain active and the referencing record is displayed or queried as thread context
- AND self-references are rejected with a typed validation error.

#### Scenario: Supersession closes only same-subject records

r[agent_harness_runtime.qualifier_quality.lifecycle.supersession]
- GIVEN a resolve or replacement record supersedes a prior record
- WHEN active quality records are computed
- THEN the superseded record is excluded from the active set and the superseding record remains as the visible tip
- AND validation rejects cross-subject supersession, duplicate-id ambiguity, and cycles without panicking.

#### Scenario: Agents do not auto-resolve user concerns

r[agent_harness_runtime.qualifier_quality.lifecycle.no_auto_resolve]
- GIVEN an agent edits code near a human-authored concern or drifted span
- WHEN runtime or sidecar tooling reports quality status
- THEN it surfaces the concern, drift, or missing span for review without resolving it automatically
- AND a resolve record is emitted only when the caller explicitly requests closure.

### Requirement: VCS-neutral span freshness

r[agent_harness_runtime.qualifier_quality.span_freshness] Animus MUST provide VCS-neutral span freshness logic that hashes the full lines covered by a valid span with BLAKE3 and classifies observations as fresh, drifted, missing, or no-hash without requiring Git, branch names, merge-base logic, or Qualifier CLI execution.

#### Scenario: Freshness states are deterministic

r[agent_harness_runtime.qualifier_quality.span_freshness.states]
- GIVEN file content lines, a valid one-indexed span, and an optional recorded content hash
- WHEN pure freshness classification runs over the supplied lines
- THEN matching hashes return fresh, mismatched hashes return drifted with expected and actual hashes, absent hashes return no-hash, and missing span lines return missing
- AND equivalent line content produces the same BLAKE3 hash regardless of caller or storage backend.

#### Scenario: Invalid spans fail closed

r[agent_harness_runtime.qualifier_quality.span_freshness.invalid]
- GIVEN a span has zero line numbers, reversed ranges, columns outside allowed bounds, or arithmetic overflow inputs
- WHEN span parsing or freshness preparation runs
- THEN it returns typed validation errors
- AND no panic, wrapped line number, silently clipped range, or raw file dump is produced.

### Requirement: AGENTS-CLI-inspired Animus guidance

r[agent_harness_runtime.qualifier_quality.agents_cli] When Animus exposes a public CLI, it SHOULD provide an `animus agents` entry point that prints version-locked markdown guidance for AI coding agents, lists available topics, supports topic pages for concepts, workflows, pitfalls, and non-trivial commands, advertises the entry point in `animus --help`, and returns a structured unknown-topic error.

#### Scenario: Bare agents invocation orients coding agents

r[agent_harness_runtime.qualifier_quality.agents_cli.orientation]
- GIVEN the Animus CLI includes the agents guidance feature
- WHEN an agent runs `animus agents`
- THEN stdout contains a self-contained orientation explaining what Animus is, when to use it, when not to use it, and which topics are available
- AND the command exits successfully without reading network resources.

#### Scenario: Topic lookup is deterministic

r[agent_harness_runtime.qualifier_quality.agents_cli.topics]
- GIVEN a known guidance topic and an unknown guidance topic
- WHEN an agent runs `animus agents <topic>` for each topic
- THEN the known topic prints its markdown page and exits successfully
- AND the unknown topic exits with a structured error that names the unknown topic and every available topic.

### Requirement: Focused Qualifier adaptation verification

r[agent_harness_runtime.qualifier_quality.verification] The Qualifier adaptation MUST include focused positive and negative verification for bounded reference scope, deterministic envelope IDs, malformed envelope rejection, safe body redaction, unknown sidecar preservation, sidecar dependency isolation, annotation lifecycle transitions, span freshness states, invalid span handling, no-auto-resolution behavior, and AGENTS-CLI guidance behavior when implemented.

#### Scenario: Closeout evidence is reviewable

r[agent_harness_runtime.qualifier_quality.verification.closeout]
- GIVEN maintainers intend to archive the Qualifier quality-record adaptation
- WHEN closeout validation runs
- THEN evidence names every successful focused rail, workspace test, clippy check, Nickel/config drift check if applicable, Cairn validation, and Cairn gate
- AND any skipped rail includes a concrete blocker and next-best validation.

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

### Requirement: Fresh-context task descriptors

r[agent_harness_runtime.zerostack_fresh_context_tasks.descriptor] Animus MUST represent fresh-context task requests as typed descriptors with goal, admitted context references, allowed tool profile, model/provider policy, token budget, tool-call budget, timeout policy, cancellation policy, and expected result schema.

#### Scenario: Missing scheduler fails closed

r[agent_harness_runtime.zerostack_fresh_context_tasks.descriptor.missing_scheduler]
- GIVEN a runtime request includes a fresh-context task descriptor
- AND no task scheduler service is injected
- WHEN runtime prepares the task
- THEN it returns a missing-scheduler failure
- AND it does not spawn processes, call providers, or run tools as a fallback.

### Requirement: Read-only side-question mode

r[agent_harness_runtime.zerostack_fresh_context_tasks.side_question] Side-question requests SHOULD be read-only by construction and MUST NOT write files, edit state, execute shell commands, mutate ledgers, or dispatch non-read tools unless a separate explicit policy admits that behavior.

#### Scenario: Side question mutation attempt is denied

r[agent_harness_runtime.zerostack_fresh_context_tasks.side_question.readonly]
- GIVEN a side-question child run requests a write, edit, shell, network mutation, ledger mutation, or other non-read action
- WHEN permission resolution evaluates the child request
- THEN the action is denied with a read-only side-question reason
- AND the child result records the denied attempt as safe metadata.

### Requirement: Context and authority isolation

r[agent_harness_runtime.zerostack_fresh_context_tasks.isolation] Fresh-context task runs MUST receive only explicitly admitted context references and capability grants, and MUST NOT inherit hidden parent prompts, credentials, session allowlists, or mutable process-global configuration.

#### Scenario: Parent allowlist is not inherited

r[agent_harness_runtime.zerostack_fresh_context_tasks.isolation.allowlist]
- GIVEN the parent run has a session allowlist entry for a guarded action
- WHEN a fresh-context task run requests the same action
- THEN the child run must satisfy its own authority and permission policy
- AND the parent allowlist does not authorize the child action.

### Requirement: Attributed task results

r[agent_harness_runtime.zerostack_fresh_context_tasks.results] Task results MUST include child-run identity, task descriptor identity, result status, budget usage, cited context references, uncertainty markers, and redacted evidence summaries before they can influence parent prompt assembly.

#### Scenario: Malformed child result is withheld

r[agent_harness_runtime.zerostack_fresh_context_tasks.results.malformed]
- GIVEN a child run returns a result missing status, child identity, context references, or result schema fields
- WHEN parent prompt assembly evaluates the result
- THEN the result is withheld with a malformed-result diagnostic
- AND its raw body is not injected into the parent model-visible context.

### Requirement: Task receipts and cancellation

r[agent_harness_runtime.zerostack_fresh_context_tasks.receipts] Fresh-context task receipts MUST record safe task identity, scheduler identity, status, cancellation reason, budget usage, redacted context/resource identifiers, and output hashes without embedding full child transcripts by default.

#### Scenario: Cancelled task records bounded receipt

r[agent_harness_runtime.zerostack_fresh_context_tasks.receipts.cancelled]
- GIVEN a fresh-context task is cancelled by user, policy, timeout, or scheduler failure
- WHEN the runtime records the outcome
- THEN the receipt names the cancellation class and consumed budget metadata
- AND no unbounded child transcript or raw sensitive output is embedded.

### Requirement: Explicit OpenAI-family API style

r[agent_harness_runtime.zerostack_provider_hardening.api_style] Animus provider registry shells MUST resolve OpenAI-family providers to explicit API style metadata such as Responses or Chat Completions before constructing a model service.

#### Scenario: Custom gateway defaults safely

r[agent_harness_runtime.zerostack_provider_hardening.api_style.gateway]
- GIVEN a custom OpenAI-compatible provider declares a base endpoint and no explicit API style
- WHEN shell provider resolution runs
- THEN the resolver selects the policy-defined compatible default or reports an ambiguous-style diagnostic
- AND reusable runtime receives only a validated injected model service.

### Requirement: Redacted custom headers

r[agent_harness_runtime.zerostack_provider_hardening.headers] Custom provider header descriptors MUST validate header names, resolve allowed environment references in the shell, and report only redacted source labels or failure classes in receipts and diagnostics.

#### Scenario: Missing header environment variable is safe

r[agent_harness_runtime.zerostack_provider_hardening.headers.missing]
- GIVEN a custom header references an unavailable environment-backed secret
- WHEN provider resolution runs
- THEN resolution fails with a missing-header-secret diagnostic
- AND the diagnostic does not include raw secret values, environment contents, or unrelated environment variables.

### Requirement: Provider-scoped extra body policy

r[agent_harness_runtime.zerostack_provider_hardening.extra_body] Provider-specific extra request body fields MUST be admitted by policy with provider/model scope, source identity, collision behavior, and receipt redaction before they can be sent with a model request.

#### Scenario: Extra body cannot cross providers silently

r[agent_harness_runtime.zerostack_provider_hardening.extra_body.scope]
- GIVEN an extra body descriptor is scoped to one provider or model
- WHEN a different provider or model is selected
- THEN the descriptor is ignored or rejected according to policy
- AND the request does not inherit unsupported provider-specific fields silently.

### Requirement: Agent-suitable model filtering

r[agent_harness_runtime.zerostack_provider_hardening.model_filter] Dynamic model listings SHOULD normalize model IDs, display names, model classes, context budgets, modalities, reasoning support, and cost hints, and MUST prevent non-agent model classes from being selected for text agent turns unless an explicit host override is present.

#### Scenario: Embedding model is rejected for agent turn

r[agent_harness_runtime.zerostack_provider_hardening.model_filter.reject]
- GIVEN model listing metadata identifies a model as embedding, image, audio, moderation, rerank, or another non-agent class
- WHEN the shell attempts to select it for a text agent turn without override
- THEN selection fails with a typed unsupported-model-class diagnostic
- AND runtime model dispatch is not attempted.

### Requirement: Provider selection receipts

r[agent_harness_runtime.zerostack_provider_hardening.receipts] Provider selection receipts MUST record safe provider ID, model ID, endpoint identity class, API style, capability metadata, credential source class, extra body descriptor IDs, and selection diagnostics without exposing raw credentials, headers, provider payloads, or full private endpoints when policy marks them sensitive.

#### Scenario: Sensitive endpoint is hashed

r[agent_harness_runtime.zerostack_provider_hardening.receipts.endpoint]
- GIVEN a provider endpoint is classified as sensitive
- WHEN a provider selection receipt is emitted
- THEN the endpoint is represented by a redaction marker or BLAKE3 hash
- AND API style, capability class, and diagnostic reason remain reviewable.

### Requirement: Animus sessions are dataset export sources

r[agent_harness_runtime.hf_dataset_export.session_source] Animus MUST export training datasets from validated Animus session snapshots or replay contexts, with typed row identity, source span metadata, target selection, and export errors, without treating Claude Code JSONL, Hugging Face libraries, filesystem paths, tokenizers, or trainer frameworks as core dependencies.

#### Scenario: Export starts from validated replay context

r[agent_harness_runtime.hf_dataset_export.session_source.validated]
- GIVEN an Animus session snapshot has been validated and replay context has been selected
- WHEN dataset export begins
- THEN the exporter consumes the selected Animus replay data and safe metadata
- AND it does not read provider-native transcripts, external agent session files, filesystem paths, tokenizers, or trainer APIs inside pure export logic.

#### Scenario: Invalid session source fails before row emission

r[agent_harness_runtime.hf_dataset_export.session_source.rejects_invalid]
- GIVEN an Animus session snapshot has broken parent links, duplicate row source spans, unsupported entry kinds, or no selected replay path
- WHEN dataset export is requested
- THEN export returns a typed source-validation error
- AND no partial dataset rows are emitted unless an explicit partial-export policy is selected.

### Requirement: Hugging Face chat-message export

r[agent_harness_runtime.hf_dataset_export.chat_messages] Animus MUST export supervised fine-tuning rows in a Hugging Face-compatible chat `messages` shape using deterministic user, assistant, system, and tool role mapping from Animus replay context.

#### Scenario: SFT row is chat-template compatible

r[agent_harness_runtime.hf_dataset_export.chat_messages.sft]
- GIVEN a validated Animus replay path contains user and assistant model-visible turns
- WHEN the chat-message exporter runs with the default SFT policy
- THEN it emits a dataset row with ordered `messages` entries containing `role` and `content` fields
- AND the row is suitable for downstream `tokenizer.apply_chat_template()` without Animus calling a tokenizer.

#### Scenario: Bad chat rows fail closed

r[agent_harness_runtime.hf_dataset_export.chat_messages.rejects_bad_rows]
- GIVEN replay context contains invalid roles, empty conversations, empty assistant targets, unsupported content blocks, or malformed tool-result ordering
- WHEN chat-message export runs
- THEN it returns a typed export error or deterministic skipped-row reason
- AND it does not silently emit malformed `messages` rows.

### Requirement: Explicit safe export policy

r[agent_harness_runtime.hf_dataset_export.policy] Animus MUST require a typed export policy for lossy or sensitive choices, including system text, tool calls, tool results, hidden reasoning, metadata projection, redaction, and quality thresholds.

#### Scenario: Default policy prevents private leakage

r[agent_harness_runtime.hf_dataset_export.policy.leakage_default]
- GIVEN a session contains hidden reasoning, raw tool outputs, environment metadata, provider details, and private provenance
- WHEN export runs with the default policy
- THEN hidden reasoning, raw tool outputs, private environment fields, and private provenance are excluded from dataset rows
- AND the inspection report records safe policy decisions without exposing the raw private payloads.

#### Scenario: Opt-in policy is explicit and testable

r[agent_harness_runtime.hf_dataset_export.policy.opt_in]
- GIVEN a host selects a policy that includes tool calls, tool results, system text, or selected metadata
- WHEN export runs
- THEN the emitted rows contain only the explicitly enabled categories
- AND negative tests prove disabled categories remain absent.

### Requirement: Preference-learning row export

r[agent_harness_runtime.hf_dataset_export.preference_rows] Animus MUST export preference-learning rows only from explicit Animus outcome annotations that identify prompt context, chosen responses, rejected responses, and safe metadata.

#### Scenario: Preference row preserves chosen and rejected alternatives

r[agent_harness_runtime.hf_dataset_export.preference_rows.validated]
- GIVEN a session records a prompt span with explicit chosen and rejected assistant alternatives
- WHEN preference export runs
- THEN it emits a row containing the prompt messages, chosen assistant content, rejected assistant content, row identity, and safe metadata
- AND it does not infer preference labels from file order or branch order alone.

#### Scenario: Malformed preference examples are rejected

r[agent_harness_runtime.hf_dataset_export.preference_rows.rejects_malformed]
- GIVEN an example lacks a prompt span, duplicates a candidate, lacks an outcome label, or requests unsafe metadata projection
- WHEN preference export runs
- THEN it returns a typed export error or deterministic skipped-row reason
- AND no fabricated chosen/rejected pair is emitted.

### Requirement: RL trajectory export

r[agent_harness_runtime.hf_dataset_export.rl_trajectories] Animus MUST export RL trajectory rows as ordered prompt, action, observation, reward, terminal outcome, and safe metadata records when sessions contain explicit trajectory annotations.

#### Scenario: Complete trajectory exports deterministically

r[agent_harness_runtime.hf_dataset_export.rl_trajectories.validated]
- GIVEN an Animus session contains an explicitly annotated trajectory with prompts, actions, observations, rewards, and terminal outcome
- WHEN RL trajectory export runs
- THEN it emits the trajectory in deterministic order with safe row identity and metadata
- AND tool/action links remain structured rather than flattened into ambiguous prose.

#### Scenario: Incomplete trajectory fails closed

r[agent_harness_runtime.hf_dataset_export.rl_trajectories.rejects_incomplete]
- GIVEN a trajectory has missing rewards, no terminal outcome, orphaned observations, invalid action/tool links, or unsafe metadata requests
- WHEN RL trajectory export runs
- THEN it returns a typed export error or deterministic skipped-row reason
- AND it does not fabricate rewards or terminal outcomes.

### Requirement: Structured dataset inspection

r[agent_harness_runtime.hf_dataset_export.inspection] Animus MUST provide structured dataset inspection reports that summarize row counts, skipped rows, empty or short turns, tool/reasoning leakage warnings, usage or token summaries when available, and export policy decisions.

#### Scenario: Inspection explains export quality safely

r[agent_harness_runtime.hf_dataset_export.inspection.safe_reports]
- GIVEN a batch of Animus sessions contains exportable rows, skipped rows, and policy-sensitive payloads
- WHEN inspection runs
- THEN it returns deterministic aggregate counts, warning classes, skipped-row reasons, and safe metadata summaries
- AND the report does not include raw hidden reasoning, raw private tool output, or private provenance payloads.

### Requirement: Closeout evidence covers dataset export targets

r[agent_harness_runtime.hf_dataset_export.closeout] Dataset export implementation MUST be closed out with focused positive and negative tests, documentation for Hugging Face/TRL integration, workspace checks for touched crates, pre-commit hooks, Cairn validation, and Cairn gates.

#### Scenario: Export change is ready to archive

r[agent_harness_runtime.hf_dataset_export.closeout.evidence]
- GIVEN the dataset export change is implemented
- WHEN closeout validation runs
- THEN evidence includes SFT chat-message tests, preference-row tests, RL trajectory tests, leakage negative tests, inspection-report tests, documentation review, pre-commit results, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Composition conformance matrix

r[agent_harness_runtime.composability.conformance_matrix] Animus MUST maintain a composition conformance matrix that names representative cross-module scenarios, owners, feature requirements, positive cases, negative cases, and safe evidence labels.

#### Scenario: Matrix entries include positive and negative coverage

r[agent_harness_runtime.composability.conformance_matrix.coverage]
- GIVEN a conformance matrix entry covers provider/model dispatch, tool/provenance receipts, headless/runtime events, middleware/failure reports, or capability/no-fallback behavior
- WHEN matrix validation runs
- THEN each entry has an owner, feature requirements, at least one positive case, at least one negative case, and a safe evidence label
- AND incomplete entries fail validation.

### Requirement: Positive cross-module conformance

r[agent_harness_runtime.composability.positive_conformance] Composition tests MUST include deterministic positive scenarios that combine independent modules using pure fixtures and fake adapters rather than live host authority.

#### Scenario: Cross-module happy paths compose without live services

r[agent_harness_runtime.composability.positive_conformance.fake_backed]
- GIVEN fake adapters and in-memory DTO fixtures for selected conformance scenarios
- WHEN the composition rail runs
- THEN provider hardening, model dispatch, tool contracts, provenance receipts, headless event projection, middleware, and runtime reports compose according to their typed contracts
- AND no live provider, tool process, network, terminal, filesystem mutation, or persistence backend is required.

### Requirement: Negative cross-module conformance

r[agent_harness_runtime.composability.negative_conformance] Composition tests MUST include deterministic negative scenarios for malformed responses, unauthorized tools, missing services, oversized output, unsafe receipts, disabled feature surfaces, and ambient fallback attempts.

#### Scenario: Cross-module failure paths fail closed

r[agent_harness_runtime.composability.negative_conformance.fail_closed]
- GIVEN a conformance scenario receives a malformed response, unauthorized capability, missing service, oversized output, unsafe receipt, disabled surface, or attempted ambient fallback
- WHEN the composition rail runs
- THEN runtime returns the matching typed failure or denial
- AND no downstream module claims completed effects or accepts unsafe evidence.

### Requirement: Focused composition rail

r[agent_harness_runtime.composability.conformance_rail] Animus MUST provide a focused composition conformance command or script that runs deterministic in-memory cross-module scenarios and emits bounded safe evidence.

#### Scenario: Conformance evidence remains safe

r[agent_harness_runtime.composability.conformance_rail.safe_evidence]
- GIVEN the conformance rail completes or fails
- WHEN it emits evidence output
- THEN evidence includes scenario names, classes, counts, and hashes
- AND raw prompts, provider payloads, credentials, full paths, and unbounded tool outputs are omitted or rejected by leak checks.

### Requirement: Composition conformance verification

r[agent_harness_runtime.composability.conformance.verification] The composition conformance change MUST include focused positive and negative tests for matrix coverage, fake-backed composition, fail-closed composition, safe evidence, workspace checks, optional Nix check coverage, and Cairn gates before archive.

#### Scenario: Closeout evidence covers composition rail behavior

r[agent_harness_runtime.composability.conformance.verification.closeout]
- GIVEN maintainers intend to archive the composition conformance change
- WHEN closeout validation runs
- THEN evidence names matrix, positive conformance, negative conformance, safe evidence, workspace, Nix if added, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Typed capability descriptors

r[agent_harness_runtime.composability.capability_descriptors] Animus runtime decisions and service-backed helpers MUST represent required host authority as typed capability descriptors rather than implicit model, tool, queue, event, policy, confirmation, cancellation, or provenance assumptions.

#### Scenario: Capability descriptors validate before resolution

r[agent_harness_runtime.composability.capability_descriptors.validation]
- GIVEN a runtime plan declares required host capabilities
- WHEN descriptor validation runs
- THEN valid descriptors are accepted with stable labels and service kinds
- AND duplicate labels, empty labels, unsupported service kinds, or unsafe scope text fail before service resolution.

### Requirement: Host-supplied capability registry

r[agent_harness_runtime.composability.capability_registry] Animus MAY use a host-supplied capability bundle or registry to resolve explicit service references, but reusable runtime code MUST NOT construct providers, tools, queues, stores, event sinks, plugin runtimes, or other ambient backends.

#### Scenario: Multiple services compose through explicit injection

r[agent_harness_runtime.composability.capability_registry.composition]
- GIVEN a host supplies model, tool, event sink, policy, queue, cancellation, confirmation, and provenance capabilities
- WHEN runtime resolves a plan requiring a subset of those capabilities
- THEN only explicitly supplied services are eligible for dispatch
- AND duplicate registrations or missing required capabilities return typed diagnostics without ambient fallback.

### Requirement: Model and tool adapter compatibility

r[agent_harness_runtime.composability.adapter_compatibility] Existing simple model/tool adapter constructors MUST remain available or be replaced by explicit migration helpers that are thin wrappers over the capability bundle.

#### Scenario: Compatibility path preserves behavior

r[agent_harness_runtime.composability.adapter_compatibility.equivalent]
- GIVEN a runtime loop uses the existing model/tool constructor path and another loop uses the explicit capability-bundle path
- WHEN both loops receive equivalent fabric decisions and adapter responses
- THEN they produce equivalent typed reports, events, failures, and receipts
- AND neither path gains additional ambient authority.

### Requirement: Capability calls fail closed

r[agent_harness_runtime.composability.capability_fail_closed] Capability resolution and calls MUST return typed absent, denied, unavailable, malformed, cancelled, timed-out, and completed outcomes, and non-completed outcomes MUST prevent success receipts and later side effects.

#### Scenario: Missing capability cannot fall back to ambient discovery

r[agent_harness_runtime.composability.capability_fail_closed.no_fallback]
- GIVEN a runtime action requires a capability that is absent, denied, unavailable, malformed, cancelled, or timed out
- WHEN dispatch attempts to resolve or call that capability
- THEN runtime returns the matching fail-closed outcome
- AND it does not read environment variables, scan files, load plugins, spawn processes, contact providers, or claim completed effects as a fallback.

### Requirement: Capability registry verification

r[agent_harness_runtime.composability.capability_registry.verification] The capability registry change MUST include focused positive and negative tests for descriptor validation, single-service and multi-service composition, compatibility constructors, fail-closed outcomes, no ambient fallback, facade inventory, workspace checks, and Cairn gates before archive.

#### Scenario: Closeout evidence covers capability composition

r[agent_harness_runtime.composability.capability_registry.verification.closeout]
- GIVEN maintainers intend to archive the capability registry change
- WHEN closeout validation runs
- THEN evidence names descriptor, registry, compatibility, fail-closed, no-fallback, inventory, workspace, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Fabric-neutral loop contracts

r[agent_harness_runtime.composability.fabric_neutral_contracts] Animus runtime loop contracts MUST use runtime-owned neutral turn and decision DTOs rather than Steel-specific types for reusable fabric input and output.

#### Scenario: Fabric decisions validate without Steel

r[agent_harness_runtime.composability.fabric_neutral_contracts.validation]
- GIVEN a runtime loop decision requests model dispatch, tool dispatch, final response, or safe failure completion
- WHEN neutral DTO validation runs
- THEN valid decisions are accepted without constructing Steel values
- AND malformed empty fields, unsupported decision kinds, or oversized safe text fail before adapter dispatch.

### Requirement: Loop fabric trait is implementation-neutral

r[agent_harness_runtime.composability.loop_fabric_trait] The `LoopFabric` trait MUST accept and return neutral runtime DTOs so embedders can provide non-Steel fabrics without depending on Steel crates or values.

#### Scenario: Non-Steel fabric drives a runtime loop

r[agent_harness_runtime.composability.loop_fabric_trait.non_steel]
- GIVEN a fake non-Steel fabric and injected model and tool adapters
- WHEN the runtime shell executes a loop
- THEN model dispatch, tool dispatch, final response, malformed decision, and missing-service paths use the same typed transition and failure semantics as the default fabric path
- AND no Steel script, Steel policy, or Steel value is required.

### Requirement: Steel integration is an adapter

r[agent_harness_runtime.composability.steel_adapter] Animus MAY provide Steel as a canonical loop fabric adapter, but Steel-specific decisions MUST be converted into neutral runtime DTOs and revalidated before model, tool, or final-response handling.

#### Scenario: Steel output cannot bypass runtime validation

r[agent_harness_runtime.composability.steel_adapter.validation]
- GIVEN a Steel script returns a dispatch or final-response decision
- WHEN the Steel adapter binds that output to the runtime loop contract
- THEN valid output is converted to neutral DTOs
- AND malformed Steel decisions, denied policy activation, or invalid adapter output produce typed fail-closed reports without dispatching unchecked effects.

### Requirement: Steel dependency is optional for neutral runtime composition

r[agent_harness_runtime.composability.steel_optional] The neutral runtime loop surface MUST compile and be documented without requiring Steel; Steel convenience runners MUST be behind explicit feature or integration boundaries.

#### Scenario: Steel-disabled runtime remains usable

r[agent_harness_runtime.composability.steel_optional.compile]
- GIVEN an embedder disables Steel integration and depends on the neutral runtime surface
- WHEN compile and facade inventory checks run
- THEN neutral loop DTOs, transition helpers, adapter dispatch, and fake-fabric tests remain available
- AND Steel runner APIs are absent or explicitly feature-gated rather than pulled in transitively.

### Requirement: Fabric decoupling verification

r[agent_harness_runtime.composability.fabric_decoupling.verification] The fabric decoupling change MUST include focused positive and negative tests for neutral DTO validation, non-Steel fabric execution, Steel adapter conversion, feature-gated compile surfaces, facade inventory, and Cairn gates before archive.

#### Scenario: Closeout evidence covers fabric composition

r[agent_harness_runtime.composability.fabric_decoupling.verification.closeout]
- GIVEN maintainers intend to archive the fabric decoupling change
- WHEN closeout validation runs
- THEN evidence names neutral-runtime, fake-fabric, Steel-adapter, feature-gate, facade-inventory, workspace, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Composition inventory metadata

r[agent_harness_runtime.composability.inventory_metadata] The runtime facade inventory MUST record feature gate, required authority, dependency links, and expected composition links for runtime-facing public exports or export groups.

#### Scenario: Inventory records carry composition metadata

r[agent_harness_runtime.composability.inventory_metadata.schema]
- GIVEN a runtime-facing public export or export group is listed in the inventory
- WHEN inventory schema validation runs
- THEN the record includes explicit or valid default composition metadata
- AND malformed or missing required metadata fails validation.

### Requirement: Inventory metadata validation

r[agent_harness_runtime.composability.inventory_validation] Inventory validation MUST enforce controlled authority values, known feature labels, known dependency targets, and known composition-link targets.

#### Scenario: Invalid inventory metadata is rejected

r[agent_harness_runtime.composability.inventory_validation.fixtures]
- GIVEN an inventory fixture has an unknown authority value, missing feature label, unknown dependency link, unknown composition link, or malformed metadata field
- WHEN inventory validation runs
- THEN validation fails with a typed diagnostic
- AND the invalid record is not accepted as facade evidence.

### Requirement: Reusable surfaces cannot depend upward on host authority

r[agent_harness_runtime.composability.inventory_dependency_direction] Inventory validation MUST reject reusable-contract exports that depend on shell adapters, terminal UI, process execution, persistence, network, provider discovery, credentials, or other ambient host authority surfaces.

#### Scenario: Hidden shell dependency fails inventory validation

r[agent_harness_runtime.composability.inventory_dependency_direction.no_upward_leak]
- GIVEN a reusable-contract inventory record declares or implies dependency on shell, terminal, process, persistence, network, provider, credential, or ambient host authority
- WHEN dependency-direction validation runs
- THEN the inventory check fails with a dependency-direction diagnostic
- AND maintainers must reclassify the export or move the dependency behind host injection.

### Requirement: Safe inventory evidence

r[agent_harness_runtime.composability.inventory_evidence] Runtime facade inventory checks MUST emit bounded safe evidence containing class counts, feature counts, authority counts, composition counts, and BLAKE3 inventory hashes.

#### Scenario: Inventory evidence omits sensitive data

r[agent_harness_runtime.composability.inventory_evidence.safe]
- GIVEN inventory validation emits evidence for review or closeout
- WHEN evidence leak checks run
- THEN raw prompts, provider payloads, credentials, full paths, and unbounded output are omitted or rejected
- AND evidence remains suitable for Cairn archive records.

### Requirement: Composition inventory verification

r[agent_harness_runtime.composability.inventory.verification] The inventory metadata change MUST include focused positive and negative tests for schema validation, metadata validation, dependency direction, safe evidence, workspace checks, and Cairn gates before archive.

#### Scenario: Closeout evidence covers inventory metadata behavior

r[agent_harness_runtime.composability.inventory.verification.closeout]
- GIVEN maintainers intend to archive the inventory metadata change
- WHEN closeout validation runs
- THEN evidence names schema, metadata, dependency-direction, safe-evidence, workspace, inventory rail, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Runtime capability-family map

r[agent_harness_runtime.composability.capability_family_map] Animus MUST maintain a capability-family map for runtime public exports and modules so embedders can distinguish loop, headless, provenance, batch, worker-team, policy, quality, research, scaffold, compaction, dataset-export, shared, and compatibility surfaces.

#### Scenario: Each export has a family classification

r[agent_harness_runtime.composability.capability_family_map.inventory]
- GIVEN a runtime-facing public export is listed in the facade inventory
- WHEN capability-family validation runs
- THEN the export belongs to exactly one primary family or an explicit shared-family class
- AND unclassified exports fail validation before archive.

### Requirement: Capability feature gates

r[agent_harness_runtime.composability.capability_feature_gates] Runtime capability families SHOULD be gated by named Cargo features where doing so reduces dependency footprint, public surface area, or accidental authority coupling, while preserving an explicit compatibility bundle.

#### Scenario: Minimal and selected-family builds compile

r[agent_harness_runtime.composability.capability_feature_gates.compile]
- GIVEN an embedder builds minimal runtime, a selected capability family, the compatibility bundle, or all families
- WHEN compile checks run
- THEN enabled family exports and tests are available
- AND disabled family exports are absent rather than partially usable.

### Requirement: Feature selection documentation

r[agent_harness_runtime.composability.capability_feature_docs] Animus documentation MUST explain how embedders choose minimal, selected-family, compatibility, and all-family runtime feature sets, including authority and dependency implications.

#### Scenario: Disabled families do not leak into minimal docs or builds

r[agent_harness_runtime.composability.capability_feature_docs.no_leak]
- GIVEN a minimal-runtime build and its documented import path
- WHEN docs examples, compile checks, and facade inventory validation run
- THEN disabled headless, provenance, batch, worker-team, TUI, provider, process, and persistence families are not required
- AND any feature-required API is labeled with its required feature.

### Requirement: Capability feature verification

r[agent_harness_runtime.composability.capability_features.verification] The capability feature change MUST include focused positive and negative tests for family mapping, feature-gated compile profiles, disabled-family leakage, facade inventory, workspace checks, and Cairn gates before archive.

#### Scenario: Closeout evidence covers feature composition

r[agent_harness_runtime.composability.capability_features.verification.closeout]
- GIVEN maintainers intend to archive the capability feature change
- WHEN closeout validation runs
- THEN evidence names family-map, minimal compile, selected-family compile, compatibility compile, disabled-family leak, inventory, workspace, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Domain prelude modules

r[agent_harness_runtime.composability.domain_preludes] Animus SHOULD provide curated domain prelude modules that expose small, documented composition surfaces for runtime loop execution, tool contracts, headless protocol, provenance, worker-team delegation, policy/middleware, and status selection.

#### Scenario: Prelude imports stay focused

r[agent_harness_runtime.composability.domain_preludes.compile]
- GIVEN an embedder imports one domain prelude
- WHEN compile or import-surface tests construct representative DTOs and call pure helpers
- THEN the prelude exposes the expected domain contracts
- AND unrelated shell, terminal, provider, process, filesystem, or persistence types are not required.

### Requirement: Compatibility facade remains stable

r[agent_harness_runtime.composability.compatibility_facade] Existing flat root and crate-level compatibility exports MUST remain available or receive explicit migration documentation when domain preludes are introduced.

#### Scenario: Existing imports are not silently removed

r[agent_harness_runtime.composability.compatibility_facade.stable]
- GIVEN existing consumers import DTOs or helpers from the broad compatibility facade
- WHEN domain preludes are added
- THEN compatibility imports continue to compile or emit explicit deprecation/migration guidance
- AND no public behavior changes solely because a prelude exists.

### Requirement: Prelude inventory classification

r[agent_harness_runtime.composability.prelude_inventory] Each domain prelude MUST be recorded in the runtime facade inventory with owning crate, authority class, side-effect class, and any feature requirements.

#### Scenario: Reusable prelude rejects authority leaks

r[agent_harness_runtime.composability.prelude_inventory.no_leak]
- GIVEN a reusable-contract prelude is inventoried
- WHEN facade inventory validation runs
- THEN shell, provider, filesystem, process, persistence, network, credential, and terminal imports are rejected
- AND host-injection or shell-adapter preludes are explicitly labeled rather than hidden inside reusable domains.

### Requirement: Domain prelude verification

r[agent_harness_runtime.composability.domain_preludes.verification] The prelude change MUST include focused positive and negative tests for import surfaces, compatibility exports, facade inventory classification, authority-leak fixtures, workspace checks, and Cairn gates before archive.

#### Scenario: Closeout evidence covers prelude composition

r[agent_harness_runtime.composability.domain_preludes.verification.closeout]
- GIVEN maintainers intend to archive the domain prelude change
- WHEN closeout validation runs
- THEN evidence names prelude compile, compatibility facade, inventory, leak-fixture, workspace, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

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

### Requirement: Dataspace Preserves schema coverage

r[agent_harness_runtime.preserves_dataspace.schema] Animus MUST provide optional Preserves schema identities and typed record coverage for dataspace scopes, assertions, subscriptions, and orchestration events before those DTOs are advertised as portable external dataspace wire contracts.

#### Scenario: Dataspace records have versioned schema identities

r[agent_harness_runtime.preserves_dataspace.schema.versioned]
- GIVEN a dataspace scope, assertion, subscription, cancellation event, expiry event, or ownership-transfer event crosses a Preserves boundary
- WHEN it is encoded as a versioned Preserves record
- THEN the record carries a schema family, supported version, and typed dataspace payload kind
- AND the schema identity is available without enabling a live Syndicate transport.

### Requirement: Typed dataspace decoding

r[agent_harness_runtime.preserves_dataspace.typed_decoding] Preserves dataspace decoding MUST produce validated `animus-adapters` dataspace DTOs before any runtime, host, policy, provenance, or future live dataspace behavior observes the payload.

#### Scenario: Raw Preserves syntax stays at the codec boundary

r[agent_harness_runtime.preserves_dataspace.typed_decoding.no_ast_logic]
- GIVEN a behavior decision depends on dataspace scope, assertion status, subscription shape, cancellation, expiry, or ownership transfer
- WHEN Preserves decoding succeeds
- THEN the decision receives typed validated dataspace DTOs
- AND reusable core, runtime, adapter, policy, or host orchestration logic does not inspect raw Preserves syntax trees.

### Requirement: Safe dataspace payload set

r[agent_harness_runtime.preserves_dataspace.safe_payloads] Dataspace Preserves records MUST contain only bounded safe coordination material such as stable IDs, normalized scope labels, status classes, assertion kinds, redaction markers, BLAKE3 hashes, counts, lease/expiry metadata, and ownership-transfer labels.

#### Scenario: Sensitive material is rejected before broadcast

r[agent_harness_runtime.preserves_dataspace.safe_payloads.rejects_raw]
- GIVEN a Preserves dataspace record attempts to carry prompt text, provider payloads, credentials, full private paths, file bodies, raw tool input/output, model output, or unbounded logs
- WHEN dataspace Preserves validation runs
- THEN decoding fails with a typed safe-payload diagnostic
- AND no dataspace assertion, subscription result, orchestration event, success receipt, or compatibility metadata echoes the raw material.

### Requirement: Dataspace Preserves round trips

r[agent_harness_runtime.preserves_dataspace.round_trip] Dataspace Preserves work MUST include positive text and binary round-trip fixtures for valid scope, assertion, subscription, cancellation, expiry, and ownership-transfer records.

#### Scenario: Valid dataspace fixture round trips

r[agent_harness_runtime.preserves_dataspace.round_trip.valid]
- GIVEN a valid dataspace DTO fixture with safe scope and redaction metadata
- WHEN it is encoded and decoded through supported Preserves text and binary forms
- THEN the decoded typed DTO is equivalent to the source value
- AND ordering, scope, status class, redaction markers, and safe hashes are preserved.

### Requirement: Dataspace Preserves malformed rejection

r[agent_harness_runtime.preserves_dataspace.malformed] Dataspace Preserves work MUST include negative fixtures for unsupported versions, wrong schema families, missing required fields, unsafe compatibility metadata, cross-scope subscriptions, raw sensitive material, and unbounded payloads.

#### Scenario: Malformed dataspace payload fails safely

r[agent_harness_runtime.preserves_dataspace.malformed.rejects]
- GIVEN a malformed, wrong-version, wrong-family, cross-scope, unsafe, or unbounded dataspace Preserves payload
- WHEN boundary decoding runs
- THEN decoding fails with a typed validation or compatibility error
- AND no partial assertion publish, subscription match, orchestration event, or live transport side effect occurs.

### Requirement: Dataspace Preserves core isolation

r[agent_harness_runtime.preserves_dataspace.core_isolation] Dataspace Preserves support MUST NOT add Preserves dependencies to `animus-core`, `animus-adapters`, default `animus-runtime`, or default `animus-std` dependency graphs.

#### Scenario: Optional bridge does not leak dependencies

r[agent_harness_runtime.preserves_dataspace.core_isolation.no_leak]
- GIVEN dependency isolation checks inspect reusable default crates
- WHEN dataspace Preserves support is present in the workspace
- THEN Preserves serialization dependencies remain isolated to the optional Preserves bridge or explicit opt-in facade features
- AND implementation-free dataspace adapter contracts remain usable without Preserves.

### Requirement: Dataspace Preserves documentation and closeout

r[agent_harness_runtime.preserves_dataspace.closeout] Dataspace Preserves work MUST update schema notes and facade inventory, run positive and negative fixtures, run dependency-isolation checks, run workspace checks, and record Cairn gate evidence before archive.

#### Scenario: Dataspace Preserves evidence is reviewable

r[agent_harness_runtime.preserves_dataspace.closeout.evidence]
- GIVEN dataspace Preserves work is ready to archive
- WHEN reviewers inspect closeout evidence
- THEN evidence names schema coverage, typed decoding, safe-payload rejection, round-trip fixtures, malformed fixtures, dependency isolation, workspace checks, and Cairn gates
- AND any skipped live-transport validation is marked as out of scope rather than a release claim.

### Requirement: Optional Syndicate host adapter

r[agent_harness_runtime.syndicate_host_adapter.optional] Animus MUST keep any Syndicate-compatible live dataspace integration behind an optional host-owned adapter or non-default feature, and reusable core/runtime/adapter crates MUST NOT construct live Syndicate backends by default.

#### Scenario: Runtime publishes through injected host service

r[agent_harness_runtime.syndicate_host_adapter.optional.injected]
- GIVEN a host enables Syndicate-compatible dataspace coordination
- WHEN runtime or host shell code wants to publish or observe dataspace state
- THEN it communicates through an injected `animus_adapters::dataspace::AdapterService` implementation
- AND default `animus-core`, `animus-runtime`, and `animus-adapters` builds do not require Syndicate crates, daemon clients, network transports, credentials, or live daemon availability.

### Requirement: Redacted coordination projection

r[agent_harness_runtime.syndicate_host_adapter.projection] The optional host adapter MUST publish only bounded redacted assertions derived from approved runtime, headless, tool lifecycle, background-work, session/provenance, and agent/subagent observations.

#### Scenario: Runtime observation becomes safe assertion

r[agent_harness_runtime.syndicate_host_adapter.projection.redacted]
- GIVEN an observation contains prompt text, provider payloads, credentials, full private paths, file bodies, tool input/output, model output, or unbounded logs
- WHEN projection builds a dataspace assertion draft
- THEN the assertion contains only stable IDs, normalized scope labels, status classes, assertion kinds, counts, BLAKE3 hashes, and explicit redaction markers
- AND raw sensitive material is omitted or rejected before publication.

### Requirement: Scoped subscription and orchestration handling

r[agent_harness_runtime.syndicate_host_adapter.orchestration] The optional host adapter MUST validate safe subscription patterns and host orchestration events for cancellation, expiry, publish acknowledgement, and ownership transfer before forwarding them to a live or fake dataspace backend.

#### Scenario: Cancellation is scoped to one owner

r[agent_harness_runtime.syndicate_host_adapter.orchestration.scoped_cancellation]
- GIVEN a cancellation event targets one tenant, workspace, branch, session, facet, actor, or assertion owner
- WHEN the host adapter handles the event
- THEN only matching scoped assertions are cancelled, expired, or acknowledged
- AND unrelated tenants, workspaces, branches, sessions, facets, actors, and owners remain unchanged.

### Requirement: Safe unavailable-backend behavior

r[agent_harness_runtime.syndicate_host_adapter.failure] The optional host adapter MUST fail safely when live transport configuration is absent, a daemon is unavailable, credentials are missing, subscriptions are unsupported, leases are stale, or ownership-transfer identities do not match.

#### Scenario: Missing live backend does not affect default runtime

r[agent_harness_runtime.syndicate_host_adapter.failure.unavailable_backend]
- GIVEN live Syndicate transport is not configured or the configured backend is unavailable
- WHEN default runtime tests, fake-backend tests, or host startup without the optional feature run
- THEN no live connection is attempted by default
- AND any explicit live-adapter failure is reported as a typed safe diagnostic without raw credentials, transport payloads, or private paths.

### Requirement: Syndicate dependency isolation

r[agent_harness_runtime.syndicate_host_adapter.dependency_isolation] Animus MUST check that Syndicate crates, daemon clients, live transport code, and host credential plumbing remain outside default reusable core/runtime/adapter dependency graphs.

#### Scenario: Live transport dependency leaks into default graph

r[agent_harness_runtime.syndicate_host_adapter.dependency_isolation.rejects_leak]
- GIVEN a Syndicate crate, daemon client, network transport, or host credential dependency is added to a default reusable core/runtime/adapter crate
- WHEN dependency-isolation checks run
- THEN the check fails with an actionable diagnostic
- AND maintainers are directed to the optional host dataspace adapter boundary.

### Requirement: Syndicate host adapter closeout evidence

r[agent_harness_runtime.syndicate_host_adapter.closeout] Syndicate host adapter work MUST document host configuration, fake-backend release evidence, optional live smoke-test gating, dependency isolation, focused tests, workspace checks, and Cairn gates before archive.

#### Scenario: Host adapter evidence is reviewable

r[agent_harness_runtime.syndicate_host_adapter.closeout.evidence]
- GIVEN optional Syndicate host adapter work is ready to archive
- WHEN reviewers inspect closeout evidence
- THEN evidence names redacted projection tests, safe subscription/orchestration tests, unavailable-backend tests, dependency-isolation checks, fake-backend tests, optional live smoke-test status, workspace checks, and Cairn gates
- AND any skipped live transport rail includes a concrete blocker, explicit opt-in requirement, and next-best fake-backend validation.

### Requirement: Pure model dispatch plan

r[agent_harness_runtime.model_dispatch.plan] Model/provider dispatch MUST have a pure planning DTO that records selected capability identity, request contract identity, prompt or content budget, stream policy, receipt policy, cancellation or timeout posture, and fallback policy before adapter execution.

#### Scenario: Model dispatch plan validates before provider effects

r[agent_harness_runtime.model_dispatch.plan.validation]
- GIVEN runtime prepares a model/provider call from validated request metadata and host policy inputs
- WHEN model dispatch planning runs
- THEN valid dispatch plans are admitted with safe identity and budget evidence
- AND missing providers, denied capability, budget overflow, unsupported stream policy, malformed request shape, missing receipt policy, cancellation, timeout, and fallback-denied cases fail before provider effects.

### Requirement: Runtime gates model adapter calls on admitted plans

r[agent_harness_runtime.model_dispatch.runtime_gate] Runtime MUST call model adapters only after an admitted model dispatch plan and MUST map denied plans to typed fail-closed outcomes.

#### Scenario: Denied model plan prevents adapter call

r[agent_harness_runtime.model_dispatch.runtime_gate.fail_closed]
- GIVEN a model dispatch plan is denied because capability, budget, stream, receipt, cancellation, timeout, or fallback policy fails
- WHEN runtime processes the model decision
- THEN the model adapter is not called
- AND runtime returns a typed fail-closed outcome without completed receipts, provider discovery, credential reads, filesystem scans, process spawning, plugin loading, or network fallback.

### Requirement: Model dispatch compatibility defaults

r[agent_harness_runtime.model_dispatch.compatibility] Existing simple model adapter behavior MUST remain available through compatibility defaults for request identity, provider-neutral capability, prompt budget, stream policy, receipt policy, and fallback denial.

#### Scenario: Existing model adapter path matches planned dispatch

r[agent_harness_runtime.model_dispatch.compatibility.equivalent]
- GIVEN one runtime loop uses the existing simple model adapter path and another loop uses explicit model dispatch planning with compatibility defaults
- WHEN both receive equivalent fabric decisions and deterministic model responses
- THEN both produce equivalent successful reports and events
- AND neither path gains additional provider, filesystem, process, network, credential, or plugin authority.

### Requirement: Model dispatch planning verification

r[agent_harness_runtime.model_dispatch.verification] Model/provider dispatch planning MUST include focused positive and negative tests for planning validation, runtime gating, compatibility defaults, no-fallback behavior, provider-contract docs, facade inventory, workspace checks, and Cairn gates before archive.

#### Scenario: Closeout evidence covers model dispatch planning

r[agent_harness_runtime.model_dispatch.verification.closeout]
- GIVEN maintainers intend to archive model/provider dispatch planning
- WHEN closeout validation runs
- THEN evidence names planning validation, runtime gating, compatibility defaults, fail-closed, no-fallback, provider-contract docs, inventory, workspace, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Typed adapter call envelope

r[agent_harness_runtime.adapter_outcomes.typed_envelope] Animus adapter calls MUST be able to return a typed envelope that distinguishes completed responses from absent, denied, unavailable, malformed, cancelled, timed-out, and execution-failure outcomes without relying on prose parsing.

#### Scenario: Adapter outcome classes validate before runtime mapping

r[agent_harness_runtime.adapter_outcomes.typed_envelope.validation]
- GIVEN a model or tool adapter returns a typed outcome envelope
- WHEN envelope validation runs
- THEN completed responses and supported fail-closed outcome classes are accepted with bounded safe metadata
- AND unsafe message text, unsafe receipt metadata, raw credentials, raw paths, raw provider payloads, and unbounded bodies are rejected before runtime mapping.

### Requirement: Legacy adapter compatibility

r[agent_harness_runtime.adapter_outcomes.compatibility] Existing `ComponentAdapter::call` implementations MUST remain compatible or have explicit migration helpers, and typed adapter outcomes MUST be additive rather than required for current embedders.

#### Scenario: Legacy and typed adapters preserve equivalent success behavior

r[agent_harness_runtime.adapter_outcomes.compatibility.equivalent]
- GIVEN one runtime loop uses a legacy adapter and another loop uses a typed-outcome adapter for equivalent successful requests
- WHEN both adapters return equivalent validated responses
- THEN runtime reports equivalent completed behavior
- AND neither path gains ambient provider, filesystem, process, network, credential, or plugin authority.

### Requirement: Runtime preserves adapter outcome classes

r[agent_harness_runtime.adapter_outcomes.runtime_mapping] Runtime dispatch MUST preserve typed adapter outcome classes in reports and receipts when supplied, and MUST map legacy display errors only to conservative execution-failure diagnostics.

#### Scenario: Fail-closed outcomes do not become string-only success claims

r[agent_harness_runtime.adapter_outcomes.runtime_mapping.fail_closed]
- GIVEN a typed adapter reports denied, unavailable, malformed, cancelled, timed-out, or execution-failure outcome
- WHEN runtime normalizes the dispatch result
- THEN the report carries the matching fail-closed class or safe outcome kind
- AND no success receipt, provenance fact, fallback provider call, shell command, environment read, filesystem scan, or network request is produced.

### Requirement: Adapter outcome verification

r[agent_harness_runtime.adapter_outcomes.verification] The typed adapter outcome change MUST include focused positive and negative tests for envelope validation, compatibility adapters, runtime mapping, leak rejection, no ambient fallback, facade inventory, workspace checks, and Cairn gates before archive.

#### Scenario: Closeout evidence covers adapter outcome hardening

r[agent_harness_runtime.adapter_outcomes.verification.closeout]
- GIVEN maintainers intend to archive typed adapter outcome hardening
- WHEN closeout validation runs
- THEN evidence names envelope validation, compatibility, runtime mapping, fail-closed, leak-rejection, no-fallback, inventory, workspace, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Required capability identity

r[agent_harness_runtime.scoped_capabilities.required_identity] Model and tool dispatch MUST declare the specific required capability identity, including stable capability label, service kind, and optional safe scope, before runtime admits an adapter call.

#### Scenario: Capability identity validates before dispatch

r[agent_harness_runtime.scoped_capabilities.required_identity.validation]
- GIVEN runtime prepares a model or tool dispatch request
- WHEN required capability identity validation runs
- THEN default and explicit scoped capability labels are accepted with supported service kinds and safe scope labels
- AND empty labels, unsupported service kinds, unsafe scope text, and duplicate labels fail before adapter dispatch.

### Requirement: Exact registry resolution

r[agent_harness_runtime.scoped_capabilities.exact_resolution] Runtime dispatch MUST resolve the declared capability label and service kind exactly, and MUST NOT treat the presence of any service with the same broad kind as sufficient authority.

#### Scenario: Unrelated service cannot satisfy scoped request

r[agent_harness_runtime.scoped_capabilities.exact_resolution.fail_closed]
- GIVEN a host registers a model or tool service that does not match the required capability label, service kind, or safe scope
- WHEN runtime attempts dispatch
- THEN dispatch fails closed with a typed missing or mismatched capability diagnostic
- AND the unrelated service is not called, no fallback service is discovered, and no completed receipt is produced.

### Requirement: Compatibility default capabilities

r[agent_harness_runtime.scoped_capabilities.compatibility_defaults] Existing model/tool runtime constructors MUST remain available as thin wrappers that bind adapters to documented default capability labels and no unsafe scope.

#### Scenario: Default constructor behavior remains equivalent

r[agent_harness_runtime.scoped_capabilities.compatibility_defaults.equivalent]
- GIVEN a runtime loop uses the existing model/tool constructor path and another loop uses explicit default capability descriptors
- WHEN both loops receive equivalent fabric decisions and adapter responses
- THEN they produce equivalent reports and events
- AND both paths perform exact capability resolution without ambient provider, filesystem, process, network, credential, or plugin fallback.

### Requirement: Scoped capability verification

r[agent_harness_runtime.scoped_capabilities.verification] The scoped capability resolution change MUST include focused positive and negative tests for identity validation, exact resolution, compatibility defaults, unrelated-service denial, no ambient fallback, facade inventory, workspace checks, and Cairn gates before archive.

#### Scenario: Closeout evidence covers scoped capability resolution

r[agent_harness_runtime.scoped_capabilities.verification.closeout]
- GIVEN maintainers intend to archive scoped capability resolution
- WHEN closeout validation runs
- THEN evidence names identity validation, exact resolution, compatibility defaults, fail-closed, no-fallback, inventory, workspace, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Stable boundary contract identities

r[agent_harness_runtime.boundary_versions.contract_identities] Public model, tool, model-stream, capability, runtime-event, and safe-receipt boundary DTOs MUST expose stable contract identity strings that can be used in docs, receipts, fixtures, and optional serialization bridges.

#### Scenario: Boundary identity is available without concrete adapters

r[agent_harness_runtime.boundary_versions.contract_identities.validation]
- GIVEN an embedder depends on reusable Animus boundary crates
- WHEN they construct or inspect public model, tool, stream, capability, runtime-event, or receipt DTOs
- THEN each maintained DTO exposes a non-empty stable contract identity
- AND obtaining that identity does not require concrete providers, filesystem access, network access, process execution, credentials, or plugin loading.

### Requirement: Explicit text normalization policy

r[agent_harness_runtime.boundary_versions.normalized_text] Adapter-boundary DTO constructors MUST store normalized text for safe labels, names, identifiers, schema identities, capability labels, and receipt metadata, while deliberately raw bodies MUST use explicit bounded raw/body policy.

#### Scenario: Safe labels normalize while raw bodies remain deliberate

r[agent_harness_runtime.boundary_versions.normalized_text.validation]
- GIVEN boundary constructor input contains leading or trailing whitespace, NUL characters, overlong text, or raw body content
- WHEN DTO validation runs
- THEN safe labels and identifiers are trimmed and stored in normalized form
- AND NUL characters, overlong text, and unsafe raw text leakage are rejected unless a bounded raw/body policy explicitly admits the body.

### Requirement: Supported version admission

r[agent_harness_runtime.boundary_versions.version_admission] DTO consumers that parse contract identity values MUST admit supported versions explicitly and MUST fail closed on missing or unsupported versions unless a reviewed compatibility mapping exists.

#### Scenario: Unsupported version fails before dispatch

r[agent_harness_runtime.boundary_versions.version_admission.fail_closed]
- GIVEN a host, serializer, replay, or adapter bridge receives a DTO with a missing or unsupported contract identity
- WHEN version admission runs
- THEN parsing fails with a typed version diagnostic
- AND no adapter dispatch, success receipt, or serialization compatibility claim is produced from the unsupported shape.

### Requirement: Boundary versioning verification

r[agent_harness_runtime.boundary_versions.verification] Boundary versioning and normalization MUST include focused positive and negative tests for identity availability, normalization, raw-body policy, unsupported version rejection, Preserves fixtures where applicable, facade inventory, workspace checks, and Cairn gates before archive.

#### Scenario: Closeout evidence covers boundary versioning

r[agent_harness_runtime.boundary_versions.verification.closeout]
- GIVEN maintainers intend to archive boundary versioning and normalization
- WHEN closeout validation runs
- THEN evidence names contract identities, normalization, raw-body policy, version admission, Preserves fixtures where applicable, inventory, workspace, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Canonical model message request

r[agent_harness_runtime.model_compat.message_request] Animus MUST define a versioned provider-neutral model message request contract that carries ordered model messages, stable role labels, typed content parts, output controls, tool declarations, response-format intent, reasoning or thinking request metadata, contract identity, and safe provenance metadata without embedding raw provider payloads.

#### Scenario: Canonical request validation admits structured conversations

r[agent_harness_runtime.model_compat.message_request.validation]
- GIVEN a host or runtime shell prepares a model request with ordered system, developer, user, assistant, or tool messages and validated text or descriptor-backed content parts
- WHEN canonical model request validation runs
- THEN valid requests are admitted with stable contract identity and bounded metadata
- AND malformed roles, empty messages, unsupported content parts, oversized metadata, raw provider payloads, credentials, and missing required fields fail before provider dispatch.

### Requirement: Canonical model response envelope

r[agent_harness_runtime.model_compat.response_envelope] Animus MUST define a versioned provider-neutral model response envelope that can represent assistant-visible text, structured payloads, tool-call requests, refusal or safety blocks, usage metadata, finish reasons, and malformed-output diagnostics without exposing raw provider payloads.

#### Scenario: Response semantics survive provider normalization

r[agent_harness_runtime.model_compat.response_envelope.validation]
- GIVEN a provider or fixture adapter returns assistant text, a structured payload, a tool-call request, a refusal, usage metadata, or a terminal finish reason
- WHEN response normalization validates the canonical envelope
- THEN supported response semantics are preserved as typed data
- AND missing finish reasons, malformed usage counts, impossible response combinations, raw provider payloads, credentials, and unbounded hidden reasoning fail closed.

### Requirement: Legacy model DTO bridge

r[agent_harness_runtime.model_compat.legacy_bridge] Existing simple prompt/message model adapter behavior MUST remain available through pure compatibility projections between legacy DTOs and canonical model request/response contracts, and those projections MUST NOT add provider, filesystem, network, credential, process, or plugin authority.

#### Scenario: Legacy prompt path is equivalent under compatibility defaults

r[agent_harness_runtime.model_compat.legacy_bridge.equivalent]
- GIVEN one runtime path dispatches a legacy prompt/message DTO and another dispatches the equivalent canonical single-user-message request with compatibility defaults
- WHEN both receive deterministic assistant-text responses
- THEN both produce equivalent successful reports and events
- AND canonical responses carrying structured output, tool calls, refusal status, or other non-text semantics fail rather than being silently collapsed into legacy message text.

### Requirement: Canonical model contract verification

r[agent_harness_runtime.model_compat.verification] Canonical model contract work MUST include focused positive and negative tests for request validation, response validation, legacy bridge equivalence, lossy projection rejection, safe metadata redaction, boundary schema notes, runtime facade inventory, workspace checks, and Cairn gates before archive.

#### Scenario: Closeout evidence covers canonical model contracts

r[agent_harness_runtime.model_compat.verification.closeout]
- GIVEN maintainers intend to archive canonical model message contract work
- WHEN closeout validation runs
- THEN evidence names request validation, response validation, legacy bridge equivalence, lossy projection rejection, safe metadata redaction, docs, schema notes, facade inventory, workspace checks, and Cairn gates
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Provider/model capability matrix

r[agent_harness_runtime.model_capability_matrix.registry] Provider/model metadata MUST expose a typed capability matrix for canonical model request compatibility, including supported message roles, content part classes, tool-call behavior, structured-output modes, stream channels, reasoning controls, context and output budgets, privacy or locality posture, and capability schema identity.

#### Scenario: Registry metadata declares compatibility features

r[agent_harness_runtime.model_capability_matrix.nickel_config]
- GIVEN Animus-owned provider registry configuration describes one or more selectable models
- WHEN Nickel export and Rust-side capability validation run
- THEN every selectable model has bounded typed compatibility metadata for the canonical request features it claims
- AND missing, unknown, contradictory, or malformed required capability fields fail before runtime can select that model.

### Requirement: Pure capability negotiation

r[agent_harness_runtime.model_capability_matrix.negotiation] Animus MUST negotiate canonical model request requirements against the selected model capability matrix through a pure deterministic function that returns admitted, downgraded, or denied compatibility decisions with safe evidence.

#### Scenario: Supported or optional features resolve deterministically

r[agent_harness_runtime.model_capability_matrix.negotiation.positive]
- GIVEN a canonical model request declares required and optional feature requirements
- AND the selected model capability matrix supports every required feature and supports or policy-downgrades optional features
- WHEN capability negotiation runs
- THEN the result is admitted or explicitly downgraded with safe feature labels, policy labels, and bounded budget evidence
- AND no provider call, credential read, network probe, filesystem scan, or router fallback occurs during negotiation.

### Requirement: Required unsupported features fail closed

r[agent_harness_runtime.model_capability_matrix.fail_closed] Capability negotiation MUST deny dispatch when a canonical request requires a role, content part, image, tool call, structured-output mode, stream channel, reasoning level, token budget, privacy class, or provider family behavior that the selected model does not support or whose support is unknown.

#### Scenario: Unsupported required feature blocks dispatch

r[agent_harness_runtime.model_capability_matrix.fail_closed.unsupported]
- GIVEN a canonical model request requires a compatibility feature that the selected model metadata lacks, rejects, or marks unknown
- WHEN capability negotiation evaluates the request
- THEN dispatch is denied with a typed unsupported-feature diagnostic
- AND runtime does not silently drop the feature, switch providers, widen authority, claim success receipts, or call a provider adapter.

### Requirement: Capability matrix verification

r[agent_harness_runtime.model_capability_matrix.verification] Model capability matrix work MUST include focused positive and negative tests for Nickel config validation, Rust metadata validation, pure negotiation, optional downgrade evidence, unsupported required features, budget overflow, denied fallback, safe receipts, docs, facade inventory, workspace checks, and Cairn gates before archive.

#### Scenario: Closeout evidence covers capability negotiation

r[agent_harness_runtime.model_capability_matrix.verification.closeout]
- GIVEN maintainers intend to archive capability matrix negotiation
- WHEN closeout validation runs
- THEN evidence names Nickel config validation, Rust metadata validation, pure negotiation, optional downgrade evidence, unsupported required features, budget overflow, denied fallback, safe receipts, docs, facade inventory, workspace checks, and Cairn gates
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Provider projection conformance harness

r[agent_harness_runtime.provider_projection_conformance.harness] Animus MUST define a provider projection conformance harness that validates canonical model request to provider-family request projection and provider-family response to canonical response normalization for every provider family or API style that an adapter claims to support.

#### Scenario: Provider projection preserves canonical request semantics

r[agent_harness_runtime.provider_projection_conformance.literal_fixtures]
- GIVEN a canonical model request contains ordered roles, content parts, tool declarations, structured-output intent, reasoning controls, stream options, and safe metadata supported by a claimed provider family
- WHEN provider projection conformance runs against literal expected provider-native fixtures
- THEN required canonical semantics are preserved in the provider-family request representation
- AND expected fixture data is reviewed literal data rather than output generated by the same builder under test.

### Requirement: Lossy provider projection fails closed

r[agent_harness_runtime.provider_projection_conformance.lossy_projection] Provider projection MUST fail closed when a claimed provider family cannot preserve required canonical request roles, content parts, image descriptors, tool declarations, structured-output intent, reasoning controls, stream options, metadata, or safe provenance.

#### Scenario: Dropped canonical field rejects projection

r[agent_harness_runtime.provider_projection_conformance.lossy_projection.rejects_drop]
- GIVEN a canonical model request requires a field or feature that a provider-family projection would omit, flatten, or misrepresent
- WHEN projection validation runs
- THEN the projection returns a typed lossy-conversion diagnostic
- AND runtime does not send a provider request, silently downgrade the feature, switch providers, or claim a successful dispatch receipt.

### Requirement: Provider response normalization conformance

r[agent_harness_runtime.provider_projection_conformance.response_normalization] Provider-native response fixtures MUST normalize into the canonical model response envelope with typed assistant text, structured payloads, tool-call requests, refusal or safety blocks, usage metadata, finish reasons, and malformed-output diagnostics.

#### Scenario: Provider output maps to canonical response envelope

r[agent_harness_runtime.provider_projection_conformance.response_fail_closed]
- GIVEN a provider-native response fixture contains assistant text, structured output, tool calls, refusal status, usage, finish reason, malformed payloads, hidden reasoning, credentials, or raw provider payload sentinels
- WHEN response normalization conformance runs
- THEN supported semantics are preserved as canonical response envelope data
- AND missing finish reason, malformed usage, impossible response combinations, hidden reasoning leakage, credentials, and raw provider payloads fail closed.

### Requirement: Streaming parser-entrypoint conformance

r[agent_harness_runtime.provider_projection_conformance.streaming_entrypoint] Provider families that claim streaming support MUST include conformance fixtures that feed raw provider stream bytes or chunks through the production parser entrypoint and compare canonical model stream frames.

#### Scenario: Raw provider stream becomes canonical frames

r[agent_harness_runtime.provider_projection_conformance.streaming_entrypoint.fixture]
- GIVEN a claimed streaming provider fixture contains fragmented content, tool-call deltas, usage records, finish, cancellation, failure, malformed records, truncated chunks, or out-of-order terminal events
- WHEN the production parser entrypoint consumes the fixture input
- THEN valid streams produce canonical ordered frames
- AND malformed, truncated, missing-terminal, or out-of-order streams fail with typed diagnostics before final response completion.

### Requirement: Provider projection conformance index

r[agent_harness_runtime.provider_projection_conformance.index] Animus MUST maintain a provider projection conformance index that records covered, unsupported, and not-applicable feature/provider cases with explicit reasons for each claimed provider family.

#### Scenario: Missing conformance reason is rejected

r[agent_harness_runtime.provider_projection_conformance.index.coverage]
- GIVEN a provider family claims support for a canonical request feature, response semantic, or streaming mode
- WHEN conformance index validation runs
- THEN the case is marked covered by fixture evidence or unsupported with a typed reason
- AND not-applicable entries without reasons are rejected.

### Requirement: Provider projection conformance verification

r[agent_harness_runtime.provider_projection_conformance.verification] Provider projection conformance work MUST include focused positive and negative tests for literal request fixtures, lossy projection rejection, response normalization, malformed response failures, streaming parser-entrypoint fixtures, conformance index validation, safe redaction, docs, facade inventory, workspace checks, and Cairn gates before archive.

#### Scenario: Closeout evidence covers provider projection conformance

r[agent_harness_runtime.provider_projection_conformance.verification.closeout]
- GIVEN maintainers intend to archive provider projection conformance work
- WHEN closeout validation runs
- THEN evidence names literal request fixtures, lossy projection rejection, response normalization, malformed response failures, streaming parser-entrypoint fixtures, conformance index validation, safe redaction, docs, facade inventory, workspace checks, and Cairn gates
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Dedicated router crate boundary

r[agent_harness_runtime.model_router.crate_barrier] Animus MUST own a dedicated `animus-router` workspace crate that is `#![no_std]` with `alloc` only and that owns the routing decision surface (recipe/role/output-contract DTOs, request-signal projection, recipe selection, execution-plan materialization, route-result aggregation, safe receipt assembly, and router-specific field validation), while `animus-adapters` keeps implementation-free model/tool/port contracts and exposes only a transition re-export shim for router items.

#### Scenario: Embedders depend on the router without all adapter contracts

r[agent_harness_runtime.model_router.crate_barrier.smallest_dependency]
- GIVEN an embedder needs only deterministic routing decisions
- WHEN they select workspace crates
- THEN they can depend on `animus-router` without pulling model, tool, host-effect, provider, filesystem, process, network, terminal, credential, or persistence behavior
- AND the crate does not depend on model, tool, host-effect, provider, filesystem, process, network, terminal, credential, or persistence crates
- AND the only temporary dependency direction for compatibility is `animus-adapters` depending on `animus-router` for the re-export shim.

#### Scenario: Router crate stays pure and Wasm-eligible

r[agent_harness_runtime.model_router.crate_barrier.no_std_pure]
- GIVEN the `animus-router` crate is compiled
- WHEN the no-std purity smoke and Wasm surface rail run
- THEN the crate contains no `std::fs`, `std::env`, `std::process`, `std::time`, `println!`, `eprintln!`, `tokio`, `async`, filesystem traversal, environment reads, process execution, or JSON rendering helpers
- AND `cargo check -p animus-router --target wasm32-unknown-unknown --locked` passes.

### Requirement: Transition re-export preserves the public surface

r[agent_harness_runtime.model_router.crate_barrier.reexport] `animus-adapters::router` MUST re-export the moved `animus-router` public items for a transition window so existing dependents compile without churn, and a parity test MUST assert the re-exported surface is item-for-item identical to the moved module.

#### Scenario: Existing call sites compile unchanged

r[agent_harness_runtime.model_router.crate_barrier.reexport.unchanged]
- GIVEN call sites import router items from `animus_adapters::router`
- WHEN the module is moved into `animus-router` and a re-export shim is added
- THEN the workspace compiles without editing those call sites
- AND a parity test confirms every re-exported item resolves to the moved `animus-router` item with identical signatures.

### Requirement: Router crate barrier verification

r[agent_harness_runtime.model_router.crate_barrier.verification] The crate barrier change MUST include focused positive and negative tests for workspace registration, re-export parity, Wasm surface eligibility, no-std purity, conformance-index registration, facade inventory, and receipt-free closeout before archive.

#### Scenario: Closeout evidence covers the crate barrier

r[agent_harness_runtime.model_router.crate_barrier.verification.closeout]
- GIVEN maintainers intend to archive the router crate barrier change
- WHEN closeout validation runs
- THEN evidence names focused crate-registration, re-export parity, Wasm-surface, purity-smoke, conformance-index, facade-inventory, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Current-shape tool calls only

r[agent_harness_runtime.tool_calling_breaking.current_shape] Runtime tool calls MUST use a current structured contract carrying tool name, schema identity, validated arguments, correlation identity, output policy, declared mutation targets, required capability labels, and approval posture before dispatch, and MUST NOT admit legacy raw string input or raw string output as the runtime tool-call contract.

#### Scenario: Legacy raw tool input is rejected

r[agent_harness_runtime.tool_calling_breaking.current_shape.rejects_legacy]
- GIVEN a loop decision, resumed ledger entry, Preserves payload, or headless request supplies only a tool name plus raw input string
- WHEN runtime validates the tool call for dispatch
- THEN validation returns a typed current-shape-required diagnostic
- AND no tool executor, shell command, provider call, plugin runtime, filesystem scan, credential read, network connection, mutation coordinator, or fallback compatibility preparer is invoked.

### Requirement: Catalog-backed tool admission

r[agent_harness_runtime.tool_calling_breaking.catalog_admission] Runtime tool prompt construction and dispatch admission MUST derive from active trusted tool definitions supplied by an Animus-owned catalog, manifest projection, or host-injected definition set, and MUST reject tools that are unknown, inactive, untrusted, schema-invalid, executor-less, or capability-denied before execution.

#### Scenario: Exact capability gates each tool

r[agent_harness_runtime.tool_calling_breaking.catalog_admission.exact_capability]
- GIVEN an active tool definition declares required capability labels and a host capability registry
- WHEN runtime plans dispatch for that tool
- THEN every required capability identity is resolved exactly with service kind and safe scope before execution
- AND a broad default tool capability does not authorize the call when the per-tool capability is absent.

### Requirement: Pure tool dispatch plan

r[agent_harness_runtime.tool_calling_breaking.dispatch_plan] Tool dispatch MUST have a pure planning DTO that records definition identity, request contract identity, schema identity, validated argument status, required capabilities, approval decision, output policy, mutation-window plan, cancellation posture, timeout posture, and fallback policy before any executor effect.

#### Scenario: Denied plan fails closed before execution

r[agent_harness_runtime.tool_calling_breaking.dispatch_plan.fail_closed]
- GIVEN a tool dispatch plan is denied because request shape, catalog membership, executor presence, capability, approval, output policy, mutation posture, cancellation, timeout, or fallback policy fails
- WHEN runtime processes the tool decision
- THEN the tool executor is not called
- AND runtime returns a typed fail-closed outcome without completed receipts, provider discovery, credential reads, filesystem scans, process spawning, plugin loading, network fallback, or raw output events.

### Requirement: Bounded tool result contract

r[agent_harness_runtime.tool_calling_breaking.bounded_result] Completed tool execution MUST return a bounded runtime-visible result containing model-visible text, completion or truncation status, byte count, character count, line count, redaction class, BLAKE3 content hash when required, and optional host-owned full-output handle, and MUST NOT store raw full output bodies in runtime state, runtime events, reports, receipts, Preserves event payloads, or headless projections.

#### Scenario: Raw tool body cannot enter runtime evidence

r[agent_harness_runtime.tool_calling_breaking.bounded_result.no_raw_body]
- GIVEN a tool executor produces an oversized body, unsafe full-output reference, NUL-containing output, or secret-adjacent output body
- WHEN runtime validates the completed tool result
- THEN it either returns a bounded result with safe visible text and BLAKE3 identity metadata or rejects the result with a typed malformed or unsafe-output outcome
- AND raw full output, private absolute paths, credentials, provider payloads, and unbounded stdout or stderr are not embedded in model-visible context or evidence.

### Requirement: Typed tool execution outcomes

r[agent_harness_runtime.tool_calling_breaking.typed_outcomes] Tool execution services MUST return typed completed or fail-closed outcomes for admitted calls, preserving invalid input, missing executor, unavailable service, denied capability, approval denial, cancelled, timed out, malformed output, unsafe output reference, unbounded oversized output, mutation conflict, and execution failure as distinct stable classes.

#### Scenario: Failure class survives runtime projection

r[agent_harness_runtime.tool_calling_breaking.typed_outcomes.projection]
- GIVEN an admitted tool executor reports denied, unavailable, cancelled, timed-out, malformed-output, unsafe-reference, unbounded-output, or execution-failure status
- WHEN runtime projects the result into events, reports, receipts, Preserves, headless, or TUI surfaces
- THEN the stable failure class is preserved with bounded safe metadata
- AND runtime does not collapse the failure into an ambiguous success, raw prose-only error, or completed tool response.

### Requirement: Mutation-window dispatch scheduler

r[agent_harness_runtime.tool_calling_breaking.mutation_scheduler] Tool dispatch coordination MUST evaluate declared opaque mutation targets before execution, serialize conflicting mutation windows deterministically, and allow independent read-only or non-overlapping tool calls only when their admitted plans cannot race on declared mutation targets.

#### Scenario: Conflicting mutation targets serialize

r[agent_harness_runtime.tool_calling_breaking.mutation_scheduler.conflict]
- GIVEN two admitted tool calls declare the same mutation target and a third admitted tool call is read-only or targets a distinct resource key
- WHEN runtime asks the host-owned coordinator for dispatch windows
- THEN the conflicting mutation calls are serialized in deterministic plan order
- AND the read-only or non-overlapping call may be admitted independently without observing a stale state caused by the serialized mutation pair.

### Requirement: Breaking tool-call verification

r[agent_harness_runtime.tool_calling_breaking.verification] The breaking tool-call replacement MUST update adapter contracts, runtime dispatch, runtime event/report projections, Preserves/headless/TUI surfaces, lifecycle docs, focused rails, and tests so the current-only structured contract is the only accepted tool-call shape.

#### Scenario: Closeout proves legacy path removal

r[agent_harness_runtime.tool_calling_breaking.verification.closeout]
- GIVEN maintainers intend to archive the breaking tool-call replacement
- WHEN closeout validation runs
- THEN evidence names positive structured-call coverage and negative coverage for legacy raw payloads, unknown tools, inactive tools, missing executors, denied capabilities, malformed arguments, unsafe output, unbounded output, typed outcome projection, and mutation conflicts
- AND any skipped rail includes a concrete blocker and next-best validation command.

### Requirement: Nickel-authored router recipe policy

r[agent_harness_runtime.model_router.policy_contracts] Animus router recipe policy MUST be authored as Nickel contracts defining allowed route families, valid role sets per family, output-contract compatibility per role, bounds floors and ceilings, allowed fallback policies, and backend allowlist labels, and runtime Rust MUST consume checked-in generated policy JSON without executing Nickel at runtime.

#### Scenario: Generated policy stays in sync

r[agent_harness_runtime.model_router.policy_contracts.drift]
- GIVEN a maintained router-recipe Nickel source and generated runtime fixture
- WHEN drift validation runs
- THEN generated data matches the Nickel source
- AND schema-invalid recipes fail before runtime use.

### Requirement: Router conformance-index fixtures

r[agent_harness_runtime.model_router.policy_contracts.fixtures] Animus MUST maintain a router conformance-index fixture set with one positive recipe per family and negative fixtures for unknown family, invalid role combo, out-of-range bound, disallowed fallback, and unknown backend.

#### Scenario: Invalid role combo rejected

r[agent_harness_runtime.model_router.policy_contracts.fixtures.invalid_role]
- GIVEN a recipe fixture pairs roles disallowed for its family by Nickel policy
- WHEN conformance validation runs
- THEN the fixture is rejected with a typed diagnostic
- AND no runtime recipe is produced from it.

### Requirement: Router policy verification

r[agent_harness_runtime.model_router.policy_contracts.verification] The change MUST include focused positive and negative tests for Nickel validation, fixture drift, role-combo rejection, bound-range enforcement, fallback policy, backend allowlists, runtime sync, workspace checks, Cairn validation, and Cairn gates before archive.

#### Scenario: Closeout evidence covers router policy

r[agent_harness_runtime.model_router.policy_contracts.verification.closeout]
- GIVEN maintainers intend to archive the router policy change
- WHEN closeout validation runs
- THEN evidence names focused Nickel, fixture, drift, runtime-sync, facade-inventory, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Pure request-signal projection from bounded metadata

r[agent_harness_runtime.model_router.request_projection] Animus MUST provide a pure deterministic `project_signals` function that derives the eight router signal bands (task shape, risk, contract pressure, context pressure, latency, cost, privacy/locality, fallback eligibility) from bounded `RequestMetadata` using named threshold constants, without requiring raw prompt bodies, provider payloads, credentials, or unbounded text.

#### Scenario: Reasoning-heavy request projects to a collaborative band

r[agent_harness_runtime.model_router.request_projection.reasoning]
- GIVEN request metadata has a large prompt estimate, no tool calls, and a remote-allowed privacy policy
- WHEN `project_signals` runs
- THEN the task shape is reasoning and risk is high or strict
- AND no raw prompt body is consumed or emitted.

#### Scenario: Missing privacy policy fails closed

r[agent_harness_runtime.model_router.request_projection.missing_privacy]
- GIVEN request metadata omits a privacy policy label
- WHEN `project_signals` runs
- THEN it returns a `MissingPrivacyPolicy` error
- AND no `SignalDraft` is produced.

### Requirement: Pluggable SignalSource adapter seam

r[agent_harness_runtime.model_router.request_projection.signal_source] Animus MUST define an implementation-free `SignalSource` trait in `animus-router` (re-exported through `animus-adapters`) that returns a `SignalDraft` from `RequestMetadata`, and the rule-based projector MUST be the default `SignalSource` implementation so hosts may inject richer classifiers without changing router core.

#### Scenario: Host classifier overrides the rule projector

r[agent_harness_runtime.model_router.request_projection.signal_source.override]
- GIVEN a host injects a custom `SignalSource`
- WHEN the router requests signals
- THEN it calls the injected source's `project`
- AND the rule projector is used only when no source is injected.

### Requirement: Request-signal projection verification

r[agent_harness_runtime.model_router.request_projection.verification] The change MUST include focused positive and negative tests for each band derivation, the default `SignalSource` end-to-end path, malformed metadata, missing privacy policy, unsafe-payload rejection, and bounded-oversized inputs before archive.

#### Scenario: Closeout evidence covers signal projection

r[agent_harness_runtime.model_router.request_projection.verification.closeout]
- GIVEN maintainers intend to archive the request-signal projection change
- WHEN closeout validation runs
- THEN evidence names focused band-derivation, signal-source, fail-closed, facade-inventory, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Runtime uses typed subagent capabilities

r[agent_harness_runtime.subagent_typed_capabilities.types] Subagent runtime decisions MUST represent capabilities as typed values for context reads, search, named tool use, named mutation intents, workspace access, model use, and host-admitted external services rather than relying only on unstructured strings.

#### Scenario: Valid catalog label becomes typed capability

- GIVEN a capability catalog contains a valid label for a read-only search tool
- WHEN adapter capability labels are converted for runtime resolution
- THEN the label becomes a typed search or named-tool capability
- AND the original stable label remains available for receipts and diagnostics.

### Requirement: Capability intersections report denial source

r[agent_harness_runtime.subagent_typed_capabilities.intersection] Effective subagent capability resolution MUST intersect typed parent policy, recipe or profile ceilings, request ceilings, workspace scope, and host admission, and MUST report which source denied each rejected capability.

#### Scenario: Workspace scope denies mutation intent

- GIVEN parent policy and host admission allow a mutation intent but the workspace scope is read-only
- WHEN a subagent requests the mutation intent
- THEN capability resolution denies the typed capability with a workspace-scope denial
- AND the child run cannot dispatch the mutation.

### Requirement: String labels remain boundary compatible

r[agent_harness_runtime.subagent_typed_capabilities.compatibility] Animus MUST preserve stable capability string labels at adapter, fixture, policy, and receipt boundaries while requiring catalog validation before those labels influence typed runtime decisions.

#### Scenario: Receipt keeps stable capability label

- GIVEN a typed capability is accepted for a subagent run
- WHEN a safe receipt is produced
- THEN the receipt records the stable catalog label and safe capability class
- AND it does not expose internal enum debug output as the public contract.

### Requirement: Capability conversion fails closed

r[agent_harness_runtime.subagent_typed_capabilities.fail_closed] Capability conversion and resolution MUST reject unknown labels, duplicate normalized labels, wildcard authority, malformed catalog entries, unauthorized tools, unsupported capability classes, and label collisions before child-run dispatch.

#### Scenario: Wildcard label is rejected

- GIVEN a subagent request includes a capability label that normalizes to wildcard authority
- WHEN capability conversion runs
- THEN conversion returns an overbroad-authority diagnostic
- AND no typed capability is admitted for dispatch.

### Requirement: Typed capability closeout is evidenced

r[agent_harness_runtime.subagent_typed_capabilities.verification] Typed subagent capability adoption MUST include positive conversion/intersection tests, negative fail-closed tests, compatibility fixture tests, docs, Cairn validation, and Cairn gates before archive.

#### Scenario: Typed capability evidence is complete

- GIVEN the typed capability change is ready to archive
- WHEN closeout validation runs
- THEN evidence names catalog conversion tests, intersection tests, compatibility tests, docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Subagents use shared minimized context packets

r[agent_harness_runtime.subagent_context_packets.shared_packet] Animus MUST provide a shared minimized context-packet contract for subagent child runs, covering selected parent excerpts, summaries, file observations, memory descriptors, bounded tool-result summaries, and admitted context references.

#### Scenario: Recipe-backed subagent receives minimized context

- GIVEN a recipe-backed review subagent needs parent context
- WHEN the child-run draft is assembled
- THEN the context is represented as a shared minimized packet with selected safe items
- AND the full parent transcript is not included by default.

### Requirement: Unsafe context requires admission and leak validation

r[agent_harness_runtime.subagent_context_packets.unsafe_admission] Shared subagent context packets MUST reject full transcripts, raw hidden memory, credentials, provider-native payloads, unbounded tool output, and unsafe raw material unless the context class is explicitly admitted and leak validation succeeds.

#### Scenario: Credential context is rejected

- GIVEN a child-run context packet contains credential-looking material without explicit admission
- WHEN packet validation runs
- THEN validation returns an unsafe-context diagnostic
- AND the child run cannot start with that packet.

### Requirement: Context packet identity is deterministic and safe

r[agent_harness_runtime.subagent_context_packets.identity] Shared subagent context packets MUST carry deterministic BLAKE3 packet identities and budget summaries derived from stable safe packet fields, and MUST NOT include raw prompt bodies, credentials, provider payloads, full paths, or unbounded output in receipts.

#### Scenario: Packet identity can be recorded without raw context

- GIVEN a valid minimized packet contains safe summaries and evidence references
- WHEN a child-run receipt is produced
- THEN the receipt records the packet BLAKE3 identity and budget summary
- AND it does not include raw parent prompt or tool output bodies.

### Requirement: Every child-run kind can project context packets

r[agent_harness_runtime.subagent_context_packets.kind_projection] Recipe-backed subagents, fresh-context investigations, read-only side questions, and worker-team tasks MUST consume or losslessly project their context inputs through the shared context-packet contract.

#### Scenario: Fresh-context task keeps admitted references

- GIVEN a fresh-context task declares admitted context references
- WHEN the task is projected into the shared packet form
- THEN each admitted reference is represented with its safe label and budget metadata
- AND no parent-only secret or hidden prompt is inherited.

### Requirement: Context-packet failures fail closed

r[agent_harness_runtime.subagent_context_packets.fail_closed] Context-packet validation MUST fail closed for missing packet identity, empty required safe items, over-budget packets, unsafe raw markers, unadmitted unsafe classes, duplicate item labels, and lossy child-run projections.

#### Scenario: Over-budget packet is rejected

- GIVEN a child-run context packet exceeds its configured item or token budget
- WHEN packet validation runs
- THEN validation returns an over-budget diagnostic
- AND no scheduler, worker, model, tool, or external adapter receives the packet.

### Requirement: Context-packet closeout is evidenced

r[agent_harness_runtime.subagent_context_packets.verification] Shared context-packet adoption MUST include positive minimized packet tests, negative unsafe-context tests, identity tests, child-kind projection tests, docs, Cairn validation, and Cairn gates before archive.

#### Scenario: Context-packet evidence is complete

- GIVEN the shared context-packet change is ready to archive
- WHEN closeout validation runs
- THEN evidence names packet validation tests, projection tests, leak tests, docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Runtime exposes a unified child-run facade

r[agent_harness_runtime.subagent_facade.boundary] Animus runtime MUST expose a unified subagent facade that represents recipe-backed subagents, fresh-context investigations, read-only side questions, and worker-team tasks as typed child-run drafts, kinds, admissions, diagnostics, and receipts.

#### Scenario: Embedder validates through one facade

r[agent_harness_runtime.subagent_facade.all_kinds]
- GIVEN an embedder has one valid recipe-backed subagent, one valid fresh-context investigation, one valid side question, and one valid worker-team task
- WHEN each draft is validated through the subagent facade
- THEN each accepted result identifies the child-run kind and safe identity
- AND validation performs no model, tool, scheduler, worker-service, filesystem, process, network, or credential side effect.

### Requirement: Facade projections are lossless

r[agent_harness_runtime.subagent_facade.lossless_projection] The subagent facade MUST preserve source-specific child-run metadata, diagnostics, handoff settings, capability ceilings, context identities, and safe receipt hashes without collapsing them into untyped free-form maps.

#### Scenario: Source-specific diagnostic survives projection

- GIVEN a recipe-backed subagent is denied because recipe trust is missing
- WHEN the denial is converted into the facade diagnostic type
- THEN the diagnostic still identifies the recipe-trust failure class and source child-run kind
- AND no lower-level denial reason is replaced by a generic unknown failure.

### Requirement: Facade validation fails closed

r[agent_harness_runtime.subagent_facade.fail_closed] The subagent facade MUST fail closed for unsupported child-run kinds, missing kind-specific required fields, invalid lower-level DTOs, lossy projection attempts, or any validation path that would require execution authority.

#### Scenario: Unsupported child-run kind is rejected

- GIVEN a child-run draft names a kind that has no registered typed validator
- WHEN facade validation runs
- THEN validation returns an unsupported-kind diagnostic
- AND no fallback child runner, shell command, external CLI, or model request is attempted.

### Requirement: Facade closeout is evidenced

r[agent_harness_runtime.subagent_facade.verification] The unified subagent facade adoption MUST include positive validation tests for every supported child-run kind, negative fail-closed tests, public export checks, documentation, Cairn validation, and Cairn gates before archive.

#### Scenario: Facade evidence is complete

- GIVEN the unified subagent facade change is ready to archive
- WHEN closeout validation runs
- THEN evidence names facade conversion tests, lower-level validator compatibility tests, docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Subagent execution uses injected services

r[agent_harness_runtime.subagent_coordinator.service] Animus runtime MUST coordinate subagent execution through injected host services for enqueue, status, cancel, and collect operations, and MUST NOT construct concrete queues, provider clients, process runners, external-agent clients, filesystem stores, network transports, or credential readers in reusable runtime code.

#### Scenario: Coordinator enqueues through an injected service

- GIVEN a validated child-run admission and an injected service for that child-run kind
- WHEN the coordinator enqueues the child run
- THEN the service receives the validated admission and returns a typed handle
- AND runtime does not discover or construct a fallback executor.

### Requirement: Coordinator transitions are deterministic

r[agent_harness_runtime.subagent_coordinator.state_machine] Subagent coordinator state transitions MUST be pure over the current child-run state, validated admission, requested operation, and injected service outcome.

#### Scenario: Running child cancellation is explicit

- GIVEN a child run is running with a valid service handle
- WHEN cancellation is requested and the injected service accepts cancellation
- THEN the coordinator moves the child run to a cancelling or cancelled state according to the typed outcome
- AND no completed receipt is emitted until a terminal collect or cancellation outcome is validated.

### Requirement: Lifecycle events are safe projections

r[agent_harness_runtime.subagent_coordinator.lifecycle] Subagent coordinator lifecycle events MUST include safe child identity, parent identity, child-run kind, status, failure class, budget summary when available, and bounded evidence references, and MUST NOT include raw transcripts, provider payloads, credentials, full path dumps, or unbounded tool output.

#### Scenario: Malformed child report is visible without leaking raw output

- GIVEN an injected service returns a malformed child report
- WHEN the coordinator classifies the outcome
- THEN the lifecycle event records a malformed-report failure class and safe evidence reference
- AND raw child output is not inserted into the event or parent context.

### Requirement: Coordinator failures fail closed

r[agent_harness_runtime.subagent_coordinator.fail_closed] The subagent coordinator MUST fail closed when required services are missing, handles are stale, service calls are denied, unavailable, timed out, cancelled, malformed, or would require ambient fallback.

#### Scenario: Missing service does not spawn fallback work

- GIVEN a valid child-run admission requires a scheduler or worker service
- WHEN no matching service is injected
- THEN the coordinator returns a service-unavailable diagnostic
- AND no shell command, background thread, external CLI, model call, or network request is attempted.

### Requirement: Coordinator closeout is evidenced

r[agent_harness_runtime.subagent_coordinator.verification] The subagent coordinator adoption MUST include positive enqueue/status/cancel/collect tests, negative fail-closed service tests, safe lifecycle event tests, docs, Cairn validation, and Cairn gates before archive.

#### Scenario: Coordinator evidence is complete

- GIVEN the coordinator API change is ready to archive
- WHEN closeout validation runs
- THEN evidence names state-machine tests, service fake tests, failure classification tests, docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Fan-out plans are declarative and bounded

r[agent_harness_runtime.subagent_fanout_fanin.plan] Animus MUST represent subagent fan-out as declarative plans containing child-run admissions or drafts, concurrency policy, shared budget, cancellation policy, aggregation schema, and mutation-arbitration policy without starting child execution during plan validation.

#### Scenario: Valid fan-out plan is accepted without execution

- GIVEN a fan-out plan contains two read-only child-run admissions, a bounded concurrency policy, and a shared budget
- WHEN plan validation runs
- THEN validation accepts the plan and returns safe plan metadata
- AND no scheduler, worker, model, tool, or external adapter is invoked.

### Requirement: Fan-in aggregation is deterministic

r[agent_harness_runtime.subagent_fanout_fanin.aggregation] Animus MUST aggregate fan-in results with a deterministic reducer that distinguishes success, partial success, failure, timeout, cancellation, malformed output, denied capability, budget exhaustion, and write conflict outcomes.

#### Scenario: Partial success is explicit

- GIVEN one child succeeds and one child times out
- WHEN fan-in aggregation runs
- THEN the aggregate status is partial success or timeout-bearing partial result according to policy
- AND the timed-out child is not counted as successful.

### Requirement: Cancellation and budget exhaustion are terminal evidence

r[agent_harness_runtime.subagent_fanout_fanin.cancellation_budget] Fan-out/fan-in orchestration MUST treat cancellation, timeout, and budget exhaustion as explicit terminal or blocking outcomes with safe diagnostics and receipt evidence.

#### Scenario: Shared budget exhaustion cancels remaining children

- GIVEN fan-out execution exhausts its shared budget before all children complete
- WHEN the coordinator records remaining child outcomes
- THEN unfinished children receive budget-exhausted or cancelled outcomes
- AND the aggregate report explains the budget stop without raw child logs.

### Requirement: Mutation intents require arbitration

r[agent_harness_runtime.subagent_fanout_fanin.mutation_arbitration] Fan-in aggregation MUST preserve, deny, or mark child mutation intents unresolved, and MUST NOT apply workspace, ledger, filesystem, process, network, or provider effects during aggregation.

#### Scenario: Conflicting mutation intents do not apply

- GIVEN two child reports propose incompatible mutation intents
- WHEN fan-in aggregation runs
- THEN the aggregate records a structured write-conflict or unresolved-mutation result
- AND neither mutation is dispatched before normal policy and coordination accept a later action.

### Requirement: Fan-out/fan-in failures fail closed

r[agent_harness_runtime.subagent_fanout_fanin.fail_closed] Fan-out/fan-in validation and aggregation MUST fail closed for missing child admissions, unsafe concurrency policies, over-budget plans, malformed child reports, denied capabilities, duplicate child ids, write conflicts, mutation bypass attempts, and lossy aggregate receipts.

#### Scenario: Duplicate child id is rejected

- GIVEN a fan-out plan contains two child runs with the same safe child identity
- WHEN plan validation runs
- THEN validation returns a duplicate-child diagnostic
- AND no child is enqueued from that plan.

### Requirement: Fan-out/fan-in closeout is evidenced

r[agent_harness_runtime.subagent_fanout_fanin.verification] Fan-out/fan-in adoption MUST include positive plan and aggregation tests, negative fail-closed tests, mutation arbitration tests, docs, Cairn validation, and Cairn gates before archive.

#### Scenario: Fan-out/fan-in evidence is complete

- GIVEN the fan-out/fan-in change is ready to archive
- WHEN closeout validation runs
- THEN evidence names plan tests, aggregate reducer tests, mutation arbitration tests, docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Ledger records parent/child subagent lineage

r[agent_harness_runtime.subagent_lineage_ledger.record] Animus MUST record subagent lineage using safe parent id, child id, child-run kind, delegation source, admission identity, effective capability identity, context packet identity, output hash, handoff mode, terminal status, and failure class fields.

#### Scenario: Completed child run has lineage record

- GIVEN a child run completes and contributes a summary to the parent
- WHEN the lineage record is built
- THEN the record links the child id to its parent id, kind, delegation source, capability identity, context packet identity, output hash, and handoff mode
- AND the record contains no raw child transcript.

### Requirement: Lifecycle events project into lineage records

r[agent_harness_runtime.subagent_lineage_ledger.lifecycle_projection] Coordinator lifecycle events SHOULD project into validated lineage records that preserve safe status, failure, budget, evidence, and receipt references while rejecting raw transcripts, provider payloads, credentials, and unbounded tool output.

#### Scenario: Failed child run produces safe lineage

- GIVEN a child run times out before returning a valid summary
- WHEN lifecycle projection runs
- THEN the lineage record stores the timeout failure class and safe evidence reference
- AND no raw partial output is copied into the ledger record.

### Requirement: Replay validates child influence

r[agent_harness_runtime.subagent_lineage_ledger.replay] Session replay MUST be able to validate parent-visible child influence against lineage records, accepted handoff mode, context packet identity, output hash, and safe receipt identity without requiring raw child transcript storage.

#### Scenario: Parent imports child summary without lineage

- GIVEN a parent session contains a child summary import
- WHEN replay cannot find a matching lineage record and output hash
- THEN replay marks the import as invalid or unaudited
- AND the child summary is not treated as verified influence.

### Requirement: Lineage validation fails closed

r[agent_harness_runtime.subagent_lineage_ledger.fail_closed] Subagent lineage validation MUST reject missing lineage for imported child output, mismatched parent or child ids, invalid BLAKE3 hashes, unsupported handoff modes, malformed receipts, raw transcript markers, unsafe summaries, and unbounded evidence references.

#### Scenario: Raw transcript marker is rejected

- GIVEN a lineage record attempts to include raw child log text or an unsafe raw transcript marker
- WHEN lineage validation runs
- THEN validation returns a leak diagnostic
- AND the record is not accepted for replay or durable evidence.

### Requirement: Lineage closeout is evidenced

r[agent_harness_runtime.subagent_lineage_ledger.verification] Subagent lineage adoption MUST include positive lineage/replay tests, negative fail-closed leak and mismatch tests, receipt tests, docs, Cairn validation, and Cairn gates before archive.

#### Scenario: Lineage evidence is complete

- GIVEN the subagent lineage change is ready to archive
- WHEN closeout validation runs
- THEN evidence names lineage validation tests, replay tests, receipt tests, docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Subagent status projection is UI-neutral

r[agent_harness_runtime.subagent_ux_hooks.status_projection] Animus MUST expose UI-neutral subagent status projections for queued, running, cancelling, completed, cancelled, failed, timed-out, blocked, and awaiting-parent states without depending on terminal widgets, headless transports, or host-control implementations.

#### Scenario: TUI and headless hosts render the same status

- GIVEN a child run is running with safe identity and budget metadata
- WHEN runtime builds a subagent status projection
- THEN TUI and headless hosts can render the same state from the projection
- AND the projection contains no terminal-specific widget state.

### Requirement: UX projections include safe metadata only

r[agent_harness_runtime.subagent_ux_hooks.safe_metadata] Subagent UX projections MUST include safe child identity, parent identity, child-run kind, budget usage, denial reason, handoff summary reference, evidence reference, and pending mutation-intent summaries when available, and MUST NOT include raw child logs, full transcripts, credentials, provider payloads, full path dumps, or unbounded tool output.

#### Scenario: Denied child run explains boundary safely

- GIVEN a child run is denied by capability policy
- WHEN the UX projection is built
- THEN the projection names the denial class and safe evidence reference
- AND it does not include raw prompt, credential, or provider payload material.

### Requirement: UX actions are command descriptors

r[agent_harness_runtime.subagent_ux_hooks.command_descriptors] Subagent UX controls MUST be represented as command descriptors for cancel, bounded detail import request, acknowledgement, and mutation-intent arbitration request rather than direct effects.

#### Scenario: Detail import request is bounded

- GIVEN a user asks to inspect child details from the UI
- WHEN the host constructs a detail import command descriptor
- THEN the descriptor includes requested bound, handoff mode, child id, and reason
- AND no detail is imported until policy and bounds checks accept the request.

### Requirement: UX commands route through normal policy

r[agent_harness_runtime.subagent_ux_hooks.policy_routing] UX command descriptors MUST route through coordinator state checks, capability policy, confirmation policy when required, and mutation arbitration policy before causing cancellation, detail import, acknowledgement, or mutation-intent follow-up effects.

#### Scenario: Unauthorized cancel is rejected

- GIVEN a UI sends a cancel command for a child run outside the user's admitted authority
- WHEN command routing evaluates the descriptor
- THEN routing returns an unauthorized-action diagnostic
- AND the child run is not cancelled.

### Requirement: UX hook failures fail closed

r[agent_harness_runtime.subagent_ux_hooks.fail_closed] Subagent UX projection and command routing MUST fail closed for raw-log exposure, unbounded detail import, unauthorized cancellation, mutation bypass attempts, stale handles, malformed command descriptors, hidden credential metadata, and lossy status projection.

#### Scenario: Raw-log projection is rejected

- GIVEN a status projection draft includes raw child log text
- WHEN projection validation runs
- THEN validation returns a leak diagnostic
- AND the projection is not emitted to TUI, headless, or host-control surfaces.

### Requirement: UX hook closeout is evidenced

r[agent_harness_runtime.subagent_ux_hooks.verification] Subagent UX hook adoption MUST include positive status and command descriptor tests, negative fail-closed tests, docs, Cairn validation, and Cairn gates before archive.

#### Scenario: UX hook evidence is complete

- GIVEN the UX hook change is ready to archive
- WHEN closeout validation runs
- THEN evidence names status projection tests, command-routing tests, leak tests, docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: External subagent fixtures cover admitted descriptors

r[agent_harness_runtime.subagent_external_conformance.positive_fixtures] Animus MUST include positive external subagent fixtures for admitted ACP and MCP descriptors with explicit protocol labels, safe external identities, injected service labels, declared tools, optional credential reference labels, and bounded capability ceilings.

#### Scenario: Valid MCP descriptor fixture is accepted

- GIVEN a fixture declares an MCP external subagent with an injected service label and declared read-only tools
- WHEN descriptor conformance validation runs
- THEN the fixture is accepted
- AND no live MCP server, CLI, network connection, or credential read is required.

### Requirement: External subagent fixtures cover denied descriptors

r[agent_harness_runtime.subagent_external_conformance.negative_fixtures] Animus MUST include negative external subagent fixtures for missing service labels, undeclared tools, ambient CLI discovery, inherited credentials, malformed protocol labels, unsafe output imports, and overbroad capability claims.

#### Scenario: Ambient CLI fixture is denied

- GIVEN a fixture requests external subagent discovery from PATH or another ambient CLI source
- WHEN descriptor conformance validation runs
- THEN validation returns an ambient-discovery-denied failure class
- AND no CLI lookup is attempted.

### Requirement: Conformance index is machine-checkable

r[agent_harness_runtime.subagent_external_conformance.index] External subagent conformance fixtures SHOULD be indexed with fixture identity, expected status, expected failure class when denied, protocol kind, declared tools, and covered requirement identifiers.

#### Scenario: Missing expected failure class is rejected

- GIVEN a negative fixture is listed without an expected failure class
- WHEN the conformance index check runs
- THEN the check fails with a missing-expected-failure diagnostic
- AND the fixture is not counted as coverage.

### Requirement: External conformance has no live dependency

r[agent_harness_runtime.subagent_external_conformance.no_live_dependency] External subagent conformance validation MUST run over checked-in fixtures and pure descriptor validation without requiring live ACP services, MCP services, network access, ambient CLI lookup, filesystem discovery outside the fixture set, or credentials.

#### Scenario: Offline conformance succeeds

- GIVEN only checked-in external subagent fixtures are available
- WHEN focused conformance validation runs
- THEN accepted and denied fixtures are classified as expected
- AND no external process, network, or credential dependency is required.

### Requirement: External conformance closeout is evidenced

r[agent_harness_runtime.subagent_external_conformance.verification] External subagent conformance adoption MUST include positive and negative fixture tests, index validation, offline validation evidence, docs, Cairn validation, and Cairn gates before archive.

#### Scenario: External conformance evidence is complete

- GIVEN the external conformance change is ready to archive
- WHEN closeout validation runs
- THEN evidence names fixture tests, index checks, offline validation, docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.
