# Proposal: Adapt OpenClaw security audit and plugin guardrails

## Why

Animus already has policy, permission, safe receipt, and plugin manifest contracts, but future host shells need product-surface diagnostics that catch dangerous compositions before effects occur. OpenClaw's security guide and plugin rules provide a strong bounded reference: audit the deployment posture, flag risky channel/tool/plugin combinations, and validate plugin manifests without executing plugin code.

This change adapts those guardrails as pure audit inputs, findings, plugin manifest validation rules, and trust diagnostics.

## What Changes

- Define a host-supplied security audit snapshot and pure audit engine for gateway exposure, remote admission, direct-message policy, group access, tool blast radius, sandbox drift, filesystem hygiene summaries, model/tool risk, and plugin trust posture.
- Define finding IDs, severities, remediation labels, safe evidence fields, and optional autofix-plan descriptors without mutating host config.
- Extend plugin manifest boundaries with OpenClaw-style config-schema-before-execution rules, unknown plugin/channel errors, slots, allow/deny checks, and disabled-plugin warnings.
- Add plugin provenance/path safety diagnostics for symlink escapes, world-writable roots, suspicious ownership summaries, registry-only install claims, and lifecycle-script risk as host-supplied metadata.
- Keep config file reads, filesystem permission checks, npm installs, plugin loading, gateway restarts, and auto-fixes outside reusable crates.

## Impact

- **Files**: future security-policy DTOs, plugin manifest validators, audit fixture suites, docs, Nickel policy hooks where useful, and facade inventory updates.
- **Testing**: positive hardened-baseline and valid-plugin fixtures; negative open-DM-with-tools, remote-bind-without-auth, sandbox-drift, unsafe plugin path, unknown channel, missing manifest, plugin code execution, and receipt leak tests.
- **Non-claims**: this does not implement OpenClaw's CLI audit command, Doctor UI, config loader, filesystem scanner, package installer, plugin runtime, gateway restart, or auto-fix executor.
