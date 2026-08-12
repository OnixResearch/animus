# Proposal: Adapt OpenClaw session isolation and session-tool contracts

## Why

Animus has neutral ledger and subagent contracts, but future host shells still need privacy-preserving session-key planning and safe cross-session coordination rules. OpenClaw's session model highlights two critical lessons: direct-message scope must be explicit to avoid shared-context leaks, and session tools need visibility clamps when sandboxed or delegated agents communicate.

This change adapts those lessons as pure session-key planners, send-policy decisions, maintenance plans, session-tool visibility rules, and inter-session provenance contracts.

## What Changes

- Define a pure session key planner for main, per-peer, per-channel-peer, per-account-channel-peer, group, cron, hook, node, and subagent/session-origin keys.
- Define identity-link normalization and privacy diagnostics for multi-sender direct-message setups.
- Define send-policy and session maintenance plans as data-only decisions over already-loaded session metadata.
- Define session-tool visibility levels for self, tree, agent, and all, including sandbox clamps and spawned-only defaults.
- Define inter-session send/spawn provenance, bounded reply/announce behavior, and subagent visibility constraints without executing child runs.

## Impact

- **Files**: future runtime/adapters session modules, ledger projection helpers, subagent facade integration, docs, fixtures, and facade inventory updates.
- **Testing**: positive key derivation, identity-link, send-policy, visibility, inter-session provenance, and maintenance-plan tests; negative shared-DM leakage, ambiguous keys, sandbox visibility widening, cross-agent access, missing provenance, unsafe transcript exposure, and ambient store discovery tests.
- **Non-claims**: this does not implement a concrete session store, JSONL transcript format, gateway-owned persistence, cleanup scheduler, channel connector, or live cross-session delivery.
