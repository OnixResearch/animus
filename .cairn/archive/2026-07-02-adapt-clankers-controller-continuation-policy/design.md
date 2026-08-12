# Design: Adapt Clankers controller continuation policy for Animus

## Context

After a model turn finishes, several follow-up paths may be possible: replay user input queued while busy, continue an explicit loop, run validation/auto-test work, or end. Clankers moved this into a deterministic controller policy. Animus should adopt the functional core of that approach before adding more workflow and headless automation behavior.

## Decisions

### 1. Continuation state is explicit

**Choice:** Represent queued prompts, loop work, validation follow-ups, busy prompt state, cancellation, and pending acknowledgements as typed continuation state.

**Rationale:** Hidden booleans and shell-local flags cause races between user input and automated follow-ups.

### 2. Decision priority is deterministic

**Choice:** Policy priority is queued user prompt first, active loop continuation second, validation/auto-test follow-up third, and done last unless policy config changes that order explicitly.

**Rationale:** User input should not be starved by automation, and automation should have predictable precedence.

### 3. Dispatch and completion use exact IDs

**Choice:** The shell must acknowledge accepted/rejected dispatch with the exact pending work ID and later complete the matching work ID. Stale, duplicate, missing, or mismatched IDs preserve state and emit typed failures.

**Rationale:** Follow-up outcomes must not clear unrelated queued work.

### 4. Shell owns effects

**Choice:** The pure continuation core returns decisions. Shell/runtime adapters perform provider calls, queue timers, persist session events, and update UI.

**Rationale:** The continuation policy should be testable without standing up the runtime.

### 5. Receipts are safe state summaries

**Choice:** Continuation receipts record prior state class, selected action, pending work ID hash, reason, rejection class, policy hash, and BLAKE3 receipt hash. Raw prompts, model outputs, tool outputs, full session transcripts, and provider payloads are excluded.

**Rationale:** Continuation evidence needs to explain control flow without leaking conversation contents.

## Risks / Trade-offs

- Fixed priority can be too rigid. Keep policy explicit and versioned if hosts need alternate ordering.
- ID matching can create more failures during integration. Negative tests should make those failures clear and recoverable.
- Continuation overlaps runtime events. Use event lifecycle for observation and continuation policy for state decisions.
