# Design: RLM host-effect executor

## Context

The RLM contract separates request admission from execution. The runtime seam intentionally supplies no concrete authority.

Existing host components cover useful mechanisms, but their DTOs and authority scopes differ. The RLM executor must compose them without duplicating their policy.

## Success Contract

One explicit executor object can process every admitted RLM operation class. Each route calls only its bound capability pack and returns one bounded application-owned result.

Denied, unavailable, stale, malformed, timed-out, cancelled, or over-limit requests produce typed failures. No route falls back to ambient authority.

A dispatch switch that directly uses `std::fs`, `std::process::Command`, environment lookup, or a global registry is false completion.

## Decisions

### 1. Use one visible composition component

A separate `animus-std-rlm-host` crate owns the application service. The crate composes narrow adapters at construction time.

The executor does not own the semantic policies of file paths, edits, commands, tools, context, children, or kernel state.

### 2. Bind each route explicitly

The route table is fixed by typed construction:

- `file_read` uses an explicit capability-root reader with lexical path admission and no-follow file access.
- `file_edit` delegates to the revision-gated workspace edit host.
- `shell` resolves an exact command profile and delegates to bounded process mechanics.
- `tool` delegates to an admitted tool-executor binding.
- `context_read` delegates to an explicit context resolver.
- `subagent` delegates to the injected `SubagentService` implementation.
- `kernel_clear` delegates to the kernel bridge owner and performs no external host lookup.

Missing bindings are construction or admission errors. They are not fallback signals.

### 3. Keep admission current

The executor receives an admitted request plus the current authority seal and host observations. It rechecks generation, cancellation, capability, target scope, and binding identity before the effect.

The executor does not repeat domain policy. It confirms that the supplied admission still matches current host facts.

### 4. Preserve result and error meaning

Each adapter result maps to an RLM result class without removing the source failure class. Bounded model-visible text is separate from safe receipt metadata.

Raw stdout, stderr, file bodies, tool bodies, child transcripts, credentials, and private paths do not enter receipts.

### 5. Keep mutation and process authority in a separate license boundary

If existing AGPL components are linked, the complete host crate uses `AGPL-3.0-or-later`. MIT or Apache reusable crates do not depend on it.

## Authority Flow

```text
admitted RLM request + current seal
  -> route selection
  -> route-specific current recheck
  -> narrow capability pack
  -> bounded observation
  -> typed RLM result
  -> safe receipt projection
```

## Alternatives Considered

### Put all effects in animus-runtime

Rejected because reusable runtime crates must not acquire operating-system authority.

### Implement direct standard-library effects in one switch

Rejected because this bypasses existing path, edit, process, tool, context, and child owners.

### Make missing packs use ambient defaults

Rejected because absence of an explicit binding is an authority denial.

## Risks and Controls

- Many routes can turn the component into a monolith. Each route stays a thin adapter over a separately owned capability pack.
- Error translation can hide important facts. The result retains route and source failure classes.
- Process and mutation code can widen licensing. The crate boundary and dependency checks keep coupling explicit.
- Context and output can leak. Separate model-visible bounds and metadata-only receipts apply.

## Non-Claims

This design does not establish sandboxing, hermetic execution, command authenticity, filesystem race freedom beyond component contracts, tool correctness, reversible mutation, task success, or production readiness.
