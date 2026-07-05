# Claude Code lifecycle hooks

Animus treats Claude Code hooks as a bounded reference for lifecycle extension points, not as hook-file authority or arbitrary shell execution. Reusable runtime crates model typed hook points, safe payloads, declared capabilities, validated outcomes, and fail-closed reports. Host shells own any concrete process, filesystem, network, clock, WASM, Steel, or service execution.

The `animus-runtime` lifecycle hook core provides:

- stable hook points for session start/end, prompt submission and assembly, model request/response, tool pre/post, mutation pre/post, validation, receipt emission, workflow completion, compaction, subagent, permission, and stop boundaries;
- host-owned hook definition contracts with explicit capabilities for observe, annotate, deny, request-continuation, and request-revalidation effects;
- safe event payloads with event IDs, event classes, redacted labels, bounded summaries, sizes, statuses, and BLAKE3 body hashes instead of raw prompts, credentials, provider payloads, raw tool bodies, full paths, or unbounded outputs;
- output validation that rejects malformed descriptors, unknown hook points, unsupported effects, undeclared capabilities, and policy-bypass attempts before any guarded action is used;
- fail-closed required-hook-unavailable and denied-hook outcome reports for shell-owned hook adapter failures.

This contract does not implement Claude Code hook files, command runners, background daemons, plugin installation, or direct adapter authority. Hooks cannot spawn shell commands from reusable runtime code, invoke tools, rewrite denied actions, weaken sandbox policy, suppress required approvals, or mutate state without normal DTO, permission, and capability revalidation.

Focused tests live in `crates/animus-runtime/src/hooks/lifecycle.rs` and cover positive hook-point parsing, safe payload metadata, request-revalidation effects, and route/trust flows plus negative unknown hook points, malformed descriptors, missing or duplicate capabilities, unsupported capabilities, leak attempts, unavailable hook services, and bypass/undeclared-capability attempts.
