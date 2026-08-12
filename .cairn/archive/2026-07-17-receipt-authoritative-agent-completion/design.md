## Context

The Ralph Steel script decides completion with `string-contains?` over model output. Rust revalidates the resulting response DTO but has no completion-evidence input to distinguish an ordinary response from a verified final response. Separately, `ValidationResultStatus` has only `Passed`, `Failed`, and `Skipped`; `reduce_validation_follow_up` treats both `Passed` and `Skipped` as requiring no follow-up. Continuation policy selects `Complete` when its pending queues are empty, even though queue emptiness is not verification evidence.

Animus already has most of the safe pieces: typed process-job profile labels, pure scoped validation planning, validation follow-up intents, exact continuation acknowledgements, safe receipts, a Nickel-governed Steel profile, and host-owned effects. This change composes those pieces around an independent Cairn completion receipt and adopts Tile's repository-owned registry pattern without importing Tile runtime code or centralizing stack checks.

## Decisions

### 1. Repository-owned Nickel registry selects checks

**Choice:** Add an Animus-owned typed registry under `config/`, wired through `config/repo.ncl` and a deterministic generated runtime export. Each row records stable check ID, profile, exact argv, affected repository-relative paths/crates/surfaces, prerequisites, capability class, risk/tier, timeout and output-budget classes, expected artifacts, exact reproduction argv, documentation refs, and concrete positive and negative self-test contracts.

**Rationale:** Selection rules must be reviewable in the repository that owns the behavior. Animus will not infer a universal stack check list or copy Tile's registry contents.

### 2. Selection is pure and change-aware

**Choice:** Extend the validation planner with a pure function over a host-supplied changed-artifact snapshot, validated registry rows, explicit tier/capability observations, and policy. It returns selected required, not-selected, and blocked sets; reasons; prerequisite order; selection hash; and final subject hash. Unknown paths choose a conservative repository baseline rather than silently selecting nothing.

**Rationale:** The logic can be tested without VCS, while a shell may use jj/Git or explicit path input to produce the snapshot.

### 3. Host executes; reusable code validates evidence

**Choice:** Process execution remains behind injected host services. The shell manages process groups, finite deadlines, bounded output, cleanup, artifact persistence, and BLAKE3 hashing. Reusable DTOs carry only exact command identity, status, exit/signal/timeout/cleanup facts, bounded tails or artifact refs, output/evidence hashes, and final subject identity.

**Rationale:** Animus core and adapters remain authority-free while hosts can provide real backpressure.

### 4. Only `passed` is passing evidence

**Choice:** Expand result status to distinguish `passed`, `failed`, `blocked`, `skipped`, `deferred`, `timed_out`, `cancelled`, `dry_run`, `cleanup_failed`, `infrastructure_error`, `not_selected`, and `unknown`. Only `passed` can satisfy a required check. Every other required status produces a blocker or feedback intent; skipped no longer reduces to no follow-up.

**Rationale:** Missing capability and dry-run plans are informative but cannot be promoted into success.

### 5. Model completion becomes a candidate

**Choice:** Replace substring matching with a typed completion-request path. A compatibility `COMPLETE` marker is accepted only as an exact bounded request spelling, never as evidence. Steel returns a neutral completion candidate; Rust validates it and schedules required verification before any final response. Prose containing the token, such as `not COMPLETE`, cannot terminate the loop.

**Rationale:** Models may ask to finish, but they do not decide that repository policy is satisfied.

### 6. Cairn receipt is the final authority

**Choice:** The completion reducer receives a normalized Cairn validation summary from an injected host adapter. Completion is eligible only when the summary is current, passing, uses the supported authoritative schema, matches the Animus registry/selection/final-subject/policy hashes, covers every required check, and has no non-passing required status. The reusable core need not link to a process-capable Cairn library; the host may call exact-pinned Cairn CLI/MCP and pass the safe summary inward.

**Rationale:** Producer execution and readiness judgment remain separate while Animus stays composable.

### 7. Verification feedback uses typed continuation work

**Choice:** Non-passing completion results become bounded validation follow-up work with exact work IDs. Diagnostics include stable ID/code, check ID/status, subject/registry/selection/evidence/receipt hashes, bounded repository-relative source span, remediation label and summary, exact reproduction argv, safe artifact refs, severity, and non-claims. Host policy decides whether to dispatch another model turn, stop blocked, or await operator action; reusable reducers never retry, edit, or self-escalate automatically.

**Rationale:** This turns validation into useful backpressure without creating an unbounded autonomous repair loop.

### 8. Completion and continuation state are distinct

**Choice:** Queue emptiness may produce a `completion_evaluation_required` decision, not unconditional `Complete`, when a completion policy requires verification. Exact dispatch/completion acknowledgements remain unchanged. A passing receipt is bound to the same work/final-subject identity before the state can enter verified complete.

**Rationale:** No pending work means only that no work is currently queued.

### 9. Revalidation follows mutation

**Choice:** Any admitted mutation after check execution invalidates the prior final-subject hash, completion summary, and candidate. The next decision must re-plan or re-run all checks selected for the new subject. Failed-check follow-up may propose work but cannot carry an old passing receipt across a mutation.

**Rationale:** Completion evidence must describe what will actually be delivered.

### 10. Workspace supervisor consumes the same authority

**Choice:** The canonical workspace helper may continue to classify one-change task reduction as `task-progress`, but repository completion requires no active change plus fresh passing repository validation, gate/archive, and authoritative agent-completion receipt links for the final subject. Archive appearance, Pi exit zero, task checkbox state, or model text never substitutes for those links. Integration is a workspace-operator handoff, not ambient sibling mutation by Animus runtime code.

**Rationale:** Progress scheduling and completion authority answer different questions.

### 11. Exact reproduction is data, not a shell escape

**Choice:** Registry and diagnostic reproduction commands are validated argv arrays plus a safe working-root label. Raw `bash -c`, interpolated shell strings, credentials, private absolute paths, and unbounded environment maps are rejected.

**Rationale:** Agents need reproducibility without receiving a new command-injection surface.

## Risks / Trade-offs

- A maintained registry adds review and export work whenever checks move or their affected surfaces change.
- A failed or unavailable required check will stop completion more often than today's marker-based flow; that is the intended backpressure.
- The Cairn authoritative receipt change is an external prerequisite for final cross-tool integration. Animus can implement and fixture-test its local ports first, but it must not claim receipt-authoritative completion until the exact supported Cairn contract is pinned and exercised.
- Bounded diagnostics may omit context useful for humans; artifact references and exact reproduction argv provide the deliberate escape hatch.
- Passing evidence remains scoped. It does not prove the checks themselves are complete or sound, and it does not promote VM, Wayland, network, production, release, push, or pull-request authority.
