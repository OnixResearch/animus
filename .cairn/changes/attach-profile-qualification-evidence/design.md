## Context

Baseline: Animus `4071c65e`. The existing building blocks are:

- `animus-runtime::eval_feedback` validates offline evaluation plans and redacted receipts. Its freshness check covers model route, scaffold, verifier, trust boundary, tool surface, runtime version, and sampling.
- `animus-runtime::harbor_eval` and `animus-eval-core` plan trial matrices, reduce verifier rewards, and project metadata-only receipts.
- The campaign-manager lane (`crates/animus-campaign-manager-eval`) runs scripted routes through that slice with a separate history-replay verifier.

Lattice (`9a74e7ef`) vendors `animus-runtime` with `default-features = false, features = ["steel"]`. Under that feature set `serde` and `serde_json` are optional and not enabled. Lattice spells the definition identity `blake3:<hex>` (`lattice-worker-definition-v1`) and the profile revision as bare 64 hex (`worker_profile_identity()`). Lattice is narrowing the revision to profile-level configuration, so it no longer covers per-attempt prompts or workspace snapshots.

## Decisions

### Decision: Pure module beside `eval_feedback`

**Choice:** `animus_runtime::qualification` lives in `crates/animus-runtime/src/eval_feedback/qualification/`. It compiles under every feature set and uses only `blake3` and `alloc`.

**Rationale:** Lattice's existing vendored dependency reaches it. No leaf crate, no `sync_animus.rs` crate-list change, and no new dependency edge are needed. `cargo check -p animus-runtime --no-default-features --features steel` proves it.

### Decision: Canonical JSON subset with a hand-written codec

**Choice:** The subset has no whitespace, no escapes, sorted unique keys, unsigned integers, bounded strings and lists, and depth at most 6. Decoding re-encodes and requires byte equality. The identity is `blake3:` + hex of `blake3::hash(canonical)`.

**Rationale:** The format stays readable to Kiln and catalog consumers, each document has exactly one spelling, and `serde_json` is unavailable in the consumer feature set.

### Decision: Strings are tokens or identities only

**Choice:** Every string field is either a Lattice capability-name token (`[a-z0-9][a-z0-9._:-]*`, at most 64 bytes, with no credential markers) or a `blake3:` identity.

**Rationale:** This is redaction by construction. A raw prompt, hidden test, provider body, credential, or path cannot fit in the schema.

### Decision: Closed component vocabulary with explicit basis

**Choice:** The subject lists all 25 component kinds in wire order. Each is `declared`, `observed`, or `unavailable`, and an unavailable component carries no identity. Evaluation and conformance evidence must bind its protocol, verifier, and corpus to observed components.

**Rationale:** Changing any component is detectable. A missing identity is explicit and is never an empty hash.

### Decision: Strict port validator, separate applicability classifier

**Choice:** `validate_attachment` qualifies only when the subject keys are equal. The checks run in stage order (malformed, subject, scope, class, unusable, stale), and `qualify` reports the furthest-stage reason. `classify_applicability` compares the complete current subject. It returns `Current`, `Historical`, `CompatibleReuse` (under an exact scoped decision that never waives evaluator components), `Unusable`, or `Inapplicable`.

**Rationale:** Lattice's contract allows `Qualified` only for exact subject keys. Cross-revision reuse is reviewable but can never strengthen a claim or turn into a qualified answer.

### Decision: Conflicts are scoped to one subject and source

**Choice:** Exact duplicates collapse. Distinct valid attachments that share a source identity, class, subject keys, and task family are `conflicting`.

**Rationale:** One job produces attachments for several routes (subjects). Those attachments share a source but do not conflict.

### Decision: The producer reuses the campaign-manager lane

**Choice:** `task_family_attachment(set, report, binding)` replans the same task-major matrix to attribute trials to routes. It counts accepted completions from the separate verifier's reduced `found` metric, maps host failures to failed, cancelled, or incomplete, records costs as unknown, and takes subject keys and freshness from a host binding.

**Rationale:** Issue #4's coding-worker lane does not exist at `4071c65e`. The campaign-manager lane is the existing native-evaluation fixture lane, and it has a separate verifier and checked tasks. No trial runs twice and no second engine exists. The fixture's Lattice keys are labelled fixture keys: one revision per route covers every run of that route, and the job identity enters only the source identity.

## Risks / Trade-offs

- The canonical codec is hand-written. Re-encoding and byte equality bound its behavior, and the negative tests cover whitespace, escapes, leading zeros, unknown fields, and schema drift.
- The scripted lane exposes no prompt, tool-implementation, or control identities, so those components are `unavailable`. A consumer that requires them gets `unusable`, which is the intended outcome.
- Accepted counts are derived from the reduced mean of a binary metric. The producer refuses inexact counts rather than rounding them.
- The fixture Lattice keys are synthetic. A real host binding supplies `worker_definition` and `worker_profile_identity()` values.
