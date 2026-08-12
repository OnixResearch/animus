# agent-harness-runtime Change Spec: OpenClaw security and plugin guardrails

## Requirements

### Requirement: Security audit uses host-supplied snapshots

r[agent_harness_runtime.openclaw_guardrails.audit] Animus MUST adapt OpenClaw-style security audit behavior as a pure classifier over host-supplied snapshots, stable finding IDs, severity labels, remediation labels, safe evidence fields, and optional autofix-plan descriptors, and MUST NOT read config files, scan filesystems, probe networks, mutate settings, restart daemons, or render Doctor UI from reusable crates.

#### Scenario: Audit snapshot classifies without host reads

- GIVEN a host supplies bind posture, auth posture, channel policy, tool policy, sandbox state, plugin trust summaries, and filesystem hygiene summaries
- WHEN the audit classifier runs
- THEN it returns deterministic findings and safe remediation labels
- AND no config file, credential store, path, network endpoint, package registry, or daemon state is inspected by reusable code.

### Requirement: Exposure rules catch dangerous combinations

r[agent_harness_runtime.openclaw_guardrails.exposure] Security guardrails MUST classify dangerous compositions including remote gateway exposure without admission, open direct messages or groups with broad tools, elevated/runtime/filesystem tools in shared contexts, sandbox configuration that does not enforce sandboxing, permissive plugin policy, and model/tool risk mismatches before effects are dispatched.

#### Scenario: Open group with runtime tools is critical

- GIVEN an audit snapshot shows public or open group access and runtime or filesystem tools are available without sandbox enforcement
- WHEN exposure classification runs
- THEN it emits a high-severity finding with safe affected labels and remediation labels
- AND no tool, model, channel, or sandbox operation is invoked.

### Requirement: Plugin manifests validate before execution

r[agent_harness_runtime.openclaw_guardrails.plugin_manifest] Plugin manifest guardrails MUST require data-only manifest validation, including plugin identity, config schema, declared channels or providers, slots, skills, UI hints, and enabled state, before plugin code execution, and MUST reject unknown plugin, channel, provider, slot, or config references unless a trusted manifest declares them.

#### Scenario: Unknown channel config fails closed

- GIVEN host config contains a channel key provided by no trusted plugin manifest
- WHEN plugin manifest validation runs
- THEN validation returns an unknown-channel diagnostic
- AND no plugin module is imported, executed, or granted config authority.

### Requirement: Plugin trust and path diagnostics are data-only

r[agent_harness_runtime.openclaw_guardrails.plugin_trust] Plugin guardrails MUST model allow/deny policy, install-source class, manifest provenance, path-safety summaries, symlink-escape summaries, writability class, ownership class, registry-only install claims, and lifecycle-script risk as host-supplied metadata, and MUST NOT resolve paths, stat directories, install packages, run lifecycle scripts, or load plugin modules in reusable validation.

#### Scenario: Symlink escape summary blocks plugin admission

- GIVEN a host-supplied plugin provenance summary says the extension entry resolves outside its declared plugin root
- WHEN plugin trust validation runs
- THEN it returns a path-escape diagnostic and blocks manifest admission
- AND reusable validation does not attempt to resolve the symlink itself.

### Requirement: Guardrail receipts are safe

r[agent_harness_runtime.openclaw_guardrails.receipts] Security audit and plugin guardrail receipts MUST record safe metadata such as finding ID, severity, source class, affected safe labels, remediation labels, policy hash, plugin ID, provenance class, path-safety class, and BLAKE3 receipt hash, and MUST NOT include raw config bodies, tokens, credentials, full private paths, provider payloads, raw prompts, raw plugin code, or unbounded logs.

#### Scenario: Receipt leak is rejected

- GIVEN a guardrail receipt fixture includes a raw token, full private plugin path, raw config body, or provider payload
- WHEN receipt validation runs
- THEN validation rejects the fixture with a leak diagnostic
- AND the receipt cannot be used as durable evidence.

### Requirement: Guardrail verification is complete

r[agent_harness_runtime.openclaw_guardrails.verification] The OpenClaw guardrail adaptation MUST include positive hardened-baseline, valid-plugin, disabled-plugin-warning, and safe-receipt tests plus negative remote-exposure, open-DM-with-tools, sandbox-drift, unknown-channel, missing-manifest, unsafe-path, plugin-execution, and receipt-leak tests before archive.

#### Scenario: Closeout evidence covers guardrails

- GIVEN maintainers intend to archive the OpenClaw security and plugin guardrail adaptation
- WHEN closeout validation runs
- THEN evidence names audit classifier, exposure rules, plugin manifest validation, plugin trust diagnostics, receipts, docs, Cairn validation, and Cairn gates
- AND omitted Doctor UI, CLI audit, config loader, filesystem scanner, package installer, auto-fix executor, and plugin runtime behavior are listed as non-claims.
