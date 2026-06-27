# Design: Adapt Pi tool contracts for Animus

## Context

Pi tool definitions combine model-facing metadata, validation schemas, execution callbacks, custom rendering, and result details. Animus should adapt the safety and contract pieces while keeping rendering and concrete execution outside the core runtime.

The current `animus-adapters` tool request/response types are intentionally minimal. This change expands the boundary in a way that remains implementation-free and `no_std`-friendly where possible.

## Decisions

### 1. Tool definitions split model-facing metadata from host execution

**Choice:** Define a tool registration/definition DTO that includes safe metadata and capability labels, while execution remains an injected adapter/service.

**Rationale:** The model needs names, descriptions, schemas, and guidance. The runtime needs capability and source metadata. Neither needs to know how the host executes the tool.

### 2. Argument preparation precedes strict validation

**Choice:** Allow an optional pure compatibility shim to transform legacy or resumed arguments before strict schema validation.

**Rationale:** Long-lived ledgers can contain old tool-call shapes. Compatibility should not weaken the public schema or allow unknown inputs after preparation.

### 3. Tool output is always bounded

**Choice:** Require tool results to expose bounded model-visible text plus optional safe references, sizes, and BLAKE3 hashes for full output.

**Rationale:** Unbounded output can exhaust context and leak sensitive data. Hashes preserve identity without embedding bodies.

### 4. Mutation coordination is explicit

**Choice:** Tools that mutate shared resources must declare mutation targets so a shell-owned coordinator can serialize conflicting mutation windows.

**Rationale:** Pi's per-file mutation queue prevents lost updates when tools run in parallel. Animus should generalize that to resource keys without requiring filesystem knowledge in core.

### 5. Tool error classes are typed

**Choice:** Tool dispatch and execution failures use typed status classes instead of free-form strings.

**Rationale:** Runtime reports, receipts, and retry/abort behavior need precise failure classes.

## Risks / Trade-offs

- Rich schemas can pull in heavy dependencies. Keep the core contract schema-format-neutral and let adapters translate to JSON Schema, Preserves, or host-native validators.
- Mutation queues can become a filesystem API. Treat mutation targets as opaque resource keys supplied by shells or tools.
- Full-output references can leak paths. Require safe relative/display references or host-owned handles, plus hashes and redaction markers.
