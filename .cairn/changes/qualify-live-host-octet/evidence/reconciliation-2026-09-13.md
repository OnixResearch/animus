# Parent reconciliation of the immutable audit

Status: bounded audit reviewed. Complete repair acceptance remains open.

## Audit ownership

The independent subject was `730904d8b3c4fcc496dc9e08f3ba4067074ba999`.
The report is `.pi/animus-host-octet-20260912/immutable-audit-report.md` in the primary pi-steel checkout.
The worker used 31 observed calls within its 40-call allowance.
The monitor observed the report and `agent_end`.
The parent stopped the remaining process and verified that its process group was empty.
The detached source remained unchanged.

This reconciliation is a correlated parent review, not another independent audit.
The worker received no continuation or second round.
Its conditional verdict does not establish acceptance.

## Recovery repair refusal

The report correctly traces a denied repair commit after a successful recovery read.
Its proposed expected result conflicts with the approved recovery contract:

> Recovery MUST refuse a kernel/session pair whose evidence does not prove safe continuation.

The interrupted-pair scenario expressly requires refusal before another tool or provider effect.
The read-only gate implements that policy before `resume()` can write a repair and clear `RepairRequired`.
Therefore, the refusal alone is not a product defect under this change.
An automatic repair needs a separate proof of safe continuation, not removal of this gate.

The paired controls distinguish an interrupted kernel commit from a completed session commit.
The first refuses reopening. The second reopens at `PersistentStatus::Idle`.
Both controls observe zero provider calls.
They do not prove recovery for arbitrary journal pairs or process crashes.
The additive owner gate retains the current-authority probe after successful recovery.

## Post-binding completion

The generic completion API can fail. Its return type alone does not prove a reachable failure in the fixed live composition.
The parent traced these concrete owners on the immutable subject:

- `src/assembly/stages.rs` validates the observation before binding or taking the pending stage.
- `crates/animus-std/src/toolexecutionstages.rs` keeps the pending plan and state private.
- Its `NoToolExecutionStageHandlers::observe` returns an exact admitted candidate without an external callback.
- `crates/animus-runtime/src/toolexecutionstages/core.rs` repeats the same pure durable observation over unchanged facts.
- That observation produces `Committed` with `PostCommitObservation` as the next phase.
- The exact candidate preserves identities, capabilities, mutation facts, bounds, redaction, and outcome class.
- The post-commit transition produces `Observed` with no next phase.
- `crates/animus-runtime/src/toolexecutionstages/receipt.rs` then checks terminal fields, identities, labels, and handler count.

For the live composition, `src/assembly/stages.rs` supplies an empty handler set.
`src/assembly/policy.rs::steel_tool_request` constructs the request with fixed Steel tool and schema labels.
`src/adapters/responses/planning.rs` advertises those same labels.
Their values are `steel_repl` and `animus.steel-repl.v1` in `crates/animus-adapter-rlm-kernel/src/lib.rs`.
Both values satisfy the inspected stage-receipt label predicate.
The plan constructor bounds handlers and validates authority and owner identities.
Final validation supplies the checked result and executor-receipt identities.
The initial durable observation validates the head identity and derives the commit identity.
The receipt projection supplies an empty sensitive draft and the fixed schema, marker, and non-claims.
These facts satisfy the inspected typed rejection branches for this constructed production state.

This argument is specific to the fixed live composition and ordinary successful allocation.
It does not make the generic completion API infallible.
The executor-kit and stage-receipt validators even use different private-marker sets.
A claim for arbitrary labels, arbitrary injected observers, or future handler composition needs new evidence.
The existing exact-observation control also completes once and rejects repetition as `Stale`.

## Correlation and duplicate observations

`CodingSessionHost::drive_one_pending_with_gate` reduces the matching effect outcome before it calls the tool observer.
`committed_tool_observation` derives the run, effect, and result fields from that commit request.
The adapter checks result and commit-owner correlation, head syntax, and synchronization before it consumes the stage.
The host owns run, effect, leaf, and cursor correlation.
It is not a standalone verifier for an arbitrary external observation envelope.
Its production contract depends on the owning host and the injected persistence contract.

The duplicate-binding control deliberately mutates private fixture state.
It proves rejection retention at the adapter boundary, not a reachable external attack.
The report found no production path that prebinds the same pending result twice.
The outer host also calls `block_after_committed_tool` on observer errors.

## Continuation controls

The existing direct-adapter controls cover snapshot refusal, provider failure, terminal refusal, and successful continuation.
The block precedes snapshot persistence, not only provider contact.
A retry returns `Stale` without another provider input or persistence record.

The outer-host fixtures already covered all three refusal classes and unchanged provider-call counts after another step.
The parent added explicit `PersistentStatus::Blocked` assertions before and after that step.
Task 6408 passed those fixtures, the paired recovery controls, strict selected-host Clippy, and workspace formatting.
The successful continuation case remains beside the negative cases.

## CLI and cleanup limits

The parent source search found argument acquisition and stdin reads only at the four binary roots in the changed host crates.
The private parsers accept explicit `RawArgs` and cursors.
`RawArgs::from_args()` still performs process argument acquisition. It is not an authority token.
The parser controls cover accepted grammar, safe refusal, and invalid UTF-8.
No reusable parser export or ambient argument reader was added.

The refused opening calls `close()`. A close error can propagate.
Neither this review nor the selected controls prove cleanup under every injected close failure.
They do not establish rollback, cross-journal atomicity, or reversal of an external effect.

## Strict-check follow-up

At `ac745f2cfceb1c7b4e04545c042dc867c3307188`, both host rails, the component catalog, and the facade inventory passed.
The full catalog returned zero provider findings and six agent findings, with zero warnings.
The combined check also returned six findings.
All six concern explicit owner paths for test imports in `recovery.rs` and `fixture/retries.rs`.
The parent qualified those uses without changing target selection or adding exemptions.
A new immutable strict result and full-flake coverage remain required.
