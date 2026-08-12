# agent-harness-runtime Change Spec: Sandlock-inspired sandbox enforcement

## Requirements

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
