# Design: Adapt OpenClaw security audit and plugin guardrails

The adaptation should make risky host compositions visible without giving reusable crates authority to inspect the host. Hosts supply snapshots; Animus validates and classifies them.

## Decisions

### 1. Audit consumes snapshots

**Choice:** The audit core accepts a host-supplied snapshot describing bind/auth posture, admission posture, channel/DM policy, tool policy, sandbox state, permission summaries, plugin manifests, and filesystem hygiene summaries.

**Rationale:** This keeps filesystem reads, network probing, config loading, and platform inspection in the host shell.

### 2. Findings are stable contracts

**Choice:** Findings have stable IDs, severity, affected safe labels, remediation labels, optional autofix-plan descriptors, and safe evidence hashes.

**Rationale:** UIs, CLI shells, and release evidence need deterministic diagnostics without embedding raw config, secrets, paths, or provider payloads.

### 3. Composition risks are first-class

**Choice:** The audit rules focus on dangerous combinations, such as open inbound access plus broad tools, remote exposure without admission, sandbox settings that do not actually enforce sandboxing, or permissive plugin policy.

**Rationale:** Many host risks are emergent; single-field validation is insufficient.

### 4. Manifest validation precedes code execution

**Choice:** Plugin config schemas, IDs, channel/provider declarations, slots, allow/deny lists, and UI hints are validated from manifest data before any plugin module can run.

**Rationale:** OpenClaw's strongest plugin lesson is that config validation must not require trusting plugin code.

### 5. Path and install checks are summarized

**Choice:** Hosts may supply path-safety and install-source summaries, but reusable validation never resolves symlinks, stats directories, installs npm packages, or runs lifecycle scripts.

**Rationale:** The guardrail remains portable while still preserving the diagnostics needed by concrete hosts.
