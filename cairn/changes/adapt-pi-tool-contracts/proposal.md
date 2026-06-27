# Proposal: Adapt Pi tool contracts for Animus

## Why

Pi's tool system has several durable ideas worth carrying into Animus: strict parameter schemas, prompt snippets and tool-specific guidance, source/provenance metadata, argument compatibility shims, output truncation, per-file mutation queues, typed result details, and fail-safe error signaling. Animus already has simple model/tool adapter DTOs, but it needs richer contracts before the runtime can safely support swappable host tools.

Adapting these patterns will make tools safer, more observable, and easier for embedders to register without pushing concrete tool implementations into the reusable runtime.

## What Changes

- Define richer tool definitions with name, label, description, parameter schema reference, prompt snippet, prompt guidelines, source metadata, capability labels, and result metadata shape.
- Add argument preparation and strict post-preparation validation for backward-compatible resumed calls.
- Add truncation requirements for large tool results, including bounded visible content, safe full-output references, and size/hash metadata.
- Add mutation serialization for tools that write the same resource, with per-resource queues owned by the shell or injected mutation coordinator.
- Add typed tool errors that distinguish invalid input, unavailable executor, denied capability, cancellation, malformed output, and execution failure.

## Impact

- **Files**: adapter/tool DTOs, schema/metadata types, runtime dispatch integration, output truncation helpers, mutation coordination tests, and docs.
- **Testing**: positive tests for valid tools and bounded output; negative tests for bad schemas, malformed arguments, oversized results, denied capabilities, missing executors, and concurrent mutation conflicts.
- **Non-claims**: this change does not add Pi's built-in tools wholesale, TUI renderers, TypeBox schemas, npm extension tools, or filesystem mutation implementations in core.
