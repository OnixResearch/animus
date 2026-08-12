# Design: Add a ledger and session adapter seam

## Context

Animus already records provenance and session-related DTOs, but concrete persistence is intentionally not claimed. The requested Lego block is the seam: data contracts and injected service boundaries that let a host provide storage while reusable crates remain pure.

## Decisions

### 1. Storage is host-injected

**Choice:** Define `LedgerService` and session-store DTO boundaries as injected host services. Reusable crates validate descriptors, plan replay/restore, and validate receipts over typed inputs; they never discover global stores, read files, open databases, or write sessions.

**Rationale:** Persistence is host authority. Absence of a ledger service must not trigger ambient discovery.

### 2. Replay and restore planning are pure

**Choice:** Replay planning consumes already-loaded ledger entries, session descriptors, branch metadata, policy, and budgets. It returns model-visible context slices, omitted-entry reasons, and safe receipt drafts without performing I/O.

**Rationale:** Pure planning is testable and keeps hidden storage details out of reusable runtime logic.

### 3. Receipts use safe summaries only

**Choice:** Ledger/session receipts include session labels, branch labels, sequence ranges, counts, redaction status, freshness class, safe hashes, and omitted-surface non-claims. They reject raw transcripts, raw tool output, credentials, provider payloads, private paths, and unbounded terminal snapshots.

**Rationale:** Session evidence must support audit and restore decisions without becoming a prompt or secret store.

### 4. No default global store

**Choice:** This seam does not define `.animus`, `.clankers`, or any other global session directory as a default. Concrete storage layouts are separate host adapter changes.

**Rationale:** A reusable harness should not claim persistence behavior or mutate host state by default.

## Risks / Trade-offs

- A session seam can be mistaken for a persistence backend. Mitigate with explicit non-claims and missing-service tests.
- Existing provenance/session modules may overlap. Mitigate by making this seam a coordinator over existing DTOs instead of replacing them.
- Restore semantics can grow broad. Mitigate by limiting this change to planning, admission, service contracts, and safe receipts.
