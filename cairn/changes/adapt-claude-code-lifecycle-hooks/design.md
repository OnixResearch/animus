# Design: Adapt Claude Code lifecycle hooks for Animus

## Context

Claude Code hooks can trigger behavior around tool calls and lifecycle events. The transferable idea is an explicit extension seam. Animus needs that seam because embedders will want formatting, validation, notifications, provenance observers, and policy-specific checks.

Hooks are risky if they become hidden shell execution or a way to modify state after validation. Animus should express hook points, inputs, outputs, and policies as typed contracts with shell-owned execution.

## Decisions

### 1. Hook points are enumerated runtime events

**Choice:** Define a stable hook-point vocabulary for session start/end, prompt assembly, model request/response, tool pre/post, mutation pre/post, validation, receipt emission, and workflow completion.

**Rationale:** A fixed vocabulary lets policy reason about hooks and lets tests prove sensitive lifecycle events are covered without discovering arbitrary names at runtime.

### 2. Hook payloads carry safe metadata only

**Choice:** Hook events expose IDs, classes, redacted resource labels, sizes, hashes, status, and bounded summaries. Raw prompts, provider payloads, credentials, full paths, and unbounded outputs are excluded by default.

**Rationale:** Hooks may be implemented by third-party adapters. They should not receive more sensitive data than their declared purpose requires.

### 3. Hook execution is an injected shell concern

**Choice:** Reusable runtime code validates hook descriptors and emits hook requests. Hosts own process execution, network calls, clocks, timeout enforcement, and adapter-specific behavior.

**Rationale:** This preserves the `no_std` reusable core and keeps side effects in the imperative shell.

### 4. Hook outcomes cannot bypass revalidation

**Choice:** Pre-action hooks may observe, annotate, deny, or request revalidation. Any modified action proposal must pass the normal DTO, permission, capability, and tool validation path again.

**Rationale:** Hooks should be extensibility points, not unreviewed authority channels.

## Risks / Trade-offs

- Hooks can increase latency and nondeterminism. Keep timeout/cancellation status in shell policy and receipts.
- Hook payloads can accidentally leak sensitive context. Use redaction tests and safe-by-default DTOs.
- Too many hook points can freeze internals. Start with lifecycle-level events, not every internal function boundary.
