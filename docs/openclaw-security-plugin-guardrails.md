# OpenClaw-inspired security audit and plugin guardrails

Animus adapts OpenClaw security and plugin lessons as pure guardrail contracts. The reusable boundary is `animus-adapters::openclaw_guardrails`, which classifies host-supplied snapshots and validates plugin manifest/trust metadata without reading host config, scanning paths, importing plugin code, installing packages, restarting daemons, rendering Doctor UI, or executing auto-fixes.

## Contracts

- `AuditSnapshot` and `classify_audit` classify gateway bind/auth posture, channel openness, tool blast radius, sandbox enforcement, plugin policy, and model/tool risk into stable findings with severity, safe affected labels, remediation labels, evidence hashes, and optional autofix descriptors.
- `validate_plugin_manifest` requires trusted data-only plugin manifests, config schemas before execution, declared channels/providers/slots/skills/UI hints, enabled-state diagnostics, and rejection of unknown config references before plugin code can run.
- `validate_plugin_trust` consumes host-supplied allow/deny policy, install-source class, path-safety summaries, writability, ownership, registry-only install claims, lifecycle-script risk, and provenance hashes without resolving symlinks, statting directories, or loading modules.
- `GuardrailReceipt` records safe finding, severity, source class, affected label, remediation label, policy hash, optional plugin ID, provenance class, path-safety class, receipt hash, and summary while rejecting raw config, tokens, credentials, full paths, provider payloads, prompts, plugin code, and unbounded logs.

## Non-claims

This contract does not implement OpenClaw's CLI audit command, Doctor UI, config loader, filesystem scanner, network probe, package installer, plugin runtime, gateway restart, auto-fix executor, symlink resolver, ownership checker, or permission stat calls. Hosts own those effects and supply safe summaries to reusable validation.

## Focused checks

```sh
cargo test -p animus-adapters openclaw_guardrails
cargo check --workspace --all-targets
scripts/cairn/check-runtime-facade-inventory --root .
nix run .#cairn -- validate --root .
```
