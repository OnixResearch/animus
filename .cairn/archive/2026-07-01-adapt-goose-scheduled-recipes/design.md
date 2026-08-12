# Design: Adapt Goose scheduled recipe workflows for Animus

Goose scheduled recipes are useful because they turn reusable workflows into recurring background jobs with visible session records. Animus should adapt the product shape after background queues and recipe trust exist, while keeping scheduling data-only in reusable core and host-owned in execution.

## Decisions

### 1. Schedules are workflow job descriptors

**Choice:** A schedule references an admitted recipe identity and version plus parameter values, cadence, timezone/calendar policy, execution mode, concurrency policy, workspace scope, retry bounds, and retention policy.

**Rationale:** This keeps recurrence separate from recipe semantics and makes schedule behavior reviewable.

### 2. Core validates plans but does not run timers

**Choice:** Reusable core validates schedule descriptors and computes next-run plans over explicit inputs. Host adapters own timers, queues, processes, and persistence.

**Rationale:** Scheduling is inherently effectful and must remain outside reusable runtime core.

### 3. Queue admission gates every run

**Choice:** Each scheduled run requires recipe trust, capability policy, sandbox policy, queue admission, and cancellation handles before execution begins.

**Rationale:** A trusted schedule should not become a permanent bypass for changing policy or workspace state.

### 4. Run records are safe ledger/evidence links

**Choice:** Run records link schedule ID, recipe BLAKE3 identity, parameter hashes, queue/job identifiers, session ledger references, retry counts, outcome, and safe evidence.

**Rationale:** Users need traceability without leaking raw prompt, secrets, paths, or tool output.

### 5. Mutating schedule controls are host-mediated

**Choice:** Pause, resume, edit, run-now, and cancel are typed control requests that require host admission and receipts.

**Rationale:** Schedule management changes future effects and must be audited like other protected actions.
