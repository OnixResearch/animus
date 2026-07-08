# Tasks

- [x] [serial] Define the OpenClaw-inspired audit scope, host-supplied snapshot inputs, finding IDs, severities, safe evidence fields, and non-claims for config loading, scanning, auto-fix, and Doctor UI behavior. r[agent_harness_runtime.openclaw_guardrails.audit]
- [x] [depends:audit] Add pure exposure and blast-radius rules for remote bind/auth posture, DM/group access, tool profiles, elevated/runtime/fs tools, sandbox drift, and model/tool risk combinations. r[agent_harness_runtime.openclaw_guardrails.exposure]
- [x] [depends:audit] Add plugin manifest validation that requires schemas before execution, rejects unknown plugin/channel/slot references, and surfaces disabled-plugin warnings as data. r[agent_harness_runtime.openclaw_guardrails.plugin_manifest]
- [x] [depends:plugin_manifest] Add plugin trust/path provenance diagnostics for allow/deny, install source, symlink escape summaries, world-writable roots, suspicious ownership, registry-only install claims, and lifecycle-script risk. r[agent_harness_runtime.openclaw_guardrails.plugin_trust]
- [x] [depends:exposure,plugin_trust] Add safe audit receipts and leak checks for finding evidence, remediation labels, autofix-plan descriptors, and plugin diagnostics. r[agent_harness_runtime.openclaw_guardrails.receipts]
- [x] [depends:audit,exposure,plugin_manifest,plugin_trust,receipts] Update docs, Nickel hooks or fixtures, facade inventory, and focused verification evidence for OpenClaw guardrails. r[agent_harness_runtime.openclaw_guardrails.verification]
