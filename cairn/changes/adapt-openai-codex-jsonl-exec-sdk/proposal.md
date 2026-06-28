# Proposal: Adapt OpenAI Codex JSONL exec and SDK control contracts for Animus

## Why

OpenAI Codex's non-interactive `exec` mode provides a useful automation contract: progress events stream separately from the final result, JSONL can expose machine-readable lifecycle events, final output can be schema-constrained, and later runs can resume an existing thread. Its SDKs expose the same thread/turn control programmatically.

Animus needs a comparable contract for CI, scripts, editor integrations, and hosted controllers. The adaptation should use Animus runtime events and typed reports rather than coupling to Codex's CLI flags or SDK packages.

## What Changes

- Define a non-interactive run request contract with prompt, optional external context, sandbox profile, output mode, and resume identity.
- Add a machine-readable event stream for thread, turn, item, model, tool, policy, hook, usage, and failure events.
- Define final response contracts for plain text and schema-validated structured output.
- Add a controller/SDK-facing thread API that can start, resume, run, and inspect threads without depending on a TUI.
- Keep provider auth, OpenAI-specific backend behavior, and Codex app-server protocols outside the reusable runtime.

## Impact

- **Files**: runtime run DTOs, event stream types, structured-output validation adapters, controller traits, CLI/std shell wiring, docs, and integration tests.
- **Testing**: positive tests for event ordering, final-output extraction, schema-valid output, and thread resume; negative tests for invalid schemas, malformed event consumers, missing thread identity, failed turns, and disallowed sandbox escalation.
- **Non-claims**: this change does not add OpenAI Codex SDK compatibility, a network app server, ChatGPT authentication, or concrete CI workflows.
