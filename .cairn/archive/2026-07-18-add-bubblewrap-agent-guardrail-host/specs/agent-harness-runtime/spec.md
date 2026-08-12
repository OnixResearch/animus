# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: Bubblewrap agent guardrail is optional and claim-bounded

r[agent_harness_runtime.bubblewrap_agent_guardrail.feature_gate] Animus MUST expose the Bubblewrap agent guardrail only through a non-default Linux host component and MUST keep default, reusable-core, Wasm, non-Linux, Sandlock, and Mantle build-sandbox surfaces independent of that component. Successful guardrail execution MUST be described as bounded accidental-disclosure and damage reduction for the exact profile, not hostile-code containment, kernel proof, fine-grained egress enforcement, authorization, or build hermeticity.

#### Scenario: Default build remains independent

- GIVEN Animus is built with default features, for a non-Linux target, or for a reusable/Wasm surface
- WHEN dependency and public-surface checks run
- THEN Bubblewrap and the guardrail executable MUST NOT become required dependencies or available host authority
- AND portable sandbox descriptors and safe receipts MUST remain usable without Bubblewrap-specific types.

#### Scenario: Guardrail evidence is not promoted

- GIVEN one Bubblewrap guardrail launch produces a passing receipt
- WHEN documentation or evidence tooling classifies the result
- THEN it MAY claim only the recorded profile, mounted roots, explicit network posture, executable cohort, and observed child outcome
- AND it MUST NOT claim Sandlock enforcement, hostile-code containment, kernel correctness, filtered egress, authorization, Mantle build hermeticity, or release readiness.

### Requirement: Guardrail profiles are typed and centrally trusted

r[agent_harness_runtime.bubblewrap_agent_guardrail.profile] Operator-authored Bubblewrap guardrail profiles MUST use typed Nickel contracts with deterministic checked runtime projections and MUST declare safe profile identity, root labels and modes, network posture, device/socket posture, inherited environment-key allowlists, backend/cohort requirements, and resource bounds. Project-local policy MUST NOT affect guardrail enforcement without a separate host trust decision.

#### Scenario: Valid central profile projects deterministically

- GIVEN a valid central Nickel profile with explicit read-only roots, one admitted writable worktree root, denied devices and sockets, and declared network posture
- WHEN the profile is exported and normalized
- THEN the same checked runtime value and domain-separated BLAKE3 profile identity MUST be produced
- AND no raw credential value, ambient host path, project-local policy body, or Bubblewrap escape argument may enter the portable profile identity.

#### Scenario: Unsafe profile surface is rejected

- GIVEN a profile requests raw Bubblewrap arguments, a command prefix, passthrough, a host command environment source, implicit writable/device/socket exposure, unknown network filtering, or untrusted project-local policy
- WHEN profile validation runs
- THEN validation MUST fail before launch planning
- AND no fallback or partially accepted profile may be emitted.

### Requirement: Guardrail launch planning is pure and deterministic

r[agent_harness_runtime.bubblewrap_agent_guardrail.plan] Animus MUST translate a normalized sandbox descriptor, matching authorization and policy identities, one selected guardrail profile, canonical active-context facts, and host-supplied root/symlink observations into a deterministic typed Bubblewrap launch plan without filesystem, environment, process, clock, network, or kernel effects. Read-only exposure MUST be the default, writable/device/socket/network-inherit exposure MUST be explicit, incompatible target operations MUST fail, and the plan MUST carry a domain-separated BLAKE3 identity.

#### Scenario: Identical admitted facts produce one ordered plan

- GIVEN identical descriptor, authorization, policy, profile, active-context, root, symlink, executable-cohort, and capability facts
- WHEN launch planning runs
- THEN it MUST return the same ancestor-before-descendant operation order, normalized compatible binds, capability labels, and BLAKE3 plan identity
- AND it MUST NOT contain raw prompts, credentials, environment values, command output, or model-visible absolute host paths.

#### Scenario: Root or operation conflicts fail closed

- GIVEN a missing required root, escaping relative root, symlink escape, source-kind mismatch, duplicate incompatible target, implicit writable upgrade, or undeclared device/socket exposure
- WHEN the planner evaluates the facts
- THEN it MUST reject the plan before a Bubblewrap invocation or child process is created
- AND it MUST NOT silently reorder, downgrade, or drop the conflicting constraint.

### Requirement: Active-context preflight is side-effect free

r[agent_harness_runtime.bubblewrap_agent_guardrail.preflight] `animus-agent-guardrail validate` MUST reuse effective profile selection, authorization binding, active-context normalization, root and source checks, typed launch planning, environment-key admission, and executable/cohort capability decisions while remaining free of configured path creation, PTY allocation, Bubblewrap launch, profile command execution, and requested child execution.

#### Scenario: Representative context validates without mutation

- GIVEN a selected profile, working context, authorization, root bindings, and compatible Bubblewrap facts
- WHEN active-context validation runs
- THEN it MUST return the exact prospective profile and plan identities with a valid status
- AND it MUST NOT create profile, state, temporary, bind-source, socket, or child-process resources.

#### Scenario: Inactive and runtime-only facts are not certified

- GIVEN another profile or path context was not selected or a runtime executable outcome was not observed
- WHEN one active-context validation succeeds
- THEN the result MUST NOT certify the inactive context or future child execution
- AND callers MUST be able to validate each representative profile/context separately.

### Requirement: Bubblewrap options use sealed argument transport

r[agent_harness_runtime.bubblewrap_agent_guardrail.sealed_transport] The guardrail executable MUST encode finalized Bubblewrap options as NUL-delimited bytes in a rewound sealed anonymous memfd, MUST launch a compatible Bubblewrap cohort through `bwrap --args FD -- COMMAND`, and MUST fail closed when option preparation, sealing, inheritance, executable identity, cohort compatibility, or consumption cannot be established. It MUST NOT retry by placing finalized options or resolved environment values on argv.

#### Scenario: Compatible executable consumes sealed options

- GIVEN an admitted plan and a compatible Bubblewrap executable implementing the required `--args` behavior
- WHEN the guardrail shell prepares and launches the child
- THEN only the final option descriptor and requested command boundary are passed to Bubblewrap as specified
- AND the receipt binds the executable/cohort and plan identities without persisting option payloads or environment values.

#### Scenario: Transport failure has no unguarded fallback

- GIVEN memfd creation, encoding, sealing, rewind, descriptor inheritance, executable verification, or Bubblewrap consumption fails
- WHEN launch handling evaluates the failure
- THEN no requested child may run outside the admitted guardrail plan
- AND the result MUST remain a typed pre-exec failure with safe diagnostics.

### Requirement: Guardrail enforcement follows authorization and excludes escape hatches

r[agent_harness_runtime.bubblewrap_agent_guardrail.authority] Bubblewrap guardrail planning and execution MUST require a prior matching Animus/Basalt authorization decision bound to the action, subject, target, descriptor, profile, and policy identities. Guardrail evidence MUST NOT create or widen authority, and the first implementation MUST NOT expose raw Bubblewrap arguments, command prefixes, passthrough, host command environment sources, implicit shells, automatic unguarded fallback, or an independent privileged host-command sidecar.

#### Scenario: Matching authorization permits preflight only

- GIVEN an allow decision matches every requested launch identity
- WHEN host admission runs
- THEN the request MAY proceed to side-effect-free preflight and later guarded execution
- AND successful preflight or execution MUST remain separate from the authorization decision.

#### Scenario: Missing authority or escape hatch is denied

- GIVEN authorization is missing, denied, stale, or identity-mismatched, or the request contains a forbidden escape-hatch surface
- WHEN admission evaluates the request
- THEN it MUST reject before Bubblewrap or child execution
- AND no sandbox success, trusted config field, or fallback path may override the denial.

### Requirement: Guardrail receipts are safe and backend-specific

r[agent_harness_runtime.bubblewrap_agent_guardrail.receipts] Bubblewrap guardrail receipts MUST contain only safe status and issue classes, profile/backend/cohort labels, descriptor, policy, authorization, executable, plan, and receipt BLAKE3 identities, requested and enforced capability labels, explicit network posture, bounded root-label summaries, exit classification, and cleanup status. Receipts MUST reject raw prompts, credentials, environment values, full host paths, mount payloads, requested command bodies, stdout/stderr bodies, debug logs, and unbounded event lists.

#### Scenario: Successful receipt binds exact guardrail evidence

- GIVEN one guarded child completes and bounded execution observations are available
- WHEN receipt projection runs
- THEN it MUST bind the exact profile, plan, executable cohort, authorization identity, enforced capability labels, network posture, and child outcome
- AND it MUST include the backend-specific non-claim boundary.

#### Scenario: Unsafe receipt input is rejected

- GIVEN receipt input contains a raw secret, environment value, host path dump, mount payload, command body, process output body, debug log, or unbounded event sequence
- WHEN receipt validation runs
- THEN it MUST fail closed without emitting an accepted durable receipt
- AND the unsafe input MUST NOT be copied into diagnostics.

### Requirement: Guardrail behavior has positive and negative evidence

r[agent_harness_runtime.bubblewrap_agent_guardrail.verification] The Bubblewrap guardrail host component MUST include positive and negative pure tests, fake-executable and sealed-transport tests, default/non-Linux dependency-isolation checks, and an opt-in real-Linux behavior rail for admitted worktree use, denied secret and sibling reads, denied outside writes, symlink escapes, explicit network modes, launch failures, and receipt safety.

#### Scenario: Closeout evidence is complete and honest

- GIVEN maintainers intend to archive the Bubblewrap guardrail change
- WHEN closeout evidence is assembled
- THEN it MUST include passing profile, planner, authorization, preflight, transport, receipt, default-feature, and negative fixtures plus Cairn gates
- AND unavailable real-kernel checks MUST be recorded as blocked with exact prerequisites and a next-best check rather than reported as passed.
