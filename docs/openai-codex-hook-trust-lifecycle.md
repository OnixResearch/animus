# OpenAI Codex hook trust lifecycle

Animus adapts OpenAI Codex hooks as typed lifecycle contracts, not as command-hook execution. Reusable runtime crates model hook events, trust, redacted inputs, event-scoped outputs, policy binding, and outcome reports. Host shells own any concrete process, WASM, Steel, or service execution.

## Runtime core contract

The `animus-runtime` lifecycle hook core provides:

- typed event kinds for session start, user prompt submit, pre/post tool use, permission requests, pre/post compaction, subagent start/stop, and stop;
- content-addressed hook definitions using BLAKE3 definition hashes;
- trust records scoped to managed, user, or project decisions;
- redacted input summaries that retain safe IDs, resource classes, safe labels, byte counts, and body hashes without raw prompt/tool/provider bodies;
- event-scoped outputs for bounded additional context, warnings, deny decisions, and stop-continuation requests;
- policy binding that can make a boundary stricter but cannot turn a denial into an allow decision or suppress required approval;
- outcome records for applied, skipped, untrusted, failed, denied, malformed, and unsupported hook results.

## Non-claims

This contract does not add a subprocess hook runner, Codex-compatible `hooks.json`, plugin installation, UI hook browser, or direct adapter authority. Hooks cannot invoke tools, rewrite denied actions, weaken sandbox policy, or suppress required approvals from reusable runtime code.

## Verification

Focused tests live in `crates/animus-runtime/src/hooks/lifecycle.rs` and cover positive routing/trust/output flows plus negative unknown events, changed definition hashes, unsupported effects, policy-bypass attempts, redaction, and required-service-unavailable outcomes.
