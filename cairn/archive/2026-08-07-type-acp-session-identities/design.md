# Design: Type ACP session identities

## Context

`animus-acp-stdio-host/src/model.rs` is transport-neutral data for the bounded ACP host. It uses different hash domains for connections, correlations, sessions, workspaces, and receipts.

The model still stores most resulting identities as strings. Runtime checks can detect some mismatches, but the Rust compiler cannot distinguish the categories.

## Decisions

### Decision: Classify ACP semantic strings

**Choice:** Inventory fields and signatures as protocol ID, Animus ID, BLAKE3 identity, external reference, display text, protocol constant, or deferred.

Only identity and reference categories enter this change.

**Rationale:** ACP methods, labels, errors, and protocol versions are text, not nominal domain identities.

### Decision: Add exact private types

**Choice:** Add private types for `AcpCorrelationId`, `AcpSessionId`, `AnimusSessionId`, `SessionIdentity`, `AuthorityProfileIdentity`, `AcpRunId`, `AcpEntryId`, `AcpConnectionIdentity`, `WorkspaceIdentity`, `ResourceIdentity`, and `SessionReference`.

Reuse one private BLAKE3 parser through marker roles for Animus-owned hashes. Keep external ACP lexical IDs in separately checked wrappers.

**Rationale:** External IDs and content identities have different grammar and ownership.

### Decision: Separate frame decoding from admission

**Choice:** Parse JSON-RPC frames into raw bounded DTOs. Convert supported requests into typed `AcpDecodedRequest` values before state transitions or facade commands.

Unsupported method diagnostics can retain raw safe method text and optional raw correlation metadata.

**Rationale:** Protocol compatibility remains structural. Internal execution receives admitted roles.

### Decision: Keep state and facade commands typed

**Choice:** Migrate session bindings, connection state, pending and completed correlations, facade commands, observations, updates, permission requests, cancellation, and receipts.

Do not convert typed values back to strings until JSON-RPC output or receipt projection.

**Rationale:** The state machine and facade boundary are the highest-risk exchange points.

### Decision: Preserve receipt and frame identity

**Choice:** Capture baseline frame JSON, safe event projections, receipt canonical material, and BLAKE3 receipt refs. Require identical outputs after migration.

**Rationale:** New Rust types must not change the reviewed ACP cohort contract.

### Decision: Compile-test category separation

**Choice:** Add compile-fail fixtures for ACP/Animus session, session/connection, correlation/run, run/entry, workspace/resource, session/authority-profile, and receipt/session identity substitutions.

**Rationale:** These fixtures demonstrate the compiler guarantee that runtime tests cannot provide.

## Functional core and shell boundary

Frame admission, transition reduction, permission decisions, cancellation decisions, safe event projection, and receipt construction remain pure over in-memory values.

Stdio reads and writes, JSON framing, provider calls, credentials, processes, clocks, and persistence remain in host shells.

## Test design

Positive tests cover valid construction, request admission, initialize, create, load, prompt, updates, permission, cancel, terminal failure, receipt projection, and fixture-client conformance.

Negative tests cover malformed and oversized IDs, wrong session ownership, unknown correlations, stale runs, authority mismatches, duplicate completions, unsafe frames, compile-time substitutions, and receipt drift.

## Risks and trade-offs

- ACP uses externally selected ID strings. Wrappers check bounds and syntax but do not claim global uniqueness.
- Rust API changes can affect local tests. Builders and compatibility accessors reduce churn.
- Some reports need heterogeneous IDs. Closed enums keep their roles explicit.
- Typed authority-profile identity does not grant permission. Existing permission checks remain.

## Claim boundary

The change prevents selected internal identity exchanges and centralizes ACP admission. It does not prove authorization, provider behavior, session replay equivalence, arbitrary client compatibility, or release eligibility.
