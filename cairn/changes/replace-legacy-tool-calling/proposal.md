# Proposal: Replace legacy tool calling contracts

## Why

Animus has the right adapter pieces for safe tools, but the runtime still admits tool calls through a legacy string-shaped path: a tool name plus raw input string, broad `default-tool` capability admission, and raw output-shaped responses. That shape hides schema identity, per-tool capabilities, approval posture, output bounds, mutation targets, and typed execution failures until too late.

We do not need backwards compatibility for this overhaul. Keeping shims for resumed legacy calls would preserve the weakest part of the contract and force every future tool path to carry dual semantics. The safer move is to make the current structured contract the only public tool-call shape and fail closed for old raw string payloads.

## What Changes

- Remove legacy raw-string tool request/response semantics from the runtime tool-call path instead of wrapping them with a compatibility layer.
- Require current-shape structured tool calls with schema identity, validated arguments, correlation identity, output policy, mutation targets, required capabilities, and approval posture before dispatch.
- Gate tool dispatch through active trusted catalog or manifest definitions; unknown, inactive, untrusted, or executor-less tools fail before adapter calls.
- Add a pure tool dispatch plan equivalent in rigor to model dispatch planning, including capability resolution, approval, output-bound, mutation, cancellation, timeout, and no-fallback evidence.
- Make bounded result DTOs the runtime-visible tool result, with model-visible text, counts, BLAKE3 identity, redaction status, and host-owned full-output handles instead of raw full bodies.
- Require typed tool execution outcomes and mutation-window coordination before claiming completed tool effects.

## Impact

- **Breaking:** embedders using `ToolRequest { name, input }`, raw output responses, default broad tool capability, or argument compatibility shims must move to current structured tool definitions and calls.
- **Files:** `animus-adapters` tool DTOs, runtime admission and shell dispatch, runtime events/reports, Preserves/headless projections, TUI render snapshots, docs, focused Cairn rails, and tests.
- **Testing:** positive tests for admitted catalog-backed structured calls, bounded results, and independent mutation windows; negative tests for raw legacy payloads, unknown tools, inactive catalog entries, denied capabilities, missing executors, malformed arguments, unbounded output, unsafe references, cancellation, timeout, and conflicting mutations.

## Non-claims

This change does not add built-in product tools, provider discovery, plugin execution, filesystem mutation implementation, shell/network authority, or persistence. Hosts still own concrete executors, schema engines, storage handles, and permission prompts.