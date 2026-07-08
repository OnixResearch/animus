# OpenClaw-inspired session isolation and session-tool boundary

Animus adapts OpenClaw's session-isolation lessons as host-store-neutral planning. The reusable contract is pure data validation and planning in `animus-adapters::openclaw_session`; it is not a session store, JSONL transcript format, cleanup scheduler, channel connector, gateway persistence layer, or live cross-session delivery system.

## Contracts

- `plan_session_key` derives deterministic session keys from explicit agent, source-kind, direct-message scope, channel, account, peer, group, thread/topic, cron, hook, node, subagent, and identity-link metadata. It rejects ambient store discovery and emits privacy diagnostics for multi-sender shared direct-message scopes.
- `plan_session_maintenance` produces previewable, ordered maintenance actions for stale pruning, entry-count caps, transcript rotation, archive retention, and disk-budget pressure over host-loaded metadata only.
- `plan_session_tool_visibility` enforces self, tree, agent, and all visibility modes with sandbox/delegation clamps, cross-agent denial, and bounded history previews before any transcript metadata is exposed.
- `InterSessionProvenance` records source session, target session, source tool, optional source channel, child lineage, handoff mode, reply and announce bounds, BLAKE3 influence identity, and a safe summary for routed messages or spawn planning.

## Non-claims

This contract does not implement a concrete session database, transcript files, global state directory, cleanup job, clock-based scheduler, gateway-owned persistence, channel connector, child-run executor, or live message delivery. Hosts may implement those effects behind injected services while reusable crates validate plans and safe provenance.

## Focused checks

```sh
cargo test -p animus-adapters openclaw_session
cargo check --workspace --all-targets
scripts/cairn/check-runtime-facade-inventory --root .
nix run .#cairn -- validate --root .
```
