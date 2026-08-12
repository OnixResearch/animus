# Proposal: Add fake-backed headless dogfood shell

## Why

The adapter conformance and dogfood-readiness rails are green, but Animus still has no executable host path that proves a bounded prompt can traverse accepted contracts. Without a fake-backed headless shell, adapter contracts can remain well-specified but unproven as an end-to-end product surface.

A non-default, deterministic shell should exercise prompt admission, model dispatch through injected fixture adapters, optional tool metadata admission, ordered runtime events, and safe receipts without live providers, credentials, process execution, network access, daemon state, terminal UI, or global persistence.

## What Changes

- Add a packaged or feature-gated headless/CLI dogfood entrypoint that runs a bounded prompt path with deterministic fake model/tool services.
- Bind the shell to existing headless request, runtime event, injected service, policy, and safe receipt contracts.
- Emit a shell-specific receipt index with safe adapter labels, event counts, redaction markers, BLAKE3 identities, and non-claim metadata.
- Add examples and docs that name the completed evidence rung and list omitted optional surfaces.
- Extend the dogfood readiness harness to recognize the fake-backed integration evidence.

## Impact

- **Files**: host-shell code, deterministic fixtures, receipt index DTO/projection, docs/examples, dogfood readiness fixtures, focused tests, and Cairn evidence.
- **Testing**: positive fake-backed prompt smoke plus negative missing model service, denied capability, unsafe prompt/evidence, malformed fixture output, receipt leak, and omitted-surface overclaim cases.
- **Non-claims**: this does not add live providers, credential discovery, network calls, default product tools, daemon/attach, TUI dogfood, plugin runtime, persistent sessions, production readiness, or Clankers parity.
