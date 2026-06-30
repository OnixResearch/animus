# Proposal: Adapt Pi runtime event lifecycle for Animus

## Why

Pi exposes a clear lifecycle around prompts, turns, model streaming, tool execution, compaction, session switching, and shutdown. That shape lets extensions observe, block, modify, or summarize behavior without patching the core loop. Animus needs comparable observability and interception, but as typed Rust runtime events and host-owned middleware rather than Pi's TypeScript extension API.

Adapting this pattern will make the upcoming runtime loop easier to test, embed, and secure because each transition can emit evidence and fail closed at explicit boundaries.

## What Changes

- Define typed runtime events for runtime/session start, agent start/end, turn start/end, Steel decision, model dispatch, tool dispatch, tool result, compaction, replay, cancellation, and shutdown.
- Define deterministic middleware/hook ordering and return contracts for observe-only, mutate, block, and replace-style hooks.
- Ensure hook failures are typed and fail safe at sensitive boundaries such as tool calls, provider calls, compaction, and session replacement.
- Keep middleware host-injected; the reusable runtime must not discover plugins, execute TypeScript, or load Pi packages.
- Add positive and negative tests for event order, hook chaining, blocked calls, hook errors, and cancellation.

## Impact

- **Files**: runtime/event DTOs, hook traits or service interfaces, tests, documentation, and possibly Preserves event schemas.
- **Testing**: event-order tests, hook-chain tests, blocked-call tests, and failure-mode tests for hook errors and cancellation.
- **Non-claims**: this change does not add Pi extensions, custom TUI components, package loading, JavaScript execution, or a built-in permission-popup UI.
